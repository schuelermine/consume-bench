use consume_bench::{step_count_from_checked_consume, step_count_from_checked_last};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_step_count_from_checked_consume(c: &mut Criterion) {
    c.bench_function("step_count_from_checked_consume", |b| {
        b.iter(|| {
            let mut range = -2..=i16::MAX as isize;
            let count: Option<i16> = step_count_from_checked_consume(&mut range, -1);
            assert!(count.is_none());
            assert!(range.is_empty());
        })
    });
}

fn bench_step_count_from_checked_last(c: &mut Criterion) {
    c.bench_function("step_count_from_checked_last", |b| {
        b.iter(|| {
            let mut range = -2..=i16::MAX as isize;
            let count: Option<i16> = step_count_from_checked_last(&mut range, -1);
            assert!(count.is_none());
            assert!(range.is_empty());
        })
    });
}

criterion_group!(
    benches,
    bench_step_count_from_checked_consume,
    bench_step_count_from_checked_last
);
criterion_main!(benches);
