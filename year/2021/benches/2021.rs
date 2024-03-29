// TODO: DRY
use aoc_2021::days::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn day_1(c: &mut Criterion) {
    c.bench_function("day 1", |b| b.iter(day1::main));
}

pub fn day_2(c: &mut Criterion) {
    c.bench_function("day 2", |b| b.iter(day2::main));
}

pub fn day_3(c: &mut Criterion) {
    c.bench_function("day 3", |b| b.iter(day3::main));
}

pub fn day_4(c: &mut Criterion) {
    c.bench_function("day 4", |b| b.iter(day4::main));
}

pub fn day_5(c: &mut Criterion) {
    c.bench_function("day 5", |b| b.iter(day5::main));
}

pub fn day_6(c: &mut Criterion) {
    c.bench_function("day 6", |b| b.iter(day6::main));
}

pub fn day_7(c: &mut Criterion) {
    c.bench_function("day 7", |b| b.iter(day7::main));
}

pub fn day_8(c: &mut Criterion) {
    c.bench_function("day 8", |b| b.iter(day8::main));
}

pub fn day_9(c: &mut Criterion) {
    c.bench_function("day 9", |b| b.iter(day9::main));
}

pub fn day_10(c: &mut Criterion) {
    c.bench_function("day 10", |b| b.iter(day10::main));
}

pub fn day_11(c: &mut Criterion) {
    c.bench_function("day 11", |b| b.iter(day11::main));
}

pub fn day_12(c: &mut Criterion) {
    c.bench_function("day 12", |b| b.iter(day12::main));
}

pub fn day_13(c: &mut Criterion) {
    c.bench_function("day 13", |b| b.iter(day13::main));
}

pub fn day_14(c: &mut Criterion) {
    c.bench_function("day 14", |b| b.iter(day14::main));
}

criterion_group!(
    all_days, day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, day_9, day_10, day_11,
    day_12, day_13, day_14,
);
criterion_main!(all_days);
