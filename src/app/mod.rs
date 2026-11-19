
use axum::{
    Router,
    routing::{get, post}
};

use std::sync::Arc;
use tokio::sync::Mutex;
use rusqlite::Connection;

use crate::controllers::projeto_controller;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
}

pub async fn start(connection: Connection) {

    let state = AppState {
        db: Arc::new(Mutex::new(connection)),
    };

    let app = Router::new()
        .route("/", get(projeto_controller::index))
        .route("/projetos/novo", get(projeto_controller::novo))
        .route("/projetos", post(projeto_controller::criar))
        .route("/projetos/{id}/editar", get(projeto_controller::editar))
        .route("/projetos/{id}/atualizar", post(projeto_controller::atualizar))
        .route("/projetos/{id}/excluir", post(projeto_controller::excluir))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Aplicação iniciada em http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
