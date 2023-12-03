mod day1;
mod day2;
mod day3;
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
}
