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
    ) -> Projeto {

        Projeto {
            id,
            titulo,
            descricao,
        }
    }

}
