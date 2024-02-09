use aws_config::BehaviorVersion;
use aws_sdk_s3::{types::BucketLocationConstraint, Client, Error};
use futures::{stream::FuturesUnordered, StreamExt};
use napi_derive::napi;
use tokio::runtime::Runtime;

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
