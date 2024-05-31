use axum::{
    routing::get,
    Router,
    response::Json,
    response::IntoResponse,
    http::StatusCode
};
use serde_json::json;
use std::net::SocketAddr;
mod controller;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(controller::todos::get_todos)
                         .post(controller::todos::create_todos)
                         .put(controller::todos::update_todos)
                         .delete(controller::todos::delete_todos));
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