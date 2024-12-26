use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::create_advent_day;

create_advent_day!("2024", "25");

fn part1_with_input(input: &str) -> i64 {
    let (locks_input, keys_input): (Vec<_>, Vec<_>) = input
        .split("\n\n")
        .partition(|value| value.starts_with("#####"));

    let locks = locks_input
        .into_iter()
        .map(parse_lock)
        .collect::<Vec<Lock>>();

    let keys = keys_input.into_iter().map(parse_key).collect::<Vec<Key>>();

    return keys
        .par_iter()
        .map(|key| {
            locks
                .par_iter()
                .filter(|lock| key_fits_lock(key, lock))
                .count()
        })
        .sum::<usize>() as i64;
}

fn parse_lock(block: &str) -> Lock {
    let mut lines = block.lines();
    let first_line = lines.next().unwrap();
    let length = first_line.len();
    let mut heights = vec![0; length];
    for line in lines {
        line.chars().enumerate().for_each(|(index, letter)| {
            if letter == '#' {
                heights[index] += 1;
            }
        })
    }
    let total_heights = heights.iter().sum();
    Lock {
        heights,
        total_heights,
    }
}

fn parse_key(block: &str) -> Key {
    let mut lines = block.lines().rev();
    let first_line = lines.next().unwrap();
    let length = first_line.len();
    let mut heights = vec![0; length];
    for line in lines {
        line.chars().enumerate().for_each(|(index, letter)| {
            if letter == '#' {
                heights[index] += 1;
            }
        })
    }
    let total_heights = heights.iter().sum();
    Key {
        heights,
        total_heights,
    }
}

struct Lock {
    heights: Vec<u32>,
    total_heights: u32,
}

struct Key {
    heights: Vec<u32>,
    total_heights: u32,
}

fn key_fits_lock(key: &Key, lock: &Lock) -> bool {
    if key.total_heights + lock.total_heights > 25 {
        return false;
    }
    for index in 0..5 {
        if key.heights[index] + lock.heights[index] > 5 {
            return false;
        }
    }

    return true;
}

fn part2_with_input(_input: &str) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("3", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("0", &actual);
    }
}
