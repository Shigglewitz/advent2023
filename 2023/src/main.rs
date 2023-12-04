mod day1;
mod day2;
mod day3;
mod day4;
mod utils;

fn main() {
    println!("Hello, world!");

    let mut days = Vec::new();
    days.push(Day {
        id: "01",
        part1: || day1::part1("real.txt"),
        part2: || day1::part2("real.txt"),
    });
    days.push(Day {
        id: "02",
        part1: || day2::part1("real.txt"),
        part2: || day2::part2("real.txt"),
    });
    days.push(Day {
        id: "03",
        part1: || day3::part1("real.txt"),
        part2: || day3::part2("real.txt"),
    });
    days.push(Day {
        id: "04",
        part1: || day4::part1("real.txt"),
        part2: || day4::part2("real.txt"),
    });
    for (index, day) in days.iter().rev().enumerate() {
        if index == 0 {
            println!("┌────────┐");
        } else {
            println!("├────────┤");
        }
        print_day_struct(day);
    }
    println!("└────────┘");

    println!("Farewell, world!");
}

struct Day {
    id: &'static str,
    part1: fn() -> i32,
    part2: fn() -> i32,
}

// get more weird ascii art chars from here https://theasciicode.com.ar/extended-ascii-code/box-drawings-single-vertical-line-character-ascii-code-179.html
fn print_day_struct(day: &Day) {
    println!("│ Day {} │", day.id);
    println!("├────────┤");
    println!("│ Part 1 │ {}", (day.part1)().to_string());
    println!("│ Part 2 │ {}", (day.part2)().to_string());
}
