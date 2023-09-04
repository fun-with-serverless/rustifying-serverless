from typing import List
from aws_lambda_powertools import Logger
from aws_lambda_powertools.event_handler import APIGatewayRestResolver
import boto3

logger = Logger()
app = APIGatewayRestResolver()
s3_client = boto3.client("s3")

@logger.inject_lambda_context(log_event=True)
def lambda_handler(event, context):
    return app.resolve(event, context)


@app.get("/buckets-python")
def get_buckets_python() -> List[dict]:
    s3_buckets = s3_client.list_buckets()
    print(s3_buckets)
    bucket_info_list = []

    for bucket in s3_buckets["Buckets"]:
        bucket_name = bucket["Name"]
        bucket_region = (
            s3_client.get_bucket_location(Bucket=bucket_name)["LocationConstraint"]
            or "us-east-1"
        )

        bucket_info_list.append({"name": bucket_name, "region": bucket_region})

    return bucket_info_list
