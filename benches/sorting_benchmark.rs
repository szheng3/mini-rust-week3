use rust_week3::fibonacci;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};



fn fibonacci_benchmark(c: &mut Criterion) {
    let mut inputs = black_box(
        10
    );

    c.bench_function(
        "fibonacci algorithm",
        |b| b.iter(|| fibonacci(inputs))
    );
}
criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);