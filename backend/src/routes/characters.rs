use axum::{Json, Router, extract::State, routing::get};
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};
use sqlx::{Execute, QueryBuilder, Sqlite};

use crate::{character::summary::CharacterSummary, state::SharedState};

pub fn router() -> Router<crate::state::SharedState> {
    Router::new()
        .route("/", get(get_character_summaries))
        .route("/heroes", get(get_hero_summaries))
        .route("/basara", get(get_basara_summaries))
}

pub async fn get_character_summaries(
    State(app_state): State<SharedState>,
    Query(params) : Query<SearchParams>
) -> Result<Json<Vec<CharacterSummary>>, axum::http::StatusCode> {
    get_summaries(app_state, params, "characters").await
}

pub async fn get_hero_summaries(
    State(app_state): State<SharedState>,
    Query(params) : Query<SearchParams>
) -> Result<Json<Vec<CharacterSummary>>, axum::http::StatusCode> {
    get_summaries(app_state, params, "heroes").await
}

pub async fn get_basara_summaries(
    State(app_state): State<SharedState>,
    Query(params) : Query<SearchParams>
) -> Result<Json<Vec<CharacterSummary>>, axum::http::StatusCode> {
    get_summaries(app_state, params, "basaras").await
}

async fn get_summaries(
    app_state: SharedState,
    params: SearchParams,
    table_name: &str
) -> Result<Json<Vec<CharacterSummary>>, axum::http::StatusCode> {
    let mut query_builder: QueryBuilder<'_, Sqlite> = QueryBuilder::new(
format!("SELECT index_id, name_id, element, main_position, style, series_id, lvl50_kick, lvl50_control, lvl50_technique, lvl50_pressure, lvl50_physical, lvl50_agility, lvl50_intelligence
FROM {table_name}
")
    );

    let mut where_used = false;

    if !params.element.is_empty() {
        query_builder.push("WHERE element IN (");
        where_used = true;

        let mut separated = query_builder.separated(",");
        for element in params.element {
            separated.push_bind(element as u8);
        }
        separated.push_unseparated(")\n");
    }

    if !params.position.is_empty() {
        if !where_used {
            query_builder.push("WHERE main_position IN (");
            where_used = false;
        } else {
            query_builder.push("AND main_position IN (");
        }

        let mut separated = query_builder.separated(",");
        for position in params.position {
            separated.push_bind(position as u8);
        }
        separated.push_unseparated(")\n");
    }

    if !params.style.is_empty() {
        if !where_used {
            query_builder.push("WHERE style IN (");
        } else {
            query_builder.push("AND style IN (");
        }

        let mut separated = query_builder.separated(",");
        for style in params.style {
            separated.push_bind(style as u8);
        }
        separated.push_unseparated(")\n");
    }

    if let Some(sort) = params.sort {
        query_builder.push("ORDER BY ");
        query_builder.push(sort.to_sql());

        if params.descending {
            query_builder.push(" DESC");
        }

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
    element: Vec<crate::character::common::Element>,

    #[serde(default)]
    position: Vec<crate::character::common::Position>,

    #[serde(default)]
    style: Vec<crate::character::common::Style>,

    #[serde(default)]
    sort: Option<SortField>,

    #[serde(default)]
    descending: bool
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum SortField {
    Kick,
    Control,
    Technique,
    Pressure,
    Physical,
    Agility,
    Intelligence
}

impl SortField {
    fn to_sql(&self) -> &'static str {
        match self {
            Self::Kick => "lvl50_kick",
            Self::Control => "lvl50_control",
            Self::Technique => "lvl50_technique",
            Self::Pressure => "lvl50_pressure",
            Self::Physical => "lvl50_physical",
            Self::Agility => "lvl50_agility",
            Self::Intelligence => "lvl50_intelligence"
        }
    }
}