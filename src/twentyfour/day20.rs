use std::collections::HashMap;

use crate::create_advent_day;

create_advent_day!("2024", "20");

fn part1_with_input(input: &str) -> i64 {
    return part1_with_threshold(input, 100);
}

fn part1_with_threshold(input: &str, threshold: usize) -> i64 {
    let maze = parse_maze(input);
    let path = maze.solve();

    let mut cheats: HashMap<usize, usize> = HashMap::new();
    let num_picosconds = path.len();
    for i in 0..num_picosconds - 3 {
        for j in (i + 3)..num_picosconds {
            let distance = distance(path[i], path[j]);
            if distance == 2 {
                *cheats.entry((j - i) - 2).or_default() += 1;
            }
        }
    }

    return cheats
        .into_iter()
        .filter(|(key, _)| *key >= threshold)
        .map(|(_, value)| value)
        .sum::<usize>() as i64;
}

fn distance(point1: (usize, usize), point2: (usize, usize)) -> usize {
    let x_diff = usize::abs_diff(point1.0, point2.0);
    let y_diff = usize::abs_diff(point1.1, point2.1);
    return x_diff + y_diff;
}

struct Maze {
    layout: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    fn solve(&self) -> Vec<(usize, usize)> {
        let mut last = self.start.clone();
        let mut next = self.neighbors(last)[0];
        let mut path = vec![last, next];
        loop {
            if self.end == next {
                break;
            }
            let neighbors = self
                .neighbors(next)
                .into_iter()
                .filter(|&point| point != last)
                .collect::<Vec<_>>();
            if neighbors.len() != 1 {
                println!("uh oh");
            }
            path.push(neighbors[0]);
            last = next;
            next = neighbors[0];
        }
        return path;
    }

    fn neighbors(&self, location: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if self.layout[location.1][location.0 + 1] != '#' {
            neighbors.push((location.0 + 1, location.1));
        }
        if self.layout[location.1][location.0 - 1] != '#' {
            neighbors.push((location.0 - 1, location.1));
        }
        if self.layout[location.1 + 1][location.0] != '#' {
            neighbors.push((location.0, location.1 + 1));
        }
        if self.layout[location.1 - 1][location.0] != '#' {
            neighbors.push((location.0, location.1 - 1));
        }

        return neighbors;
    }
}

fn parse_maze(input: &str) -> Maze {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let layout = input
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            line.chars()
                .enumerate()
                .map(|(letter_num, letter)| {
                    if letter == 'S' {
                        start = (letter_num, line_num)
                    }
                    if letter == 'E' {
                        end = (letter_num, line_num)
                    }
                    letter
                })
                .collect()
        })
        .collect();
    return Maze { layout, start, end };
}

fn part2_with_input(_input: &str) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(2, 44)]
    #[case(4, 30)]
    #[case(6, 16)]
    #[case(8, 14)]
    fn part1_works(#[case] threshold: usize, #[case] expected: i64) {
        let input = utils::read_file("2024/day20", "test.txt");
        let actual = part1_with_threshold(&input, threshold);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("0", &actual);
    }
}
