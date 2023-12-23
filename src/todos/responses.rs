use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};

use axum_extra::extract::Form;

use serde::Deserialize;
use sqlx::{postgres::PgArguments, query::Query, PgPool, Pool, Postgres};

use crate::utils;
use crate::utils::HtmlTemplate;

use super::{db, templates};

// todos routes, nested under /todos
pub fn routes(pool: &Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", put(update).delete(destroy))
        .route("/move_complete_to_bottom", post(move_complete_to_bottom))
        .route("/delete_completed", delete(delete_completed))
        .route("/ordering", post(update_order))
        .route("/:id/edit", get(edit))
        .with_state(pool.clone())
}

#[derive(Deserialize)]
struct TodoCreateParams {
    description: String,
}

// post /todos
async fn create(
    State(pool): State<PgPool>,
    Form(params): Form<TodoCreateParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    sqlx::query!(
        "INSERT INTO todos (description,position) VALUES ($1,((select max(position) from todos) + 1));",
        params.description,
    )
    .execute(&pool)
    .await
    .map_err(utils::internal_error)?;

    let template = templates::render_all_todos(&pool).await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        "HX-Trigger",
        "todoFormReset"
            .parse()
            .expect("should be able to create a HX-Trigger header"),
    );
    Ok((headers, HtmlTemplate(template)))
}

// get /todos
async fn list(State(pool): State<PgPool>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let inner_template = templates::render_all_todos(&pool).await?;
    let template = templates::TodosUlTemplate {
        todos: inner_template,
    };
    Ok(HtmlTemplate(template))
}

// get /todos/:id/edit
async fn edit(
    Path(editable_id): Path<i32>,
    pool: State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let todos = db::get_todos(&pool).await?;
    let template = templates::render_todos(todos, Some(editable_id));
    Ok(HtmlTemplate(template))
}

// post /todos/move_complete_to_bottom
async fn move_complete_to_bottom(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut todos = db::get_todos(&pool).await?;
    todos.sort_by(|a, b| a.position.cmp(&b.position));
    let (mut completed, mut pending): (Vec<_>, Vec<_>) = todos.into_iter().partition(|t| t.done);
    completed.append(&mut pending);
    let positions = completed
        .iter()
        .enumerate()
        .map(|(position, todo)| (position as i32, todo.id))
        .collect::<Vec<_>>();
    db::set_positions(positions, &pool).await?;
    let template = templates::render_all_todos(&pool).await?;
    Ok(HtmlTemplate(template))
}

// post /todos/delete_completed
async fn delete_completed(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let todos = db::get_todos(&pool).await?;
    let (completed, pending): (Vec<_>, Vec<_>) = todos.into_iter().partition(|t| t.done);

    // Delete the completed ones
    db::delete_todos(completed, &pool).await?;

    let template = templates::render_todos(pending, None);
    Ok(HtmlTemplate(template))
}

#[derive(Deserialize)]
struct TodoOrderingParams {
    order: Vec<String>,
}

// post /todos/ordering
async fn update_order(
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
    db::set_positions(positions, &pool).await?;

    let template = templates::render_all_todos(&pool).await?;
    Ok(HtmlTemplate(template))
}

#[derive(Debug, Deserialize)]
struct TodoUpdateParams {
    done: Option<String>,
    description: Option<String>,
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

// put /todos/:id
async fn update(
    Path(todo_id): Path<i32>,
    State(pool): State<PgPool>,
    Form(params): Form<TodoUpdateParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let check_box: CheckBox = params.done.unwrap_or(String::from("Off")).into();
    let check_box: bool = check_box.into();
    let query: Query<'_, Postgres, PgArguments>;

    // Right now, updates come from either the edit form (which just ships up a description)
    // or clicking the checkbox (which just ships up the check_box)
    // So we only set one or the other
    if let Some(description) = params.description {
        query = sqlx::query!(
            "Update todos set description = $1 where id = $2",
            description,
            todo_id,
        )
    } else {
        query = sqlx::query!(
            "UPDATE todos set done = $1 where id = $2",
            check_box,
            todo_id,
        )
    }
    query.execute(&pool).await.map_err(utils::internal_error)?;

    let template = templates::render_all_todos(&pool).await?;
    Ok(HtmlTemplate(template))
}

// delete /todos/:id
async fn destroy(
    Path(todo_id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    sqlx::query!("DELETE FROM todos where id = $1", todo_id)
        .execute(&pool)
        .await
        .map_err(utils::internal_error)?;

    let template = templates::render_all_todos(&pool).await?;
    Ok(HtmlTemplate(template))
}
