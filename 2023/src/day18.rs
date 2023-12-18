use crate::create_advent_day;

create_advent_day!("18");

fn part1_with_input(input: &str) -> i64 {
    let instructions = DiggingInstructions::parse(input);
    return instructions.lava_area();
}

fn part2_with_input(input: &str) -> i64 {
    let instructions = DiggingInstructions::parse_hex(input);
    return instructions.lava_area();
}

struct DiggingInstructions {
    instructions: Vec<Instruction>,
}

struct Instruction {
    direction: u8,
    num: i64,
}

struct Point {
    x: i64,
    y: i64,
}

impl DiggingInstructions {
    fn parse(input: &str) -> DiggingInstructions {
        let instructions = input
            .lines()
            .map(|line| {
                let mut split = line.split(" ");
                let direction = split.next().unwrap().as_bytes()[0];
                let num = split.next().unwrap().parse::<i64>().unwrap();
                Instruction { direction, num }
            })
            .collect::<Vec<_>>();

        return DiggingInstructions { instructions };
    }

    fn parse_hex(input: &str) -> DiggingInstructions {
        let instructions = input
            .lines()
            .map(|line| {
                let hex_value = line.split("#").nth(1).unwrap().trim_end_matches(")");
                let distance = i64::from_str_radix(&hex_value[0..5], 16).unwrap();
                let direction_num = i64::from_str_radix(&hex_value[5..], 16).unwrap();
                let direction = match direction_num {
                    0 => b'R',
                    1 => b'D',
                    2 => b'L',
                    3 => b'U',
                    _ => unreachable!("unexpected number when parsing direction"),
                };
                Instruction {
                    direction,
                    num: distance,
                }
            })
            .collect::<Vec<_>>();
        return DiggingInstructions { instructions };
    }

    fn create_points(&self) -> Vec<Point> {
        let mut curr_x = 0;
        let mut curr_y = 0;
        let mut previous_turn_clockwise = Self::is_clockwise_turn(
            &self.instructions[self.instructions.len() - 1],
            &self.instructions[0],
        );

        let mut points = self
            .instructions
            .windows(2)
            .map(|arr| {
                let instruction = &arr[0];
                let next_instruction = &arr[1];
                let is_clockwise_turn = Self::is_clockwise_turn(instruction, next_instruction);
                let delta = if previous_turn_clockwise != is_clockwise_turn {
                    0
                } else if is_clockwise_turn {
                    1
                } else {
                    -1
                };
                previous_turn_clockwise = is_clockwise_turn;
                match instruction.direction {
                    b'R' => curr_x += instruction.num + delta,
                    b'L' => curr_x -= instruction.num + delta,
                    b'D' => curr_y -= instruction.num + delta,
                    b'U' => curr_y += instruction.num + delta,
                    _ => unreachable!("unexpected char while matching instruction"),
                }
                Point {
                    x: curr_x,
                    y: curr_y,
                }
            })
            .collect::<Vec<_>>();

        points.push(Point { x: 0, y: 0 });

        return points;
    }

    fn is_clockwise_turn(instruction_1: &Instruction, instruction_2: &Instruction) -> bool {
        // assuming instructions are clockwise
        return match (instruction_1.direction, instruction_2.direction) {
            (b'U', b'R') | (b'R', b'D') | (b'D', b'L') | (b'L', b'U') => true,
            _ => false,
        };
    }

    fn lava_area(&self) -> i64 {
        let points = self.create_points();

        return points
            .windows(2)
            .map(|arr| {
                let val = (arr[1].x + arr[0].x) * (arr[1].y - arr[0].y) / 2;
                val
            })
            .sum::<i64>()
            .abs();
    }

    fn _are_points_clockwise(points: &Vec<Point>) -> bool {
        let mut smallest_y = i64::MAX;
        let mut smallest_x = i64::MAX;
        let mut index_of_smallest = 0;

        for (index, point) in points.iter().enumerate() {
            if point.y < smallest_y {
                smallest_y = point.y;
                smallest_x = point.x;
                index_of_smallest = index;
            } else if point.y == smallest_y && point.x < smallest_x {
                smallest_y = point.y;
                smallest_x = point.x;
                index_of_smallest = index;
            }
        }

        let before_point = &points[(index_of_smallest + points.len() - 1) % points.len()];
        let after_point = &points[(index_of_smallest + 1) % points.len()];
        let smallest_point = &points[index_of_smallest];

        let vector_a = Point {
            x: before_point.x - smallest_point.x,
            y: before_point.y - smallest_point.y,
        };
        let vector_b = Point {
            x: after_point.x - smallest_point.x,
            y: after_point.y - smallest_point.y,
        };
        let cross_product = (vector_a.x * vector_b.y) - (vector_a.y * vector_b.x);

        return cross_product > 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!(&actual, "62");
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(&actual, "952408144115");
    }

    #[rstest]
    #[case(0, b'R', 6)]
    #[case(1, b'D', 5)]
    #[case(12, b'L', 2)]
    #[case(13, b'U', 2)]
    fn digging_instructions_parse_works(
        #[case] index: usize,
        #[case] expected_direction: u8,
        #[case] expected_num: i64,
    ) {
        let instructions = DiggingInstructions::parse(&utils::read_file("day18", "test.txt"));

        assert_eq!(
            instructions.instructions[index].direction,
            expected_direction
        );
        assert_eq!(instructions.instructions[index].num, expected_num);
    }

    #[test]
    fn diggint_instructions_parse_hex_works() {
        let instructions = DiggingInstructions::parse_hex(&utils::read_file("day18", "test.txt"));

        assert_eq!(instructions.instructions[0].direction, b'R');
        assert_eq!(instructions.instructions[0].num, 461937);
    }
}
