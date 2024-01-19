use std::collections::HashSet;

use crate::create_advent_day;

create_advent_day!("2022", "18");

fn part1_with_input(input: &str) -> usize {
    let cubes = input
        .lines()
        .map(|line| {
            let split = line
                .split(",")
                .map(|str| str.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (split[0], split[1], split[2])
        })
        .collect::<HashSet<_>>();
    let borders = cubes
        .iter()
        .map(|cube| {
            vec![
                (cube.0 - 1, cube.1, cube.2),
                (cube.0 + 1, cube.1, cube.2),
                (cube.0, cube.1 - 1, cube.2),
                (cube.0, cube.1 + 1, cube.2),
                (cube.0, cube.1, cube.2 - 1),
                (cube.0, cube.1, cube.2 + 1),
            ]
        })
        .flatten()
        .collect::<Vec<_>>();
    return borders
        .iter()
        .filter(|border| !cubes.contains(border))
        .count();
}

fn part2_with_input(_input: &str) -> u32 {
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "64");
    }

    #[test]
    fn part2_works() {
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "0");
    }
}
