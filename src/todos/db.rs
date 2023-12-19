use axum::http::StatusCode;
use sqlx::PgPool;

use crate::utils;

use super::Todo;

pub async fn create(description: &str, pool: &PgPool) -> Result<(), (StatusCode, String)> {
    sqlx::query!(
        "INSERT INTO todos (description,position) VALUES ($1,((select max(position) from todos) + 1));",
        description,
    )
    .execute(pool)
    .await
    .map_err(utils::internal_error)?;
    Ok(())
}

pub async fn delete(id: &i32, pool: &PgPool) -> Result<(), (StatusCode, String)> {
    sqlx::query!("DELETE FROM todos where id = $1", id)
        .execute(pool)
        .await
        .map_err(utils::internal_error)?;
    Ok(())
}

pub async fn get_todos(pool: &PgPool) -> Result<Vec<Todo>, (StatusCode, String)> {
    sqlx::query_as!(
        Todo,
        "select id, done, description, position from todos ORDER BY position desc"
    )
    .fetch_all(pool)
    .await
    .map_err(utils::internal_error)
}

pub async fn delete_todos(todos: Vec<Todo>, pool: &PgPool) -> Result<(), (StatusCode, String)> {
    let delete_ids = todos.iter().map(|t| t.id as i32).collect::<Vec<_>>();
    // https://github.com/launchbadge/sqlx/blob/main/FAQ.md#how-can-i-do-a-select--where-foo-in--query
    sqlx::query!("delete from todos where id = ANY($1)", &delete_ids)
        .execute(pool)
        .await
        .map_err(utils::internal_error)?;
    Ok(())
}

// Given a vec of (position, id), set the position for each todo by id
pub async fn set_positions(
    position_data: Vec<(i32, i32)>,
    pool: &PgPool,
) -> Result<(), (StatusCode, String)> {
    let positions = position_data
        .clone()
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<Vec<_>>();
    let ids = position_data
        .into_iter()
        .map(|(_, id)| id)
        .collect::<Vec<_>>();
    sqlx::query!(
        "update todos as original
         set position=new.position
         from (select unnest($1::int4[]) as position, unnest($2::int4[]) as id) as new
         where original.id=new.id;",
        &positions[..],
        &ids[..],
    )
    .execute(pool)
    .await
    .map_err(utils::internal_error)?;
    Ok(())
}
