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
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

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
    map.insert("2024_11".to_owned(), day11::create("real.txt"));
    map.insert("2024_12".to_owned(), day12::create("real.txt"));
    map.insert("2024_13".to_owned(), day13::create("real.txt"));
    map.insert("2024_14".to_owned(), day14::create("real.txt"));
    map.insert("2024_15".to_owned(), day15::create("real.txt"));
    map.insert("2024_16".to_owned(), day16::create("real.txt"));
    map.insert("2024_17".to_owned(), day17::create("real.txt"));
    map.insert("2024_18".to_owned(), day18::create("real.txt"));
    map.insert("2024_19".to_owned(), day19::create("real.txt"));
    map.insert("2024_20".to_owned(), day20::create("real.txt"));
    map.insert("2024_21".to_owned(), day21::create("real.txt"));
    map.insert("2024_22".to_owned(), day22::create("real.txt"));
    map.insert("2024_23".to_owned(), day23::create("real.txt"));
    map.insert("2024_24".to_owned(), day24::create("real.txt"));
    map.insert("2024_25".to_owned(), day25::create("real.txt"));
}
