use std::sync::Arc;

use axum::Json;
use sqlx::SqlitePool;
use moka::future::Cache;

use crate::models::character_summary::CharacterSummary;

pub struct AppState {
    data_db: SqlitePool,
    character_cache: Cache<String, Arc<Json<Vec<CharacterSummary>>>>,
}

impl AppState {
    pub fn new(data_db: SqlitePool) -> AppState {
        AppState { 
            data_db,
            character_cache: Cache::new(50)
        }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.data_db
    }

    pub fn character_cache(&self) -> &Cache<String, Arc<Json<Vec<CharacterSummary>>>> {
        &self.character_cache
    }
}

pub type SharedState = Arc<AppState>;