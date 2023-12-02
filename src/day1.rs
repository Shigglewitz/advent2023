use lazy_static::lazy_static;
use regex::Regex;
use rstest::rstest;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::collections::HashMap;


lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();
    static ref TRANSLATE_MAP: HashMap<&'static str, &'static str> = generate_translate_map();
}

pub fn solve(file_name: &str) -> i32 {
    let input = read_file(file_name.to_string());
    let total: i32 = input.lines()
        .map(|line| calibration_value(line)).sum();

    return total;
}

#[test]
fn solve_basic() {
    let actual: i32 = solve("test_1.txt");

    assert_eq!(142, actual);
}

#[test]
fn solve_advanced() {
    let actual: i32 = solve("test_2.txt");

    assert_eq!(281, actual);
}

fn read_file(file_name: String) -> String {
    let path = format!("data/day1/{file_name}");
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    return contents;
}

#[test]
fn read_file_works() {
    let expected: String = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".lines().collect();

    let actual: String = read_file("test_1.txt".to_string()).lines().collect();

    assert_eq!(expected, actual);
}

fn calibration_value(input: &str) -> i32 {
    let translated = translate_to_numeric(input.to_string());
    let no_art = strip_alpha(translated);
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
fn parameterized(#[case] input: &str, #[case] expected: i32) {
    assert_eq!(expected, calibration_value(input))
}

fn to_two_digit_num(input: String) -> i32 {
    let first: i32 = input.chars().nth(0).unwrap().to_digit(10).unwrap() as i32;
    let second: i32 = input.chars().nth(input.len() - 1).unwrap().to_digit(10).unwrap() as i32;

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

fn generate_translate_map() -> HashMap<&'static str,&'static str> {
    let mut map = HashMap::new();
    map.insert("one", "1");
    map.insert("two", "2");
    map.insert("three", "3");
    map.insert("four", "4");
    map.insert("five", "5");
    map.insert("six", "6");
    map.insert("seven", "7");
    map.insert("eight", "8");
    map.insert("nine", "9");

    return map;
}

// source: https://stackoverflow.com/questions/56921637/how-do-i-split-a-string-using-a-rust-regex-and-keep-the-delimiters
fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for matched in r.find_iter(text) {
        let index = matched.start();
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched.as_str());
        last = index + matched.as_str().len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

fn translate_to_numeric(input: String) -> String {
    return split_keep(&NUM_REGEX, &input)
        .iter()
        .map(|word| if TRANSLATE_MAP.contains_key(word) {
            TRANSLATE_MAP.get(word).unwrap()
        } else {
            word
        })
        .map(|word| word.to_string())
        .collect();
}

#[test]
fn translate_to_numeric_basic() {
    let input = "23oneabc";

    let actual = translate_to_numeric(input.to_string());

    assert_eq!("231abc", actual);
}

#[test]
fn translate_to_numeric_advanced() {
    let input = "onetwothreefourfivesixseveneightnine";

    let actual = translate_to_numeric(input.to_string());

    assert_eq!("123456789", actual);
}
