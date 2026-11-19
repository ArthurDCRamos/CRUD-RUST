use axum::{
    Router,
    routing::get
};

use crate::controllers::projeto_controller;

pub async fn start(){
    let app = Router::new ()
        .route("/",get(projeto_controller::index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Aplicação iniciada em http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
    
}