use std::time::Duration;

use advent2023::*;

use advent2023::utils;

use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("advent_2023");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));

    benchmark_day(&mut group, &day17::create("real.txt"));
    benchmark_day(&mut group, &day16::create("real.txt"));
    benchmark_day(&mut group, &day15::create("real.txt"));
    benchmark_day(&mut group, &day14::create("real.txt"));
    benchmark_day(&mut group, &day13::create("real.txt"));
    benchmark_day(&mut group, &day12::create("real.txt"));
    benchmark_day(&mut group, &day11::create("real.txt"));
    benchmark_day(&mut group, &day10::create("real.txt"));
    benchmark_day(&mut group, &day09::create("real.txt"));
    benchmark_day(&mut group, &day08::create("real.txt"));
    benchmark_day(&mut group, &day07::create("real.txt"));
    benchmark_day(&mut group, &day06::create("real.txt"));
    benchmark_day(&mut group, &day05::create("real.txt"));
    benchmark_day(&mut group, &day04::create("real.txt"));
    benchmark_day(&mut group, &day03::create("real.txt"));
    benchmark_day(&mut group, &day02::create("real.txt"));
    benchmark_day(&mut group, &day01::create("real.txt"));

    group.finish();
}

type Group<'a> = BenchmarkGroup<'a, WallTime>;

fn benchmark_day(group: &mut Group, day: &AdventDay) {
    group.bench_function(format!("d{}_p1", day.id), |bencher| {
        bencher.iter(|| day.solve_part1())
    });
    group.bench_function(format!("d{}_p2", day.id), |bencher| {
        bencher.iter(|| day.solve_part2())
    });
}

criterion_group!(benches, benchmark_all);
criterion_main!(benches);
