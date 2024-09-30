use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::verify_password;
use serde::Deserialize;

use crate::entities::user::{self, User};

use super::db::PoolWrapper;

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.pw_hash.as_bytes()
    }
}

#[derive(Clone)]
pub struct Backend {
    pool: &'static PoolWrapper,
}

impl Backend {
    pub fn new(pool: &'static PoolWrapper) -> Backend {
        Backend { pool }
    }
}

#[derive(Clone, Deserialize)]
pub struct Credentials {
    login: Box<str>,
    password: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Postgres(#[from] tokio_postgres::Error),

    #[error(transparent)]
    TaskJoin(#[from] tokio::task::JoinError),
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        Credentials { login, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let db_client = self.pool.try_get().await.unwrap();
        let u = user::get_by_login(&db_client, &login).await?;

        tokio::task::spawn_blocking(move || match verify_password(password, &u.pw_hash) {
            Ok(_) => Ok(Some(u)),
            Err(_) => Ok(None),
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let db_client = self.pool.try_get().await.unwrap();
        let u = user::get(&db_client, *user_id).await?;

        Ok(Some(u))
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;
