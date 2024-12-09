use std::collections::HashMap;

use crate::AdventDay;

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

pub fn add_twenty_four(map: &mut HashMap<String, AdventDay>) {
    map.insert("2024_01".to_owned(), day01::create("real.txt"));
    map.insert("2024_02".to_owned(), day02::create("real.txt"));
    map.insert("2024_03".to_owned(), day03::create("real.txt"));
    map.insert("2024_04".to_owned(), day04::create("real.txt"));
    map.insert("2024_05".to_owned(), day05::create("real.txt"));
    map.insert("2024_06".to_owned(), day06::create("real.txt"));
    map.insert("2024_07".to_owned(), day07::create("real.txt"));
    map.insert("2024_08".to_owned(), day08::create("real.txt"));
    map.insert("2024_09".to_owned(), day09::create("real.txt"));
    map.insert("2024_10".to_owned(), day10::create("real.txt"));
    map.insert("2024_11".to_owned(), day10::create("real.txt"));
}
