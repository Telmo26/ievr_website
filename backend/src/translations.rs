use std::collections::HashMap;

use sqlx::{Row, SqlitePool};

pub struct Translation {
    character_names: HashMap<i32, String>,
    character_names_roma: HashMap<i32, String>,
    character_desc: HashMap<i32, String>,
    series_name: HashMap<i32, String>,
    skills: HashMap<i32, (String, String)>,
}

impl Translation {
    pub async fn parse(pool: SqlitePool) -> sqlx::Result<Translation> {
        Ok(Translation {
            character_names: Self::parse_text(&pool, "character_names", ["id", "name"]).await?,
            character_names_roma: Self::parse_text(&pool, "character_names_roma", ["id", "name"]).await?,
            character_desc: Self::parse_text(&pool, "character_descriptions", ["id", "description"]).await?,
            series_name: Self::parse_text(&pool, "series_names", ["id", "name"]).await?,
            skills: HashMap::new(),
        })
    }

    pub fn name(&self, name_id: i32) -> Option<&String> {
        self.character_names.get(&name_id)
    }

    pub fn name_roma(&self, name_id: i32) -> String {
        self.character_names_roma[&name_id].clone()
    }

    pub fn description(&self, description_id: i32) -> String {
        self.character_desc[&description_id].clone()
    }

    pub fn series(&self, series_id: i32) -> String {
        self.series_name[&series_id].clone()
    }

    async fn parse_text(
        pool: &SqlitePool,
        table_name: &str,
        column_names: [&str; 2],
    ) -> sqlx::Result<HashMap<i32, String>> {
        let result = sqlx::query(&format!("SELECT * FROM {table_name}"))
            .fetch_all(pool)
            .await?;

        Ok(result
            .into_iter()
            .map(|row| (row.get(column_names[0]), row.get(column_names[1])))
            .collect())
    }
}
