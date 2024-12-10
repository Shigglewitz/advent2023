use std::collections::HashSet;

use crate::create_advent_day;

create_advent_day!("2024", "10");

fn part1_with_input(input: &str) -> i64 {
    let heights: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|letter| char::to_digit(letter, 10).unwrap())
                .collect()
        })
        .collect();
    let trailheads: Vec<Trailhead> = heights
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

    let map = Map {
        altitudes: heights,
        width: input.lines().into_iter().next().unwrap().len(),
        height: input.lines().count(),
    };
    return trailheads
        .iter()
        .map(|trailhead| trailhead.score(&map))
        .sum();
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
        let neighbors = map.neighbors(self.x, self.y);
        let candidates: Vec<(usize, usize)> = neighbors
            .into_iter()
            .filter(|(x, y)| map.altitudes[*y][*x] == 1)
            .collect();
        let possibilities: Vec<Option<(usize, usize)>> = candidates
            .iter()
            .map(|(x, y)| self.traverse(map, 1, *x, *y))
            .flatten()
            .collect();

        return possibilities
            .iter()
            .filter_map(|entry| *entry)
            .collect::<HashSet<(usize, usize)>>()
            .len() as i64;
    }

    fn traverse(
        &self,
        map: &Map,
        altitude: u32,
        x: usize,
        y: usize,
    ) -> Vec<Option<(usize, usize)>> {
        if altitude == 9 {
            return vec![Some((x, y))];
        }
        let neighbors = map.neighbors(x, y);
        let candidates: Vec<(usize, usize)> = neighbors
            .into_iter()
            .filter(|(x, y)| map.altitudes[*y][*x] == altitude + 1)
            .collect();
        return candidates
            .iter()
            .map(|(x, y)| self.traverse(map, altitude + 1, *x, *y))
            .flatten()
            .collect();
    }

    fn rating(&self, map: &Map) -> i64 {
        let neighbors = map.neighbors(self.x, self.y);
        let candidates: Vec<(usize, usize)> = neighbors
            .into_iter()
            .filter(|(x, y)| map.altitudes[*y][*x] == 1)
            .collect();
        return candidates
            .iter()
            .map(|(x, y)| self.count_trails(map, 1, *x, *y))
            .sum();
    }

    fn count_trails(&self, map: &Map, altitude: u32, x: usize, y: usize) -> i64 {
        if altitude == 9 {
            return 1;
        }
        let neighbors = map.neighbors(x, y);
        let candidates: Vec<(usize, usize)> = neighbors
            .into_iter()
            .filter(|(x, y)| map.altitudes[*y][*x] == altitude + 1)
            .collect();
        return candidates
            .iter()
            .map(|(x, y)| self.count_trails(map, altitude + 1, *x, *y))
            .sum();
    }
}

fn part2_with_input(input: &str) -> i64 {
    let heights: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|letter| char::to_digit(letter, 10).unwrap())
                .collect()
        })
        .collect();
    let trailheads: Vec<Trailhead> = heights
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

    let map = Map {
        altitudes: heights,
        width: input.lines().into_iter().next().unwrap().len(),
        height: input.lines().count(),
    };
    return trailheads
        .iter()
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
