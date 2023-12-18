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
