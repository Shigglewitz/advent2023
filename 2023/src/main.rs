mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod utils;

fn main() {
    println!("Hello, world!");

    let mut days = Vec::new();
    days.push(Day {
        id: "01",
        part1: || day1::part1("real.txt").to_string(),
        part2: || day1::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "02",
        part1: || day2::part1("real.txt").to_string(),
        part2: || day2::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "03",
        part1: || day3::part1("real.txt").to_string(),
        part2: || day3::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "04",
        part1: || day4::part1("real.txt").to_string(),
        part2: || day4::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "05",
        part1: || day5::part1("real.txt").to_string(),
        part2: || day5::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "06",
        part1: || day6::part1("test.txt").to_string(),
        part2: || day6::part2("test.txt").to_string(),
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
    part1: fn() -> String,
    part2: fn() -> String,
}

// get more weird ascii art chars from here https://theasciicode.com.ar/extended-ascii-code/box-drawings-single-vertical-line-character-ascii-code-179.html
fn print_day_struct(day: &Day) {
    println!("│ Day {} │", day.id);
    println!("├────────┤");
    println!("│ Part 1 │ {}", (day.part1)().to_string());
    println!("│ Part 2 │ {}", (day.part2)().to_string());
}
