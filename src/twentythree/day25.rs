use crate::create_advent_day;

create_advent_day!("2023", "25");

fn part1_with_input(input: &str) -> usize {
    return input.len();
}

fn part2_with_input(input: &str) -> usize {
    return input.len();
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!(actual, "208");
    }

    #[ignore]
    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(actual, "208");
    }
}
