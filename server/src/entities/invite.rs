use nanoid::nanoid;

use crate::libs::db::DbClient;

pub async fn create(db_client: &DbClient<'_>) -> Result<Box<str>, tokio_postgres::Error> {
    let token = tokio::task::spawn_blocking(move || nanoid!())
        .await
        .unwrap();

    let _ = db_client
        .execute(
            "INSERT INTO invite_tokens (token, is_expired) VALUES ($1, $2)",
            &[&token, &false],
        )
        .await?;

    Ok(token.into())
}

pub async fn is_expired(
    db_client: &DbClient<'_>,
    token: &str,
) -> Result<bool, tokio_postgres::Error> {
    Ok(db_client
        .query_one(
            "SELECT is_expired FROM invite_tokens WHERE token = $1",
            &[&token],
        )
        .await?
        .get("is_expired"))
}

pub async fn expire(db_client: &DbClient<'_>, token: &str) -> Result<u64, tokio_postgres::Error> {
    db_client
        .execute(
            "UPDATE invite_tokens SET is_expired = true WHERE token = $1",
            &[&token],
        )
        .await
}
