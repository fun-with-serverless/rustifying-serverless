from typing import Optional
from aws_lambda_powertools import Logger
from aws_lambda_powertools.utilities.data_classes import event_source
import base64
from aws_lambda_powertools.utilities import parameters

import os
from aws_lambda_powertools.utilities.data_classes.api_gateway_authorizer_event import (
    DENY_ALL_RESPONSE,
    APIGatewayAuthorizerRequestEvent,
    APIGatewayAuthorizerResponse,
    HttpVerb,
)

logger = Logger()
USER = "admin"


@logger.inject_lambda_context(log_event=True)
@event_source(data_class=APIGatewayAuthorizerRequestEvent)
def lambda_handler(event: APIGatewayAuthorizerRequestEvent, context):
    user = get_user_by_token(event.get_header_value("authorization"))

    if user is None:
        # No user was found, so we return not authorized
        return DENY_ALL_RESPONSE

    # Found the user and setting the details in the context
    arn = event.parsed_arn
    policy = APIGatewayAuthorizerResponse(
        principal_id=user,
        context={"user":user},
        region=arn.region,
        aws_account_id=arn.aws_account_id,
        api_id=arn.api_id,
        stage=arn.stage,
    )

    policy.allow_all_routes()
    logger.info("Policy for user", policy=policy.asdict())
    return policy.asdict()


def get_user_by_token(token: Optional[str]) -> Optional[str]:
    if token and token.startswith("Basic"):
        logger.info("Basic auth arrived", auth_details=token)
        decoded_auth = base64.b64decode(token[6:]).decode()
        username, password = decoded_auth.split(":")

        # Check if the username and password match the expected values
        if username == os.environ.get(
            "LOGIN_USER", USER
        ) and password == parameters.get_secret(os.environ["SECRETAUTH_PARAM_NAME"]):
            logger.info("User logged in")
            return USER
    logger.info("User not found")
    return None
