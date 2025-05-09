use dotenvy::dotenv;
use tokio::net::unix::SocketAddr;
use crate::app_state::AppState;
use crate::db::init_db;

mod app_state;
mod db;
mod routes;
mod models;
mod handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = init_db().await.expect("Failed to initialize DB");
    let state = AppState { db };

    let app = create_routes().with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {:?}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}