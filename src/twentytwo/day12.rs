use rayon::prelude::*;
use std::collections::HashSet;

use crate::create_advent_day;

create_advent_day!("2022", "12");

fn part1_with_input(input: &str) -> u32 {
    let map = Map::parse(input, vec![b'S']);
    return map.shortest_path_to_e();
}

fn part2_with_input(input: &str) -> u32 {
    let map = Map::parse(input, vec![b'S', b'a']);
    return map.shortest_path_to_e();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct Map {
    elevations: Vec<Vec<u8>>,
    start_positions: Vec<Point>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse(input: &str, allowed_starts: Vec<u8>) -> Self {
        let elevations = input.lines().map(|line| line.bytes().collect()).collect();
        let start_positions = Self::find_start_positions(&elevations, allowed_starts);
        let width = elevations[0].len();
        let height = elevations.len();

        return Self {
            elevations,
            start_positions,
            width,
            height,
        };
    }

    fn find_start_positions(elevations: &Vec<Vec<u8>>, allowed_starts: Vec<u8>) -> Vec<Point> {
        let mut points = Vec::new();
        for (row_index, row) in elevations.iter().enumerate() {
            for (col_index, char) in row.iter().enumerate() {
                if allowed_starts.contains(char) {
                    points.push(Point {
                        x: col_index,
                        y: row_index,
                    });
                }
            }
        }
        return points;
    }

    fn altitude(label: &u8) -> u8 {
        return match label {
            b'S' => 1,
            b'E' => 26,
            _ => label - b'a' + 1,
        };
    }

    fn can_traverse(&self, from: &Point, to: &Point) -> bool {
        let from_altitude = Self::altitude(&self.elevations[from.y][from.x]);
        let to_altitude = Self::altitude(&self.elevations[to.y][to.x]);

        return to_altitude <= from_altitude + 1;
    }

    fn shortest_path_to_e(&self) -> u32 {
        return self
            .start_positions
            .par_iter()
            .map(|start| self.shortest_path_to_e_from_point(start))
            .filter(|steps| steps.is_some())
            .map(|step| step.unwrap())
            .min()
            .unwrap();
    }

    fn shortest_path_to_e_from_point(&self, start_position: &Point) -> Option<u32> {
        let mut explorers: Vec<(Point, u32)> = Vec::new();
        explorers.push((start_position.clone(), 0));
        let mut next_explorers: Vec<(Point, u32)> = Vec::new();
        let mut explored: HashSet<Point> = HashSet::new();
        explored.insert(start_position.clone());
        while !explorers.is_empty() {
            for (point, steps) in explorers {
                if self.elevations[point.y][point.x] == b'E' {
                    return Some(steps);
                }
                let mut potential_steps: Vec<Point> = Vec::new();
                if point.x != 0 {
                    potential_steps.push(Point {
                        x: point.x - 1,
                        y: point.y,
                    });
                }
                if point.y != 0 {
                    potential_steps.push(Point {
                        x: point.x,
                        y: point.y - 1,
                    });
                }
                if point.x < self.width - 1 {
                    potential_steps.push(Point {
                        x: point.x + 1,
                        y: point.y,
                    });
                }
                if point.y < self.height - 1 {
                    potential_steps.push(Point {
                        x: point.x,
                        y: point.y + 1,
                    });
                }
                let actual_steps = potential_steps
                    .iter()
                    .filter(|step| self.can_traverse(&point, step))
                    .filter(|step| !explored.contains(step))
                    .collect::<Vec<_>>();
                actual_steps.iter().for_each(|&step| {
                    next_explorers.push((step.clone(), steps + 1));
                    explored.insert(step.clone());
                });
            }
            explorers = next_explorers;
            next_explorers = Vec::new();
        }

        return None;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "31");
    }

    #[test]
    fn part2_works() {
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "29");
    }
}
