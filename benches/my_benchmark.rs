use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_sandbox::my_iterators::sum_of_vector;
use rust_sandbox::my_rayon::{sum_of_squares_sequential, sum_of_squares};

// https://bheisler.github.io/criterion.rs/book/getting_started.html

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    // c.bench_function("Sum of vector", |b| b.iter(|| sum_of_vector(black_box(vec![10, 10, 20]))));

    
    let vector: Vec<u64> = (1..999999).map(|x| x*x).collect();
    c.bench_function("Sum of squares sequential", |b| b.iter(|| sum_of_squares_sequential(black_box(&vector))));
    c.bench_function("Sum of squares", |b| b.iter(|| sum_of_squares(black_box(&vector))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
