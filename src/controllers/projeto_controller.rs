
use axum::{
    extract::{State, Path},
    Form,
    response::{Html, Redirect},
    http::StatusCode,
};

use serde::Deserialize;

use crate::app::AppState;
use crate::repositories::projeto_repository;

#[derive(Deserialize)]
pub struct ProjetoForm {
    titulo: String,
    descricao: String,
}

// Escapa caracteres especiais para evitar injeção de HTML
fn escapar_html(valor: &str) -> String {
    valor
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

// LISTAR
pub async fn index(
    State(state): State<AppState>
) -> Result<Html<String>, (StatusCode, String)> {

    let connection = state.db.lock().await;

    let projetos = projeto_repository::listar(&connection)
        .map_err(|erro| (StatusCode::INTERNAL_SERVER_ERROR, erro))?;

    let mut linhas = String::new();

    for projeto in projetos {

        linhas.push_str(&format!(
            r#"
            <tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <a href="/projetos/{}/editar">Editar</a>

                    <form action="/projetos/{}/excluir" method="POST">
                        <button type="submit">Excluir</button>
                    </form>
                </td>
            </tr>
            "#,
            projeto.get_id(),
            escapar_html(projeto.get_titulo()),
            escapar_html(projeto.get_descricao()),
            projeto.get_id(),
            projeto.get_id()
        ));
    }

    let html = include_str!("../views/projetos/index.html")
        .replace("{{PROJETOS}}", &linhas);

    Ok(Html(html))
}

// FORMULÁRIO DE CADASTRO
pub async fn novo() -> Html<&'static str> {
    Html(include_str!("../views/projetos/create.html"))
}

// CADASTRAR
pub async fn criar(
    State(state): State<AppState>,
    Form(form): Form<ProjetoForm>
) -> Result<Redirect, (StatusCode, String)> {

    let connection = state.db.lock().await;

    projeto_repository::criar(
        &connection,
        form.titulo,
        form.descricao
    )
    .map_err(|erro| (StatusCode::BAD_REQUEST, erro))?;

    Ok(Redirect::to("/"))
}

// FORMULÁRIO DE EDIÇÃO
pub async fn editar(
    State(state): State<AppState>,
    Path(id): Path<u32>
) -> Result<Html<String>, (StatusCode, String)> {

    let connection = state.db.lock().await;

    let projeto = projeto_repository::buscar_por_id(&connection, id)
        .map_err(|erro| (StatusCode::INTERNAL_SERVER_ERROR, erro))?;

    let projeto = projeto.ok_or((
        StatusCode::NOT_FOUND,
        "Projeto não encontrado.".to_string()
    ))?;

    let html = include_str!("../views/projetos/edit.html")
        .replace("{{ID}}", &projeto.get_id().to_string())
        .replace("{{TITULO}}", &escapar_html(projeto.get_titulo()))
        .replace("{{DESCRICAO}}", &escapar_html(projeto.get_descricao()));

    Ok(Html(html))
}

// ATUALIZAR
pub async fn atualizar(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Form(form): Form<ProjetoForm>
) -> Result<Redirect, (StatusCode, String)> {

    let connection = state.db.lock().await;

    projeto_repository::atualizar(
        &connection,
        id,
        form.titulo,
        form.descricao
    )
    .map_err(|erro| (StatusCode::BAD_REQUEST, erro))?;

    Ok(Redirect::to("/"))
}

// EXCLUIR
pub async fn excluir(
    State(state): State<AppState>,
    Path(id): Path<u32>
) -> Result<Redirect, (StatusCode, String)> {

    let connection = state.db.lock().await;

    projeto_repository::excluir(&connection, id)
        .map_err(|erro| (StatusCode::NOT_FOUND, erro))?;

    Ok(Redirect::to("/"))
}
