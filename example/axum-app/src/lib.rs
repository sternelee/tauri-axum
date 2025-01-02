mod routes;
mod todo;

use axum::{
    routing::{get, post},
    Router,
};
use routes::{create_todo, delete_todo, list_todos, toggle_todo};
use std::sync::{Arc, Mutex};
use todo::Todo;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    todos: Arc<Mutex<Vec<Todo>>>,
}

pub fn create_axum_app() -> Router {
    let state = AppState {
        todos: Arc::new(Mutex::new(Vec::new())),
    };

    Router::new()
        .route("/", get(list_todos))
        .route("/todo", post(create_todo))
        .route("/todo/{:id}/delete", post(delete_todo))
        .route("/todo/{:id}/toggle", post(toggle_todo))
        .with_state(state)
}
