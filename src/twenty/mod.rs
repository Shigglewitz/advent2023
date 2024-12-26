use std::collections::HashMap;

use crate::AdventDay;

pub mod day01;

pub fn add_twenty(map: &mut HashMap<String, AdventDay>) {
    map.insert("2020_01".to_owned(), day01::create("real.txt"));
}
