use axum::Router;

mod characters;

pub fn router() -> Router<crate::state::SharedState> {
    Router::new()
        .nest("/characters", characters::router())

}