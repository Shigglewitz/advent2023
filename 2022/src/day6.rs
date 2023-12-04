use std::collections::HashSet;

use crate::utils;

pub fn part1(file_name: &str) -> i32 {
    let input = utils::read_file("day6", file_name);

    return find_non_repeating_segment(&input, 4);   
}

pub fn part2(file_name: &str) -> i32 {
    let input = utils::read_file("day6", file_name);

    return find_non_repeating_segment(&input, 14);
}

fn find_non_repeating_segment(input: &str, length: usize) -> i32{
    let mut vec = Vec::new();
    for _ in 0..length {
        vec.push(' ');
    }

    for (index, letter) in input.chars().enumerate() {
        vec[index % length] = letter;
        if index > 3 && !has_duplicates(&vec) {
            return index as i32 + 1;
        }
    }

    panic!("Did not find starting symbol");
}

fn has_duplicates(vec: &Vec<char>) -> bool {
    let mut set = HashSet::new();

    for letter in vec {
        if set.contains(&letter) {
            return true;
        }
        set.insert(letter);
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 7);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 19);
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4, 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 4, 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 4, 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4, 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4, 11)]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14, 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 14, 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 14, 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14, 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14, 26)]
    fn find_non_repeating_segment_tests(#[case] input: &str, #[case] length_to_find: usize, #[case] expected: i32) {
        let actual = find_non_repeating_segment(input, length_to_find);

        assert_eq!(actual, expected);
    }
}
