use api::api;
use axum::Router;
use lazy_static::lazy_static;
use std::env;
use tera::Tera;

mod api;
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
    // TODO pool: &'static PoolWrapper,
    template: &'static Tera,
}

#[tokio::main]
async fn main() {
    // db
    // TODO connect to db
    // templates
    let tera = Tera::new(&format!("{}/templates/**/*", *STATIC_PATH)).unwrap();
    // app state
    let state = AppState {
        template: Box::leak(Box::new(tera)),
    };
    // launch server
    let app = Router::new().nest("/", api(state.clone()).with_state(state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
