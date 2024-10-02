use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_login::login_required;
use tera::Context;
use tokio::join;
use tower_http::services::ServeDir;

use crate::{
    entities::{task, user},
    libs::{
        ai,
        auth::{AuthSession, Backend},
    },
    AppState, STATIC_PATH,
};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/", get(index))
        .route("/tasks", get(tasks))
        .route("/profile", get(profile))
        .route("/guideStart", get(guide_start))
        .route("/guideShelter", get(guide_shelter))
        .route("/guideQuestboard", get(guide_quest_board))
        .route("/guideInn", get(guide_inn))
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
        // get recommended
        let (time, complexity) = join!(
            task::get_avg_duration(&db_client, u.id),
            task::get_avg_complexity(&db_client, u.id)
        );
        let recommended_indexes = ai::get_recommended(
            state.http_client,
            complexity.unwrap_or(0.0),
            time.unwrap_or(5.0),
            u.tags.clone(),
            &tasks,
        )
        .await
        .unwrap_or(vec![]);
        ctx.insert("recommended_indexes", &recommended_indexes);
    }
    if let Ok(tasks) = task::get_assigned(&db_client, u.id).await {
        // get all
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

async fn guide_shelter(State(state): State<AppState>) -> impl IntoResponse {
    let ctx = Context::new();
    let r = state.template.render("guideShelter.html", &ctx).unwrap();

    Html::from(r)
}

async fn guide_quest_board(
    auth_session: AuthSession,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let u = &auth_session.user.unwrap();

    let mut ctx = Context::new();
    ctx.insert("user", u);
    let r = state.template.render("guideQuestboard.html", &ctx).unwrap();

    Html::from(r)
}

async fn guide_inn(State(state): State<AppState>) -> impl IntoResponse {
    let ctx = Context::new();
    let r = state.template.render("guideInn.html", &ctx).unwrap();

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
