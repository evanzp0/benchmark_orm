use criterion::*;
use criterion::async_executor::FuturesExecutor;

async fn do_something(size: usize) {
    // Do something async with the size
}

fn from_elem(c: &mut Criterion) {
    let size: usize = 1024;

    c.bench_with_input(BenchmarkId::new("input_example", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.to_async(FuturesExecutor).iter(|| do_something(s));
    });
}

criterion_group!(benches_1, from_elem);