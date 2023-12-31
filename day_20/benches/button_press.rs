use std::array;

use criterion::{criterion_group, criterion_main, Criterion};
use day_20::{Schema, BROADCASTER};

const INPUT: &str = include_str!("../src/input.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    let schema = Schema::parse(INPUT);
    let rx = schema.ids["rx"];
    let mut queues = array::from_fn(|_| Vec::with_capacity(schema.modules.len()));

    c.bench_function("button press", |b| {
        b.iter(|| {
            queues[0].push(schema.ids[BROADCASTER]);
            schema.button_press(rx, &mut queues)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
