use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tera::Context;
use tower_http::services::ServeDir;

use crate::{AppState, STATIC_PATH};

pub fn router() -> Router<AppState> {
    Router::new()
        .nest_service("/dist", ServeDir::new(format!("{}/dist", *STATIC_PATH)))
        .route("/", get(index))
        .route("/signin", get(signin))
        .route("/signout", get(signout))
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let ctx = Context::new();
    let r = state.template.render("welcome.html", &ctx).unwrap();

    Html::from(r)
}

async fn signin(State(state): State<AppState>) -> impl IntoResponse {
    let ctx = Context::new();
    let r = state.template.render("signin.html", &ctx).unwrap();

    Html::from(r)
}

async fn signout(State(state): State<AppState>) -> impl IntoResponse {
    let ctx = Context::new();
    let r = state.template.render("signout.html", &ctx).unwrap();

    Html::from(r)
}
