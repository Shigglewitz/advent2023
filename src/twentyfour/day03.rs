use crate::create_advent_day;
use regex::Regex;
create_advent_day!("2024", "03");

fn part1_with_input(input: &str) -> i64 {
    let mul_regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();
    return process(input, &mul_regex);
}

fn part2_with_input(input: &str) -> i64 {
    let mul_regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
    return process(input, &mul_regex);
}

fn process(input: &str, regex: &Regex) -> i64 {
    let num_regex = Regex::new(r"(\d{1,3},\d{1,3})").unwrap();
    let matches = regex.captures_iter(input);
    let mut sum = 0;
    let mut process: bool = true;
    for hit in matches {
        let (_, [instruction]) = hit.extract();
        if instruction == "do()" {
            process = true;
            continue;
        } else if instruction == "don't()" {
            process = false;
            continue;
        }
        if !process {
            continue;
        }
        let num_matches = num_regex.captures_iter(instruction);
        for hit in num_matches {
            let (_, [nums]) = hit.extract();
            let mut split = nums.split(",");
            let first: i64 = split.next().unwrap().parse().unwrap();
            let second: i64 = split.next().unwrap().parse().unwrap();

            sum += first * second;
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("161", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("48", &actual);
    }
}
