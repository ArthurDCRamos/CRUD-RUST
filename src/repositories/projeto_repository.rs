use rusqlite::{Connection, params, OptionalExtension};

use crate::models::projeto::Projeto;

// cria um produto

pub fn criar(
    connection: &Connection,
    titulo: String,
    descricao: String
) -> Result<Projeto, String> {
    // é aqui onde vai colocar os parametros do produto
    let projeto = Projeto::novo(
        0,
        titulo,
        descricao
    )?;

    //insere no banco

    connection.execute(
        "INSERT INTO projetos (titulo, descricao) VALUES (?1, ?2)",
        params![
            projeto.get_titulo(),
            projeto.get_descricao()
        ],
    ).map_err(|erro| erro.to_string())?;

    let id = connection.last_insert_rowid() as u32;

    Projeto::novo(
        id,
        projeto.get_titulo().to_string(),
        projeto.get_descricao().to_string()
    )

}

// lista o produto

pub fn listar(
    connection: &Connection
) -> Result<Vec<Projeto>, String> {

    let mut statement = connection
        .prepare("SELECT id, titulo, descricao FROM projetos")
        .map_err(|erro| erro.to_string())?;

    let projetos = statement
        .query_map([], |row| {

            let id: i64 = row.get(0)?;
            let titulo: String = row.get(1)?;
            let descricao: String = row.get(2)?;

            Ok((id, titulo, descricao))
        })
        .map_err(|erro| erro.to_string())?;

    let mut resultado = Vec::new();

    for projeto in projetos {

        let (id, titulo, descricao) =
            projeto.map_err(|erro| erro.to_string())?;

        let projeto = Projeto::novo(
            id as u32,
            titulo,
            descricao
        )?;

        resultado.push(projeto);
    }

    Ok(resultado)
}

// buscar por id 

pub fn buscar_por_id(
    connection: &Connection,
    id: u32
) -> Result<Option<Projeto>, String> {

    let resultado = connection
        .query_row(
            "SELECT id, titulo, descricao FROM projetos WHERE id = ?1",
            [id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?
                ))
            }
        )
        .optional()
        .map_err(|erro| erro.to_string())?;

    match resultado {
        Some((id, titulo, descricao)) => {
            let projeto = Projeto::novo(
                id as u32,
                titulo,
                descricao
            )?;

            Ok(Some(projeto))
        }

        None => Ok(None)
    }
}

// excluir 

pub fn excluir(
    connection: &Connection,
    id: u32
) -> Result<(), String> {

    let excluidos = connection.execute(
        "DELETE FROM projetos WHERE id = ?1",
        [id],
    ).map_err(|erro| erro.to_string())?;

    if excluidos == 0 {
        return Err("Projeto não encontrado.".to_string());
    }

    Ok(())
}

//atualizar

 pub fn atualizar(
    connection: &Connection,
    id: u32,
    titulo: String,
    descricao: String
) -> Result<(), String> {

    let projeto = Projeto::novo(id, titulo, descricao)?;

    let alterados = connection.execute(
        "UPDATE projetos SET titulo = ?1, descricao = ?2 WHERE id = ?3",
        params![
            projeto.get_titulo(),
            projeto.get_descricao(),
            projeto.get_id()
        ],
    ).map_err(|erro| erro.to_string())?;

    if alterados == 0 {
        return Err("Projeto não encontrado.".to_string());
    }

    Ok(())
}