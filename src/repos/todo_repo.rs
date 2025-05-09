use sqlx::PgPool;
use crate::models::todo::{Todo, CreateTodo, UpdateTodo};
use uuid::Uuid;
use chrono::{Utc, DateTime};


pub async fn create_todo(
    pool: &PgPool,
    todo: CreateTodo,
) -> Result<Todo, sqlx::Error> {
    let id = Uuid::new_v4();
    let created_at: DateTime<Utc> = Utc::now();

    let todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (id, title, created_at)
        VALUES ($1, $2, $3)
        RETURNING id, title, completed, created_at
        "#,
        id,
        todo.title,
        created_at
    )
        .fetch_one(pool)
        .await?;

    Ok(todo)
}

pub async fn get_all_todos(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, completed, created_at FROM todos
        "#,
    )
        .fetch_all(pool)
        .await?;

    Ok(todos)
}

pub async fn get_todo_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Todo>, sqlx::Error> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, completed, created_at
        FROM todos
        WHERE id = $1
        "#,
        id
    )
        .fetch_optional(pool)
        .await?;

    Ok(todo)
}

pub async fn update_todo(
    pool: &PgPool,
    id: Uuid,
    update: UpdateTodo,
) -> Result<Option<Todo>, sqlx::Error> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET title = COALESCE($1, title), completed = COALESCE($2, completed)
        WHERE id = $3
        RETURNING id, title, completed, created_at
        "#,
        update.title,
        update.completed,
        id
    )
        .fetch_optional(pool)
        .await?;

    Ok(todo)
}

pub async fn delete_todo(
    pool: &PgPool,
    id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM todos
        WHERE id = $1
        "#,
        id
    )
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}