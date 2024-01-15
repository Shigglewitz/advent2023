use std::env;

use advent::*;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let pattern = if args.len() > 1 { &args[1] } else { "" };

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
    println!("│ Part 1 │ {}", day.solve_part1());
    println!("│ Part 2 │ {}", day.solve_part2());
}
