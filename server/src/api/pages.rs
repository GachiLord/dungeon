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
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let ctx = Context::new();
    let r = state.template.render("base.html", &ctx).unwrap();

    Html::from(r)
}
