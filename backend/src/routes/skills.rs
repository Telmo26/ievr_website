use axum::{Json, Router, extract::State, routing::get};
use sqlx::QueryBuilder;

use crate::{
    models::skill::{Aura, Hissatsu}, 
    state::SharedState
};



pub fn router() -> Router<crate::state::SharedState> {
    Router::new()
        .route("/", get(get_hissatsu))
        .route("/aura", get(get_auras))
}

async fn get_hissatsu(
    State(app_state): State<SharedState>,
) -> Result<Json<Vec<Hissatsu>>, axum::http::StatusCode> {
    let mut query_builder = QueryBuilder::new(
"
SELECT skill_id, n.name, power, element, category, growth_rate, is_block, is_longshot, tp_consumption, cooldown
FROM skills.hissatsu
JOIN en.skill_names n
ON n.id = skills.hissatsu.name_id
"
    );

    let query = query_builder.build();
    let result = query
        .fetch_all(app_state.pool())
        .await;

    match result {
        Ok(skills) => {
            let skills = skills.into_iter()
                .filter_map(Hissatsu::parse)
                .collect();

            Ok(Json(skills))
        }

        Err(e) => {
            eprintln!("{e}");
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_auras(
    State(app_state): State<SharedState>,
) -> Result<Json<Vec<Aura>>, axum::http::StatusCode> {
    let mut query_builder = QueryBuilder::new(
"
SELECT 
    aura_id, 
    n1.name AS aura_name, 
    aura_type, 
    a.element AS aura_element, 
    ah.skill_id, 
    n2.name, 
    ah.power, 
    ah.element, 
    ah.category, 
    ah.growth_rate, 
    ah.is_block, 
    ah.is_longshot, 
    ah.tp_consumption, 
    ah.cooldown

FROM skills.aura a

JOIN en.skill_names n1
    ON n1.id = a.name_id

LEFT JOIN skills.aura_hissatsu ah
    ON ah.skill_id = a.skill_id

LEFT JOIN en.skill_names n2
    ON n2.id = ah.name_id
"
    );

    let query = query_builder.build();
    let result = query
        .fetch_all(app_state.pool())
        .await;

    match result {
        Ok(skills) => {
            let skills = skills.into_iter()
                .filter_map(Aura::parse)
                .collect();

            Ok(Json(skills))
        }

        Err(e) => {
            eprintln!("{e}");
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}