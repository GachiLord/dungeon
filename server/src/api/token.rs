use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Serialize;

use crate::{entities::invite, libs::auth::AuthSession, AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(create))
}

#[derive(Serialize)]
struct TokenData {
    token: Box<str>,
}

async fn create(auth_session: AuthSession, State(state): State<AppState>) -> impl IntoResponse {
    let u = auth_session.user.unwrap();

    if u.is_admin {
        if let Ok(token) = invite::create(&state.pool.try_get().await.unwrap()).await {
            return Json::from(TokenData { token }).into_response();
        }
    }
    StatusCode::FORBIDDEN.into_response()
}
