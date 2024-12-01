use std::collections::HashMap;

use crate::AdventDay;

pub mod day01;

pub fn add_twenty_four(map: &mut HashMap<String, AdventDay>) {
    map.insert("2024_01".to_owned(), day01::create("real.txt"));
}
