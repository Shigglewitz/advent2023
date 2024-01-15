use crate::create_advent_day;

create_advent_day!("2022", "10");
use std::str;

fn part1_with_input(input: &str) -> i32 {
    let program = executed_program(input);
    let cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    return cycles
        .iter()
        .map(|&cycle| program.signal_strength(cycle))
        .sum();
}

fn part2_with_input(input: &str) -> i32 {
    let program = executed_program(input);
    let art = program.draw();
    println!("{}", art);
    return 0;
}

fn executed_program(input: &str) -> Program {
    let mut program = Program::new();
    program.execute(&input);

    return program;
}

struct Program {
    signal: i32,
    signal_history: Vec<i32>,
}

impl Program {
    fn new() -> Self {
        return Program {
            signal: 1,
            signal_history: Vec::new(),
        };
    }

    fn execute(&mut self, instructions: &str) {
        instructions.lines().for_each(|line| {
            if line == "noop" {
                self.signal_history.push(self.signal);
            } else {
                let (_, strength) = line.split_once(" ").unwrap();
                self.signal_history.push(self.signal);
                self.signal_history.push(self.signal);
                self.signal += strength.parse::<i32>().unwrap()
            }
        })
    }

    fn draw(&self) -> String {
        let mut pixels = Vec::new();

        for (index, signal) in self.signal_history.iter().enumerate() {
            let drawn_index = index as i32 % 40;
            if (signal - drawn_index).abs() <= 1 {
                pixels.push(b'#')
            } else {
                pixels.push(b'.')
            }
        }

        return pixels
            .chunks(40)
            .map(|chunk| str::from_utf8(chunk).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
    }

    fn signal_strength(&self, cycle: usize) -> i32 {
        return self.signal_history[cycle - 1] * cycle as i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "13140");
    }

    #[test]
    fn part2_works() {
        let program = executed_program(&utils::read_file("2022/day10", "test.txt"));
        let art = program.draw();

        assert_eq!(
            art,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
