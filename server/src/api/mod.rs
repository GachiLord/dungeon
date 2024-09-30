use crate::AppState;
use axum::Router;

mod auth;
mod pages;
mod token;

pub fn api(state: AppState) -> Router<AppState> {
    let api = Router::new()
        .nest("/auth", auth::router())
        .nest("/token", token::router());

    Router::new()
        .nest("/", pages::router())
        .nest("/api", api)
        .with_state(state)
}
