use crate::utils;
use std::collections::HashSet;

pub fn part1(file_name: &str) -> i32 {
    let input = utils::read_file("day3", file_name);

    return input.lines().map(find_overlap).map(get_priority).sum();
}

fn get_priority(input: char) -> i32 {
    let num = input as i32;

    if num > 96 {
        return num - 96;
    } else {
        return num - 38;
    }
}

fn find_overlap(input: &str) -> char {
    let half_size = input.len() / 2;
    let left = &input[..half_size];
    let right = &input[half_size..];

    let mut set = HashSet::new();
    for letter in left.chars() {
        set.insert(letter);
    }
    for letter in right.chars() {
        if set.contains(&letter) {
            return letter;
        }
    }
    panic!("No overlap!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 157);
    }

    #[rstest]
    #[case('a', 1)]
    #[case('z', 26)]
    #[case('A', 27)]
    #[case('Z', 52)]
    fn get_priority_tests(#[case] input: char, #[case] expected: i32) {
        let actual = get_priority(input);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("abccde", 'c')]
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp", 'p')]
    #[case("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L')]
    #[case("PmmdzqPrVvPwwTWBwg", 'P')]
    #[case("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v')]
    #[case("ttgJtRGJQctTZtZT", 't')]
    #[case("CrZsJsPPZsGzwwsLwLmpwMDw", 's')]
    fn find_overlap_tests(#[case] input: &str, #[case] expected: char) {
        let actual = find_overlap(input);

        assert_eq!(actual, expected);
    }
}
