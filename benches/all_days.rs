use aoc_2021::days::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn day_1(c: &mut Criterion) {
    c.bench_function("day 1", |b| b.iter(|| day01::main()));
}

pub fn day_2(c: &mut Criterion) {
    c.bench_function("day 2", |b| b.iter(|| day02::main()));
}

pub fn day_3(c: &mut Criterion) {
    c.bench_function("day 3", |b| b.iter(|| day03::main()));
}

pub fn day_4(c: &mut Criterion) {
    c.bench_function("day 4", |b| b.iter(|| day04::main()));
}

pub fn day_5(c: &mut Criterion) {
    c.bench_function("day 5", |b| b.iter(|| day05::main()));
}

pub fn day_6(c: &mut Criterion) {
    c.bench_function("day 6", |b| b.iter(|| day06::main()));
}

pub fn day_7(c: &mut Criterion) {
    c.bench_function("day 7", |b| b.iter(|| day07::main()));
}

criterion_group!(all_days, day_1, day_2, day_3, day_4, day_5, day_6, day_7);
criterion_main!(all_days);
