use crate::create_advent_day;

create_advent_day!("2024", "15");

fn part1_with_input(input: &str) -> i64 {
    let mut warehouse = parse_warehouse(input);
    for direction in warehouse.directions.clone().iter() {
        warehouse.march(*direction);
    }
    return warehouse.calculate_gps() as i64;
}

fn parse_warehouse(input: &str) -> Warehouse {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut directions: Vec<char> = Vec::new();
    let mut found_empty = false;
    let mut guard_x = 0_usize;
    let mut guard_y = 0_usize;
    for (line_num, line) in input.lines().enumerate() {
        if line.is_empty() {
            found_empty = true;
            continue;
        }
        if !found_empty {
            map.push(line.chars().collect());
            if line.contains("@") {
                let row_num = line.find("@").unwrap();
                guard_x = row_num;
                guard_y = line_num;
            }
        } else {
            directions.extend(line.chars());
        }
    }
    return Warehouse {
        map,
        directions,
        guard_x,
        guard_y,
    };
}

struct Warehouse {
    map: Vec<Vec<char>>,
    directions: Vec<char>,
    guard_x: usize,
    guard_y: usize,
}

impl Warehouse {
    fn _debug(&self) {
        let lines = self
            .map
            .iter()
            .map(|line| {
                let value: String = line.iter().collect::<String>();
                value
            })
            .collect::<Vec<String>>();
        let print_me = lines.join("\n");
        println!("{print_me}");
    }

    fn march(&mut self, direction: char) {
        match direction {
            '<' => self.march_left(),
            '>' => self.march_right(),
            '^' => self.march_up(),
            'v' => self.march_down(),
            _ => todo!("invalid direction"),
        }
    }

    fn march_up(&mut self) {
        let mut lookahead = (self.guard_x, self.guard_y);
        let can_move: bool;
        loop {
            if self.map[lookahead.1][lookahead.0] == '.' {
                can_move = true;
                break;
            } else if self.map[lookahead.1][lookahead.0] == '#' {
                can_move = false;
                break;
            }
            lookahead.1 -= 1;
        }
        if !can_move {
            return;
        }
        self.map[self.guard_y][self.guard_x] = '.';
        self.map[self.guard_y - 1][self.guard_x] = '@';
        self.guard_y -= 1;
        if lookahead.1 < self.guard_y {
            self.map[lookahead.1][lookahead.0] = 'O';
        }
    }

    fn march_down(&mut self) {
        let mut lookahead = (self.guard_x, self.guard_y);
        let can_move: bool;
        loop {
            if self.map[lookahead.1][lookahead.0] == '.' {
                can_move = true;
                break;
            } else if self.map[lookahead.1][lookahead.0] == '#' {
                can_move = false;
                break;
            }
            lookahead.1 += 1;
        }
        if !can_move {
            return;
        }
        self.map[self.guard_y][self.guard_x] = '.';
        self.map[self.guard_y + 1][self.guard_x] = '@';
        self.guard_y += 1;
        if lookahead.1 > self.guard_y {
            self.map[lookahead.1][lookahead.0] = 'O';
        }
    }

    fn march_left(&mut self) {
        let mut lookahead = (self.guard_x, self.guard_y);
        let can_move: bool;
        loop {
            if self.map[lookahead.1][lookahead.0] == '.' {
                can_move = true;
                break;
            } else if self.map[lookahead.1][lookahead.0] == '#' {
                can_move = false;
                break;
            }
            lookahead.0 -= 1;
        }
        if !can_move {
            return;
        }
        self.map[self.guard_y][self.guard_x] = '.';
        self.map[self.guard_y][self.guard_x - 1] = '@';
        self.guard_x -= 1;
        if lookahead.0 < self.guard_x {
            self.map[lookahead.1][lookahead.0] = 'O';
        }
    }

    fn march_right(&mut self) {
        let mut lookahead = (self.guard_x, self.guard_y);
        let can_move: bool;
        loop {
            if self.map[lookahead.1][lookahead.0] == '.' {
                can_move = true;
                break;
            } else if self.map[lookahead.1][lookahead.0] == '#' {
                can_move = false;
                break;
            }
            lookahead.0 += 1;
        }
        if !can_move {
            return;
        }
        self.map[self.guard_y][self.guard_x] = '.';
        self.map[self.guard_y][self.guard_x + 1] = '@';
        self.guard_x += 1;
        if lookahead.0 > self.guard_x {
            self.map[lookahead.1][lookahead.0] = 'O';
        }
    }

    fn calculate_gps(&self) -> usize {
        return self
            .map
            .iter()
            .enumerate()
            .map(|(line_num, line)| {
                line.iter()
                    .enumerate()
                    .map(|(row_num, letter)| {
                        if *letter != 'O' {
                            0
                        } else {
                            (100 * line_num) + row_num
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
    }
}

fn part2_with_input(_input: &str) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1_works() {
        let actual = create("test1.txt").solve_part1();

        assert_eq!("10092", &actual);
    }
    #[test]
    fn part1_2_works() {
        let actual = create("test2.txt").solve_part1();

        assert_eq!("2028", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test1.txt").solve_part2();

        assert_eq!("0", &actual);
    }
}
