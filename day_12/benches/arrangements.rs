use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_12::{parse_row, try_match};

pub fn criterion_benchmark(c: &mut Criterion) {
    let (record, groups) = parse_row("?????#???????????#?# 1,4,1,2,1,1");
    let (record, groups) = (record.into_iter(), groups.into_iter());
    c.bench_function("arrangements", |b| {
        b.iter(|| try_match(black_box(record.clone()), black_box(groups.clone()), &[]))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
