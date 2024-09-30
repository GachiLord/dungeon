use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_login::login_required;
use tera::Context;
use tower_http::services::ServeDir;

use crate::{
    libs::auth::{AuthSession, Backend},
    AppState, STATIC_PATH,
};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/", get(index))
        .route("/profile", get(profile))
        .route("/guideStart", get(guide_start))
        .route_layer(login_required!(Backend, login_url = "/welcome"));

    Router::new()
        .nest_service("/dist", ServeDir::new(format!("{}/dist", *STATIC_PATH)))
        .route("/welcome", get(welcome))
        .route("/signin", get(signin))
        .route("/signup", get(signout))
        .nest("/", protected)
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let ctx = Context::new();
    let r = state.template.render("inn.html", &ctx).unwrap();

    Html::from(r)
}

async fn profile(auth_session: AuthSession, State(state): State<AppState>) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("user", &auth_session.user.unwrap());
    let r = state.template.render("shelter.html", &ctx).unwrap();

    Html::from(r)
}

async fn guide_start(State(state): State<AppState>) -> impl IntoResponse {
    let ctx = Context::new();
    let r = state.template.render("guideStart.html", &ctx).unwrap();

    Html::from(r)
}

async fn welcome(State(state): State<AppState>) -> impl IntoResponse {
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
    let r = state.template.render("signup.html", &ctx).unwrap();

    Html::from(r)
}
