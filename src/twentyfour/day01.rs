use std::collections::HashMap;

use crate::create_advent_day;

create_advent_day!("2024", "01");

fn part1_with_input(input: &str) -> i64 {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse::<i64>().unwrap());
        right.push(split.next().unwrap().parse::<i64>().unwrap());
    });

    left.sort();
    right.sort();

    let mut sum = 0;

    for it in left.iter().zip(right.iter()) {
        let (a, b) = it;
        sum += (a-b).abs();
    }
    

    return sum;
}

fn part2_with_input(input: &str) -> i64 {
    let mut left: Vec<i64> = Vec::new();
    let mut right: HashMap<i64, i64> = HashMap::new();

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse::<i64>().unwrap());
        let right_val = split.next().unwrap().parse::<i64>().unwrap();
        let curr_val = right.get(&right_val).unwrap_or(&0);
        right.insert(right_val, curr_val + 1);
    });

    return left.iter().map(|location| {
        location * right.get(location).unwrap_or(&0)
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("11", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("31", &actual);
    }
}