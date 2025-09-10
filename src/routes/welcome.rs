use axum::{Json, Router, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct IncomingGreet {
    name: String,
}

#[derive(Serialize)]
struct OutcomingGreet {
    text: String,
}

async fn welcome(Json(payload): Json<IncomingGreet>) -> (StatusCode, Json<OutcomingGreet>) {
    let greeting_text = format!("Hello {}", payload.name);
    let response = OutcomingGreet {
        text: greeting_text,
    };

    (StatusCode::ACCEPTED, Json(response))
}

pub fn welcome_router() -> Router {
    Router::new().route("/welcome", post(welcome))
}
