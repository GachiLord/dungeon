use crate::libs::db::DbClient;

pub async fn get_count(
    db_client: &DbClient<'_>,
    user_id: i32,
) -> Result<i64, tokio_postgres::Error> {
    Ok(db_client
        .query_one(
            "SELECT COUNT(*) FROM completed_tasks WHERE user_id = $1",
            &[&user_id],
        )
        .await?
        .get("count"))
}

pub async fn get_avg_complexity(
    db_client: &DbClient<'_>,
    user_id: i32,
) -> Result<f64, tokio_postgres::Error> {
    Ok(db_client
        .query_one(
            "SELECT AVG(complexity) FROM completed_tasks JOIN tasks ON user_id = $1",
            &[&user_id],
        )
        .await?
        .try_get("avg")
        .unwrap_or(0.0))
}

pub async fn get_avg_duration(
    db_client: &DbClient<'_>,
    user_id: i32,
) -> Result<f32, tokio_postgres::Error> {
    Ok(db_client
        .query_one(
            "SELECT AVG(expected_time) FROM completed_tasks JOIN tasks ON user_id = $1",
            &[&user_id],
        )
        .await?
        .try_get("avg")
        .unwrap_or(0.0))
}
