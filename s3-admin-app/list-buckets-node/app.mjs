import { S3Client, ListBucketsCommand, GetBucketLocationCommand } from "@aws-sdk/client-s3"

const client = new S3Client();
const command = new ListBucketsCommand({});

const getBucketRegion = async (bucket) => {
    const bucketName = bucket.Name;
    const command = new GetBucketLocationCommand({ Bucket: bucketName });

    try {
        const response = await client.send(command);
        const region  = response.LocationConstraint || "us-east-1";
        return [bucketName, region]
    } catch (error) {
        console.error(`Error getting region for bucket ${bucketName}:`, error);
        throw error;
    }
}

export const lambdaHandler = async (event, context) => {

    try {
        const response = await client.send(command);
        const regionPromises = response.Buckets.map(getBucketRegion);
        const results = await Promise.all(regionPromises);
        return {
            statusCode: 200,
            body: JSON.stringify(results),
        }        
    } catch (err) {
        console.log(err);
        return err;
    }
};
