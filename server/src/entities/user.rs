use serde::{Serialize, Serializer};

use crate::libs::db::DbClient;

#[derive(Debug, Clone, Copy)]
pub enum Class {
    C,
    B,
    A,
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
    })
}

pub async fn set_class(
    db_client: &DbClient<'_>,
    id: i64,
    class: Class,
) -> Result<u64, tokio_postgres::Error> {
    let class: i16 = class.into();
    db_client
        .execute("UPDATE users SET class = $1 WHERE id = $2", &[&class, &id])
        .await
}
