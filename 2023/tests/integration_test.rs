#[cfg(test)]
mod test {
    use advent2023::*;

    #[test]
    fn day01_part1() {
        let actual = day01::create("real.txt").solve_part1();

        assert_eq!(actual, "55834");
    }

    #[test]
    fn day01_part2() {
        let actual = day01::create("real.txt").solve_part2();

        assert_eq!(actual, "53221");
    }

    #[test]
    fn day02_part1() {
        let actual = day02::create("real.txt").solve_part1();

        assert_eq!(actual, "2156");
    }

    #[test]
    fn day02_part2() {
        let actual = day02::create("real.txt").solve_part2();

        assert_eq!(actual, "66909");
    }
}
