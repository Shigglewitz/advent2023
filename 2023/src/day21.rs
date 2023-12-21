use std::collections::HashSet;

use crate::create_advent_day;

create_advent_day!("21");

fn part1_with_input(input: &str) -> usize {
    let garden = Garden::parse(input);
    return garden.calculate_plot_count(64);
}

fn part2_with_input(input: &str) -> usize {
    let garden = Garden::parse(input);
    // TODO: needs better scaling to run this right
    // return garden.calculate_plot_count_infinite(26501365);
    return garden.calculate_plot_count_infinite(64);
}

struct Garden {
    layout: Vec<Vec<u8>>,
    start: Point,
    width: i32,
    height: i32,
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Garden {
    fn parse(input: &str) -> Garden {
        let layout = input
            .lines()
            .map(|line| line.bytes().collect::<Vec<u8>>())
            .collect::<Vec<_>>();
        let start = layout
            .iter()
            .enumerate()
            .map(|(line_index, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, &byte)| byte == b'S')
                    .next()
                    .map(|(char_index, _)| (char_index, line_index))
            })
            .filter(Option::is_some)
            .next()
            .flatten()
            .map(|(char_index, line_index)| Point {
                x: char_index as i32,
                y: line_index as i32,
            })
            .unwrap();

        let width = layout[0].len() as i32;
        let height = layout.len() as i32;
        return Garden {
            layout,
            start,
            width,
            height,
        };
    }

    fn char_at(&self, point: &Point) -> u8 {
        let mut x = point.x % self.width;
        if x < 0 {
            x += self.width;
        }
        let mut y = point.y % self.height;
        if y < 0 {
            y += self.width;
        }
        return self.layout[y as usize][x as usize];
    }

    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        if point.x > 0 && self.layout[point.y as usize][point.x as usize - 1] != b'#' {
            neighbors.push(Point {
                x: point.x - 1,
                y: point.y,
            });
        }
        if point.y > 0 && self.layout[point.y as usize - 1][point.x as usize] != b'#' {
            neighbors.push(Point {
                x: point.x,
                y: point.y - 1,
            });
        }
        if point.x < self.width - 1 && self.layout[point.y as usize][point.x as usize + 1] != b'#' {
            neighbors.push(Point {
                x: point.x + 1,
                y: point.y,
            });
        }
        if point.y < self.height - 1 && self.layout[point.y as usize + 1][point.x as usize] != b'#'
        {
            neighbors.push(Point {
                x: point.x,
                y: point.y + 1,
            });
        }

        return neighbors;
    }

    fn calculate_plot_count(&self, num_steps: u32) -> usize {
        let mut plots: HashSet<Point> = HashSet::new();
        let mut next_plots: HashSet<Point> = HashSet::new();
        plots.insert(self.start.clone());
        for _ in 0..num_steps {
            for plot in plots {
                let neighbors = self.get_neighbors(&plot);
                for neighbor in neighbors {
                    next_plots.insert(neighbor);
                }
            }
            plots = next_plots;
            next_plots = HashSet::new();
        }
        return plots.len();
    }

    fn get_neighbors_infinite(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        let left = Point {
            x: point.x - 1,
            y: point.y,
        };
        if self.char_at(&left) != b'#' {
            neighbors.push(left);
        }
        let right = Point {
            x: point.x + 1,
            y: point.y,
        };
        if self.char_at(&right) != b'#' {
            neighbors.push(right);
        }
        let up = Point {
            x: point.x,
            y: point.y - 1,
        };
        if self.char_at(&up) != b'#' {
            neighbors.push(up);
        }
        let down = Point {
            x: point.x,
            y: point.y + 1,
        };
        if self.char_at(&down) != b'#' {
            neighbors.push(down);
        }

        return neighbors;
    }

    fn calculate_plot_count_infinite(&self, num_steps: u32) -> usize {
        let mut plots: HashSet<Point> = HashSet::new();
        let mut next_plots: HashSet<Point> = HashSet::new();
        plots.insert(self.start.clone());
        for _ in 0..num_steps {
            for plot in plots {
                let neighbors = self.get_neighbors_infinite(&plot);
                for neighbor in neighbors {
                    next_plots.insert(neighbor);
                }
            }
            plots = next_plots;
            next_plots = HashSet::new();
        }
        return plots.len();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part1_works() {
        let input = utils::read_file("day21", "test.txt");
        let garden = Garden::parse(&input);
        let actual = garden.calculate_plot_count(6);

        assert_eq!(actual, 16);
    }

    #[test]
    fn part2_works() {
        let input = utils::read_file("day21", "test.txt");
        let garden = Garden::parse(&input);
        let actual = garden.calculate_plot_count_infinite(6);

        assert_eq!(actual, 16);
    }

    #[test]
    fn garden_parse_works() {
        let input = utils::read_file("day21", "test.txt");
        let garden = Garden::parse(&input);

        assert_eq!(garden.start.x, 5);
        assert_eq!(garden.start.y, 5);
    }

    #[test]
    fn garden_parse_works_real() {
        let input = utils::read_file("day21", "real.txt");
        let garden = Garden::parse(&input);

        assert_eq!(garden.start.x, 65);
        assert_eq!(garden.start.y, 65);
    }

    #[rstest]
    #[case(6, 16)]
    #[case(10, 50)]
    #[case(50, 1594)]
    #[case(100, 6536)]
    // TODO: needs scaling improvements for these tests
    // #[case(500, 167004)]
    // #[case(1000, 668697)]
    // #[case(5000, 16733044)]
    fn garden_calculate_infinite_tests(#[case] num_steps: u32, #[case] expected: usize) {
        let input = utils::read_file("day21", "test.txt");
        let garden = Garden::parse(&input);
        let actual = garden.calculate_plot_count_infinite(num_steps);

        assert_eq!(actual, expected);
    }
}
