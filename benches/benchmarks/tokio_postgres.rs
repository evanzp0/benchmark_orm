use std::time::Instant;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use deadpool_postgres::Config;
use deadpool_postgres::ManagerConfig;
use deadpool_postgres::Pool;
use deadpool_postgres::PoolConfig;
use deadpool_postgres::RecyclingMethod;
use tokio::runtime::Runtime;
// use tokio_postgres::Client;
// use tokio_postgres::Config;
// use tokio_postgres::connect;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::NoTls;

#[derive(Debug)]
#[derive(PostgresMapper, PartialEq)]
#[pg_mapper(table="test_user")]
struct User {
    id: i64,
    name: Option<String>,
    age: Option<i32>
}

async fn connect_db() -> Pool {
    // let (client, connection) = connect("host=127.0.0.1 user=root password=111111 dbname=my_database", NoTls).await.unwrap();
    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         eprintln!("connection error: {}", e);
    //     }
    // });
    // client // tokio_postgres::Client
    let mut cfg = Config::new();
    cfg.host = Some("127.0.0.1".to_owned());
    cfg.port = Some(5432);
    cfg.user = Some("root".to_owned());
    cfg.password = Some("111111".to_owned());
    cfg.dbname = Some("my_database".to_owned());
    cfg.pool = Some(PoolConfig::new(5));
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool = cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls).unwrap();

    pool
}

async fn fetch_all_user(pool: &Pool) -> dysql::DySqlResult<Vec<User>> {
    let conn = pool.get().await.unwrap();

    let sql = "select * from test_user";
    let rows = conn.query(sql, &[]).await?;
    let users = rows
        .iter()
        .map(|row| User::from_row_ref(row).expect("query unexpected error"))
        .collect::<Vec<User>>();
    Ok(users)
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("tokio-postgres");
    let rt  = Runtime::new().unwrap();
    let pool = rt.block_on(connect_db());

    group.bench_function("simple", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                // take the start time inside the future to make sure we only count once it's running
                let start = Instant::now();
                for _ in 0..iters {
                    criterion::black_box(
                        fetch_all_user(&pool).await.unwrap()
                    );
                }
                start.elapsed()
            })
        });
    });
}

criterion_group!(benches, bench);
criterion_main! (benches);