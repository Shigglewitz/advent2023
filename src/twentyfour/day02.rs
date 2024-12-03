use num::cast::AsPrimitive;

use crate::create_advent_day;

create_advent_day!("2024", "02");

fn part1_with_input(input: &str) -> i64 {
    return input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|arr| is_safe(arr))
        .count()
        .as_();
}

fn part2_with_input(input: &str) -> i64 {
    return input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|arr| is_safe_dampener(arr))
        .count()
        .as_();
}

fn is_safe(arr: &Vec<i64>) -> bool {
    let ascending = if arr[0] > arr[1] {
        false
    } else if arr[0] < arr[1] {
        true
    } else {
        return false;
    };
    return arr.windows(2).fold(true, |acc, window| {
        if !acc {
            return acc;
        }
        if ascending && window[0] > window[1] {
            return false;
        } else if !ascending && window[0] < window[1] {
            return false;
        } else if window[0] == window[1] {
            return false;
        }
        return (window[0] - window[1]).abs() <= 3;
    });
}

fn is_safe_dampener(arr: &Vec<i64>) -> bool {
    if is_safe(arr) {
        return true;
    }
    for i in 0..arr.len() {
        let mut new_arr = arr.to_vec();
        new_arr.remove(i);
        if is_safe(&new_arr) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("2", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("4", &actual);
    }
}
