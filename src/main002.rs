mod app;
mod controllers;
mod models;
mod database;

use models::projeto::Projeto;

#[tokio::main]
async fn main(){

    let mut projeto = match Projeto::novo(
        1,
        String::from("Rust CRUD"),
        String::from("Meu primeiro sistema em Rust")
    ) {
        Ok(projeto) => projeto,
        Err(erro) => {
            println!("Erro: {}", erro);
            return;
        }
    };


    println!("ID: {}", projeto.id);
    println!("Título: {}", projeto.titulo);
    println!("Descrição: {}", projeto.descricao);


    match projeto.atualizar(
        String::from("Rust CRUD atualizado"),
        String::from("Aprendendo encapsulmento")
    ) {
        Ok(()) => println!("Projeto atualizado!"),
        Err(erro) => println!("Erro: {}", erro),
    }

    println!("Novo título: {}", projeto.get_titulo());

    app::start().await;
}