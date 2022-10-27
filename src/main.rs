
use rbatis::{Rbatis, py_sql, executor::Executor};
use serde::{Serialize, Deserialize};
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

#[tokio::main]
async fn main() {
    let rb = connect_db().await;
    rb.init(rbdc_pg::driver::PgDriver {}, "postgres://root:111111@127.0.0.1/my_database").unwrap();
    let mut rb = rb.clone();

    let users = fetch_all_user_simple(&mut rb, "").await.unwrap();
    println!("{:?}", users);
}