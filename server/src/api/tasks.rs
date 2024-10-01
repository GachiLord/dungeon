use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::{delete, patch, post},
    Form, Router,
};
use serde::Deserialize;

use crate::{
    entities::{
        task::{self, TaskCreateData},
        user::Class,
    },
    libs::auth::AuthSession,
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/edit", post(create))
        .route("/edit/:task_id", delete(delete_task))
        .route("/manage/assign/:task_id", patch(assign_to))
        .route("/manage/resign/:task_id", patch(resign))
        .route("/manage/complete/:task_id", patch(complete))
}

#[derive(Deserialize)]
struct TaskCreateForm {
    tags: Box<str>,
    description: Box<str>,
    complexity: i16,
    expected_time: f32,
}

async fn create(
    session: AuthSession,
    State(state): State<AppState>,
    Form(payload): Form<TaskCreateForm>,
) -> impl IntoResponse {
    let u = session.user.unwrap();

    if !u.is_admin {
        return Html::from("<p>Недостаточно прав для совершения заклинания</p>").into_response();
    }

    if let Ok(task) = task::create(
        &state.pool.try_get().await.unwrap(),
        &TaskCreateData {
            complexity: Class::from(payload.complexity),
            expected_time: payload.expected_time,
            description: payload.description,
            tags: payload.tags.split(" ").map(|v| v.into()).collect(),
        },
    )
    .await
    {
        // f this template lib not allowing me to do this
        return Html::from(format!("
            <div class='rpgui-container framed-golden' style='position: relative; max-width: 600px; margin-bottom: 20px; display: flex; flex-direction: column; justify-content: space-evenly; margin: 5px;'>
                <h1 style='color: #ff0; display: none;'>Выполняется</h1>
                <p>Тэги: <font color='#ff0'>[{}]</font></p>
                <p>Рекомендуемый класс авантюриста: <font color='#ff0'>{}</font></p>
                <p>Ожидаемое время выполнения в часах: <font color='#ff0'>{}</font></p>
                <div>
                    <hr>
                    <p sytle='line-break: normal;'>{}</p>
                </div>
                <div class='rpgui-center' style='position: relative;'>
                    <hr>
                    <button class='rpgui-button' type='button' hx-patch='/api/task/manage/assign/{}' hx-target='this' hx-swap='outerHTML' onclick='setTaskActive(this)'><p>Принять</p></button>
                    <button class='rpgui-button' type='button' hx-delete='/api/task/edit/{}' hx-target='closest div'><p>Удалить</p></button>
                </div>
            </div>
        ",
        task.tags.join(","),
        task.complexity,
        task.expected_time,
        task.description,
        task.id,
        task.id
        )).into_response();
    }

    return Html::from("<p>Неожиданная ошибка судьбы</p>").into_response();
}

async fn delete_task(
    session: AuthSession,
    Path(task_id): Path<i32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let u = session.user.unwrap();

    if !u.is_admin {
        return Html::from("<p>Недостаточно прав для совершения заклинания</p>").into_response();
    }

    if let Ok(_) = task::delete(&state.pool.try_get().await.unwrap(), task_id).await {
        return Html::from(format!("<p>Задание удалено</p>")).into_response();
    }

    return Html::from("<p>Неожиданная ошибка судьбы</p>").into_response();
}

async fn assign_to(
    session: AuthSession,
    Path(task_id): Path<i32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let u = session.user.unwrap();
    let db_client = state.pool.try_get().await.unwrap();

    if let Ok(v) = task::assigned_to(&db_client, task_id).await {
        match v {
            Some(v) => {
                if v != u.id {
                    return Html::from("<p>Вы не можете взять чужое задание</p>").into_response();
                }
            }
            None => (),
        }
        let _ = task::assign_to(&db_client, task_id, Some(u.id)).await;
        return Html::from(format!("
                <button class='rpgui-button' type='button' hx-patch='/api/task/manage/complete/{task_id}' hx-target='closest div' onclick='setTaskInactive(this)'><p>Завершить</p></button>
                <button class='rpgui-button' type='button' hx-patch='/api/task/manage/resign/{task_id}' hx-target='previous button' hx-swap='outerHTML' hx-on::before-request='this.remove()' onclick='setTaskInactive(this)'><p>Отказаться</p></button>"
                ))
            .into_response();
    }

    return Html::from("<p>Неожиданная ошибка судьбы</p>").into_response();
}

async fn resign(
    session: AuthSession,
    Path(task_id): Path<i32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let u = session.user.unwrap();
    let db_client = state.pool.try_get().await.unwrap();

    if let Ok(v) = task::assigned_to(&db_client, task_id).await {
        match v {
            Some(v) => {
                if v != u.id {
                    return Html::from("<p>Вы не можете отказаться от чужого задания</p>")
                        .into_response();
                }
            }
            None => (),
        }
        let _ = task::assign_to(&db_client, task_id, None).await;
        return Html::from(format!(
            "<button class='rpgui-button' type='button' hx-patch='/api/task/manage/assign/{task_id}' hx-target='this' hx-swap='outerHTML' onclick='setTaskActive(this)'><p>Принять</p></button>"
        ))
        .into_response();
    }

    return Html::from("<p>Неожиданная ошибка судьбы</p>").into_response();
}

async fn complete(
    session: AuthSession,
    Path(task_id): Path<i32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let u = session.user.unwrap();
    let db_client = state.pool.try_get().await.unwrap();

    if let Ok(v) = task::assigned_to(&db_client, task_id).await {
        match v {
            Some(v) => {
                if v != u.id {
                    return Html::from("<p>Вы не можете завершить чужое задание</p>")
                        .into_response();
                }
            }
            None => (),
        }
        let _ = task::complete(&db_client, task_id, u).await;
        return Html::from(format!("<p>Вы завершили заказ под номером {task_id}</p>"))
            .into_response();
    }

    return Html::from("<p>Неожиданная ошибка судьбы</p>").into_response();
}
