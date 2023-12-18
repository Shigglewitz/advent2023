use std::collections::HashMap;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod utils;

pub struct AdventDay {
    pub id: &'static str,
    input: String,
    part1: fn(&str) -> String,
    part2: fn(&str) -> String,
}

impl AdventDay {
    pub fn solve_part1(&self) -> String {
        return (self.part1)(&self.input);
    }
    pub fn solve_part2(&self) -> String {
        return (self.part2)(&self.input);
    }
}

pub fn advent_day_map() -> HashMap<String, AdventDay> {
    let mut map = HashMap::new();
    map.insert("01".to_owned(), day01::create("real.txt"));
    map.insert("02".to_owned(), day02::create("real.txt"));
    map.insert("03".to_owned(), day03::create("real.txt"));
    map.insert("04".to_owned(), day04::create("real.txt"));
    map.insert("05".to_owned(), day05::create("real.txt"));
    map.insert("06".to_owned(), day06::create("real.txt"));
    map.insert("07".to_owned(), day07::create("real.txt"));
    map.insert("08".to_owned(), day08::create("real.txt"));
    map.insert("09".to_owned(), day09::create("real.txt"));
    map.insert("10".to_owned(), day10::create("real.txt"));
    map.insert("11".to_owned(), day11::create("real.txt"));
    map.insert("12".to_owned(), day12::create("real.txt"));
    map.insert("13".to_owned(), day13::create("real.txt"));
    map.insert("14".to_owned(), day14::create("real.txt"));
    map.insert("15".to_owned(), day15::create("real.txt"));
    map.insert("16".to_owned(), day16::create("real.txt"));
    map.insert("17".to_owned(), day17::create("real.txt"));
    map.insert("18".to_owned(), day18::create("real.txt"));
    map.insert("19".to_owned(), day19::create("real.txt"));
    return map;
}

#[macro_export]
macro_rules! create_advent_day {
    ($id:tt) => {
        use crate::utils;
        use crate::AdventDay;

        pub fn create(file_name: &str) -> AdventDay {
            let folder_name = format!("day{}", $id);
            let input = utils::read_file(&folder_name, file_name);
            return AdventDay {
                id: $id,
                input,
                part1: |input| part1_with_input(input).to_string(),
                part2: |input| part2_with_input(input).to_string(),
            };
        }
    };
}
