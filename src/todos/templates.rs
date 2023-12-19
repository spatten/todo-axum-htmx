use super::{db, Todo};
use askama::Template;
use axum::http::StatusCode;
use sqlx::PgPool;

impl From<Todo> for TodoLiTemplate {
    fn from(todo: Todo) -> Self {
        TodoLiTemplate {
            done: todo.done,
            id: todo.id,
            description: todo.description,
        }
    }
}

#[derive(Template)]
#[template(path = "todos_ul.html")]
pub struct TodosUlTemplate {
    pub todos: TodosInnerTemplate,
}

#[derive(Template)]
#[template(path = "todos_inner.html")]
pub struct TodosInnerTemplate {
    pub todos: Vec<TodoLiTemplate>,
}

impl TodosInnerTemplate {
    fn done_count(&self) -> usize {
        self.todos
            .iter()
            .filter(|t| t.done)
            .collect::<Vec<_>>()
            .len()
    }

    fn pending_count(&self) -> usize {
        self.todos
            .iter()
            .filter(|t| !t.done)
            .collect::<Vec<_>>()
            .len()
    }
}

#[derive(Template)]
#[template(path = "todo_li.html")]
pub struct TodoLiTemplate {
    id: i32,
    done: bool,
    description: String,
}

pub async fn render_all_todos(pool: &PgPool) -> Result<TodosInnerTemplate, (StatusCode, String)> {
    let todos = db::get_todos(pool).await?;
    Ok(render_todos(todos))
}

pub fn render_todos(todos: Vec<Todo>) -> TodosInnerTemplate {
    let todos: Vec<TodoLiTemplate> = todos.into_iter().map(|t| t.into()).collect::<Vec<_>>();
    TodosInnerTemplate { todos }
}
