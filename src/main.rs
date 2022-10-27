
// use dysql_macro::fetch_all;
// use ramhorns::Content;
// use sqlx::FromRow;
// use sqlx::Pool;
// use sqlx::Postgres;
// use sqlx::postgres::PgPoolOptions;

// // #[tokio::main]
// // async fn main() {
// //     fetch_all_user();
// // }

// #[derive(Content)]
// struct UserDto {
//     id: Option<i64>,
//     name: Option<String>,
//     age: Option<i32>
// }

// #[allow(dead_code)]
// #[derive(Debug, PartialEq)]
// #[derive(FromRow)]
// struct User {
//     id: i64,
//     name: Option<String>,
//     age: Option<i32>
// }

// async fn connect_postgres_db() -> Pool<Postgres> {
//     let conn = PgPoolOptions::new()
//         .max_connections(5)
//         .connect("postgres://root:111111@127.0.0.1/my_database").await.unwrap();

//     conn
// }

// async fn fetch_all_user(conn: &Pool<Postgres>) -> dysql::DySqlResult<Vec<User>> {
//     // let conn = connect_postgres_db().await;
    
//     // fetch all
//     let dto = UserDto{ id: None, name: None, age: Some(15) };
//     let _rst = fetch_all!(|dto, conn| -> User {
//         "SELECT * FROM test_user 
//         WHERE 1 = 1
//           {{#name}}AND name = :name{{/name}}
//           {{#age}}AND age > :age{{/age}}
//         ORDER BY id"
//     });

//     Ok(_rst)
// }

// use tokio::runtime::Runtime;

// fn main() {
    // let rt  = Runtime::new().unwrap();
    // let conn = rt.block_on(connect_postgres_db());
    // let a = rt.block_on(fetch_all_user(&conn));
    // let a = a.unwrap();

    // println!("{:?}", a);
// }

use tokio_postgres::{NoTls, Error};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    Ok(())
}