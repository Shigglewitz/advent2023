use std::collections::HashSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::create_advent_day;

create_advent_day!("2024", "06");

fn part1_with_input(input: &str) -> i64 {
    let mut guard = Guard {
        x_position: 0,
        y_position: 0,
        direction: Direction::NORTH,
    };
    let mut map: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            line.chars()
                .enumerate()
                .map(|(char_num, letter)| {
                    let cell_type = if letter == '#' {
                        CellType::OBSTACLE
                    } else {
                        CellType::EMPTY
                    };
                    let visited = if letter == '^' {
                        guard.x_position = char_num as i32;
                        guard.y_position = line_num as i32;
                        true
                    } else {
                        false
                    };
                    Cell { cell_type, visited }
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    while guard.patrol(&mut map) {}

    return map
        .iter()
        .map(|row| row.iter().filter(|cell| cell.visited).count() as i64)
        .sum();
}

#[derive(Clone)]
struct Cell {
    cell_type: CellType,
    visited: bool,
}

#[derive(PartialEq, Clone)]
enum CellType {
    EMPTY,
    OBSTACLE,
}

struct Guard {
    x_position: i32,
    y_position: i32,
    direction: Direction,
}

impl Guard {
    fn next_position(&self) -> (i32, i32) {
        return match self.direction {
            Direction::NORTH => (self.x_position, self.y_position - 1),
            Direction::EAST => (self.x_position + 1, self.y_position),
            Direction::SOUTH => (self.x_position, self.y_position + 1),
            Direction::WEST => (self.x_position - 1, self.y_position),
        };
    }

    fn patrol(&mut self, map: &mut Vec<Vec<Cell>>) -> bool {
        let (x, y) = self.next_position();
        if x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32 {
            return false;
        }
        let next_cell = &mut map[y as usize][x as usize];
        if next_cell.cell_type == CellType::OBSTACLE {
            self.direction = self.direction.rotate();
        } else {
            self.x_position = x;
            self.y_position = y;
            next_cell.visited = true;
        }
        return true;
    }

    fn patrol_with_cache(
        &mut self,
        map: &Vec<Vec<Cell>>,
        cache: &mut HashSet<(i32, i32, Direction)>,
    ) -> PatrolOutcome {
        let (x, y) = self.next_position();
        if x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32 {
            return PatrolOutcome::EXIT;
        }
        let next_cell = map.get(y as usize).unwrap().get(x as usize).unwrap();
        if next_cell.cell_type == CellType::OBSTACLE {
            self.direction = self.direction.rotate();
        } else {
            self.x_position = x;
            self.y_position = y;
        }
        let tuple = (self.x_position, self.y_position, self.direction);
        if cache.contains(&tuple) {
            return PatrolOutcome::INFINITE;
        } else {
            cache.insert(tuple);
            return PatrolOutcome::CONTINUE;
        }
    }
}

#[derive(PartialEq)]
enum PatrolOutcome {
    CONTINUE,
    EXIT,
    INFINITE,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    fn rotate(&self) -> Direction {
        return match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        };
    }
}

fn part2_with_input(input: &str) -> i64 {
    let mut guard = Guard {
        x_position: 0,
        y_position: 0,
        direction: Direction::NORTH,
    };
    let mut map: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            line.chars()
                .enumerate()
                .map(|(char_num, letter)| {
                    let cell_type = if letter == '#' {
                        CellType::OBSTACLE
                    } else {
                        CellType::EMPTY
                    };
                    let visited = if letter == '^' {
                        guard.x_position = char_num as i32;
                        guard.y_position = line_num as i32;
                        true
                    } else {
                        false
                    };
                    Cell { cell_type, visited }
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    let original_position = (guard.x_position as usize, guard.y_position as usize);
    while guard.patrol(&mut map) {}
    let visited_cells: Vec<(i64, i64)> = map
        .iter()
        .enumerate()
        .map(|(row_num, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(cell_num, cell)| {
                    if cell.visited {
                        if cell_num == original_position.0 && row_num == original_position.1 {
                            Err(0)
                        } else {
                            Ok((cell_num as i64, row_num as i64))
                        }
                    } else {
                        Err(0)
                    }.ok()
                })
                .collect::<Vec<(i64, i64)>>()
        })
        .flatten()
        .collect();

    return visited_cells
        .par_iter()
        .map(|(x_int, y_int)| {
            let x_size = *x_int as usize;
            let y_size = *y_int as usize;
            let mut my_map = map.clone();
            my_map[y_size][x_size].cell_type = CellType::OBSTACLE;
            let mut my_guard = Guard {
                x_position: original_position.0 as i32,
                y_position: original_position.1 as i32,
                direction: Direction::NORTH,
            };
            let mut outcome: PatrolOutcome;
            let mut cache = HashSet::new();
            loop {
                outcome = my_guard.patrol_with_cache(&my_map, &mut cache);
                if outcome != PatrolOutcome::CONTINUE {
                    break;
                }
            }
            if outcome == PatrolOutcome::INFINITE {
                1
            } else {
                0
            }
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("41", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("6", &actual);
    }
}
