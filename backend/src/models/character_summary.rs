use serde::Serialize;
use sqlx::{Row, sqlite::SqliteRow};

use super::common::*;

#[derive(Debug, Clone, Serialize)]
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
    pub fn parse(sqlite_row: SqliteRow) -> Option<CharacterSummary> {

        Some(CharacterSummary {
            index: sqlite_row.get("index_id"),
            name: sqlite_row.get("name"),
            element: Element::from(sqlite_row.get::<i32, &str>("element")),
            main_positon: Position::from(sqlite_row.get::<i32, &str>("main_position")),
            style: Style::from(sqlite_row.get::<i32, &str>("style")),
            series: sqlite_row.get("series_name"),
            stats: Stats::try_from(sqlite_row).ok()?
        })
    }
}