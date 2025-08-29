use axum::{Json, Router, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(welcome));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn welcome(Json(payload): Json<Greet>) -> (StatusCode, Json<SayHello>) {
    let greeting_text = format!("Hello {}", payload.name);
    let response = SayHello {
        text: greeting_text,
    };

    (StatusCode::ACCEPTED, Json(response))
}

#[derive(Deserialize)]
struct Greet {
    name: String,
}

#[derive(Serialize)]
struct SayHello {
    text: String,
}
