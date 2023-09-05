use anyhow::{Context, Result};
use aws_lambda_events::apigw::{
    ApiGatewayCustomAuthorizerPolicy, ApiGatewayCustomAuthorizerRequestTypeRequest,
    ApiGatewayCustomAuthorizerResponse, IamPolicyStatement,
};
use aws_sdk_secretsmanager::Client;
use base64::decode;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::option::Option;
use std::str;
use std::sync::{Mutex, Arc, MutexGuard};

const DEFAULT_USER: &'static str = "admin";
const SECRET_AUTH_PARAM_NAME_ENV_VARIABLE: &'static str = "SECRETAUTH_PARAM_NAME";

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(tracing::Level::INFO)
        .init();

    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let cache = Arc::new(Mutex::new(HashMap::<String, String>::new()));
    let func = service_fn(move |event| {
        let client_ref = client.clone();
        let cache_ref = cache.clone();
        async move { function_handler(event, &client_ref, &cache_ref).await }
    });

    lambda_runtime::run(func).await?;
    Ok(())
}

async fn function_handler(
    event: LambdaEvent<ApiGatewayCustomAuthorizerRequestTypeRequest>,
    client: &Client,
    cache: &Arc<std::sync::Mutex<HashMap<std::string::String, std::string::String>>>,
) -> Result<ApiGatewayCustomAuthorizerResponse, Error> {
    let method_arn = event
        .payload
        .method_arn
        .as_ref()
        .context("Method ARN is missing")?;
    let cache = cache.lock().unwrap();
    if let Some(header_value) = event.payload.headers.get("authorization") {
        if let Ok(token_str) = str::from_utf8(header_value.as_bytes()) {
            let user_result = get_user_by_token(token_str, client, cache).await?;
            let user = user_result.unwrap_or(String::from("NoUser"));
            let policy = if user == "NoUser" { "DENY" } else { "ALLOW" };
            return Ok(custom_authorizer_response(policy, &user, method_arn));
        }
    }

    Ok(custom_authorizer_response("DENY", "NoUser", method_arn))
}

fn custom_authorizer_response(
    effect: &str,
    principal: &str,
    method_arn: &str,
) -> ApiGatewayCustomAuthorizerResponse {
    let stmt = IamPolicyStatement {
        action: vec!["execute-api:Invoke".to_string()],
        resource: vec![method_arn.to_owned()],
        effect: Some(effect.to_owned()),
    };
    let policy = ApiGatewayCustomAuthorizerPolicy {
        version: Some("2012-10-17".to_string()),
        statement: vec![stmt],
    };
    ApiGatewayCustomAuthorizerResponse {
        principal_id: Some(principal.to_owned()),
        policy_document: policy,
        context: json!({ "user": principal }),
        usage_identifier_key: None,
    }
}

async fn get_user_by_token(token: &str, client: &Client, mut cache: MutexGuard<'_, HashMap<String, String>>) -> Result<Option<String>, Error> {
    if token.starts_with("Basic") {
        let decoded_auth = decode(&token[6..]).unwrap();
        let decoded_str = String::from_utf8(decoded_auth).unwrap();
        let split: Vec<&str> = decoded_str.split(":").collect();
        let (username, password) = (split[0], split[1]);

        let login_user = env::var("LOGIN_USER").unwrap_or(DEFAULT_USER.to_string());
        let secret_param = env::var(SECRET_AUTH_PARAM_NAME_ENV_VARIABLE).unwrap();
        
        let mut secret = cache.get(SECRET_AUTH_PARAM_NAME_ENV_VARIABLE).cloned();

        if secret.is_none() {
            // Fetch secret asynchronously and update the cache
            let fetched_secret = client.get_secret_value().secret_id(&secret_param).send().await?;
            secret = fetched_secret.secret_string.clone();
            
            if let Some(s) = secret.clone() {
                cache.insert(SECRET_AUTH_PARAM_NAME_ENV_VARIABLE.to_string(), s);
            }
        }

        if username == login_user && Some(password) == secret.as_deref() {
            return Ok(Some(login_user));
        }
    }
    Ok(None)
}
