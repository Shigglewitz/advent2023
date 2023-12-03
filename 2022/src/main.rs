mod day1;
mod utils;

fn main() {
    println!("Day 1");
    println!("{}", day1::part1("real.txt").to_string());
    println!("{}", day1::part2("real.txt").to_string());
}
