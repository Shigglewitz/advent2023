use crate::create_advent_day;

create_advent_day!("2022", "03");
use std::collections::HashSet;

fn part1_with_input(input: &str) -> i32 {
    return input.lines().map(find_word_overlap).map(get_priority).sum();
}

fn part2_with_input(input: &str) -> i32 {
    let mut lines = input.lines().peekable();
    let mut sum = 0;
    while lines.peek().is_some() {
        let overlap = find_set_overlap(
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
        );
        sum += get_priority(overlap);
    }

    return sum;
}

fn get_priority(input: char) -> i32 {
    let num = input as i32;

    if num > 96 {
        return num - 96;
    } else {
        return num - 38;
    }
}

fn find_set_overlap(word1: &str, word2: &str, word3: &str) -> char {
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();

    for letter in word1.chars() {
        set1.insert(letter);
    }
    for letter in word2.chars() {
        set2.insert(letter);
    }
    for letter in word3.chars() {
        if set1.contains(&letter) && set2.contains(&letter) {
            return letter;
        }
    }
    panic!("No overlap!")
}

fn find_word_overlap(input: &str) -> char {
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
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "157");
    }

    #[test]
    fn part2_works() {
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "70");
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

    #[test]
    fn find_set_overlap_test() {
        let actual = find_set_overlap("abc", "CDa", "ZaZ");

        assert_eq!(actual, 'a');
    }

    #[rstest]
    #[case("abccde", 'c')]
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp", 'p')]
    #[case("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L')]
    #[case("PmmdzqPrVvPwwTWBwg", 'P')]
    #[case("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v')]
    #[case("ttgJtRGJQctTZtZT", 't')]
    #[case("CrZsJsPPZsGzwwsLwLmpwMDw", 's')]
    fn find_word_overlap_tests(#[case] input: &str, #[case] expected: char) {
        let actual = find_word_overlap(input);

        assert_eq!(actual, expected);
    }
}
