use crate::utils;
use rstest::rstest;

pub fn part1(file_name: &str) -> i32 {
    let input = utils::read_file("day1", file_name);
    let total: i32 = input
        .lines()
        .map(|line| line.to_string())
        .map(calibration_value)
        .sum();

    return total;
}

pub fn part2(file_name: &str) -> i32 {
    let input = utils::read_file("day1", file_name);
    let total: i32 = input
        .lines()
        .map(|line| line.to_string())
        .map(translate_to_numeric)
        .map(calibration_value)
        .sum();

    return total;
}

#[test]
fn part1_works() {
    let actual: i32 = part1("test_1.txt");

    assert_eq!(142, actual);
}

#[test]
fn part2_works() {
    let actual: i32 = part2("test_2.txt");

    assert_eq!(281, actual);
}

fn calibration_value(input: String) -> i32 {
    let no_art = strip_alpha(input);
    let calibration = to_two_digit_num(no_art);
    return calibration;
}

#[rstest]
#[case("a1b2c3d4e5f", 15)]
#[case("two1nine", 29)]
#[case("eightwothree", 83)]
#[case("abcone2threexyz", 13)]
#[case("xtwone3four", 24)]
#[case("4nineeightseven2", 42)]
#[case("zoneight234", 14)]
#[case("7pqrstsixteen", 76)]
#[case("nineight", 98)]
#[case("stbqnrhdqnjcvjgthtmht8xndfgprq3eightwol", 82)]
fn translate_and_calibration_tests(#[case] input: &str, #[case] expected: i32) {
    let translated = translate_to_numeric(input.to_string());
    let actual = calibration_value(translated);

    assert_eq!(expected, actual)
}

fn to_two_digit_num(input: String) -> i32 {
    let first: i32 = input.chars().nth(0).unwrap().to_digit(10).unwrap() as i32;
    let second: i32 = input
        .chars()
        .nth(input.len() - 1)
        .unwrap()
        .to_digit(10)
        .unwrap() as i32;

    return first * 10 + second;
}

#[test]
fn to_two_digit_num_one_digit() {
    let input = "7".to_string();

    let actual = to_two_digit_num(input);

    assert_eq!(77, actual)
}

#[test]
fn to_two_digit_num_two_digits() {
    let input = "56".to_string();

    let actual = to_two_digit_num(input);

    assert_eq!(56, actual)
}

#[test]
fn to_two_digit_num_three_digits() {
    let input = "123".to_string();

    let actual = to_two_digit_num(input);

    assert_eq!(13, actual)
}

fn strip_alpha(mut input: String) -> String {
    input.retain(|c| c.is_digit(10));

    return input;
}

#[test]
fn strip_alpha_works() {
    let input: String = "1abc2".to_string();

    let actual = strip_alpha(input);

    assert_eq!("12", actual)
}

fn translate_to_numeric(input: String) -> String {
    return input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
}
