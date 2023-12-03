use std::ops::RangeInclusive;

use crate::utils;

pub fn part1(file_name: &str) -> i32 {
    let input = utils::read_file("day4", file_name);

    return input
        .lines()
        .map(parse_assignment)
        .filter(Assignment::contains_full_overlap)
        .count() as i32;
}

pub fn part2(file_name: &str) -> i32 {
    let input = utils::read_file("day4", file_name);

    return input
        .lines()
        .map(parse_assignment)
        .filter(Assignment::contains_any_overlap)
        .count() as i32;
}

struct Assignment {
    first_range: RangeInclusive<i32>,
    second_range: RangeInclusive<i32>,
}

impl Assignment {
    fn contains_full_overlap(&self) -> bool {
        return fully_contains(&self.first_range, &self.second_range)
            || fully_contains(&self.second_range, &self.first_range);
    }

    fn contains_any_overlap(&self) -> bool {
        return &self.first_range.end() >= &self.second_range.start()
            && &self.first_range.start() <= &self.second_range.end();
    }
}

fn fully_contains(first: &RangeInclusive<i32>, second: &RangeInclusive<i32>) -> bool {
    return first.start() <= second.start() && first.end() >= second.end();
}

fn parse_assignment(input: &str) -> Assignment {
    let mut comma_split = input.split(",");
    let mut first_hyphen_split = comma_split.next().unwrap().split("-");
    let mut second_hyphen_split = comma_split.next().unwrap().split("-");

    return Assignment {
        first_range: (first_hyphen_split.next().unwrap().parse::<i32>().unwrap()
            ..=first_hyphen_split.next().unwrap().parse::<i32>().unwrap()),
        second_range: (second_hyphen_split.next().unwrap().parse::<i32>().unwrap()
            ..=second_hyphen_split.next().unwrap().parse::<i32>().unwrap()),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 2);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 4);
    }

    #[rstest]
    #[case("2-4,6-8", false)]
    #[case("2-3,4-5", false)]
    #[case("5-7,7-9", false)]
    #[case("2-8,3-7", true)]
    #[case("6-6,4-6", true)]
    #[case("2-6,4-8", false)]
    fn contains_full_overlap_tests(#[case] input: &str, #[case] expected: bool) {
        let assignment = parse_assignment(input);
        let actual = assignment.contains_full_overlap();

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("2-4,6-8", false)]
    #[case("2-3,4-5", false)]
    #[case("5-7,7-9", true)]
    #[case("2-8,3-7", true)]
    #[case("6-6,4-6", true)]
    #[case("2-6,4-8", true)]
    fn contains_any_overlap_tests(#[case] input: &str, #[case] expected: bool) {
        let assignment = parse_assignment(input);
        let actual = assignment.contains_any_overlap();

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("2-4,6-8", 2, 4, 6, 8)]
    #[case("2-3,4-5", 2, 3, 4, 5)]
    #[case("5-7,7-9", 5, 7, 7, 9)]
    #[case("2-8,3-7", 2, 8, 3, 7)]
    #[case("6-6,4-6", 6, 6, 4, 6)]
    #[case("2-6,4-8", 2, 6, 4, 8)]
    fn parse_assignment_works(
        #[case] input: &str,
        #[case] first_lower: i32,
        #[case] first_upper: i32,
        #[case] second_lower: i32,
        #[case] second_upper: i32,
    ) {
        let actual = parse_assignment(input);

        assert_eq!(actual.first_range.start(), &first_lower);
        assert_eq!(actual.first_range.end(), &first_upper);
        assert_eq!(actual.second_range.start(), &second_lower);
        assert_eq!(actual.second_range.end(), &second_upper);
    }
}
