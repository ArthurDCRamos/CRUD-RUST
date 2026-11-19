mod app;
mod controllers;
mod models;
mod database;
mod repositories;

#[tokio::main]
async fn main() {

    let connection = match database::connection::connect() {
        Ok(connection) => {
            println!("Banco de dados conectado!");
            connection
        }

        Err(error) => {
            eprintln!("Erro ao conectar ao banco: {}", error);
            return;
        }
    };

    app::start(connection).await;
}
