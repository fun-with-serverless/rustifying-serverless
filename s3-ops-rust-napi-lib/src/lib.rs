use aws_config::BehaviorVersion;
use aws_sdk_s3::{types::BucketLocationConstraint, Client, Error};
use futures::{stream::FuturesUnordered, StreamExt};
use napi_derive::napi;
use tokio::runtime::Runtime;
use aws_credential_types::provider::ProvideCredentials;

#[napi(object)]
pub struct BucketDetails {
  pub name: String,
  pub location: String,
}

#[napi]
pub struct S3OpsRust {
  client: Client,
}

#[napi]
impl S3OpsRust {
  #[napi(constructor)]
  pub fn new() -> Self {
    let rt = Runtime::new().unwrap();
    let client = rt.block_on(async {
      let shared_config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;
      S3OpsRust {
        client: Client::new(&shared_config),
      }
    });

    client
  }

  #[napi]
  pub fn list_buckets(&self) -> napi::Result<Vec<BucketDetails>> {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async {
      let result = list_buckets_internal(&self.client).await;
      result.map_err(|err| napi::Error::from_reason(err.to_string()))
    });
    result
  }
}
async fn list_buckets_internal(client: &Client) -> Result<Vec<BucketDetails>, Error> {
  let resp = client.list_buckets().send().await?;
  let buckets = resp.buckets();
  let tasks = FuturesUnordered::new();
  for bucket in buckets {
    let bucket_name = bucket.name().unwrap_or_default();
    let task = client.get_bucket_location().bucket(bucket_name).send();
    let wrapped_task = async move {
      let result = task.await;
      match result {
        Ok(response) => {
          let location = response
            .location_constraint()
            .unwrap_or(&BucketLocationConstraint::UsWest1)
            .as_str();
          Ok(BucketDetails {
            name: String::from(bucket_name),
            location: String::from(location),
          })
        }
        Err(e) => Err(e),
      }
    };

    tasks.push(wrapped_task);
  }
  let results = tasks.collect::<Vec<_>>().await;
  let bucket_locations: Vec<BucketDetails> = results.into_iter().filter_map(Result::ok).collect();

  Ok(bucket_locations)
}

#[napi]
pub async fn sign(host: String, url: String) -> napi::Result<String> {
  let config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;
  let credentials = config.credentials_provider();
  if let Some(credentials) = credentials {
    let credentials = credentials.provide_credentials().await.expect("Err");

    let datetime = chrono::Utc::now();
    let mut headers = http::header::HeaderMap::new();

    headers.insert(
      "X-Amz-Date",
      datetime
        .format("%Y%m%dT%H%M%SZ")
        .to_string()
        .parse()
        .unwrap(),
    );
    headers.insert("host", host.parse().unwrap());

    let s = aws_sign_v4::AwsSign::new(
      "GET",
      url.as_str(),
      &datetime,
      &headers,
      "us-east-1",
      credentials.access_key_id(),
      credentials.secret_access_key(),
      "execute-api",
      "",
    );
    let signature = s.sign();
    Ok(signature)
  } else {
    Err(napi::Error::from_reason("No credentials found"))
  }
}
