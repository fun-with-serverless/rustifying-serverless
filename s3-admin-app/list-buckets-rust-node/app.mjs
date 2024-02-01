import { S3OpsRust, sign } from "./index.js"

const s3ops = new S3OpsRust();

export const lambdaHandler = async (event, context) => {

    try {
        const results = s3ops.listBuckets();
        const query = results.map(bucket => {sign("cwferfwfwe.execute-api.aws.com", "https://cwferfwfwe.execute-api.aws.com/dev/list-buckets-rust-node")})
        const signedRequest = await Promise.all(query);
        
        return {
            statusCode: 200,
            body: JSON.stringify({
                "buckets": results,
                signedRequest,
            }),
        }
    } catch (err) {
        console.log(err);
        return err;
    }
};
