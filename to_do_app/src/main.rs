mod models;
mod handlers;

use axum::{routing::{get, post}, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/todos", get(handlers::get_todos))
        .route("/api/todos", post(handlers::add_todo))
        .route("/api/todos/{id}/toggle", post(handlers::toggle_todo))
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("launch sucessful go to http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
