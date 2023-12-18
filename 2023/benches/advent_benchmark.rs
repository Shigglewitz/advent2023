use std::time::Duration;

use advent2023::*;

use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("advent_2023");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));

    let map = advent_day_map();
    let mut days = map.iter().collect::<Vec<_>>();
    days.sort_by(|a, b| b.0.cmp(a.0));
    days.iter()
        .for_each(|(_, day)| benchmark_day(&mut group, day));

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
