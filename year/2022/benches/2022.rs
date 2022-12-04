// TODO: DRY
use aoc_2022::days::*;
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

// pub fn day_5(c: &mut Criterion) {
//     c.bench_function("day 5", |b| b.iter(day2::main));
// }
//
// pub fn day_6(c: &mut Criterion) {
//     c.bench_function("day 6", |b| b.iter(day3::main));
// }

// pub fn day_7(c: &mut Criterion) {
//     c.bench_function("day 7", |b| b.iter(day1::main));
// }
//
// pub fn day_8(c: &mut Criterion) {
//     c.bench_function("day 8", |b| b.iter(day2::main));
// }
//
// pub fn day_9(c: &mut Criterion) {
//     c.bench_function("day 9", |b| b.iter(day3::main));
// }

// pub fn day_10(c: &mut Criterion) {
//     c.bench_function("day 10", |b| b.iter(day1::main));
// }
//
// pub fn day_11(c: &mut Criterion) {
//     c.bench_function("day 11", |b| b.iter(day2::main));
// }
//
// pub fn day_12(c: &mut Criterion) {
//     c.bench_function("day 12", |b| b.iter(day3::main));
// }

// pub fn day_13(c: &mut Criterion) {
//     c.bench_function("day 13", |b| b.iter(day1::main));
// }
//
// pub fn day_14(c: &mut Criterion) {
//     c.bench_function("day 14", |b| b.iter(day2::main));
// }
//
// pub fn day_15(c: &mut Criterion) {
//     c.bench_function("day 15", |b| b.iter(day3::main));
// }

// pub fn day_16(c: &mut Criterion) {
//     c.bench_function("day 16", |b| b.iter(day1::main));
// }
//
// pub fn day_17(c: &mut Criterion) {
//     c.bench_function("day 17", |b| b.iter(day2::main));
// }
//
// pub fn day_18(c: &mut Criterion) {
//     c.bench_function("day 18", |b| b.iter(day3::main));
// }

// pub fn day_19(c: &mut Criterion) {
//     c.bench_function("day 19", |b| b.iter(day1::main));
// }
//
// pub fn day_20(c: &mut Criterion) {
//     c.bench_function("day 20", |b| b.iter(day2::main));
// }
//
// pub fn day_21(c: &mut Criterion) {
//     c.bench_function("day 21", |b| b.iter(day3::main));
// }

// pub fn day_22(c: &mut Criterion) {
//     c.bench_function("day 22", |b| b.iter(day1::main));
// }
//
// pub fn day_23(c: &mut Criterion) {
//     c.bench_function("day 23", |b| b.iter(day2::main));
// }
//
// pub fn day_24(c: &mut Criterion) {
//     c.bench_function("day 24", |b| b.iter(day3::main));
// }

// pub fn day_25(c: &mut Criterion) {
//     c.bench_function("day 25", |b| b.iter(day3::main));
// }

criterion_group!(
    all_days, day_1, day_2, day_3,
    day_4, /* day_5, day_6, day_7, day_8, day_9, day_10, day_11,
           day_12, day_13, day_14, */
);
criterion_main!(all_days);
