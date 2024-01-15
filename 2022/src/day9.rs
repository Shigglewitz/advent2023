use std::collections::HashSet;

use crate::utils;

pub fn part1(file_name: &str) -> usize {
    let input = &utils::read_file("day9", file_name);
    let mut simulation = Simulation::new(input);
    simulation.run();
    return simulation.num_visited_locations();
}

pub fn part2(file_name: &str) -> usize {
    return 0;
}

struct Point {
    x: i32,
    y: i32,
}

struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    fn right(&mut self) {
        self.head.x += 1;
    }

    fn left(&mut self) {
        self.head.x -= 1;
    }

    fn up(&mut self) {
        self.head.y += 1;
    }

    fn down(&mut self) {
        self.head.y -= 1;
    }

    fn align(&mut self) {
        let x_delta = self.head.x - self.tail.x;
        let y_delta = self.head.y - self.tail.y;

        match (x_delta, y_delta) {
            (2, 0) => self.tail.x += 1,
            (-2, 0) => self.tail.x -= 1,
            (0, 2) => self.tail.y += 1,
            (0, -2) => self.tail.y -= 1,
            (1, 2) | (2, 1) => {
                self.tail.x += 1;
                self.tail.y += 1;
            }
            (-1, 2) | (-2, 1) => {
                self.tail.x -= 1;
                self.tail.y += 1;
            }
            (1, -2) | (2, -1) => {
                self.tail.x += 1;
                self.tail.y -= 1;
            }
            (-1, -2) | (-2, -1) => {
                self.tail.x -= 1;
                self.tail.y -= 1;
            }
            _ => (),
        }
    }
}

struct Instruction {
    direction: u8,
    repeat: u32,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let parts = input.split_once(" ").unwrap();
        return Instruction {
            direction: parts.0.bytes().next().unwrap(),
            repeat: parts.1.parse::<u32>().unwrap(),
        };
    }
}

struct Simulation {
    rope: Rope,
    instructions: Vec<Instruction>,
    visited_locations: HashSet<(i32, i32)>,
}

impl Simulation {
    fn new(input: &str) -> Self {
        let instructions = input.lines().map(Instruction::parse).collect::<Vec<_>>();
        return Simulation {
            rope: Rope {
                head: Point { x: 0, y: 0 },
                tail: Point { x: 0, y: 0 },
            },
            instructions,
            visited_locations: HashSet::new(),
        };
    }

    fn run(&mut self) {
        for instruction in &self.instructions {
            let function = match instruction.direction {
                b'R' => Rope::right,
                b'L' => Rope::left,
                b'U' => Rope::up,
                b'D' => Rope::down,
                _ => unreachable!("Unexpected direction!"),
            };
            for _ in 0..instruction.repeat {
                function(&mut self.rope);
                self.rope.align();
                self.visited_locations
                    .insert((self.rope.tail.x, self.rope.tail.y));
            }
        }
    }

    fn num_visited_locations(&self) -> usize {
        return self.visited_locations.len();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 13);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 0);
    }

    #[test]
    fn instruction_parse_works() {
        let actual = Instruction::parse("U 32");

        assert_eq!(actual.direction, b'U');
        assert_eq!(actual.repeat, 32);
    }

    #[test]
    fn right_works() {
        let mut simulation = Simulation::new("R 2");
        simulation.run();

        assert_eq!(simulation.rope.head.x, 2);
        assert_eq!(simulation.rope.head.y, 0);
        assert_eq!(simulation.rope.tail.x, 1);
        assert_eq!(simulation.rope.tail.y, 0);
        assert_eq!(simulation.num_visited_locations(), 2);
    }

    #[test]
    fn left_works() {
        let mut simulation = Simulation::new("L 2");
        simulation.run();

        assert_eq!(simulation.rope.head.x, -2);
        assert_eq!(simulation.rope.head.y, 0);
        assert_eq!(simulation.rope.tail.x, -1);
        assert_eq!(simulation.rope.tail.y, 0);
        assert_eq!(simulation.num_visited_locations(), 2);
    }

    #[test]
    fn up_works() {
        let mut simulation = Simulation::new("U 2");
        simulation.run();

        assert_eq!(simulation.rope.head.x, 0);
        assert_eq!(simulation.rope.head.y, 2);
        assert_eq!(simulation.rope.tail.x, 0);
        assert_eq!(simulation.rope.tail.y, 1);
        assert_eq!(simulation.num_visited_locations(), 2);
    }

    #[test]
    fn down_works() {
        let mut simulation = Simulation::new("D 2");
        simulation.run();

        assert_eq!(simulation.rope.head.x, 0);
        assert_eq!(simulation.rope.head.y, -2);
        assert_eq!(simulation.rope.tail.x, 0);
        assert_eq!(simulation.rope.tail.y, -1);
        assert_eq!(simulation.num_visited_locations(), 2);
    }

    #[rstest]
    #[case("R 1;U 2", (1, 2, 1, 1))]
    #[case("U 1;R 2", (2, 1, 1, 1))]
    #[case("L 1;U 2", (-1, 2, -1, 1))]
    #[case("U 1;L 2", (-2, 1, -1, 1))]
    #[case("R 1;D 2", (1, -2, 1, -1))]
    #[case("D 1;R 2", (2, -1, 1, -1))]
    #[case("L 1;D 2", (-1, -2, -1, -1))]
    #[case("D 1;L 2", (-2, -1, -1, -1))]
    fn diagonals_work(#[case] input: &str, #[case] locations: (i32, i32, i32, i32)) {
        let new_input = input.split(";").collect::<Vec<_>>().join("\n");
        let mut simulation = Simulation::new(&new_input);

        simulation.run();

        assert_eq!(simulation.rope.head.x, locations.0);
        assert_eq!(simulation.rope.head.y, locations.1);
        assert_eq!(simulation.rope.tail.x, locations.2);
        assert_eq!(simulation.rope.tail.y, locations.3);
        assert_eq!(simulation.num_visited_locations(), 2);
    }
}
