use axum::{
    routing::get,
    Router,
    response::Json,
    response::IntoResponse,
    http::StatusCode,
    http::HeaderValue
};
use serde_json::json;
use std::net::SocketAddr;
use hyper::header::CONTENT_TYPE;
use tower_http::cors::{Any,CorsLayer};
mod controller;

#[tokio::main]
async fn main() {
    let origins = [
        "http://localhost:3000/".parse::<HeaderValue>().unwrap()
    ];

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(controller::todos::get_todos)
                         .post(controller::todos::create_todos)
                         .put(controller::todos::update_todos)
                         .delete(controller::todos::delete_todos))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!("OK"))
    )
}