use indoc::formatdoc;

pub mod responses;

#[derive(Debug, Clone)]
struct Todo {
    id: i64,
    done: bool,
    description: String,
    position: i64,
}

impl Todo {
    fn to_li(&self) -> String {
        let checked = if self.done { "checked=checked" } else { "" };
        let id = self.id;
        formatdoc!(
            r##"
            <li>
              <span class="delete" hx-delete="/todo/{id}" hx-target="closest li" hx-swap="delete">&#10060;</span>
              <input type="checkbox" id="todo-{id}-checkbox" {checked} name="done" hx-put="/todo/{id}" hx-swap="none" hx-include="this">
              <label for="todo-{id}-checkbox">{}</label>
              <input type='hidden' name='order' value='{id}'/>
            </li>"##,
            html_escape::encode_safe(&self.description),
        )
    }
}

fn todos_inner(todos: Vec<Todo>) -> String {
    todos
        .iter()
        .map(|t| t.to_li())
        .collect::<Vec<_>>()
        .join("\n")
}

fn todos_ul(todos: Vec<Todo>) -> String {
    formatdoc!(
        r#"
        <ul id="todos" hx-post="todos/ordering" hx-swap="this" hx-trigger="drop-end" hx-include="[name=order]">
            {}
        </form>
    "#,
        todos_inner(todos)
    )
}
