use crate::AppState;
use axum::Router;

mod pages;

pub fn api(state: AppState) -> Router<AppState> {
    Router::new().nest("/", pages::router()).with_state(state)
}
