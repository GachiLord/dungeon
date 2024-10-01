use axum::{
    extract::State,
    http::HeaderValue,
    response::{Html, IntoResponse},
    routing::post,
    Form, Router,
};
use password_auth::generate_hash;
use serde::Deserialize;

use crate::{
    entities::{invite, user},
    libs::auth::{AuthSession, Credentials},
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
        .route("/logout", post(logout))
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
    let db_client = state.pool.try_get().await.unwrap();
    // verity invite token
    let expired = match invite::is_expired(&db_client, &payload.secret).await {
        Ok(v) => v,
        Err(_) => return Html::from("<p>Священное слово заклинателя - ложно</p>").into_response(),
    };
    if expired {
        return Html::from("<p>Священное слово заклинателя уже было использовано</p>")
            .into_response();
    }
    // try to create user
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
            tags: vec![],
        };
        if let Ok(created_user) = user::create(&db_client, u).await {
            if let Ok(_) = auth_session.login(&created_user).await {
                // expire the invite token
                let _ = invite::expire(&db_client, &payload.secret).await;
                // redirect to index
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

pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => {
            let mut r = Html::from("").into_response();
            r.headers_mut()
                .insert("HX-Redirect", HeaderValue::from_static("/welcome"));
            r
        }
        Err(_) => Html::from("<p>Неожиданная ошибка судьбы</p>").into_response(),
    }
}
