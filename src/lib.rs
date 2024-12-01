use std::collections::HashMap;

pub mod twentyfour;
pub mod twentyone;
pub mod twentythree;
pub mod twentytwo;
pub mod utils;

pub struct AdventDay {
    pub id: &'static str,
    pub year: &'static str,
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
    twentyone::add_twenty_one(&mut map);
    twentytwo::add_twenty_two(&mut map);
    twentythree::add_twenty_three(&mut map);
    twentyfour::add_twenty_four(&mut map);
    return map;
}

#[macro_export]
macro_rules! create_advent_day {
    ($year:tt, $id:tt) => {
        use crate::utils;
        use crate::AdventDay;

        pub fn create(file_name: &str) -> AdventDay {
            let folder_name = format!("{}/day{}", $year, $id);
            let input = utils::read_file(&folder_name, file_name);
            return AdventDay {
                id: $id,
                year: $year,
                input,
                part1: |input| part1_with_input(input).to_string(),
                part2: |input| part2_with_input(input).to_string(),
            };
        }
    };
}
