use std::collections::HashMap;

use crate::create_advent_day;

create_advent_day!("2024", "11");

fn part1_with_input(input: &str) -> u64 {
    let stones: HashMap<u64, u64> = parse_stones(input);
    let stones = blink(&stones, 25);
    return stones.values().sum();
}

fn _transform_stone(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }
    let num_digits = num_digits(stone);
    if num_digits % 2 == 0 {
        return _split_num(stone, num_digits);
    } else {
        return vec![stone * 2024];
    }
}

fn num_digits(input: u64) -> u32 {
    return input.checked_ilog10().unwrap_or(0) + 1;
}

fn _split_num(input: u64, num_digits: u32) -> Vec<u64> {
    let splitter = 10_u32.pow(num_digits / 2) as u64;
    return vec![input / splitter, input % splitter];
}

fn part2_with_input(input: &str) -> u64 {
    let stones: HashMap<u64, u64> = parse_stones(input);
    let stones = blink(&stones, 75);
    return stones.values().sum();
}

fn parse_stones(input: &str) -> HashMap<u64, u64> {
    return input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<u64>().unwrap())
                .map(|num| (num, 1))
                .collect::<HashMap<_, _>>()
        })
        .flatten()
        .collect();
}

fn blink(initial: &HashMap<u64, u64>, blink_count: usize) -> HashMap<u64, u64> {
    let mut stones: HashMap<u64, u64> = initial.clone();
    for _ in 0..blink_count {
        stones = count_stones(&stones);
    }
    return stones;
}

fn count_stones(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut count = HashMap::new();

    for (&inscription, &stone_count) in stones {
        if inscription == 0 {
            *count.entry(1).or_default() += stone_count;
        } else {
            let num_digits = num_digits(inscription);
            if num_digits % 2 == 0 {
                let splitter = 10_u32.pow(num_digits / 2) as u64;
                *count.entry(inscription / splitter).or_default() += stone_count;
                *count.entry(inscription % splitter).or_default() += stone_count;
            } else {
                *count.entry(inscription * 2024).or_default() += stone_count;
            }
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("55312", &actual);
    }

    #[test]
    fn test_split_num() {
        let actual = _split_num(1024, num_digits(1024));

        assert_eq!(10, actual[0]);
        assert_eq!(24, actual[1]);
    }

    #[test]
    fn test_aggregate() {
        let entries = vec![(1, 5), (2, 6), (1, 2)];
        let map: HashMap<u32, u32> = entries
            .into_par_iter()
            .fold(
                || HashMap::new(),
                |mut map, tuple| {
                    let (key, value) = tuple;
                    *map.entry(key).or_default() += value;
                    map
                },
            )
            .reduce(
                || HashMap::new(),
                |mut a, b| {
                    for (k, v) in b {
                        *a.entry(k).or_default() += v;
                    }
                    a
                },
            );

        assert_eq!(map[&2], 6);
        assert_eq!(map[&1], 7);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("65601038650482", &actual);
    }
}
