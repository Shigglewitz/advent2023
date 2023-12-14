use crate::utils;

pub fn part1(file_name: &str) -> u64 {
    let input = utils::read_file("day14", file_name);

    let mut platform = Platform::parse(&input);
    platform.shift_north();
    return platform.calculate_load();
}

#[allow(dead_code)]
pub fn part2(file_name: &str) -> u64 {
    let input = utils::read_file("day14", file_name);

    let mut platform = Platform::parse(&input);
    // TODO: need to figure out how to programmatically find a cycle
    (0..97).for_each(|_| {
        platform.spin_cycle();
        // println!("after {} cycles, score is {}", i, platform.calculate_load());
    });
    return platform.calculate_load();
}

struct Platform {
    rows: Vec<String>,
}

impl Platform {
    fn parse(input: &str) -> Platform {
        let rows = input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        return Platform { rows };
    }

    fn shift_north(&mut self) {
        let mut northmost_blocker: Vec<usize> = vec![usize::MAX; self.rows[0].len()];
        for (index, char) in self.rows[0].chars().enumerate() {
            match char {
                'O' => northmost_blocker[index] = 0,
                '#' => northmost_blocker[index] = 0,
                _ => (),
            }
        }
        let num_rows = self.rows.len();
        for row_num in 1..num_rows {
            let initial_string = self.rows[row_num].clone();
            for (index, char) in initial_string.chars().enumerate() {
                match char {
                    '#' => northmost_blocker[index] = row_num,
                    'O' => {
                        let row_to_move_to = if northmost_blocker[index] == usize::MAX {
                            0
                        } else {
                            northmost_blocker[index] + 1
                        };
                        if row_to_move_to == row_num {
                            northmost_blocker[index] = row_num;
                        } else {
                            self.rows[row_to_move_to].replace_range(index..(index + 1), "O");
                            self.rows[row_num].replace_range(index..(index + 1), ".");
                            northmost_blocker[index] = row_to_move_to;
                        }
                    }
                    _ => (),
                };
            }
        }
    }

    fn shift_west(&mut self) {
        let num_rows = self.rows.len();
        for row_num in 0..num_rows {
            let mut westmost_blocker = usize::MAX;
            let row = self.rows[row_num].clone();
            let bytes = row.bytes();
            for (index, byte) in bytes.enumerate() {
                match byte {
                    b'#' => westmost_blocker = index,
                    b'O' => {
                        let index_to_move_to = if westmost_blocker == usize::MAX {
                            0
                        } else {
                            westmost_blocker + 1
                        };
                        if index_to_move_to == index {
                            westmost_blocker = index;
                        } else {
                            self.rows[row_num]
                                .replace_range(index_to_move_to..(index_to_move_to + 1), "O");
                            self.rows[row_num].replace_range(index..(index + 1), ".");
                            westmost_blocker = index_to_move_to;
                        }
                    }
                    b'.' => (),
                    _ => unreachable!("unexpected char when shifting west"),
                }
            }
        }
    }

    fn shift_south(&mut self) {
        let mut southmost_blocker: Vec<usize> = vec![usize::MAX; self.rows[0].len()];
        let num_rows = self.rows.len();
        for row_num in (0..num_rows).rev() {
            let row = self.rows[row_num].clone();
            let bytes = row.bytes();
            for (index, byte) in bytes.enumerate() {
                match byte {
                    b'#' => southmost_blocker[index] = row_num,
                    b'O' => {
                        let row_to_move_to = if southmost_blocker[index] == usize::MAX {
                            num_rows - 1
                        } else {
                            southmost_blocker[index] - 1
                        };
                        if row_to_move_to == row_num {
                            southmost_blocker[index] = row_num;
                        } else {
                            self.rows[row_to_move_to].replace_range(index..(index + 1), "O");
                            self.rows[row_num].replace_range(index..(index + 1), ".");
                            southmost_blocker[index] = row_to_move_to;
                        }
                    }
                    b'.' => (),
                    _ => unreachable!("unexpected char when shifting south"),
                }
            }
        }
    }

    fn shift_east(&mut self) {
        let num_rows = self.rows.len();
        for row_num in 0..num_rows {
            let mut eastmost_blocker = usize::MAX;
            let row = self.rows[row_num].clone();
            let bytes = row.bytes();
            for (index, byte) in bytes.enumerate().rev() {
                match byte {
                    b'#' => eastmost_blocker = index,
                    b'O' => {
                        let index_to_move_to = if eastmost_blocker == usize::MAX {
                            row.len() - 1
                        } else {
                            eastmost_blocker - 1
                        };
                        if index_to_move_to == index {
                            eastmost_blocker = index;
                        } else {
                            self.rows[row_num]
                                .replace_range(index_to_move_to..(index_to_move_to + 1), "O");
                            self.rows[row_num].replace_range(index..(index + 1), ".");
                            eastmost_blocker = index_to_move_to;
                        }
                    }
                    b'.' => (),
                    _ => unreachable!("unexpected char when shifting east"),
                }
            }
        }
    }

    fn spin_cycle(&mut self) {
        self.shift_north();
        self.shift_west();
        self.shift_south();
        self.shift_east();
    }

    fn calculate_load(&self) -> u64 {
        let num_rows = self.rows.len();
        let mut sum = 0;
        for (index, row) in self.rows.iter().enumerate() {
            let this_row = row.chars().filter(|&c| c == 'O').count() * (num_rows - index);
            sum += this_row;
        }
        return sum as u64;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_platform() -> Platform {
        return Platform::parse(&utils::read_file("day14", "test.txt"));
    }

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 136);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 64);
    }

    #[test]
    fn platform_shift_north_works() {
        let mut platform = test_platform();
        platform.shift_north();

        assert_eq!(&platform.rows[0], "OOOO.#.O..");
        assert_eq!(&platform.rows[1], "OO..#....#");
        assert_eq!(&platform.rows[7], "..O.......");
        assert_eq!(&platform.rows[9], "#....#....");
    }

    #[test]
    fn platform_calculate_load_works() {
        let mut platform = test_platform();
        platform.shift_north();

        let actual = platform.calculate_load();

        assert_eq!(actual, 136);
    }

    #[test]
    fn platform_spin_cycle_works() {
        let mut platform = test_platform();
        platform.spin_cycle();

        assert_eq!(&platform.rows[0], ".....#....");
        assert_eq!(&platform.rows[1], "....#...O#");
        assert_eq!(&platform.rows[8], "#...O###..");
        assert_eq!(&platform.rows[9], "#..OO#....");
    }

    #[test]
    fn multiple_spin_cycles() {
        let mut platform = test_platform();
        (0..3).for_each(|_| platform.spin_cycle());

        assert_eq!(&platform.rows[0], ".....#....");
        assert_eq!(&platform.rows[1], "....#...O#");
        assert_eq!(&platform.rows[8], "#...O###.O");
        assert_eq!(&platform.rows[9], "#.OOO#...O");
    }
}
