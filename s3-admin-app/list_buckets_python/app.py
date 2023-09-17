import asyncio
from typing import List, Tuple
from aws_lambda_powertools import Logger
from aws_lambda_powertools.event_handler import APIGatewayRestResolver
import boto3
from aiobotocore.session import get_session
from aiobotocore.config import AioConfig
import uvloop

logger = Logger()
app = APIGatewayRestResolver()
s3_client = boto3.client("s3")


@logger.inject_lambda_context(log_event=True)
def lambda_handler(event, context):
    return app.resolve(event, context)


@app.get("/buckets-python")
def get_buckets_python() -> List[dict]:
    s3_buckets = s3_client.list_buckets()
    bucket_info_list = []

    with asyncio.Runner(loop_factory=uvloop.new_event_loop) as runner:
        bucket_info_list = runner.run(get_buckets_region(s3_buckets["Buckets"]))

    return bucket_info_list


async def get_buckets_region(buckets: List[str]) -> List[Tuple[str, str]]:
    session = get_session()
    bucket_info_list = []
    
    async with session.create_client("s3", config=AioConfig(retries={"max_attempts": 0}) ) as s3_client:
        tasks = [
            get_bucket_info(s3_client, bucket["Name"]) for bucket in buckets
        ]

        # Run the batch of tasks and collect results
        results = await asyncio.gather(*tasks)

    # Add results to bucket_info_list
    bucket_info_list.extend(results)

    return bucket_info_list


async def get_bucket_info(s3_client, bucket_name: str):
        bucket_location = await s3_client.get_bucket_location(Bucket=bucket_name)
        region = bucket_location["LocationConstraint"] or "us-east-1"
        return {"name": bucket_name, "region": region}
