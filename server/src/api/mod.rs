use crate::AppState;
use axum::Router;

mod auth;
mod pages;

pub fn api(state: AppState) -> Router<AppState> {
    let api = Router::new().nest("/auth", auth::router());

    Router::new()
        .nest("/", pages::router())
        .nest("/api", api)
        .with_state(state)
}
