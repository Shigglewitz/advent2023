mod day1;
mod day2;

fn main() {
    println!("Hello, world!");

    let total = day1::part1("real.txt");
    // 55834
    println!("day1, part 2 is {}", total.to_string());
    // 2156
    println!("day2 part 1 is {}", day2::part1("real.txt").to_string());
    println!("day2 part 2 is {}", day2::part2("real.txt").to_string());

    println!("Farewell, world!");
}
