mod benchmarks;

use criterion::criterion_main;

criterion_main! {
    // benchmarks::bench_1::benches_1,
    // benchmarks::bench_2::benches_a,
    
    benchmarks::tokio_postgres::benches,
    benchmarks::sqlx::benches,
    benchmarks::dysql_sqlx::benches,
}