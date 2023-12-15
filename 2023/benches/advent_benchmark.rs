use std::time::Duration;

use advent2023::day01;
use advent2023::day02;
use advent2023::day03;
use advent2023::day04;
use advent2023::day05;
use advent2023::day06;
use advent2023::day07;
use advent2023::day08;
use advent2023::day09;
use advent2023::day10;
use advent2023::day11;
use advent2023::day12;
use advent2023::day13;
use advent2023::day14;
use advent2023::day15;

use advent2023::utils;

use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("advent_2023");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));

    let file_name = "real.txt";

    let day15_input = &utils::read_file("day15", "real.txt");
    group.bench_function("d15_p1", |bencher| {
        bencher.iter(|| day15::part1_with_input(day15_input))
    });
    group.bench_function("d15_p2", |bencher| {
        bencher.iter(|| day15::part2_with_input(day15_input))
    });
    group.bench_function("d14_p1", |bencher| bencher.iter(|| day14::part1(file_name)));
    group.bench_function("d14_p2", |bencher| bencher.iter(|| day14::part2(file_name)));
    group.bench_function("d13_p1", |bencher| bencher.iter(|| day13::part1(file_name)));
    group.bench_function("d13_p2", |bencher| bencher.iter(|| day13::part2(file_name)));
    group.bench_function("d12_p1", |bencher| bencher.iter(|| day12::part1(file_name)));
    group.bench_function("d12_p2", |bencher| bencher.iter(|| day12::part2(file_name)));
    group.bench_function("d11_p1", |bencher| bencher.iter(|| day11::part1(file_name)));
    group.bench_function("d11_p2", |bencher| bencher.iter(|| day11::part2(file_name)));
    group.bench_function("d10_p1", |bencher| bencher.iter(|| day10::part1(file_name)));
    group.bench_function("d10_p2", |bencher| bencher.iter(|| day10::part2(file_name)));
    group.bench_function("d09_p1", |bencher| bencher.iter(|| day09::part1(file_name)));
    group.bench_function("d09_p2", |bencher| bencher.iter(|| day09::part2(file_name)));
    group.bench_function("d08_p1", |bencher| bencher.iter(|| day08::part1(file_name)));
    group.bench_function("d08_p2", |bencher| bencher.iter(|| day08::part2(file_name)));
    group.bench_function("d07_p1", |bencher| bencher.iter(|| day07::part1(file_name)));
    group.bench_function("d07_p2", |bencher| bencher.iter(|| day07::part2(file_name)));
    group.bench_function("d06_p1", |bencher| bencher.iter(|| day06::part1(file_name)));
    group.bench_function("d06_p2", |bencher| bencher.iter(|| day06::part2(file_name)));
    group.bench_function("d05_p1", |bencher| bencher.iter(|| day05::part1(file_name)));
    group.bench_function("d05_p2", |bencher| bencher.iter(|| day05::part2(file_name)));
    group.bench_function("d04_p1", |bencher| bencher.iter(|| day04::part1(file_name)));
    group.bench_function("d04_p2", |bencher| bencher.iter(|| day04::part2(file_name)));
    group.bench_function("d03_p1", |bencher| bencher.iter(|| day03::part1(file_name)));
    group.bench_function("d03_p2", |bencher| bencher.iter(|| day03::part2(file_name)));
    benchmark_day03(&mut group);
    benchmark_day02(&mut group);
    benchmark_day01(&mut group);

    group.finish();
}

fn benchmark_day03(group: &mut BenchmarkGroup<WallTime>) {
    let input = &utils::read_file("day03", "real.txt");
    group.bench_function("d03_p1", |bencher| {
        bencher.iter(|| day03::part1_with_input(input))
    });
    group.bench_function("d03_p2", |bencher| {
        bencher.iter(|| day03::part2_with_input(input))
    });
}

fn benchmark_day02(group: &mut BenchmarkGroup<WallTime>) {
    let input = &utils::read_file("day02", "real.txt");
    group.bench_function("d02_p1", |bencher| {
        bencher.iter(|| day02::part1_with_input(input))
    });
    group.bench_function("d02_p2", |bencher| {
        bencher.iter(|| day02::part2_with_input(input))
    });
}

fn benchmark_day01(group: &mut BenchmarkGroup<WallTime>) {
    let input = &utils::read_file("day01", "real.txt");
    group.bench_function("d01_p1", |bencher| {
        bencher.iter(|| day01::part1_with_input(input))
    });
    group.bench_function("d01_p2", |bencher| {
        bencher.iter(|| day01::part2_with_input(input))
    });
}

criterion_group!(benches, benchmark_all);
criterion_main!(benches);
