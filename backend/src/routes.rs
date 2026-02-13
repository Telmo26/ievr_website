use axum::Router;

mod characters;
mod heroes;

pub fn router() -> Router<crate::state::SharedState> {
    Router::new()
        .nest("/characters", characters::router())
        .nest("/heroes", heroes::router())
}