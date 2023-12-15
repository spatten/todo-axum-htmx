use super::Todo;
use askama::Template;

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

#[derive(Template)]
#[template(path = "todo_swap_oob.html")]
pub struct TodoSwapOOBTemplate {
    pub todo: TodoLiTemplate,
}

#[derive(Template)]
#[template(path = "todo_li.html")]
pub struct TodoLiTemplate {
    id: i64,
    done: bool,
    description: String,
}
