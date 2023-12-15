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
    id: i64,
    done: bool,
    description: String,
}
