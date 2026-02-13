use std::{collections::HashMap, sync::Arc};

use sqlx::SqlitePool;

use crate::translations::Translation;

pub struct AppState {
    data_db: SqlitePool,
    translations: HashMap<&'static str, Translation>
}

impl AppState {
    pub fn new(data_db: SqlitePool, translations: HashMap<&'static str, Translation>) -> AppState {
        AppState { data_db, translations }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.data_db
    }

    pub fn translation(&self, language: &str) -> &Translation {
        &self.translations[language]
    }
}

pub type SharedState = Arc<AppState>;