use aws_sdk_s3::{Client, Error};
use pyo3::{exceptions::PyValueError, prelude::*, types::PyDict};
use tokio::runtime::Runtime;

#[pyclass]
pub struct S3OpsRust {
    client: Client,
}

#[pymethods]
impl S3OpsRust {
    #[new]
    fn new(_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let rt = Runtime::new().unwrap();
        let client = rt.block_on(async {
            let shared_config = aws_config::load_from_env().await;
            S3OpsRust {
                client: Client::new(&shared_config),
            }
        });

        Ok(client)
    }

    fn list_buckets(&self) -> PyResult<Vec<(String, String)>> {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(async {
            let result = list_buckets_internal(&self.client).await;
            result.map_err(|err| PyValueError::new_err(err.to_string()))
        });
        result
    }
}
async fn list_buckets_internal(client: &Client) -> Result<Vec<(String, String)>, Error> {
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();

    let mut bukcet_locations = Vec::new();
    for bucket in buckets {
        let bucket_name = bucket.name().unwrap_or_default();
        let resp = client
            .get_bucket_location()
            .bucket(bucket_name)
            .send()
            .await?;
        let location = String::from(resp.location_constraint().unwrap().clone().as_str());
        bukcet_locations.push((String::from(bucket_name), location));
    }

    Ok(bukcet_locations)
}

/// A Python module implemented in Rust.
#[pymodule]
fn s3_ops_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<S3OpsRust>()?;
    Ok(())
}
