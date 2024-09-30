use std::{env, fs};

use bb8::{Pool, PooledConnection, RunError};
use bb8_postgres::{
    tokio_postgres::{self, NoTls},
    PostgresConnectionManager,
};

#[derive(Clone)]
pub struct PoolWrapper {
    inner: &'static Pool<PostgresConnectionManager<NoTls>>,
}

pub type DbClient<'a> = PooledConnection<'static, PostgresConnectionManager<NoTls>>;

impl PoolWrapper {
    pub async fn try_get(&self) -> Result<DbClient, RunError<tokio_postgres::Error>> {
        self.inner.get_owned().await
    }
}

pub async fn init_db() -> &'static PoolWrapper {
    let db_user = &env::var("DB_USER").expect("$DB_USER is not provided");
    let db_host = &env::var("DB_HOST").expect("$DB_HOST is not provided");
    let db_port = &env::var("DB_PORT").expect("$DB_PORT is not provided");
    let db_password = fs::read_to_string(
        &env::var("DB_PASSWORD_PATH").unwrap_or("/run/secrets/db_password".to_string()),
    )
    .expect("db_password is not found");
    let init_sql =
        fs::read_to_string(&env::var("DB_INIT_PATH").expect("$DB_INIT_PATH is not provided"))
            .unwrap();
    let manager = PostgresConnectionManager::new_from_stringlike(
        &format!("host={db_host} port={db_port} user={db_user} password={db_password}"),
        NoTls,
    )
    .expect("failed to create db connection pool");
    let pool = Box::leak(Box::new(Pool::builder().build(manager).await.unwrap()));
    // initialize db
    let pool_wrapper = Box::leak(Box::new(PoolWrapper { inner: pool }));
    pool.get_owned()
        .await
        .unwrap()
        .batch_execute(&init_sql)
        .await
        .unwrap();

    pool_wrapper
}
