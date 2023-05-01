use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::Postgres;
use sqlx::pool::Pool;

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
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@pgrusty:5432/rusty").await?;

    let users = list_users(pool).await?;

    dbg!(users);

    Ok(())
}

async fn list_users(pool: Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT id, username, first, last, age FROM public.user;")
        .fetch_all(&pool)
        .await
}
