use std::fmt::{self};

use crate::BaseTemplate;

use super::{db, Todo};
use askama::Template;
use axum::http::StatusCode;
use sqlx::PgPool;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum TodoUiState {
    Normal,
    Editable,
    Disabled,
}

impl fmt::Display for TodoUiState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            TodoUiState::Normal => "normal",
            TodoUiState::Editable => "editable",
            TodoUiState::Disabled => "disabled",
        })?;
        Ok(())
    }
}
#[derive(Template)]
#[template(path = "todos_index.html")]
pub struct TodosIndexTemplate<'a> {
    _parent: &'a BaseTemplate,
}

// impl<'a> Deref for TodosIndexTemplate<'a> {
//     type Target = BaseTemplate;

//     fn deref(&self) -> &Self::Target {
//         self._parent
//     }
// }

pub fn render_index<'a>() -> TodosIndexTemplate<'a> {
    TodosIndexTemplate {
        _parent: &BaseTemplate {},
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
    pub editable: bool,
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
    ui_state: TodoUiState,
}

impl From<Todo> for TodoLiTemplate {
    fn from(todo: Todo) -> Self {
        TodoLiTemplate {
            done: todo.done,
            id: todo.id,
            description: todo.description,
            ui_state: TodoUiState::Normal,
        }
    }
}

pub async fn render_all_todos(pool: &PgPool) -> Result<TodosInnerTemplate, (StatusCode, String)> {
    let todos = db::get_todos(pool).await?;
    Ok(render_todos(todos, None))
}

pub fn render_todos(todos: Vec<Todo>, editable_id: Option<i32>) -> TodosInnerTemplate {
    let todos: Vec<TodoLiTemplate> = todos.into_iter().map(|t| t.into()).collect::<Vec<_>>();
    if let Some(editable_id) = editable_id {
        let todos = todos
            .into_iter()
            .map(|mut t| {
                if t.id == editable_id {
                    t.ui_state = TodoUiState::Editable;
                } else {
                    t.ui_state = TodoUiState::Disabled;
                }
                t
            })
            .collect::<Vec<_>>();
        return TodosInnerTemplate {
            todos,
            editable: true,
        };
    }
    TodosInnerTemplate {
        todos,
        editable: false,
    }
}
