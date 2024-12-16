use std::fs;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::create_advent_day;

create_advent_day!("2024", "14");

fn part1_with_input(input: &str) -> i64 {
    return part1_with_confines(input, 101, 103);
}

fn part1_with_confines(input: &str, width: i64, height: i64) -> i64 {
    let robots = parse_robots(input);
    let second_num = 100;
    let positions = simulate_second(&robots, second_num, width, height);
    let quadrants = positions.into_iter().fold((0, 0, 0, 0), |acc, (x, y)| {
        let center_width = width / 2;
        let center_height = height / 2;
        if x == center_width {
            acc
        } else if y == center_height {
            acc
        } else if x < center_width && y < center_height {
            (acc.0 + 1, acc.1, acc.2, acc.3)
        } else if x > center_width && y < center_height {
            (acc.0, acc.1 + 1, acc.2, acc.3)
        } else if x < center_width && y > center_height {
            (acc.0, acc.1, acc.2 + 1, acc.3)
        } else {
            (acc.0, acc.1, acc.2, acc.3 + 1)
        }
    });
    return quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;
}

fn simulate_second(
    robots: &Vec<Robot>,
    second_num: i64,
    width: i64,
    height: i64,
) -> Vec<(i64, i64)> {
    let positions: Vec<(i64, i64)> = robots
        .iter()
        .map(|robot| {
            let x = (robot.initial_x + (second_num * robot.velocity_x)) % width;
            let y = (robot.initial_y + (second_num * robot.velocity_y)) % height;
            let x = if x < 0 { x + width } else { x };
            let y = if y < 0 { y + height } else { y };
            (x, y)
        })
        .collect();
    positions
}

fn parse_robots(input: &str) -> Vec<Robot> {
    let robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let mut whitespace = line.split_whitespace();
            let mut position_split = whitespace.next().unwrap().split("=");
            position_split.next();
            let mut position_values = position_split.next().unwrap().split(",");
            let initial_x = position_values.next().unwrap().parse::<i64>().unwrap();
            let initial_y = position_values.next().unwrap().parse::<i64>().unwrap();
            let mut velocity_split = whitespace.next().unwrap().split("=");
            velocity_split.next();
            let mut velocity_values = velocity_split.next().unwrap().split(",");
            let velocity_x = velocity_values.next().unwrap().parse::<i64>().unwrap();
            let velocity_y = velocity_values.next().unwrap().parse::<i64>().unwrap();
            Robot {
                initial_x,
                initial_y,
                velocity_x,
                velocity_y,
            }
        })
        .collect();
    robots
}

fn print_locations(robots: &Vec<Robot>, second_num: i64, width: i64, height: i64) -> String {
    let mut dot_matrix = vec![vec!['.'; width as usize]; height as usize];

    let locations = simulate_second(robots, second_num, width, height);
    locations.iter().for_each(|(x, y)| {
        dot_matrix[*y as usize][*x as usize] = '0';
    });
    let mut lines = vec![format!("{second_num}").to_string()];
    lines.extend(
        dot_matrix
            .iter()
            .map(|line| line.iter().collect::<String>()),
    );

    return lines.join("\n");
}

struct Robot {
    initial_x: i64,
    initial_y: i64,
    velocity_x: i64,
    velocity_y: i64,
}

fn part2_with_input(input: &str) -> i64 {
    let robots = parse_robots(input);
    let interesting_seconds: Vec<i64> = (20..100)
        .map(|i| vec![86 + (i * 101), 154 + (i * 103)])
        .flatten()
        .collect();
    let drawings = interesting_seconds
        .into_par_iter()
        .map(|second_num| print_locations(&robots, second_num, 101, 103))
        .collect::<Vec<String>>();
    let final_drawing = drawings.join("\n");
    fs::write("data/tree_finder.txt", final_drawing).expect("Unable to write file");
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = utils::read_file("2024/day14", "test.txt").to_string();

        let actual = part1_with_confines(&input, 11, 7);

        assert_eq!(12, actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("0", &actual);
    }
}
