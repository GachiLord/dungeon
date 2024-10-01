use std::fmt::{self, Display};

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize, Serializer,
};
use tokio::join;

use crate::libs::db::DbClient;

use super::task;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Class {
    C,
    B,
    A,
}

impl Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = match self {
            Class::C => "C",
            Class::B => "B",
            Class::A => "A",
        };
        f.write_str(l)
    }
}

impl Serialize for Class {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Class::C => "C",
            Class::B => "B",
            Class::A => "A",
        })
    }
}

struct ClassVisitor;

impl<'de> Visitor<'de> for ClassVisitor {
    type Value = Class;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 3")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v.as_str() {
            "C" => Ok(Class::C),
            "B" => Ok(Class::B),
            "A" => Ok(Class::A),
            _ => Err(E::custom("value must be one of A, B, C")),
        }
    }
}

impl<'de> Deserialize<'de> for Class {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(ClassVisitor)
    }
}

impl From<i16> for Class {
    fn from(value: i16) -> Self {
        match value {
            0 => Class::C,
            1 => Class::B,
            2 => Class::A,
            _ => Class::C,
        }
    }
}

impl Into<i16> for Class {
    fn into(self) -> i16 {
        match self {
            Class::C => 0,
            Class::B => 1,
            Class::A => 2,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub login: Box<str>,
    pub name: Box<str>,
    pub pw_hash: Box<str>,
    pub class: Class,
    pub is_admin: bool,
    pub tags: Vec<Box<str>>,
}

pub async fn create(db_client: &DbClient<'_>, user: User) -> Result<User, tokio_postgres::Error> {
    let u = db_client.query_one(
        "INSERT INTO users (login, name, password, class, is_admin) VALUES ($1, $2, $3, 0, false) RETURNING *",
        &[&user.login, &user.name, &user.pw_hash],
    ).await?;

    Ok(User {
        id: u.get("id"),
        login: u.get("login"),
        name: u.get("name"),
        pw_hash: u.get("password"),
        class: u.get::<&str, i16>("class").into(),
        is_admin: u.get("is_admin"),
        tags: u.get("tags"),
    })
}

pub async fn exists(db_client: &DbClient<'_>, login: &str) -> Result<bool, tokio_postgres::Error> {
    let row = db_client
        .query_one("SELECT COUNT(*) FROM users WHERE login = $1", &[&login])
        .await?;

    Ok(row.get::<&str, i64>("count") == 1)
}

pub async fn get(db_client: &DbClient<'_>, id: i32) -> Result<User, tokio_postgres::Error> {
    let u = db_client
        .query_one("SELECT * FROM users where id = $1", &[&id])
        .await?;

    Ok(User {
        id: u.get("id"),
        login: u.get("login"),
        name: u.get("name"),
        pw_hash: u.get("password"),
        class: u.get::<&str, i16>("class").into(),
        is_admin: u.get("is_admin"),
        tags: u.get("tags"),
    })
}

pub async fn get_by_login(
    db_client: &DbClient<'_>,
    login: &str,
) -> Result<User, tokio_postgres::Error> {
    let u = db_client
        .query_one("SELECT * FROM users where login = $1", &[&login])
        .await?;

    Ok(User {
        id: u.get("id"),
        login: u.get("login"),
        name: u.get("name"),
        pw_hash: u.get("password"),
        class: u.get::<&str, i16>("class").into(),
        is_admin: u.get("is_admin"),
        tags: u.get("tags"),
    })
}

pub async fn top_players(db_client: &DbClient<'_>) -> Result<Vec<User>, tokio_postgres::Error> {
    let users = db_client.query("SELECT * FROM users ORDER BY (SELECT COUNT(*) FROM completed_tasks WHERE user_id = users.id) DESC LIMIT 10", &[]).await.unwrap().into_iter().map(|row| User {
        id: row.get("id"),
        login: row.get("login"),
        name: row.get("name"),
        pw_hash: row.get("password"),
        class: row.get::<&str, i16>("class").into(),
        is_admin: row.get("is_admin"),
        tags: row.get("tags"),
    }).collect();

    Ok(users)
}

pub async fn top_players_by_class(
    db_client: &DbClient<'_>,
    class: Class,
) -> Result<Vec<User>, tokio_postgres::Error> {
    let c: i16 = class.into();
    let users = db_client.query("SELECT * FROM users WHERE class = $1 ORDER BY (SELECT COUNT(*) FROM completed_tasks WHERE user_id = users.id) DESC LIMIT 10", &[&c]).await.unwrap().into_iter().map(|row| User {
        id: row.get("id"),
        login: row.get("login"),
        name: row.get("name"),
        pw_hash: row.get("password"),
        class: row.get::<&str, i16>("class").into(),
        is_admin: row.get("is_admin"),
        tags: row.get("tags"),
    }).collect();

    Ok(users)
}

pub async fn calibrate_class(
    db_client: &DbClient<'_>,
    user_id: i32,
    user_class: Class,
) -> Result<u64, tokio_postgres::Error> {
    let (total, complexity) = join!(
        task::get_count(db_client, user_id),
        task::get_avg_complexity(db_client, user_id)
    );

    if total? % 10 != 0 {
        return Ok(0);
    }

    let estimated_class = (complexity?.round() as i16).into();
    if user_class != estimated_class {
        return set_class(db_client, user_id, estimated_class).await;
    }

    Ok(0)
}

pub async fn set_class(
    db_client: &DbClient<'_>,
    id: i32,
    class: Class,
) -> Result<u64, tokio_postgres::Error> {
    let class: i16 = class.into();
    db_client
        .execute("UPDATE users SET class = $1 WHERE id = $2", &[&class, &id])
        .await
}

pub async fn add_tags(
    db_client: &DbClient<'_>,
    id: i32,
    tags: Vec<Box<str>>,
) -> Result<u64, tokio_postgres::Error> {
    db_client
        .execute(
            "UPDATE users SET tags = (tags || $1) WHERE id = $2",
            &[&tags, &id],
        )
        .await
}
