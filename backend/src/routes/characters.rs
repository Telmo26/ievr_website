use axum::{Json, Router, extract::{Query, State}, routing::get};
use serde::{Deserialize, Serialize};
use sqlx::{Execute, QueryBuilder, Sqlite};

use crate::{character::summary::CharacterSummary, state::SharedState};

pub fn router() -> Router<crate::state::SharedState> {
    Router::new()
        .route("/summary", get(get_character_summaries))
}

pub async fn get_character_summaries(
    State(app_state): State<SharedState>,
    Query(params) : Query<SearchParams>
) -> Result<Json<Vec<CharacterSummary>>, axum::http::StatusCode> {

    let mut query_builder: QueryBuilder<'_, Sqlite> = QueryBuilder::new(
"SELECT *
FROM characters
"
    );

    let mut where_used = false;

    if let Some(element) = params.element {
        query_builder.push("WHERE element = ");
        where_used = true;

        query_builder.push_bind(element as u8);
        query_builder.push("\n");
    }

    if let Some(position) = params.position {
        if !where_used {
            query_builder.push("WHERE main_position = ");
            where_used = false;
        } else {
            query_builder.push("AND main_position = ");
        }

        query_builder.push_bind(position as u8);
        query_builder.push("\n");
    }

    if let Some(style) = params.style {
        if !where_used {
            query_builder.push("WHERE style = ");
        } else {
            query_builder.push("AND style = ");
        }

        query_builder.push_bind(style as u8);
        query_builder.push("\n");
    }
    
    let query = query_builder.build();

    #[cfg(debug_assertions)]
    println!("{}", query.sql());

    let result = query
        .fetch_all(app_state.pool())
        .await;

    match result {
        Ok(characters) => {
            let chars = characters.into_iter().filter_map(|row| {
                CharacterSummary::parse(row, app_state.translation("fr"))
            })
            .filter(|summary| { summary.name().contains(&params.name) })
            .collect();

            Ok(Json(chars))
        },
        Err(e) => {
            eprintln!("{e}");
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchParams {
    #[serde(default)]
    name: String,

    #[serde(default)]
    element: Option<crate::character::common::Element>,

    #[serde(default)]
    position: Option<crate::character::common::Position>,

    #[serde(default)]
    style: Option<crate::character::common::Style>,
}