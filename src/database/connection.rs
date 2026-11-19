use rusqlite::Connection;

pub fn connect() -> Result<Connection, rusqlite::Error> {

    let connection = Connection::open("database/app.db")?;

    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS projetos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo TEXT NOT NULL,
            descricao TEXT NOT NULL
        )
        ",
        [],

    )?;

    Ok(connection)
}