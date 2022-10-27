use std::time::Instant;

use criterion::{Criterion, criterion_group};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use sea_orm::entity::prelude::*;
use tokio::runtime::Runtime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "test_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// Entity is auto gen by sea-orm
type User = Entity; 

async fn connect_db() -> DatabaseConnection {
    let mut opt = ConnectOptions::new("postgres://root:111111@127.0.0.1/my_database".to_owned());
    opt.max_connections(5)
        .min_connections(0)
        // .connect_timeout(Duration::from_secs(8))
        // .idle_timeout(Duration::from_secs(8))
        // .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(opt).await.unwrap();
    db
}

#[inline]
async fn fetch_all_user_simple(conn: &DatabaseConnection) -> dysql::DySqlResult<Vec<Model>> {
    let users = User::find().all(conn).await?;
    Ok(users)
}


#[inline]
async fn fetch_all_user_complex(conn: &DatabaseConnection) -> dysql::DySqlResult<Vec<Model>> {
    let users = User::find()
        .filter(Column::Name.eq("zhangsan".to_owned()))
        .filter(Column::Age.eq(35))
        .all(conn).await?;
    Ok(users)
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sea-orm");
    let rt  = Runtime::new().unwrap();
    let conn = rt.block_on(connect_db());

    group.bench_function("simple", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = Instant::now();
                for _ in 0..iters {
                    criterion::black_box(
                        fetch_all_user_simple(&conn).await.unwrap()
                    );
                }
                start.elapsed()
            })
        });
    });

    group.bench_function("complex", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = Instant::now();
                for _ in 0..iters {
                    criterion::black_box(
                        fetch_all_user_complex(&conn).await.unwrap()
                    );
                }
                start.elapsed()
            })
        });
    });
}

criterion_group!(benches, bench);