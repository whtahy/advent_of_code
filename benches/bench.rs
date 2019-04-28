#[macro_use]
extern crate criterion;

use advent_of_code::*;
use criterion::Criterion;

// fn day1p1(c: &mut Criterion) {
//     c.bench_function("day1p1", |b| b.iter(day1::part1));
// }

// fn day1p2(c: &mut Criterion) {
//     c.bench_function("day1p2", |b| b.iter(day1::part2));
// }

// fn day2p1(c: &mut Criterion) {
//     c.bench_function("day2p1", |b| b.iter(day2::part1));
// }

// fn day2p2(c: &mut Criterion) {
//     c.bench_function("day2p2", |b| b.iter(day2::part2));
// }

// fn day3p1(c: &mut Criterion) {
//     c.bench_function("day3p1", |b| b.iter(day3::part1));
// }

// fn day3p2(c: &mut Criterion) {
//     c.bench_function("day3p2", |b| b.iter(day3::part2));
// }

fn day4p1(c: &mut Criterion) {
    c.bench_function("day4p1", |b| b.iter(day4::part1));
}

fn day4p2(c: &mut Criterion) {
    c.bench_function("day4p2", |b| b.iter(day4::part2));
}

criterion_group!(b_day4p1, day4p1);
criterion_group!(b_day4p2, day4p2);

criterion_main!(b_day4p1, b_day4p2);
