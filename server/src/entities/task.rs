use std::{collections::HashSet, hash::RandomState};

use serde::{Deserialize, Serialize};
use tokio::join;

use super::user::{Class, User};
use crate::{entities::user, libs::db::DbClient};

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub id: i32,
    pub complexity: Class,
    pub description: Box<str>,
    pub expected_time: f32,
    pub tags: Vec<Box<str>>,
    pub assigned_to: Option<i32>,
}

pub struct TaskCreateData {
    pub complexity: Class,
    pub expected_time: f32,
    pub tags: Vec<Box<str>>,
    pub description: Box<str>,
}

pub async fn create(
    db_client: &DbClient<'_>,
    task: &TaskCreateData,
) -> Result<Task, tokio_postgres::Error> {
    let c: i16 = task.complexity.into();
    let row = db_client
        .query_one(
            "INSERT INTO tasks (complexity, expected_time, tags, description) VALUES ($1, $2, $3, $4) returning *",
            &[&c, &task.expected_time, &task.tags, &task.description],
        )
        .await?;
    Ok(Task {
        id: row.get("id"),
        complexity: row.get::<&str, i16>("complexity").into(),
        description: row.get("description"),
        expected_time: row.get("expected_time"),
        tags: row.get("tags"),
        assigned_to: row.get("assigned_to"),
    })
}

pub async fn assigned_to(
    db_client: &DbClient<'_>,
    task_id: i32,
) -> Result<Option<i32>, tokio_postgres::Error> {
    let row = db_client
        .query_one("SELECT assigned_to FROM tasks WHERE id = $1", &[&task_id])
        .await;
    match row {
        Ok(r) => Ok(r.get("assigned_to")),
        Err(_) => Ok(None),
    }
}

pub async fn assign_to(
    db_client: &DbClient<'_>,
    task_id: i32,
    user_id: Option<i32>,
) -> Result<u64, tokio_postgres::Error> {
    db_client
        .execute(
            "UPDATE tasks SET assigned_to = $1 WHERE id = $2",
            &[&user_id, &task_id],
        )
        .await
}

pub async fn complete(
    db_client: &DbClient<'_>,
    task_id: i32,
    user: User,
) -> Result<u64, tokio_postgres::Error> {
    let u_tags: HashSet<Box<str>, RandomState> = HashSet::from_iter(user.tags.into_iter());
    let t_tags: HashSet<Box<str>, RandomState> =
        HashSet::from_iter(get_tags(&db_client, task_id).await?.into_iter());
    let extra_tags = t_tags.difference(&u_tags).cloned().collect();
    let (q1, q2, q3) = join!(
        // add tags
        async { user::add_tags(&db_client, user.id, extra_tags).await },
        // mark task as completed
        async {
            db_client
                .execute(
                    "INSERT INTO completed_tasks (user_id, task_id) VALUES ($1, $2)",
                    &[&user.id, &task_id],
                )
                .await
        },
        // calibrate class
        async {
            user::calibrate_class(&db_client, user.id, user.class)
                .await
                .unwrap()
        }
    );
    Ok(q1? + q2? + q3)
}

pub async fn delete(db_client: &DbClient<'_>, task_id: i32) -> Result<u64, tokio_postgres::Error> {
    db_client
        .execute("DELETE FROM tasks WHERE id = $1", &[&task_id])
        .await
}

pub async fn get_assigned(
    db_client: &DbClient<'_>,
    user_id: i32,
) -> Result<Vec<Task>, tokio_postgres::Error> {
    Ok(db_client
        .query("SELECT * from tasks WHERE assigned_to = $1 AND NOT EXISTS(SELECT * FROM completed_tasks WHERE task_id = tasks.id)", &[&user_id])
        .await?
        .into_iter()
        .map(|row| Task {
            id: row.get("id"),
            complexity: row.get::<&str, i16>("complexity").into(),
            description: row.get("description"),
            expected_time: row.get("expected_time"),
            tags: row.get("tags"),
            assigned_to: row.get("assigned_to"),
        })
        .collect())
}

pub async fn get_available(db_client: &DbClient<'_>) -> Result<Vec<Task>, tokio_postgres::Error> {
    Ok(db_client
        .query("SELECT * from tasks WHERE assigned_to is NULL AND NOT EXISTS (SELECT 1 FROM completed_tasks WHERE task_id = tasks.id)", &[])
        .await
        .unwrap()
        .into_iter()
        .map(|row| Task {
            id: row.get("id"),
            description: row.get("description"),
            complexity: row.get::<&str, i16>("complexity").into(),
            expected_time: row.get("expected_time"),
            tags: row.get("tags"),
            assigned_to: row.get("assigned_to"),
        })
        .collect())
}

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
) -> Result<f32, tokio_postgres::Error> {
    Ok(db_client
        .query_one(
            "SELECT CAST(AVG(complexity) AS REAL) FROM completed_tasks JOIN tasks ON user_id = $1",
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
            "SELECT CAST(AVG(expected_time) AS REAL) FROM completed_tasks JOIN tasks ON user_id = $1",
            &[&user_id],
        )
        .await?
        .try_get("avg")
        .unwrap_or(0.0))
}

pub async fn get_tags(
    db_client: &DbClient<'_>,
    task_id: i32,
) -> Result<Vec<Box<str>>, tokio_postgres::Error> {
    Ok(db_client
        .query_one("SELECT tags FROM tasks WHERE id = $1", &[&task_id])
        .await?
        .get("tags"))
}
