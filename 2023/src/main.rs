mod day1;
mod day2;
mod day3;
mod day4;
mod utils;

fn main() {
    println!("Hello, world!");

    println!("┌────────┐");
    print_day("01", day1::part1("real.txt"), day1::part2("real.txt"));
    println!("├────────┤");
    print_day("02", day2::part1("real.txt"), day2::part2("real.txt"));
    println!("├────────┤");
    print_day("03", day3::part1("real.txt"), day3::part2("real.txt"));
    println!("├────────┤");
    print_day("04", day4::part1("test.txt"), day4::part2("test.txt"));
    println!("└────────┘");

    println!("Farewell, world!");
}

// get more weird ascii art chars from here https://theasciicode.com.ar/extended-ascii-code/box-drawings-single-vertical-line-character-ascii-code-179.html
fn print_day(day_id: &str, part1: i32, part2: i32) {
    println!("│ Day {} │", day_id);
    println!("├────────┤");
    println!("│ Part 1 │ {}", part1.to_string());
    println!("│ Part 2 │ {}", part2.to_string());
}
