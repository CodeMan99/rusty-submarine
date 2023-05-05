use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

#[derive(Debug)]
struct User {
    id: i64,
    username: String,
    first: Option<String>,
    last: Option<String>,
    age: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("Must set DATABASE_URL.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let user = User {
        id: 0,
        username: String::from("xavier5a"),
        first: None,
        last: None,
        age: Some(12),
    };
    let id = create_user(&pool, user).await?;

    dbg!(id);

    let users = list_users(&pool).await?;

    dbg!(users);

    if let Some(tris) = find_user(&pool, String::from("tris")).await? {
        dbg!(&tris);

        let age = tris.age.unwrap_or(10);
        let update_count = set_user_age(&pool, tris.id, age - 1).await?;

        dbg!(update_count);
    } else {
        eprintln!("User 'tris' was not found");
    }

    let delete_count = delete_user(&pool, id).await?;

    dbg!(delete_count);

    Ok(())
}

async fn list_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, username, first, last, age FROM public.user;"
    )
    .fetch_all(pool)
    .await
}

async fn create_user(pool: &PgPool, user: User) -> Result<i64, sqlx::Error> {
    let record = sqlx::query!(
        "INSERT INTO public.user(username, first, last, age) VALUES ($1, $2, $3, $4) RETURNING id;",
        user.username,
        user.first,
        user.last,
        user.age
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}

async fn find_user(pool: &PgPool, username: String) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, username, first, last, age FROM public.user WHERE username = $1 LIMIT 1;",
        username
    )
    .fetch_optional(pool)
    .await
}

async fn set_user_age(pool: &PgPool, id: i64, age: i32) -> Result<u64, sqlx::Error> {
    let record = sqlx::query!("UPDATE public.user SET age = $1 WHERE id = $2;", age, id)
        .execute(pool)
        .await?;

    Ok(record.rows_affected())
}

async fn delete_user(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM public.user WHERE id = $1;", id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
