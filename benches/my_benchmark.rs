use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_sandbox::my_iterators::sum_of_vector;

// https://bheisler.github.io/criterion.rs/book/getting_started.html

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("Sum of vector", |b| b.iter(|| sum_of_vector(black_box(vec![10, 10, 20]))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
