use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::Postgres;

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
        .connect("postgres://postgres:postgres@pgrusty:5432/rusty")
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

    Ok(())
}

async fn list_users(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, username, first, last, age FROM public.user;"
    )
    .fetch_all(pool)
    .await
}

async fn create_user(pool: &Pool<Postgres>, user: User) -> Result<i64, sqlx::Error> {
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
