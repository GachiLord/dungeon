use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Redirect},
    routing::post,
    Form, Json, Router,
};
use password_auth::generate_hash;
use serde::Deserialize;

use crate::{
    entities::user,
    libs::auth::{AuthSession, Credentials},
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
}

#[derive(Deserialize)]
struct UserRegisterData {
    login: Box<str>,
    name: Box<str>,
    password: Box<str>,
    secret: Box<str>,
}

async fn signup(
    mut auth_session: AuthSession,
    State(state): State<AppState>,
    Form(payload): Form<UserRegisterData>,
) -> impl IntoResponse {
    // TODO: verity invite token
    let db_client = state.pool.try_get().await.unwrap();
    if let Ok(v) = user::exists(&db_client, &payload.login).await {
        // check if login exists
        if v {
            return Html::from("<p>Такое имя уже принадлежит другому авантюристу</p>")
                .into_response();
        }
        let hash = tokio::task::spawn_blocking(move || {
            generate_hash(payload.password.as_bytes()).into_boxed_str()
        })
        .await
        .unwrap();
        let u = user::User {
            id: 0,
            login: payload.login,
            name: payload.name,
            pw_hash: hash,
            class: user::Class::C,
            is_admin: false,
        };
        if let Ok(created_user) = user::create(&db_client, u).await {
            if let Ok(_) = auth_session.login(&created_user).await {
                let mut r = Html::from("").into_response();
                r.headers_mut()
                    .insert("HX-Redirect", HeaderValue::from_static("/guideStart"));
                return r;
            }
        }
    }
    return Html::from("<p>Неожиданная ошибка судьбы</p>").into_response();
}

async fn signin(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Html::from("<p>Такого имени не существует или тайное слово - ложно</p>")
                .into_response()
        }
        Err(_) => {
            return Html::from("<p>Такого имени не существует или тайное слово - ложно</p>")
                .into_response();
        }
    };

    if let Err(_) = auth_session.login(&user).await {
        return Html::from("<p>Неожиданная ошибка судьбы</p>").into_response();
    }

    let mut r = Html::from("").into_response();
    r.headers_mut()
        .insert("HX-Redirect", HeaderValue::from_static("/"));
    r
}
