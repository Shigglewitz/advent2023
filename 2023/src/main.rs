use advent2023::*;

fn main() {
    println!("Hello, world!");

    let mut advent_days: Vec<AdventDay> = Vec::new();
    advent_days.push(day01::create("real.txt"));
    advent_days.push(day02::create("real.txt"));
    advent_days.push(day03::create("real.txt"));
    advent_days.push(day04::create("real.txt"));
    advent_days.push(day05::create("real.txt"));
    advent_days.push(day06::create("real.txt"));
    advent_days.push(day07::create("real.txt"));
    advent_days.push(day08::create("real.txt"));
    advent_days.push(day09::create("real.txt"));
    advent_days.push(day10::create("real.txt"));
    advent_days.push(day11::create("real.txt"));
    advent_days.push(day12::create("real.txt"));
    advent_days.push(day13::create("real.txt"));
    advent_days.push(day14::create("real.txt"));
    advent_days.push(day15::create("real.txt"));
    advent_days.push(day16::create("real.txt"));
    advent_days.push(day17::create("real.txt"));
    advent_days.push(day18::create("real.txt"));

    for (index, day) in advent_days.iter().rev().enumerate() {
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
