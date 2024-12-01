use std::collections::HashMap;

use crate::AdventDay;

pub mod day01;
pub mod day02;
pub mod day03;

pub fn add_twenty_four(map: &mut HashMap<String, AdventDay>) {
    map.insert("2024_01".to_owned(), day01::create("real.txt"));
    map.insert("2024_02".to_owned(), day02::create("real.txt"));
    map.insert("2024_03".to_owned(), day03::create("real.txt"));
}
