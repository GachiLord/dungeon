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
