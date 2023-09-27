use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgPoolOptions,
    types::{BigDecimal, Uuid},
};

struct AppState {
    pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgrespw@localhost:5432/banco_dos_amigos")
        .await
        .unwrap();

    let app_state = Arc::new(AppState { pool });
    let address: SocketAddr = "0.0.0.0:3000".parse().unwrap();

    let app = Router::new()
        .route("/pessoas", post(create_pessoas))
        .route("/pessoas", get(get_pessoa))
        .with_state(app_state);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_pessoas(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Pessoa>,
) -> Json<Pessoa> {
    let pessoa = sqlx::query_file!(
        "src/sql/create_pessoa.sql",
        payload.nome,
        payload.idade.to_i32().unwrap(),
        format!("{}", payload.altura).parse::<BigDecimal>().unwrap(),
        format!("{}", payload.peso).parse::<BigDecimal>().unwrap(),
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Pessoa::new(
        &pessoa.nome,
        pessoa.idade.to_u8().unwrap(),
        pessoa.altura.to_f32().unwrap(),
        pessoa.peso.to_f32().unwrap(),
    )
    .add_id(pessoa.id)
    .into()
}

async fn get_pessoa(State(state): State<Arc<AppState>>) -> Json<Vec<Pessoa>> {
    let pessoas = sqlx::query_file!("src/sql/busca_pessoas.sql",)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    let pessoas: Vec<Pessoa> = pessoas
        .into_iter()
        .map(|pessoa| {
            Pessoa::new(
                &pessoa.nome,
                pessoa.idade.to_u8().unwrap(),
                pessoa.altura.to_f32().unwrap(),
                pessoa.peso.to_f32().unwrap(),
            )
            .add_id(pessoa.id)
        })
        .collect();

    Json(pessoas)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Pessoa {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    nome: String,
    idade: u8,
    altura: f32,
    peso: f32,
}

impl Pessoa {
    fn new(nome: &str, idade: u8, altura: f32, peso: f32) -> Self {
        Self {
            id: None,
            nome: nome.to_string(),
            idade,
            altura,
            peso,
        }
    }

    fn add_id(mut self, id: Uuid) -> Self {
        self.id = Some(id.to_string());
        self
    }
}
