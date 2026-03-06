use axum::Router;

mod characters;
mod skills;

pub fn router() -> Router<crate::state::SharedState> {
    Router::new()
        .nest("/characters", characters::router())
        .nest("/skills", skills::router())
}