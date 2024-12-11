use std::collections::HashSet;

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::create_advent_day;

create_advent_day!("2024", "10");

fn part1_with_input(input: &str) -> i64 {
    let heights = parse_heights(input);
    let trailheads: Vec<Trailhead> = parse_trailheads(&heights);

    let map = Map {
        altitudes: heights,
        width: input.lines().into_iter().next().unwrap().len(),
        height: input.lines().count(),
    };
    return trailheads
        .par_iter()
        .map(|trailhead| trailhead.score(&map))
        .sum();
}

fn parse_heights(input: &str) -> Vec<Vec<u32>> {
    return input
        .lines()
        .map(|line| {
            line.chars()
                .map(|letter| char::to_digit(letter, 10).unwrap())
                .collect()
        })
        .collect();
}

fn parse_trailheads(heights: &Vec<Vec<u32>>) -> Vec<Trailhead> {
    return heights
        .iter()
        .enumerate()
        .map(|(line_num, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(row_num, &height)| {
                    if height == 0 {
                        Some(Trailhead {
                            x: row_num,
                            y: line_num,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Trailhead>>()
        })
        .flatten()
        .collect();
}

struct Map {
    altitudes: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Map {
    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut candidates = Vec::new();

        if x > 0 {
            candidates.push((x - 1, y));
        }
        if x < self.width - 1 {
            candidates.push((x + 1, y));
        }
        if y > 0 {
            candidates.push((x, y - 1));
        }
        if y < self.height - 1 {
            candidates.push((x, y + 1));
        }

        return candidates;
    }
}

struct Trailhead {
    x: usize,
    y: usize,
}

impl Trailhead {
    fn score(&self, map: &Map) -> i64 {
        return self
            .traverse(map, 0, self.x, self.y)
            .par_iter()
            .collect::<HashSet<&(usize, usize)>>()
            .len() as i64;
    }

    fn traverse(
        &self,
        map: &Map,
        altitude: u32,
        x: usize,
        y: usize,
    ) -> Vec<(usize, usize)> {
        if altitude == 9 {
            return vec![(x, y)];
        }
        return map
            .neighbors(x, y)
            .into_par_iter()
            .filter(|(x, y)| map.altitudes[*y][*x] == altitude + 1)
            .map(|(x, y)| self.traverse(map, altitude + 1, x, y))
            .flatten()
            .collect();
    }

    fn rating(&self, map: &Map) -> i64 {
        return self.count_trails(map, 0, self.x, self.y);
    }

    fn count_trails(&self, map: &Map, altitude: u32, x: usize, y: usize) -> i64 {
        if altitude == 9 {
            return 1;
        }
        return map
            .neighbors(x, y)
            .into_par_iter()
            .filter(|(x, y)| map.altitudes[*y][*x] == altitude + 1)
            .map(|(x, y)| self.count_trails(map, altitude + 1, x, y))
            .sum();
    }
}

fn part2_with_input(input: &str) -> i64 {
    let heights = parse_heights(input);
    let trailheads = parse_trailheads(&heights);

    let map = Map {
        altitudes: heights,
        width: input.lines().into_iter().next().unwrap().len(),
        height: input.lines().count(),
    };
    return trailheads
        .par_iter()
        .map(|trailhead| trailhead.rating(&map))
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("36", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("81", &actual);
    }
}
