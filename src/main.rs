mod day1;
mod day2;
mod utils;

fn main() {
    println!("Hello, world!");

    println!("day1, part 1 is {}", day1::part1("real.txt").to_string());
    println!("day1, part 2 is {}", day1::part2("real.txt").to_string());

    println!("day2 part 1 is {}", day2::part1("real.txt").to_string());
    println!("day2 part 2 is {}", day2::part2("real.txt").to_string());

    println!("Farewell, world!");
}
