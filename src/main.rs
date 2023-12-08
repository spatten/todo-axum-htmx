use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post, put},
    Router,
};
use axum_extra::extract::Form;
use indoc::formatdoc;
use listenfd::ListenFd;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    // Connect to postgres
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://localhost/todo-axum-htmx")
        .await
        .expect("should be able to connect to DB");
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await
        .expect("should be able to make a query");
    assert_eq!(row.0, 150);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Serve files from the client directory, falling back to client/404.html
    let serve_dir = ServeDir::new("client").not_found_service(ServeFile::new("client/404.html"));

    // Respond to GET /search, otherwise attempt to serve the file from the client directory
    // Also, add our postgres pool to the state so that our routes can use it
    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todo/:id", put(update_todo).delete(delete_todo))
        .route("/todos/ordering", post(update_order))
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // Auto-reload if you use `make watch`: https://github.com/tokio-rs/axum/blob/main/examples/auto-reload/src/main.rs
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd
        .take_tcp_listener(0)
        .expect("should be able to find an existing listener")
    {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            TcpListener::from_std(listener).expect("should be able to listen to existing listener")
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:3000")
            .await
            .expect("should be able to bind to 3000"),
    };
    axum::serve(listener, app)
        .await
        .expect("should be able to serve");
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[derive(Debug, Clone)]
struct Todo {
    id: i64,
    done: bool,
    description: String,
    position: f32,
}

impl Todo {
    fn to_li(&self) -> String {
        let checked = if self.done { "checked=checked" } else { "" };
        let id = self.id;
        formatdoc!(
            r##"
            <li>
              <span class="delete" hx-delete="/todo/{id}" hx-target="closest li" hx-swap="delete">&#10060;</span>
              <input type="checkbox" id="todo-{id}-checkbox" {checked} name="done" hx-put="/todo/{id}" hx-include="#todo-{id}-checkbox input[name=done]">
              <label for="todo-{id}-checkbox">{}</label>
              <input type='hidden' name='order' value='{id}'/>
            </li>"##,
            self.description,
        )
    }
}

fn todos_ul(todos: Vec<Todo>) -> String {
    formatdoc!(
        r#"
        <ul id="todos" hx-post="todos/ordering" hx-trigger="drop-end" hx-include="[name=order]">
            {}
        </form>
    "#,
        todos
            .iter()
            .map(|t| t.to_li())
            .collect::<Vec<_>>()
            .join("\n")
    )
}
async fn list_todos(State(pool): State<PgPool>) -> Result<Html<String>, (StatusCode, String)> {
    let todos = sqlx::query_as!(
        Todo,
        "select id, done, description, position from todos ORDER BY id desc"
    )
    .fetch_all(&pool)
    .await
    .map_err(internal_error)?;
    let ul = todos_ul(todos);
    Ok(Html(ul))
}

#[derive(Deserialize)]
struct TodoOrderingParams {
    order: Vec<String>,
}

async fn update_order(
    State(pool): State<PgPool>,
    Form(params): Form<TodoOrderingParams>,
) -> Result<Html<String>, (StatusCode, String)> {
    println!("order params: {:?}", params.order);
    let todos = sqlx::query_as!(
        Todo,
        "select id, done, description, position from todos ORDER BY id desc"
    )
    .fetch_all(&pool)
    .await
    .map_err(internal_error)?;
    let ul = todos_ul(todos);
    Ok(Html(ul))
}

#[derive(Deserialize)]
struct TodoCreateParams {
    description: String,
}

async fn create_todo(
    State(pool): State<PgPool>,
    Form(params): Form<TodoCreateParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let sql_result = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (description,position) VALUES ($1,((select max(position) from todos) + 1)) returning id, done, description, position;",
        params.description,
    )
    .fetch_one(&pool)
    .await
    .map_err(internal_error)?;
    let todo: String = sql_result.to_li();
    let wrapped = formatdoc!(
        r#"
    <div hx-swap-oob="afterbegin:#todos">
      {todo}
    </div>
    "#
    );
    let mut headers = HeaderMap::new();
    headers.insert("HX-Trigger", "todoFormReset".parse().unwrap());
    Ok((headers, Html(wrapped)))
}

#[derive(Debug, Deserialize)]
struct TodoUpdateParams {
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

async fn update_todo(
    Path(todo_id): Path<i32>,
    State(pool): State<PgPool>,
    Form(params): Form<TodoUpdateParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("todo id: {todo_id}");
    println!("form: {:?}", params);
    let check_box: CheckBox = params.done.unwrap_or(String::from("Off")).into();
    let check_box: bool = check_box.into();
    println!("checkbox: {:?}", check_box);

    let sql_result = sqlx::query_as!(
        Todo,
        "UPDATE todos set done = $1 where id = $2 RETURNING id, done, description, position",
        check_box,
        todo_id,
    )
    .fetch_one(&pool)
    .await
    .map_err(internal_error)?;
    let todo: String = sql_result.to_li();
    Ok(Html(todo))
}

async fn delete_todo(
    Path(todo_id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("todo id: {todo_id}");

    sqlx::query!("DELETE FROM todos where id = $1", todo_id)
        .execute(&pool)
        .await
        .map_err(internal_error)?;
    Ok(Html(""))
}
