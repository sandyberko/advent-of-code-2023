use criterion::{criterion_group, criterion_main, Criterion};
use day_20::{Schema, BROADCASTER};

const INPUT: &str = include_str!("../src/input.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    let schema = Schema::parse(INPUT);
    let rx = schema.ids["rx"];
    let inital_dest = [schema.ids[BROADCASTER]];

    c.bench_function("button press", |b| {
        b.iter(|| schema.button_press(inital_dest, rx))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
