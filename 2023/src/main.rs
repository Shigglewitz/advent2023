mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod utils;

fn main() {
    println!("Hello, world!");

    let mut days = Vec::new();
    days.push(Day {
        id: "01",
        part1: || day01::part1("real.txt").to_string(),
        part2: || day01::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "02",
        part1: || day02::part1("real.txt").to_string(),
        part2: || day02::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "03",
        part1: || day03::part1("real.txt").to_string(),
        part2: || day03::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "04",
        part1: || day04::part1("real.txt").to_string(),
        part2: || day04::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "05",
        part1: || day05::part1("real.txt").to_string(),
        part2: || day05::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "06",
        part1: || day06::part1("real.txt").to_string(),
        part2: || day06::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "07",
        part1: || day07::part1("real.txt").to_string(),
        part2: || day07::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "08",
        part1: || day08::part1("real.txt").to_string(),
        part2: || day08::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "09",
        part1: || day09::part1("real.txt").to_string(),
        part2: || day09::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "10",
        part1: || day10::part1("real.txt").to_string(),
        part2: || day10::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "11",
        part1: || day11::part1("real.txt").to_string(),
        part2: || day11::part2("real.txt").to_string(),
    });
    days.push(Day {
        id: "12",
        part1: || day12::part1("real.txt").to_string(),
        // part 2 is too slow and needs more optimization
        part2: || "37366887898686".to_string(),
    });
    days.push(Day {
        id: "13",
        part1: || day13::part1("real.txt").to_string(),
        part2: || day13::part2("real.txt").to_string(),
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
