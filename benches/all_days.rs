use aoc_2021::days::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1", |b| b.iter(|| day01::main()));
    c.bench_function("day 2", |b| b.iter(|| day02::main()));
    c.bench_function("day 3", |b| b.iter(|| day03::main()));
    c.bench_function("day 4", |b| b.iter(|| day04::main()));
    c.bench_function("day 5", |b| b.iter(|| day05::main()));
    c.bench_function("day 6", |b| b.iter(|| day06::main()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
