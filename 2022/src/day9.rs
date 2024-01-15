use std::collections::HashSet;

use crate::utils;

pub fn part1(file_name: &str) -> usize {
    let input = &utils::read_file("day9", file_name);
    let mut simulation = Simulation::new(input, 2);
    simulation.run();
    return simulation.num_visited_locations();
}

pub fn part2(file_name: &str) -> usize {
    let input = &utils::read_file("day9", file_name);
    let mut simulation = Simulation::new(input, 10);
    simulation.run();
    return simulation.num_visited_locations();
}

struct Point {
    x: i32,
    y: i32,
}

struct Rope {
    knots: Vec<Point>,
    num_knots: usize,
}

impl Rope {
    fn right(&mut self) {
        self.knots[0].x += 1;
    }

    fn left(&mut self) {
        self.knots[0].x -= 1;
    }

    fn up(&mut self) {
        self.knots[0].y += 1;
    }

    fn down(&mut self) {
        self.knots[0].y -= 1;
    }

    fn tail(&self) -> &Point {
        return &self.knots[self.num_knots - 1];
    }

    fn align(&mut self) {
        for index in 0..self.num_knots - 1 {
            self.align_knots(index, index + 1);
        }
    }

    fn align_knots(&mut self, head_index: usize, tail_index: usize) {
        let x_delta = self.knots[head_index].x - self.knots[tail_index].x;
        let y_delta = self.knots[head_index].y - self.knots[tail_index].y;
        let tail = &mut self.knots[tail_index];

        match (x_delta, y_delta) {
            (2, 0) => tail.x += 1,
            (-2, 0) => tail.x -= 1,
            (0, 2) => tail.y += 1,
            (0, -2) => tail.y -= 1,
            (1, 2) | (2, 1) | (2, 2) => {
                tail.x += 1;
                tail.y += 1;
            }
            (-1, 2) | (-2, 1) | (-2, 2) => {
                tail.x -= 1;
                tail.y += 1;
            }
            (1, -2) | (2, -1) | (2, -2) => {
                tail.x += 1;
                tail.y -= 1;
            }
            (-1, -2) | (-2, -1) | (-2, -2) => {
                tail.x -= 1;
                tail.y -= 1;
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
    fn new(input: &str, num_knots: usize) -> Self {
        let instructions = input.lines().map(Instruction::parse).collect::<Vec<_>>();
        let mut knots = Vec::new();
        for _ in 0..num_knots {
            knots.push(Point { x: 0, y: 0 })
        }
        return Simulation {
            rope: Rope { knots, num_knots },
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
                    .insert((self.rope.tail().x, self.rope.tail().y));
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
        let actual = part1("test1.txt");

        assert_eq!(actual, 13);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test1.txt");

        assert_eq!(actual, 1);
    }

    #[test]
    fn part2_works_2() {
        let actual = part2("test2.txt");

        assert_eq!(actual, 36);
    }

    #[test]
    fn instruction_parse_works() {
        let actual = Instruction::parse("U 32");

        assert_eq!(actual.direction, b'U');
        assert_eq!(actual.repeat, 32);
    }

    #[test]
    fn right_works() {
        let mut simulation = Simulation::new("R 2", 2);
        simulation.run();

        assert_eq!(simulation.rope.knots[0].x, 2);
        assert_eq!(simulation.rope.knots[0].y, 0);
        assert_eq!(simulation.rope.tail().x, 1);
        assert_eq!(simulation.rope.tail().y, 0);
        assert_eq!(simulation.num_visited_locations(), 2);
    }

    #[test]
    fn left_works() {
        let mut simulation = Simulation::new("L 2", 2);
        simulation.run();

        assert_eq!(simulation.rope.knots[0].x, -2);
        assert_eq!(simulation.rope.knots[0].y, 0);
        assert_eq!(simulation.rope.tail().x, -1);
        assert_eq!(simulation.rope.tail().y, 0);
        assert_eq!(simulation.num_visited_locations(), 2);
    }

    #[test]
    fn up_works() {
        let mut simulation = Simulation::new("U 2", 2);
        simulation.run();

        assert_eq!(simulation.rope.knots[0].x, 0);
        assert_eq!(simulation.rope.knots[0].y, 2);
        assert_eq!(simulation.rope.tail().x, 0);
        assert_eq!(simulation.rope.tail().y, 1);
        assert_eq!(simulation.num_visited_locations(), 2);
    }

    #[test]
    fn down_works() {
        let mut simulation = Simulation::new("D 2", 2);
        simulation.run();

        assert_eq!(simulation.rope.knots[0].x, 0);
        assert_eq!(simulation.rope.knots[0].y, -2);
        assert_eq!(simulation.rope.tail().x, 0);
        assert_eq!(simulation.rope.tail().y, -1);
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
        let mut simulation = Simulation::new(&new_input, 2);

        simulation.run();

        assert_eq!(simulation.rope.knots[0].x, locations.0);
        assert_eq!(simulation.rope.knots[0].y, locations.1);
        assert_eq!(simulation.rope.tail().x, locations.2);
        assert_eq!(simulation.rope.tail().y, locations.3);
        assert_eq!(simulation.num_visited_locations(), 2);
    }
}
