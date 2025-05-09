use std::net::SocketAddr;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use dotenvy::dotenv;
use uuid::Uuid;
use crate::app_state::AppState;
use crate::common::universal::UniversalResponse;
use crate::db::init_db;
use crate::models::todo::CreateTodo;
use crate::repos::todo_repo;
use crate::repos::todo_repo::{create_todo, delete_todo, get_all_todos, get_todo_by_id};

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
        .route("/api/v1/todo/all",post(get_all_todos_handler))
        .route("/api/v1/todo/{todo_id}",post(fetch_by_id_handle))
        .route("/api/v1/todo/{todo_id}",delete(delete_todo_id))
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


async fn get_all_todos_handler(
    State(state): State<AppState>
) -> impl IntoResponse {

    println!("Get all Todos !");

    let my_todos_list = get_all_todos(&state.db).await;

    match my_todos_list {
        Ok(list) => {
            println!("Success");
            UniversalResponse::success("All Todo's",list)
        }
        Err(e) => {
            println!("Error: {e:?}");
            UniversalResponse::failure(format!("Error: {e:?}"))
        }
    }.into_response()

}



async fn fetch_by_id_handle(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>
) -> impl IntoResponse{

    println!("Fetch by id request for id : {todo_id:?}");

    let fetch_by_id = get_todo_by_id(&state.db,todo_id).await;

    match fetch_by_id {
        Ok(todo) => {
            println!("Success");
            UniversalResponse::success("Fetched Todo",todo)
        }
        Err(e) => {
            println!("Error: {e:?}");
            UniversalResponse::failure(format!("Error: {e:?}"))
        }
    }.into_response()

}


async fn delete_todo_id(
    State(state) : State<AppState>,
    Path(todo_id) : Path<Uuid>
) -> impl IntoResponse{

    println!("Delete by ID using a Todo !");

    let delete_by_id = delete_todo(&state.db,todo_id).await;

    match delete_by_id {
        Ok(todo) => {
            println!("Success");
            UniversalResponse::success("Deleted Successfully","Deleted Successfully")
        }
        Err(e) => {
            println!("Error: {e:?}");
            UniversalResponse::failure(format!("Error: {e:?}"))
        }
    }.into_response()

}