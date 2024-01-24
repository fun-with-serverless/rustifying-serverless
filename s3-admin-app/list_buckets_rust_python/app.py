from typing import List
from aws_lambda_powertools import Logger
from aws_lambda_powertools.event_handler import APIGatewayRestResolver
import s3_ops_rust

logger = Logger()
app = APIGatewayRestResolver()

s3_client = s3_ops_rust.S3OpsRust()

# @logger.inject_lambda_context(log_event=True)
def lambda_handler(event, context):
    return app.resolve(event, context)


@app.get("/buckets-rust")
def get_buckets_rust() -> List[dict]:
    return s3_client.list_buckets()

