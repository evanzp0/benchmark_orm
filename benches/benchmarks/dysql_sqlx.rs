use std::time::Instant;

use criterion::Criterion;
use criterion::criterion_group;
use dysql_macro::fetch_all;
use ramhorns::Content;
use sqlx::FromRow;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Runtime;

#[derive(Content)]
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[derive(FromRow)]
struct User {
    id: i64,
    name: Option<String>,
    age: Option<i32>
}

async fn connect_db() -> Pool<Postgres> {
    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:111111@127.0.0.1/my_database").await.unwrap();

    conn
}

async fn fetch_all_user(conn: &Pool<Postgres>) -> dysql::DySqlResult<Vec<User>> {
    let dto = UserDto{ id: None, name: None, age: Some(15) };
    let users = fetch_all!(|dto, conn| -> User {
        "SELECT * FROM test_user"
    });

    Ok(users)
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("dysql_sqlx");
    let rt  = Runtime::new().unwrap();
    let conn = rt.block_on(connect_db());

    group.bench_function("iter", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = Instant::now();
                for _ in 0..iters {
                    criterion::black_box(
                        fetch_all_user(&conn).await.unwrap()
                    );
                }
                start.elapsed()
            })
        });
    });
}

criterion_group!(benches, bench);
