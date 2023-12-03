use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_3::Schematic;

const INPUT: &str = include_str!("../src/input.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gear ratios", |b| {
        b.iter(|| {
            let schematic = Schematic::from(INPUT);
            black_box(schematic.sum_gear_ratios());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
