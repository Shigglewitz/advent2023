mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn main() {
    println!("Day 1");
    println!("{}", day1::part1("real.txt").to_string());
    println!("{}", day1::part2("real.txt").to_string());
    println!("");

    println!("Day 2");
    println!("{}", day2::part1("real.txt").to_string());
    println!("{}", day2::part2("real.txt").to_string());
    println!("");

    println!("Day 3");
    println!("{}", day3::part1("real.txt").to_string());
    println!("{}", day3::part2("real.txt").to_string());
    println!("");

    println!("Day 4");
    println!("{}", day4::part1("real.txt").to_string());
    println!("{}", day4::part2("real.txt").to_string());
    println!("");

    println!("Day 5");
    println!("{}", day5::part1("real.txt"));
    println!("{}", day5::part2("real.txt"));
    println!("");

    println!("Day 6");
    println!("{}", day6::part1("real.txt"));
    println!("{}", day6::part2("real.txt"));
    println!("");

    println!("Day 7");
    println!("{}", day7::part1("real.txt"));
    println!("{}", day7::part2("real.txt"));
    println!("");

    println!("Day 8");
    println!("{}", day8::part1("real.txt"));
    println!("{}", day8::part2("real.txt"));
    println!("");

    println!("Day 9");
    println!("{}", day9::part1("real.txt"));
    println!("{}", day9::part2("real.txt"));
    println!("");
}
