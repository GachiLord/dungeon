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
    entities::{
        task,
        user::{self, Class},
    },
    libs::auth::{AuthSession, Backend},
    AppState, STATIC_PATH,
};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/", get(index))
        .route("/tasks", get(tasks))
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

async fn index(auth_session: AuthSession, State(state): State<AppState>) -> impl IntoResponse {
    let u = &auth_session.user.unwrap();
    let db_client = state.pool.try_get().await.unwrap();
    let mut ctx = Context::new();

    if let Ok(users) = user::top_players(&db_client).await {
        ctx.insert("top_users", &users);
    }
    if let Ok(users) = user::top_players_by_class(&db_client, u.class).await {
        ctx.insert("top_class_users", &users);
    }

    let r = state.template.render("inn.html", &ctx).unwrap();

    Html::from(r)
}

async fn tasks(auth_session: AuthSession, State(state): State<AppState>) -> impl IntoResponse {
    let u = &auth_session.user.unwrap();
    let db_client = state.pool.try_get().await.unwrap();

    let mut ctx = Context::new();
    ctx.insert("user", &u);
    if let Ok(tasks) = task::get_available(&db_client).await {
        ctx.insert("tasks", &tasks);
    }
    if let Ok(tasks) = task::get_assigned(&db_client, u.id).await {
        ctx.insert("tasks_in_progress", &tasks);
    }

    let r = state.template.render("questBoard.html", &ctx).unwrap();

    Html::from(r)
}

async fn profile(auth_session: AuthSession, State(state): State<AppState>) -> impl IntoResponse {
    let u = &auth_session.user.unwrap();
    let total = task::get_count(&state.pool.try_get().await.unwrap(), u.id)
        .await
        .unwrap_or(-1);

    let mut ctx = Context::new();
    ctx.insert("completed_tasks", &total);
    ctx.insert("user", &u);
    let r = state.template.render("shelter.html", &ctx).unwrap();

    Html::from(r).into_response()
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
