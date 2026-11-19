pub struct Projeto {
    pub id: u32,
    pub titulo: String,
    pub descricao: String,
}

impl Projeto {

    pub fn novo(
        id: u32,
        titulo: String,
        descricao: String
    ) -> Result<Projeto, String> {

        if titulo.trim().is_empty(){
            return Err("O título não pode estar vazio.".to_string());

        }
        if descricao.trim().is_empty(){
            return Err("A descrição não pode estar vazio.".to_string());

        }
        

        Ok(Projeto {
            id,
            titulo,
            descricao,
        })
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_titulo(&self) -> &str {
        &self.titulo
    }
    pub fn get_descricao(&self) -> &str {
        &self.descricao
    }

    pub fn atualizar(
        &mut self,
        titulo: String,
        descricao: String,
    ) -> Result<(), String> {
        if titulo.trim().is_empty() {
            return Err("O título não pode estar vazio.".to_string());
        }

        if descricao.trim().is_empty() {
            return Err("A descrição não pode estar vazio.".to_string());
        }

        self.titulo = titulo;
        self.descricao = descricao;

        Ok(())
    }

}
