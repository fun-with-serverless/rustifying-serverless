from typing import List

class S3OpsRust:
    def list_buckets() -> List[(str, str)]: 
        """
        List available buckets and their respective regions in the AWS account.
        """
        ...
