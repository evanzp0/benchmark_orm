use std::time::Instant;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use sqlx::FromRow;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Runtime;

#[allow(dead_code)]
#[derive(Debug)]
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
    let sql = "select * from test_user";
    let users = sqlx::query_as::<_, User>(&sql).fetch_all(conn).await.unwrap();
    Ok(users)
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sqlx");
    let rt  = Runtime::new().unwrap();
    let conn = rt.block_on(connect_db());

    group.bench_function("iter", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                // take the start time inside the future to make sure we only count once it's running
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
criterion_main! (benches);