use std::collections::HashSet;

use crate::create_advent_day;

create_advent_day!("21");

fn part1_with_input(input: &str) -> usize {
    let garden = Garden::parse(input);
    return garden.calculate_plot_count(64);
}

fn part2_with_input(input: &str) -> usize {
    return input.len();
}

struct Garden {
    layout: Vec<Vec<u8>>,
    start: Point,
    width: usize,
    height: usize,
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
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
                x: char_index,
                y: line_index,
            })
            .unwrap();

        let width = layout[0].len();
        let height = layout.len();
        return Garden {
            layout,
            start,
            width,
            height,
        };
    }

    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        if point.x > 0 && self.layout[point.y][point.x - 1] != b'#' {
            neighbors.push(Point {
                x: point.x - 1,
                y: point.y,
            });
        }
        if point.y > 0 && self.layout[point.y - 1][point.x] != b'#' {
            neighbors.push(Point {
                x: point.x,
                y: point.y - 1,
            });
        }
        if point.x < self.width - 1 && self.layout[point.y][point.x + 1] != b'#' {
            neighbors.push(Point {
                x: point.x + 1,
                y: point.y,
            });
        }
        if point.y < self.height - 1 && self.layout[point.y + 1][point.x] != b'#' {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = utils::read_file("day21", "test.txt");
        let garden = Garden::parse(&input);
        let actual = garden.calculate_plot_count(6);

        assert_eq!(actual, 16);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(actual, "141");
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
}
