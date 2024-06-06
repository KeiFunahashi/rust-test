use axum::{
    response::Json,
    response::IntoResponse,
    http::StatusCode
};
use serde::{Deserialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct CreatetodosRequest {
    todo: String
}

#[derive(Deserialize)]
pub struct UpdatetodosRequest {
    todo: String
}

#[derive(Deserialize)]
pub struct DeletetodosRequest {
    id: i64 
}

pub async fn get_todos() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!(
            [
                {
                    "id": 1,
                    "title": "test",
                    "isCompleted": false,
                },
                {
                    "id": 2,
                    "title": "test",
                    "isCompleted": false,
                }
            ]
        ))
    )
}

pub async fn create_todos(Json(_payload): Json<CreatetodosRequest>) -> impl IntoResponse {
    println!("{}", _payload.todo);
    (
        StatusCode::OK,
        Json(json!({"message": "create todos!"}))
    )
}

pub async fn update_todos(Json(_payload): Json<UpdatetodosRequest>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({"message":"update todos!"}))
    )
}

pub async fn delete_todos(Json(_payload): Json<DeletetodosRequest>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({"message":"delete todos!"}))
    )
}
