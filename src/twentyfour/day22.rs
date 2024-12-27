use std::ops::BitXor;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::create_advent_day;

create_advent_day!("2024", "22");

fn part1_with_input(input: &str) -> i64 {
    let initial = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    return initial
        .par_iter()
        .map(|first_value| {
            let mut value = *first_value;
            for _ in 0..2000 {
                value = calculate(value);
            }
            value
        })
        .sum::<u64>() as i64;
}

fn calculate(secret: u64) -> u64 {
    return step3(step2(step1(secret)));
}

fn step1(secret: u64) -> u64 {
    return prune(mix(secret * 64, secret));
}

fn step2(secret: u64) -> u64 {
    return prune(mix(secret / 32, secret));
}

fn step3(secret: u64) -> u64 {
    return prune(mix(secret * 2048, secret));
}

fn mix(given: u64, secret: u64) -> u64 {
    return given.bitxor(secret);
}

fn prune(given: u64) -> u64 {
    return given % 16777216;
}

fn part2_with_input(_input: &str) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("37327623", &actual);
    }

    #[rstest]
    #[case(123, 15887950)]
    #[case(15887950, 16495136)]
    #[case(16495136, 527345)]
    #[case(527345, 704524)]
    #[case(704524, 1553684)]
    #[case(1553684, 12683156)]
    #[case(12683156, 11100544)]
    #[case(11100544, 12249484)]
    #[case(12249484, 7753432)]
    #[case(7753432, 5908254)]
    fn calculate_works(#[case] input: u64, #[case] expected: u64) {
        let actual = calculate(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn mix_works() {
        let actual = mix(15, 42);

        assert_eq!(37, actual);
    }

    #[test]
    fn prune_works() {
        let actual = prune(100000000);

        assert_eq!(16113920, actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("0", &actual);
    }
}
