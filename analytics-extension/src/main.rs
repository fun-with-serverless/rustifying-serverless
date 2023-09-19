use std::sync::Arc;

use aws_sdk_sqs::Client;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use lambda_extension::{service_fn, Error, LambdaEvent, NextEvent};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    task,
};
use tracing::{debug, error};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    pub sender: Sender<AnalyticsData>,
}

#[derive(Deserialize, Serialize, Debug)]
struct AnalyticsData {
    event: String,
}

async fn my_extension(
    client: Client,
    receiver: Arc<Mutex<Receiver<AnalyticsData>>>,
    event: LambdaEvent,
) -> Result<(), Error> {
    let queue_url = std::env::var("ANALYTICS_SQS_URL").expect("Missing ANALYTICS_SQS_URL env!");
    match event.next {
        NextEvent::Shutdown(_e) => {
            debug!(target: "extension","Shutdown Event");
        }
        NextEvent::Invoke(_e) => {
            debug!(target: "extension","Invoke Event");
        }
    }
    let mut rx = receiver.lock().await;
    while let Ok(message) = rx.try_recv() {
        debug!(target: "extension","GOT = {:?}", message);
        // Convert your data to payload, serialize it to JSON string for example
        let payload = serde_json::to_string(&message).unwrap();

        // Send the message to SQS
        let response = client
            .send_message()
            .queue_url(queue_url.clone())
            .message_body(payload)
            .send()
            .await;

        if let Err(error) = response {
            error!(target: "extension","ERROR = {:?}", error)
        }
    }
    Ok(())
}

async fn post_message(
    State(state): State<AppState>,
    Json(payload): Json<AnalyticsData>,
) -> StatusCode {
    debug!(target: "extension","Sending = {:?}", payload);
    // Send a message to the queue
    state.sender.send(payload).await.unwrap();

    StatusCode::OK
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .without_time()
        .init();

    debug!(target: "extension", "Starting extension");
    let client = Client::new(&aws_config::load_from_env().await);
    let (sender, receiver) = channel(100);
    let state = AppState { sender: sender };
    let route = Router::new()
        .route("/v1/analytics", post(post_message))
        .with_state(state);

    let server =
        axum::Server::bind(&"0.0.0.0:3001".parse().unwrap()).serve(route.into_make_service());
    task::spawn(async move {
        server.await.unwrap();
    });

    let rec = Arc::new(Mutex::new(receiver));
    let func = |event| my_extension(client.clone(), rec.clone(), event);
    let func = service_fn(func);
    lambda_extension::run(func).await
}
