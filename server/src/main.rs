use api::api;
use axum::Router;
use axum_login::{
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use lazy_static::lazy_static;
use libs::{
    auth::Backend,
    db::{init_db, PoolWrapper},
};
use std::env;
use tera::Tera;

mod api;
mod entities;
mod libs;

// env

lazy_static! {
    pub static ref STATIC_PATH: &'static str = {
        let s = &env::var("STATIC_PATH").unwrap_or("static".to_owned());
        let s: &'static str = s.clone().leak();

        s
    };
    pub static ref AI_HOST: &'static str = {
        let s = &env::var("AI_HOST").unwrap_or("http://localhost:8080".to_owned());
        let s: &'static str = s.clone().leak();

        s
    };
}

// app state

#[derive(Clone)]
struct AppState {
    http_client: reqwest::Client,
    pool: &'static PoolWrapper,
    template: &'static Tera,
}

#[tokio::main]
async fn main() {
    // db
    let pool = init_db().await;
    // templates
    let tera = Tera::new(&format!("{}/templates/**/*", *STATIC_PATH)).unwrap();
    // app state
    let state = AppState {
        pool,
        template: Box::leak(Box::new(tera)),
        http_client: reqwest::Client::new(),
    };
    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);
    // Auth service.
    let backend = Backend::new(&pool);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();
    // launch server
    let app = Router::new()
        .nest("/", api(state.clone()).with_state(state))
        .layer(auth_layer);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
