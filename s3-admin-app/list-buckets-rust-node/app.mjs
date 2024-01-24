import {S3OpsRust} from "./index.js"


export const lambdaHandler = async (event, context) => {

    try {
        const s3ops = new S3OpsRust();
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
