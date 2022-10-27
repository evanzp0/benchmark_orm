use criterion::*;
use criterion::black_box;
use criterion::async_executor::FuturesExecutor;
use std::time::Instant;

async fn foo() {
    // ...
}

fn bench(c: &mut Criterion) {
    c.bench_function("iter", move |b| {
        b.to_async(FuturesExecutor).iter_custom(|iters| {
            async move {
                let start = Instant::now();
                for _i in 0..iters {
                    black_box(foo().await);
                }
                start.elapsed()
            }
        })
    });
}

criterion_group!(benches_a, bench);
