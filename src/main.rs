use axum::{routing::post, Json, Router, Server};
use axum_test::multiply_service::multiply;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;


#[derive(Deserialize)]
struct Request {
    pub vector: Vec<f64>,
    pub factor: f64,
}

#[derive(Serialize)]
struct Response {
    product: Vec<f64>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/api/v1/multiplication", post(multiplication_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    
    let server = Server::bind(&addr).serve(app.into_make_service());
    tracing::info!("Listening on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}

async fn multiplication_handler(Json(data): Json<Request>) -> Json<Response> {
    Json(Response {
        product: multiply(data.vector, data.factor),
    })
}
