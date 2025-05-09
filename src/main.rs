use std::net::SocketAddr;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::extract::State;
use axum::routing::{get, post};
use dotenvy::dotenv;
use crate::app_state::AppState;
use crate::common::universal::UniversalResponse;
use crate::db::init_db;
use crate::models::todo::CreateTodo;
use crate::repos::todo_repo;
use crate::repos::todo_repo::create_todo;

mod app_state;
mod db;
mod routes;
mod models;
mod handlers;
mod repos;
mod common;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = init_db().await.expect("Failed to initialize DB");
    let state = AppState { db };

    // let app = create_routes().with_state(state);

    let app = Router::new()
        .route("/", get(todo_app_ping))
        .route("/api/v1/todo/add",post(add_todo_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {:?}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}



async fn todo_app_ping() -> &'static str{
    return "Ping me Todo App Apis"
}


async fn add_todo_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    println!("Add Todo request: {payload:?}");

    let result = create_todo(&state.db, payload).await;
    let response = match result {
        Ok(todo) => {
            println!("Success: {todo:?}");
            UniversalResponse::success("Added Todo Successfully", todo)
        }
        Err(e) => {
            println!("Error: {e:?}");
            UniversalResponse::failure(format!("Failed to Add Todo: {}", e))
        }
    };

    response.into_response()
}