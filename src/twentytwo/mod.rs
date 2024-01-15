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

pub fn add_twenty_two(map: &mut HashMap<String, AdventDay>) {
    map.insert("2022_01".to_owned(), day01::create("real.txt"));
    map.insert("2022_02".to_owned(), day02::create("real.txt"));
    map.insert("2022_03".to_owned(), day03::create("real.txt"));
    map.insert("2022_04".to_owned(), day04::create("real.txt"));
    map.insert("2022_05".to_owned(), day05::create("real.txt"));
    map.insert("2022_06".to_owned(), day06::create("real.txt"));
    map.insert("2022_07".to_owned(), day07::create("real.txt"));
    map.insert("2022_08".to_owned(), day08::create("real.txt"));
    map.insert("2022_09".to_owned(), day09::create("real.txt"));
    map.insert("2022_10".to_owned(), day10::create("real.txt"));
    map.insert("2022_11".to_owned(), day11::create("real.txt"));
}