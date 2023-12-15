use askama::Template;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
};
use axum_extra::extract::Form;
use futures::future::try_join_all;

use serde::Deserialize;
use sqlx::PgPool;

use crate::todos::Todo;

use super::templates;

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[derive(Deserialize)]
pub struct TodoCreateParams {
    description: String,
}

pub async fn create(
    State(pool): State<PgPool>,
    Form(params): Form<TodoCreateParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let todo = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (description,position) VALUES ($1,((select max(position) from todos) + 1)) returning id, done, description, position;",
        params.description,
    )
    .fetch_one(&pool)
    .await
    .map_err(internal_error)?;
    let todo: templates::TodoLiTemplate = todo.into();
    let template = templates::TodoSwapOOBTemplate { todo };
    let mut headers = HeaderMap::new();
    headers.insert("HX-Trigger", "todoFormReset".parse().unwrap());
    Ok((headers, HtmlTemplate(template)))
}

pub async fn list(State(pool): State<PgPool>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let todos = sqlx::query_as!(
        Todo,
        "select id, done, description, position from todos ORDER BY position desc"
    )
    .fetch_all(&pool)
    .await
    .map_err(internal_error)?;
    let todos: Vec<templates::TodoLiTemplate> =
        todos.into_iter().map(|t| t.into()).collect::<Vec<_>>();
    let template = templates::TodosUlTemplate {
        todos: templates::TodosInnerTemplate { todos },
    };
    Ok(HtmlTemplate(template))
}

#[derive(Deserialize)]
pub struct TodoOrderingParams {
    order: Vec<String>,
}

pub async fn update_order(
    State(pool): State<PgPool>,
    Form(params): Form<TodoOrderingParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("order params: {:?}", params.order);
    let positions: Vec<(i32, i32)> = params
        .order
        .iter()
        .rev()
        .enumerate()
        .map(|(pos, id)| (pos as i32, id.parse().unwrap_or(0)))
        .collect::<Vec<_>>();
    let tx = pool.begin().await.map_err(internal_error)?;

    let queries = positions.iter().map(|(position, id)| async {
        sqlx::query_as!(
            Todo,
            "update todos set position = $1 where id = $2 RETURNING *;",
            position.clone(),
            id.clone()
        )
        .fetch_one(&pool)
        .await
    });
    let mut todos = try_join_all(queries).await.map_err(internal_error)?;
    todos.sort_by(|a, b| b.position.cmp(&a.position));
    tx.commit().await.map_err(internal_error)?;

    let todos: Vec<templates::TodoLiTemplate> =
        todos.into_iter().map(|t| t.into()).collect::<Vec<_>>();
    let template = templates::TodosInnerTemplate { todos };
    Ok(HtmlTemplate(template))
}

#[derive(Debug, Deserialize)]
pub struct TodoUpdateParams {
    done: Option<String>,
}

#[derive(Debug, Deserialize)]
enum CheckBox {
    On,
    Off,
}

impl From<CheckBox> for bool {
    fn from(val: CheckBox) -> Self {
        match val {
            CheckBox::On => true,
            CheckBox::Off => false,
        }
    }
}

impl From<String> for CheckBox {
    fn from(val: String) -> Self {
        if val == "on" {
            CheckBox::On
        } else {
            CheckBox::Off
        }
    }
}

pub async fn update(
    Path(todo_id): Path<i32>,
    State(pool): State<PgPool>,
    Form(params): Form<TodoUpdateParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let check_box: CheckBox = params.done.unwrap_or(String::from("Off")).into();
    let check_box: bool = check_box.into();

    sqlx::query_as!(
        Todo,
        "UPDATE todos set done = $1 where id = $2",
        check_box,
        todo_id,
    )
    .execute(&pool)
    .await
    .map_err(internal_error)?;
    Ok(StatusCode::OK)
}

pub async fn delete(
    Path(todo_id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    sqlx::query!("DELETE FROM todos where id = $1", todo_id)
        .execute(&pool)
        .await
        .map_err(internal_error)?;
    Ok(StatusCode::OK)
}
