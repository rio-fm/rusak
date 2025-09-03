use axum::{
    Json, Router,
    extract::{MatchedPath},
    http::{Request, StatusCode},
    routing::post,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::{Span, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let request_tracing = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            let matched_part = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);

            info_span!("http request", method = ?request.method(), matched_part, some_other_field = tracing::field::Empty)
        })
        .on_request(|_request: &Request<_>, _span: &Span| {
            // span.record("some_other_field", request.method().as_str());
        });

    let app = Router::new()
        .route("/", post(welcome))
        .route("/ask", post(welcome))
        .layer(request_tracing);

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
