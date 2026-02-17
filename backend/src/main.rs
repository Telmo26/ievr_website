use std::{collections::HashMap, sync::Arc};

use axum::Router;

use tower_http::cors::{CorsLayer, Any};

use sqlx::SqlitePool;

mod state;
mod translations;
mod character;
mod routes;

use state::AppState;
use tower_http::services::ServeDir;
use translations::Translation;

use crate::state::SharedState;

const LANGUAGES: [&str; 9] = [
    "de",
    "en",
    "es",
    "fr",
    "it",
    "ja",
    "pt",
    "zh_hans",
    "zh_hant"
];

#[tokio::main]
async fn main() {
    // Create SQLite connection pool
    let data_db = SqlitePool::connect("sqlite:databases/characters.sqlite").await.unwrap();
    
    sqlx::query("ATTACH DATABASE 'databases/skills.sqlite' AS skills").execute(&data_db).await.unwrap();

    let mut translations = HashMap::with_capacity(LANGUAGES.len());

    for language in LANGUAGES {
        let language_db = SqlitePool::connect(&format!("sqlite:databases/translations/{language}.sqlite")).await.unwrap();
        translations.insert(
            language, 
            Translation::parse(language_db).await.unwrap()
        );
    }

    let app_state: SharedState = Arc::new(AppState::new(data_db, translations));

    let app = Router::new()
        .nest("/api", routes::router())
        .with_state(app_state)
        .layer(CorsLayer::new().allow_origin(Any))
        .fallback_service(ServeDir::new("assets"));

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
