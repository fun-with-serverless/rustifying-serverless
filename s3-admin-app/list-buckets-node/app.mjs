import { S3Client, ListBucketsCommand, GetBucketLocationCommand } from "@aws-sdk/client-s3"
import { SignatureV4 } from "@smithy/signature-v4"
import { HttpRequest } from "@smithy/protocol-http"
import { defaultProvider } from "@aws-sdk/credential-provider-node"
import { Sha256 } from "@aws-crypto/sha256-browser"

const client = new S3Client();
const command = new ListBucketsCommand({});

const getBucketRegion = async (bucket) => {
    const bucketName = bucket.Name;
    const command = new GetBucketLocationCommand({ Bucket: bucketName });

    try {
        const response = await client.send(command);
        const region = response.LocationConstraint || "us-east-1";
        return [bucketName, region]
    } catch (error) {
        console.error(`Error getting region for bucket ${bucketName}:`, error);
        throw error;
    }
}

async function makeSignedRequest(host, path, method) {
    const credentials = defaultProvider();

    const request = new HttpRequest({
        headers: {
            "Content-Type": "application/json",
            Host: host,
        },
        hostname: host,
        method: method,
        path: path,
        body: JSON.stringify({}),
    });

    const signer = new SignatureV4({
        credentials: credentials,
        region: "us-east-1",
        service: "execute-api",
        sha256: Sha256,
    });

    const signedRequest = await signer.sign(request);

    return signedRequest;
}

export const lambdaHandler = async (event, context) => {

    try {
        const response = await client.send(command);
        const regionPromises = response.Buckets.map(getBucketRegion);
        const results = await Promise.all(regionPromises);
        const query = results.map(bucket => {makeSignedRequest("cwferfwfwe.execute-api.aws.com", "dev/list-buckets-rust-node", "GET")})
        const signedRequest = await Promise.all(query);
        return {
            statusCode: 200,
            body: JSON.stringify({ "buckets": results, signedRequest }),
        }
    } catch (err) {
        console.log(err);
        return err;
    }
};
