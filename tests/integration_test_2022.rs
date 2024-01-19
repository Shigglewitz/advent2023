#[cfg(test)]
mod test {
    use advent::twentytwo::*;

    #[test]
    fn day01_part1() {
        let actual = day01::create("real.txt").solve_part1();

        assert_eq!(actual, "64929");
    }

    #[test]
    fn day01_part2() {
        let actual = day01::create("real.txt").solve_part2();

        assert_eq!(actual, "193697");
    }

    #[test]
    fn day02_part1() {
        let actual = day02::create("real.txt").solve_part1();

        assert_eq!(actual, "13565");
    }

    #[test]
    fn day02_part2() {
        let actual = day02::create("real.txt").solve_part2();

        assert_eq!(actual, "12424");
    }

    #[test]
    fn day03_part1() {
        let actual = day03::create("real.txt").solve_part1();

        assert_eq!(actual, "7872");
    }

    #[test]
    fn day03_part2() {
        let actual = day03::create("real.txt").solve_part2();

        assert_eq!(actual, "2497");
    }

    #[test]
    fn day04_part1() {
        let actual = day04::create("real.txt").solve_part1();

        assert_eq!(actual, "503");
    }

    #[test]
    fn day04_part2() {
        let actual = day04::create("real.txt").solve_part2();

        assert_eq!(actual, "827");
    }

    #[test]
    fn day05_part1() {
        let actual = day05::create("real.txt").solve_part1();

        assert_eq!(actual, "TGWSMRBPN");
    }

    #[test]
    fn day05_part2() {
        let actual = day05::create("real.txt").solve_part2();

        assert_eq!(actual, "TZLTLWRNF");
    }

    #[test]
    fn day06_part1() {
        let actual = day06::create("real.txt").solve_part1();

        assert_eq!(actual, "1658");
    }

    #[test]
    fn day06_part2() {
        let actual = day06::create("real.txt").solve_part2();

        assert_eq!(actual, "2260");
    }

    #[test]
    fn day07_part1() {
        let actual = day07::create("real.txt").solve_part1();

        assert_eq!(actual, "1908462");
    }

    #[test]
    fn day07_part2() {
        let actual = day07::create("real.txt").solve_part2();

        assert_eq!(actual, "3979145");
    }

    #[test]
    fn day08_part1() {
        let actual = day08::create("real.txt").solve_part1();

        assert_eq!(actual, "1681");
    }

    #[test]
    fn day08_part2() {
        let actual = day08::create("real.txt").solve_part2();

        assert_eq!(actual, "201684");
    }

    #[test]
    fn day09_part1() {
        let actual = day09::create("real.txt").solve_part1();

        assert_eq!(actual, "6044");
    }

    #[test]
    fn day09_part2() {
        let actual = day09::create("real.txt").solve_part2();

        assert_eq!(actual, "2384");
    }

    #[test]
    fn day10_part1() {
        let actual = day10::create("real.txt").solve_part1();

        assert_eq!(actual, "12740");
    }

    #[test]
    fn day10_part2() {
        let actual = day10::create("real.txt").solve_part2();

        assert_eq!(actual, "RBPARAGF");
    }

    #[test]
    fn day11_part1() {
        let actual = day11::create("real.txt").solve_part1();

        assert_eq!(actual, "99852");
    }

    #[test]
    fn day11_part2() {
        let actual = day11::create("real.txt").solve_part2();

        assert_eq!(actual, "25935263541");
    }

    #[test]
    fn day12_part1() {
        let actual = day12::create("real.txt").solve_part1();

        assert_eq!(actual, "412");
    }

    #[test]
    fn day12_part2() {
        let actual = day12::create("real.txt").solve_part2();

        assert_eq!(actual, "402");
    }

    #[test]
    fn day13_part1() {
        let actual = day13::create("real.txt").solve_part1();

        assert_eq!(actual, "6656");
    }

    #[test]
    fn day13_part2() {
        let actual = day13::create("real.txt").solve_part2();

        assert_eq!(actual, "19716");
    }

    #[test]
    fn day14_part1() {
        let actual = day14::create("real.txt").solve_part1();

        assert_eq!(actual, "913");
    }

    #[test]
    fn day14_part2() {
        let actual = day14::create("real.txt").solve_part2();

        assert_eq!(actual, "30762");
    }

    #[test]
    fn day15_part1() {
        let actual = day15::create("real.txt").solve_part1();

        assert_eq!(actual, "5688618");
    }

    #[test]
    fn day15_part2() {
        let actual = day15::create("real.txt").solve_part2();

        assert_eq!(actual, "12625383204261");
    }

    #[test]
    fn day16_part1() {
        let actual = day16::create("real.txt").solve_part1();

        assert_eq!(actual, "1617");
    }

    // TODO: bring this back i optimize it under 6 seconds
    #[ignore]
    #[test]
    fn day16_part2() {
        let actual = day16::create("real.txt").solve_part2();

        assert_eq!(actual, "2171");
    }

    #[test]
    fn day17_part1() {
        let actual = day17::create("real.txt").solve_part1();

        assert_eq!(actual, "3090");
    }

    #[test]
    fn day17_part2() {
        let actual = day17::create("real.txt").solve_part2();

        assert_eq!(actual, "1514285714288");
    }

    #[test]
    fn day18_part1() {
        let actual = day18::create("real.txt").solve_part1();

        assert_eq!(actual, "4314");
    }

    // TODO: bring this back when i figure it out
    #[test]
    #[ignore]
    fn day18_part2() {
        let actual = day18::create("real.txt").solve_part2();

        assert_eq!(actual, "0");
    }
}
