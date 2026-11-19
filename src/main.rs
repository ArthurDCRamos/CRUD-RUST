mod app;
mod controllers;
mod models;

use models::projeto::Projeto;

#[tokio::main]
async fn main(){

    let projeto = Projeto::novo(
        1,
        String::from("Rust CRUD"),
        String::from("Meu primeiro sistema em Rust")
    );

    println!("ID: {}", projeto.id);
    println!("Título: {}", projeto.titulo);
    println!("Descrição: {}", projeto.descricao);

    app::start().await;
}