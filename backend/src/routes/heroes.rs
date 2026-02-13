use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};

use crate::{character::summary::CharacterSummary, state::AppState};

pub fn router() -> Router<crate::state::SharedState> {
    Router::new()
        .route("/summary", get(get_hero_summaries))
}

pub async fn get_hero_summaries(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<CharacterSummary>>, axum::http::StatusCode> {

    let result = sqlx::query(
        r#"
        SELECT index_id, name_id, element, main_position, style, series_id
        FROM heroes
        "#
    )
    .fetch_all(app_state.pool())
    .await;

    match result {
        Ok(characters) => {
            let chars = characters.into_iter().map(|row| {
                CharacterSummary::parse(row, app_state.translation("fr")).unwrap()
            }).collect();

            Ok(Json(chars))
        },
        Err(e) => {
            eprintln!("{e}");
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}