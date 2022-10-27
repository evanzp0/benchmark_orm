use std::time::Instant;

use criterion::{Criterion, criterion_group, criterion_main};
use rbatis::{Rbatis, py_sql, executor::Executor};
use serde::{Serialize, Deserialize};
use tokio::runtime::Runtime;
use rbatis::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    id: i64,
    name: Option<String>,
    age: Option<i32>
}

async fn connect_db() -> Rbatis { 
    let rb = Rbatis::new();
    rb.init(rbdc_pg::driver::PgDriver {}, "postgres://root:111111@127.0.0.1/my_database").unwrap();
    rb
}

#[py_sql(
    "`select * from test_user"
)]
async fn fetch_all_user_simple(rb: &mut dyn Executor, name: &str) -> Result<Vec<User>, Error> {
    impled!()
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("rbatis");
    let rt  = Runtime::new().unwrap();
    let mut rb = rt.block_on(connect_db());

    group.bench_function("simple", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                // take the start time inside the future to make sure we only count once it's running
                let start = Instant::now();
                for _ in 0..iters {
                    criterion::black_box(
                        fetch_all_user_simple(&mut rb, "").await.unwrap()
                    );
                }
                start.elapsed()
            })
        });
    });
}

criterion_group!(benches, bench);
criterion_main! (benches);