use criterion::{black_box, criterion_group, criterion_main, Criterion};
use database::relational::{self, *};
pub fn criterion_benchmark(c: &mut Criterion) {
    let y = relational::bench_test();
    c.bench_function("fib 20", |b| b.iter(|| black_box(relational::bench_test())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
