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
}

// app state

#[derive(Clone)]
struct AppState {
    // TODO: task queue for AI
    //
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
