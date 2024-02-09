import { S3OpsRust } from "./index.js"

const s3ops = new S3OpsRust();

export const lambdaHandler = async (event, context) => {

    try {
        const results = s3ops.listBuckets();
        return {
            statusCode: 200,
            body: JSON.stringify(results),
        }
    } catch (err) {
        console.log(err);
        return err;
    }
};
