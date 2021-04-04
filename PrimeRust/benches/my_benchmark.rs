use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
const N_MAX: usize = 1_000_000;

use primelib::Prime;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| {
            let mut sieve = Prime::new(black_box(N_MAX));
            sieve.run_sieve();
        })
    });
}

// criterion_group!(benches, criterion_benchmark);

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().measurement_time(Duration::from_secs(6));
    targets = criterion_benchmark
}

criterion_main!(benches);
