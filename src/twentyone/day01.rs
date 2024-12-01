use crate::create_advent_day;

create_advent_day!("2021", "01");

fn part1_with_input(input: &str) -> i64 {
    let depths = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    return depths
        .windows(2)
        .filter(|window| {
            let first = window[0];
            let second = window[1];
            first < second
        })
        .count()
        .try_into()
        .unwrap();
}

fn part2_with_input(input: &str) -> i64 {
    let depths = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let windows_sum = depths
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect::<Vec<i64>>();
    return windows_sum
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
        .try_into()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("7", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("5", &actual);
    }
}
