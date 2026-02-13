use serde::Serialize;
use sqlx::{Row, sqlite::SqliteRow};

use crate::Translation;

use super::common::*;

#[derive(Debug, Serialize)]
pub struct CharacterSummary {
    index: u16,
    name: String,
    element: Element,
    main_positon: Position,
    style: Style,
    series: String,
    stats: Stats
}

impl CharacterSummary {
    pub fn parse(sqlite_row: SqliteRow, translation: &Translation) -> Option<CharacterSummary> {
        Some(CharacterSummary {
            index: sqlite_row.get("index_id"),
            name: translation.name(sqlite_row.get("name_id"))?.to_string(),
            element: Element::from(sqlite_row.get::<i32, &str>("element")),
            main_positon: Position::from(sqlite_row.get::<i32, &str>("main_position")),
            style: Style::from(sqlite_row.get::<i32, &str>("style")),
            series: translation.series(sqlite_row.get("series_id")),
            stats: Stats::try_from(sqlite_row).ok()?
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}