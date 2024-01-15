use crate::create_advent_day;

create_advent_day!("2022", "05");

fn part1_with_input(input: &str) -> String {
    let mut puzzle = Puzzle::from(&input);
    puzzle.resolve();

    return puzzle.get_code();
}

fn part2_with_input(input: &str) -> String {
    let mut puzzle = Puzzle::from(&input);
    puzzle.resolve_9001();

    return puzzle.get_code();
}

struct CargoBox {
    id: char,
}

struct Instruction {
    num_to_move: i32,
    from_stack: i32,
    to_stack: i32,
}

struct Puzzle {
    stacks: Vec<Vec<CargoBox>>,
    instructions: Vec<Instruction>,
}

impl Puzzle {
    fn from(input: &str) -> Puzzle {
        let mut cargo_lines = Vec::new();
        let mut instruction_lines = Vec::new();
        let mut found_blank = false;

        for line in input.lines() {
            if line.len() == 0 {
                found_blank = true;
                continue;
            }
            if found_blank {
                instruction_lines.push(line);
            } else {
                cargo_lines.push(line);
            }
        }

        let mut stacks: Vec<Vec<CargoBox>> = Vec::new();
        let num_stacks = (cargo_lines[0].len() + 1) / 4;
        for _ in 0..num_stacks {
            stacks.push(Vec::new());
        }

        for line in cargo_lines.get(0..(cargo_lines.len() - 1)).unwrap() {
            for i in 0..num_stacks {
                let start_index = i * 4;
                let potential_box = line.get(start_index..start_index + 3).unwrap();
                if potential_box.starts_with('[') {
                    stacks[i].insert(
                        0,
                        CargoBox {
                            id: potential_box.chars().nth(1).unwrap(),
                        },
                    );
                }
            }
        }

        let mut instructions: Vec<Instruction> = Vec::new();
        for line in instruction_lines {
            let mut space_split = line.split(" ");
            instructions.push(Instruction {
                num_to_move: space_split.nth(1).unwrap().parse::<i32>().unwrap(),
                from_stack: space_split.nth(1).unwrap().parse::<i32>().unwrap(),
                to_stack: space_split.nth(1).unwrap().parse::<i32>().unwrap(),
            })
        }

        return Puzzle {
            stacks: stacks,
            instructions: instructions,
        };
    }

    fn resolve(&mut self) {
        while self.instructions.len() > 0 {
            self.apply_next_instruction();
        }
    }

    fn apply_next_instruction(&mut self) {
        let instruction = self.instructions.remove(0);
        for _ in 0..instruction.num_to_move {
            let cargo = self.stacks[instruction.from_stack as usize - 1]
                .pop()
                .unwrap();
            self.stacks[instruction.to_stack as usize - 1].push(cargo);
        }
    }

    fn resolve_9001(&mut self) {
        while self.instructions.len() > 0 {
            self.apply_next_instruction_9001();
        }
    }

    fn apply_next_instruction_9001(&mut self) {
        let instruction = self.instructions.remove(0);
        let index_to_remove = self.stacks[instruction.from_stack as usize - 1].len()
            - instruction.num_to_move as usize;
        for _ in 0..instruction.num_to_move {
            let cargo = self.stacks[instruction.from_stack as usize - 1].remove(index_to_remove);
            self.stacks[instruction.to_stack as usize - 1].push(cargo);
        }
    }

    fn get_code(&self) -> String {
        let code: String = self
            .stacks
            .iter()
            .map(|stack| stack.last())
            .map(Option::unwrap)
            .map(|cargo| cargo.id)
            .collect();
        return code;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "CMZ");
    }

    #[test]
    fn part2_works() {
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "MCD");
    }

    fn test_puzzle() -> Puzzle {
        let input = utils::read_file("2022/day05", "test.txt");
        return Puzzle::from(&input);
    }

    #[test]
    fn puzzle_from_works() {
        let actual = test_puzzle();

        assert_eq!(actual.stacks.len(), 3);
        assert_eq!(actual.stacks[0].len(), 2);
        assert_eq!(actual.stacks[0][0].id, 'Z');
        assert_eq!(actual.stacks[0][1].id, 'N');
        assert_eq!(actual.stacks[1].len(), 3);
        assert_eq!(actual.stacks[1][0].id, 'M');
        assert_eq!(actual.stacks[1][1].id, 'C');
        assert_eq!(actual.stacks[1][2].id, 'D');
        assert_eq!(actual.stacks[2].len(), 1);
        assert_eq!(actual.stacks[2][0].id, 'P');
        assert_eq!(actual.instructions.len(), 4);
        assert_eq!(actual.instructions[0].num_to_move, 1);
        assert_eq!(actual.instructions[0].from_stack, 2);
        assert_eq!(actual.instructions[0].to_stack, 1);
    }

    #[test]
    fn puzzle_apply_next_instruction_works() {
        let mut puzzle = test_puzzle();

        puzzle.apply_next_instruction();

        assert_eq!(&puzzle.get_code(), "DCP");
    }

    #[test]
    fn puzzle_get_code_works() {
        let actual = test_puzzle();

        assert_eq!(&actual.get_code(), "NDP")
    }
}
