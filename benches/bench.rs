#[macro_use]
extern crate criterion;

use advent_of_code::*;
use criterion::Criterion;

// fn day1p1(c: &mut Criterion) {
//     c.bench_function("day1p1", |b| b.iter(day1::part1));
// }
// criterion_group!(b_day1p1, day1p1);

// fn day1p2(c: &mut Criterion) {
//     c.bench_function("day1p2", |b| b.iter(day1::part2));
// }
// criterion_group!(b_day1p2, day1p2);

// fn day2p1(c: &mut Criterion) {
//     c.bench_function("day2p1", |b| b.iter(day2::part1));
// }
// criterion_group!(b_day2p1, day2p1);

// fn day2p2(c: &mut Criterion) {
//     c.bench_function("day2p2", |b| b.iter(day2::part2));
// }
// criterion_group!(b_day2p2, day2p2);

// fn day3p1(c: &mut Criterion) {
//     c.bench_function("day3p1", |b| b.iter(day3::part1));
// }
// criterion_group!(b_day3p1, day3p1);

// fn day3p2(c: &mut Criterion) {
//     c.bench_function("day3p2", |b| b.iter(day3::part2));
// }
// criterion_group!(b_day3p2, day3p2);

// fn day4p1(c: &mut Criterion) {
//     c.bench_function("day4p1", |b| b.iter(day4::part1));
// }
// criterion_group!(b_day4p1, day4p1);

// fn day4p2(c: &mut Criterion) {
//     c.bench_function("day4p2", |b| b.iter(day4::part2));
// }
// criterion_group!(b_day4p2, day4p2);

fn day5p1(c: &mut Criterion) {
    c.bench_function("day5p1", |b| b.iter(day5::part1));
}
criterion_group!(b_day5p1, day5p1);

fn day5p2(c: &mut Criterion) {
    c.bench_function("day5p2", |b| b.iter(day5::part2));
}
criterion_group!(b_day5p2, day5p2);

criterion_main!(b_day5p1);
