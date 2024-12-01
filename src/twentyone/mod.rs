use std::collections::HashMap;

use crate::AdventDay;

pub mod day01;
pub mod day02;

pub fn add_twenty_one(map: &mut HashMap<String, AdventDay>) {
    map.insert("2021_01".to_owned(), day01::create("real.txt"));
    map.insert("2021_02".to_owned(), day02::create("real.txt"));
}
