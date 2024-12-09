use std::collections::{HashMap, HashSet};

use crate::create_advent_day;

create_advent_day!("2024", "08");

fn part1_with_input(input: &str) -> i64 {
    let mut antenna_positions: HashMap<char, Vec<Coordinate>> = HashMap::new();

    input.lines().enumerate().for_each(|(line_num, line)| {
        line.chars().enumerate().for_each(|(row_num, letter)| {
            if letter != '.' {
                if !antenna_positions.contains_key(&letter) {
                    antenna_positions.insert(letter, Vec::new());
                }
                antenna_positions
                    .get_mut(&letter)
                    .unwrap()
                    .push(Coordinate {
                        x: row_num as i64,
                        y: line_num as i64,
                    });
            }
        })
    });

    let height = input.lines().count() as i64;
    let width = *input
        .lines()
        .map(|line| line.len())
        .collect::<Vec<usize>>()
        .first()
        .unwrap() as i64;

    let mut antinodes: HashSet<Coordinate> = HashSet::new();

    for (_, positions) in antenna_positions {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let first = &positions[i];
                let second = &positions[j];

                let first_antinode = Coordinate {
                    x: first.x - (second.x - first.x),
                    y: first.y - (second.y - first.y),
                };
                let second_antinode = Coordinate {
                    x: second.x + (second.x - first.x),
                    y: second.y + (second.y - first.y),
                };

                if first_antinode.in_bounds(width, height) {
                    antinodes.insert(first_antinode);
                }
                if second_antinode.in_bounds(width, height) {
                    antinodes.insert(second_antinode);
                }
            }
        }
    }

    return antinodes.len() as i64;
}

fn part2_with_input(input: &str) -> i64 {
    let mut antenna_positions: HashMap<char, Vec<Coordinate>> = HashMap::new();

    input.lines().enumerate().for_each(|(line_num, line)| {
        line.chars().enumerate().for_each(|(row_num, letter)| {
            if letter != '.' {
                if !antenna_positions.contains_key(&letter) {
                    antenna_positions.insert(letter, Vec::new());
                }
                antenna_positions
                    .get_mut(&letter)
                    .unwrap()
                    .push(Coordinate {
                        x: row_num as i64,
                        y: line_num as i64,
                    });
            }
        })
    });

    let height = input.lines().count() as i64;
    let width = *input
        .lines()
        .map(|line| line.len())
        .collect::<Vec<usize>>()
        .first()
        .unwrap() as i64;

    let mut antinodes: HashSet<Coordinate> = HashSet::new();

    for (_, positions) in antenna_positions {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let first = &positions[i];
                let second = &positions[j];

                let x_diff = second.x - first.x;
                let y_diff = second.y - first.y;

                let mut antinode = Coordinate {
                    x: first.x,
                    y: first.y,
                };

                loop {
                    if antinode.in_bounds(width, height) {
                        antinodes.insert(antinode.clone());
                        antinode.x = antinode.x - x_diff;
                        antinode.y = antinode.y - y_diff;
                    } else {
                        break;
                    }
                }

                let mut antinode = Coordinate {
                    x: second.x,
                    y: second.y,
                };

                loop {
                    if antinode.in_bounds(width, height) {
                        antinodes.insert(antinode.clone());
                        antinode.x = antinode.x + x_diff;
                        antinode.y = antinode.y + y_diff;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    return antinodes.len() as i64;
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    fn in_bounds(&self, width: i64, height: i64) -> bool {
        if self.x < 0 {
            return false;
        }
        if self.y < 0 {
            return false;
        }
        if self.x >= width {
            return false;
        }
        if self.y >= height {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("14", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("34", &actual);
    }
}
