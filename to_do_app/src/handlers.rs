use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};
use crate::models::Todo;
use tokio::fs;

const FILE_PATH: &str = "todos.json";

pub async fn get_todos() -> Json<Vec<Todo>> {
    let data = fs::read_to_string(FILE_PATH).await.unwrap_or_else(|_| "[]".to_string());
    let todos: Vec<Todo> = serde_json::from_str(&data).unwrap_or_default();
    Json(todos)
}

pub async fn add_todo(Json(new_todo): Json<Todo>) -> Json<Todo> {
    let data = fs::read_to_string(FILE_PATH).await.unwrap_or_else(|_| "[]".to_string());
    let mut todos: Vec<Todo> = serde_json::from_str(&data).unwrap_or_default();

    todos.push(new_todo.clone());

    let json = serde_json::to_string(&todos).unwrap();
    fs::write(FILE_PATH, json).await.unwrap();

    Json(new_todo)
}

pub async fn toggle_todo(
    Path(id): Path<u64>) -> StatusCode {
    let data = fs::read_to_string(FILE_PATH).await.unwrap_or_else(|_| "[]".to_string());
    let mut todos: Vec<Todo> = serde_json::from_str(&data).unwrap_or_default();
    
    // Find the todo and flip the completed boolean
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.completed = !todo.completed;
        let json = serde_json::to_string(&todos).unwrap();
        fs::write(FILE_PATH, json).await.unwrap();
        axum::http::StatusCode::OK
    } else {
        axum::http::StatusCode::NOT_FOUND
    }
}