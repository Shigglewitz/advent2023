use std::{env, time::Instant};

use advent::*;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let pattern = if args.len() > 1 { &args[1] } else { "2022" };

    let map = advent_day_map();
    let mut days = map
        .iter()
        .filter(|(title, _)| title.matches(pattern).next().is_some())
        .collect::<Vec<_>>();
    days.sort_by(|a, b| b.0.cmp(a.0));
    for (index, (_, day)) in days.iter().enumerate() {
        if index == 0 {
            println!("┌────────┐");
        } else {
            println!("├────────┤");
        }
        print_advent_day(day);
    }

    println!("└────────┘");

    println!("Farewell, world!");
}

fn print_advent_day(day: &AdventDay) {
    println!("│ Day {} │", day.id);
    println!("├────────┤");
    let p1_start = Instant::now();
    let p1 = day.solve_part1();
    let p1_duration = p1_start.elapsed();
    println!("│ Part 1 │ {} {:?}", p1, p1_duration);
    let p2_start = Instant::now();
    let p2 = day.solve_part2();
    let p2_duration = p2_start.elapsed();
    println!("│ Part 2 │ {} {:?}", p2, p2_duration);
}
