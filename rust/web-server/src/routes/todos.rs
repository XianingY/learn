use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::todo::{CreateTodo, Todo, UpdateTodo};

pub fn todo_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(list_todos).post(create_todo))
        .route("/{id}", get(get_todo).put(update_todo).delete(delete_todo))
}

async fn list_todos(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, title, description, completed, created_at, updated_at FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

async fn get_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, AppError> {
    let todo = sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, completed, created_at, updated_at FROM todos WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Todo with id {} not found", id)))?;

    Ok(Json(todo))
}

async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    let result = sqlx::query(
        "INSERT INTO todos (title, description) VALUES (?, ?) RETURNING id, title, description, completed, created_at, updated_at",
    )
    .bind(&input.title)
    .bind(&input.description)
    .fetch_one(&pool)
    .await?;

    let todo = Todo {
        id: sqlx::Row::get(&result, "id"),
        title: sqlx::Row::get(&result, "title"),
        description: sqlx::Row::get(&result, "description"),
        completed: sqlx::Row::get(&result, "completed"),
        created_at: sqlx::Row::get(&result, "created_at"),
        updated_at: sqlx::Row::get(&result, "updated_at"),
    };

    Ok((StatusCode::CREATED, Json(todo)))
}

async fn update_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let existing = sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, completed, created_at, updated_at FROM todos WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Todo with id {} not found", id)))?;

    let new_title = input.title.unwrap_or(existing.title);
    let new_description = input.description.or(existing.description);
    let new_completed = input.completed.unwrap_or(existing.completed);

    let result = sqlx::query(
        "UPDATE todos SET title = ?, description = ?, completed = ?, updated_at = datetime('now') WHERE id = ? RETURNING id, title, description, completed, created_at, updated_at",
    )
    .bind(&new_title)
    .bind(&new_description)
    .bind(new_completed)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    let todo = Todo {
        id: sqlx::Row::get(&result, "id"),
        title: sqlx::Row::get(&result, "title"),
        description: sqlx::Row::get(&result, "description"),
        completed: sqlx::Row::get(&result, "completed"),
        created_at: sqlx::Row::get(&result, "created_at"),
        updated_at: sqlx::Row::get(&result, "updated_at"),
    };

    Ok(Json(todo))
}

async fn delete_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Todo with id {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}
