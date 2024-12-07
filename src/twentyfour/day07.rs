use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::create_advent_day;

create_advent_day!("2024", "07");

fn part1_with_input(input: &str) -> u64 {
    let problems: Vec<Problem> = input
        .lines()
        .map(|line| {
            let mut colon = line.split(":");
            let target = colon.next().unwrap().parse().unwrap();
            let inputs = colon
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            Problem { target, inputs }
        })
        .collect();

    return problems
        .par_iter()
        .filter(|problem| problem.solveable())
        .map(|problem| problem.target)
        .sum();
}

fn part2_with_input(input: &str) -> u64 {
    let problems: Vec<Problem> = input
        .lines()
        .map(|line| {
            let mut colon = line.split(":");
            let target = colon.next().unwrap().parse().unwrap();
            let inputs = colon
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            Problem { target, inputs }
        })
        .collect();

    return problems
        .par_iter()
        .filter(|problem| problem.solveable_pt2())
        .map(|problem| problem.target)
        .sum();
}

struct Problem {
    target: u64,
    inputs: Vec<u64>,
}

impl Problem {
    fn solveable(&self) -> bool {
        let max_attempts = 2_i32.pow((self.inputs.len() - 1) as u32);
        for attempt in 0..max_attempts {
            let mut this_attempt = attempt;
            let mut total = self.inputs[0];
            for i in 1..self.inputs.len() {
                if this_attempt % 2 == 0 {
                    total += self.inputs[i];
                } else {
                    total *= self.inputs[i];
                }
                this_attempt = this_attempt >> 1;
            }
            if total == self.target {
                return true;
            }
        }
        return false;
    }

    fn solveable_pt2(&self) -> bool {
        let max_attempts = 3_i32.pow((self.inputs.len() - 1) as u32);
        for attempt in 0..max_attempts {
            let mut this_attempt = attempt;
            let mut total = self.inputs[0];
            for i in 1..self.inputs.len() {
                if this_attempt % 3 == 0 {
                    total += self.inputs[i];
                } else if this_attempt % 3 == 1 {
                    total *= self.inputs[i];
                } else {
                    let num_digits = self.inputs[i].checked_ilog10().unwrap_or(0) + 1;
                    let multiplier = 10_u64.pow(num_digits);
                    total *= multiplier;
                    total += self.inputs[i];
                }
                this_attempt /= 3;
            }
            if total == self.target {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("3749", &actual);
    }

    #[test]
    fn bitshift() {
        assert_eq!(3, 7 >> 1);
    }

    #[rstest]
    #[case(1, 1)]
    #[case(9, 1)]
    #[case(10, 2)]
    #[case(11, 2)]
    #[case(99, 2)]
    #[case(100, 3)]
    #[case(101, 3)]
    fn num_digits(#[case] input: u64, #[case] expected: u32) {
        let num_digits = input.checked_ilog10().unwrap_or(0) + 1;

        assert_eq!(expected, num_digits);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("11387", &actual);
    }
}
