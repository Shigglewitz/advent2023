use std::collections::HashSet;

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
                    Cell {
                        cell_type,
                        visited,
                    }
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    while guard.patrol(&mut map) {}

    return map
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| if cell.visited { 1 } else { 0 })
                .sum::<i64>()
        })
        .sum();
}

struct Cell {
    cell_type: CellType,
    visited: bool,
}

#[derive(PartialEq)]
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
        let next_cell = map
            .get_mut(y as usize)
            .unwrap()
            .get_mut(x as usize)
            .unwrap();
        if next_cell.cell_type == CellType::OBSTACLE {
            self.direction = self.direction.rotate();
        } else {
            next_cell.visited = true;
            self.x_position = x;
            self.y_position = y;
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
                    Cell {
                        cell_type,
                        visited,
                    }
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    let mut sum = 0;

    let original_position = (guard.x_position as usize, guard.y_position as usize);
    let length = map.len();
    for y in 0..length {
        for x in 0..length {
            let cell = &map[y][x];
            if cell.cell_type == CellType::OBSTACLE {
                continue;
            }
            if y == original_position.1 && x == original_position.0 {
                continue;
            }
            map[y][x].cell_type = CellType::OBSTACLE;
            guard.x_position = original_position.0 as i32;
            guard.y_position = original_position.1 as i32;
            guard.direction = Direction::NORTH;
            let mut outcome: PatrolOutcome;
            let mut cache = HashSet::new();
            loop {
                outcome = guard.patrol_with_cache(&map, &mut cache);
                if outcome != PatrolOutcome::CONTINUE {
                    break;
                }
            }
            map[y][x].cell_type = CellType::EMPTY;
            if outcome == PatrolOutcome::INFINITE {
                sum += 1;
            }
        }
    }

    return sum;
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
