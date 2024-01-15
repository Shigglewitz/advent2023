#[cfg(test)]
mod test {
    use advent::*;

    #[test]
    fn day01_part1() {
        let actual = twentythree::day01::create("real.txt").solve_part1();

        assert_eq!(actual, "55834");
    }

    #[test]
    fn day01_part2() {
        let actual = twentythree::day01::create("real.txt").solve_part2();

        assert_eq!(actual, "53221");
    }

    #[test]
    fn day02_part1() {
        let actual = twentythree::day02::create("real.txt").solve_part1();

        assert_eq!(actual, "2156");
    }

    #[test]
    fn day02_part2() {
        let actual = twentythree::day02::create("real.txt").solve_part2();

        assert_eq!(actual, "66909");
    }

    #[test]
    fn day03_part1() {
        let actual = twentythree::day03::create("real.txt").solve_part1();

        assert_eq!(actual, "525181");
    }

    #[test]
    fn day03_part2() {
        let actual = twentythree::day03::create("real.txt").solve_part2();

        assert_eq!(actual, "84289137");
    }

    #[test]
    fn day04_part1() {
        let actual = twentythree::day04::create("real.txt").solve_part1();

        assert_eq!(actual, "25571");
    }

    #[test]
    fn day04_part2() {
        let actual = twentythree::day04::create("real.txt").solve_part2();

        assert_eq!(actual, "8805731");
    }

    #[test]
    fn day05_part1() {
        let actual = twentythree::day05::create("real.txt").solve_part1();

        assert_eq!(actual, "551761867");
    }

    #[test]
    fn day05_part2() {
        let actual = twentythree::day05::create("real.txt").solve_part2();

        assert_eq!(actual, "57451709");
    }

    #[test]
    fn day06_part1() {
        let actual = twentythree::day06::create("real.txt").solve_part1();

        assert_eq!(actual, "449550");
    }

    #[test]
    fn day06_part2() {
        let actual = twentythree::day06::create("real.txt").solve_part2();

        assert_eq!(actual, "28360140");
    }

    #[test]
    fn day07_part1() {
        let actual = twentythree::day07::create("real.txt").solve_part1();

        assert_eq!(actual, "253954294");
    }

    #[test]
    fn day07_part2() {
        let actual = twentythree::day07::create("real.txt").solve_part2();

        assert_eq!(actual, "254837398");
    }

    #[test]
    fn day08_part1() {
        let actual = twentythree::day08::create("real.txt").solve_part1();

        assert_eq!(actual, "12599");
    }

    #[test]
    fn day08_part2() {
        let actual = twentythree::day08::create("real.txt").solve_part2();

        assert_eq!(actual, "8245452805243");
    }

    #[test]
    fn day09_part1() {
        let actual = twentythree::day09::create("real.txt").solve_part1();

        assert_eq!(actual, "1993300041");
    }

    #[test]
    fn day09_part2() {
        let actual = twentythree::day09::create("real.txt").solve_part2();

        assert_eq!(actual, "1038");
    }

    #[test]
    fn day10_part1() {
        let actual = twentythree::day10::create("real.txt").solve_part1();

        assert_eq!(actual, "6897");
    }

    #[test]
    fn day10_part2() {
        let actual = twentythree::day10::create("real.txt").solve_part2();

        assert_eq!(actual, "367");
    }

    #[test]
    fn day11_part1() {
        let actual = twentythree::day11::create("real.txt").solve_part1();

        assert_eq!(actual, "9795148");
    }

    #[test]
    fn day11_part2() {
        let actual = twentythree::day11::create("real.txt").solve_part2();

        assert_eq!(actual, "650672493820");
    }

    #[test]
    fn day12_part1() {
        let actual = twentythree::day12::create("real.txt").solve_part1();

        assert_eq!(actual, "7916");
    }

    #[test]
    fn day12_part2() {
        let actual = twentythree::day12::create("real.txt").solve_part2();

        assert_eq!(actual, "37366887898686");
    }

    #[test]
    fn day13_part1() {
        let actual = twentythree::day13::create("real.txt").solve_part1();

        assert_eq!(actual, "31877");
    }

    #[test]
    fn day13_part2() {
        let actual = twentythree::day13::create("real.txt").solve_part2();

        assert_eq!(actual, "42996");
    }

    #[test]
    fn day14_part1() {
        let actual = twentythree::day14::create("real.txt").solve_part1();

        assert_eq!(actual, "109654");
    }

    #[test]
    fn day14_part2() {
        let actual = twentythree::day14::create("real.txt").solve_part2();

        assert_eq!(actual, "94876");
    }

    #[test]
    fn day15_part1() {
        let actual = twentythree::day15::create("real.txt").solve_part1();

        assert_eq!(actual, "516469");
    }

    #[test]
    fn day15_part2() {
        let actual = twentythree::day15::create("real.txt").solve_part2();

        assert_eq!(actual, "221627");
    }

    #[test]
    fn day16_part1() {
        let actual = twentythree::day16::create("real.txt").solve_part1();

        assert_eq!(actual, "6994");
    }

    #[test]
    fn day16_part2() {
        let actual = twentythree::day16::create("real.txt").solve_part2();

        assert_eq!(actual, "7488");
    }

    // TODO: bring these back when i figure out how to solve these
    #[ignore]
    #[test]
    fn day17_part1() {
        let actual = twentythree::day17::create("real.txt").solve_part1();

        assert_eq!(actual, "");
    }

    // TODO: bring these back when i figure out how to solve these
    #[ignore]
    #[test]
    fn day17_part2() {
        let actual = twentythree::day17::create("real.txt").solve_part2();

        assert_eq!(actual, "");
    }

    #[test]
    fn day18_part1() {
        let actual = twentythree::day18::create("real.txt").solve_part1();

        assert_eq!(actual, "67891");
    }

    #[test]
    fn day18_part2() {
        let actual = twentythree::day18::create("real.txt").solve_part2();

        assert_eq!(actual, "94116351948493");
    }

    #[test]
    fn day19_part1() {
        let actual = twentythree::day19::create("real.txt").solve_part1();

        assert_eq!(actual, "418498");
    }

    #[test]
    fn day19_part2() {
        let actual = twentythree::day19::create("real.txt").solve_part2();

        assert_eq!(actual, "123331556462603");
    }

    #[test]
    fn day20_part1() {
        let actual = twentythree::day20::create("real.txt").solve_part1();

        assert_eq!(actual, "807069600");
    }

    // TODO: bring this back when i figure out how to solve it
    #[ignore]
    #[test]
    fn day20_part2() {
        let actual = twentythree::day20::create("real.txt").solve_part2();

        assert_eq!(actual, "");
    }

    #[test]
    fn day21_part1() {
        let actual = twentythree::day21::create("real.txt").solve_part1();

        assert_eq!(actual, "3562");
    }

    // TODO: bring this back when i figure it out
    #[ignore]
    #[test]
    fn day21_part2() {
        let actual = twentythree::day21::create("real.txt").solve_part2();

        assert_eq!(actual, "");
    }

    #[test]
    fn day22_part1() {
        let actual = twentythree::day22::create("real.txt").solve_part1();

        assert_eq!(actual, "448");
    }

    #[test]
    fn day22_part2() {
        let actual = twentythree::day22::create("real.txt").solve_part2();

        assert_eq!(actual, "57770");
    }

    #[test]
    fn day23_part1() {
        let actual = twentythree::day23::create("real.txt").solve_part1();

        assert_eq!(actual, "2246");
    }

    // TODO: profile and improve this execution, takes too long for integration tests
    #[ignore]
    #[test]
    fn day23_part2() {
        let actual = twentythree::day23::create("real.txt").solve_part2();

        assert_eq!(actual, "6622");
    }

    #[test]
    fn day24_part1() {
        let actual = twentythree::day24::create("real.txt").solve_part1();

        assert_eq!(actual, "20361");
    }

    // TODO: bring this back when i figure it out
    #[ignore]
    #[test]
    fn day24_part2() {
        let actual = twentythree::day24::create("real.txt").solve_part2();

        assert_eq!(actual, "");
    }

    // TODO: bring this back when i figure it out
    #[ignore]
    #[test]
    fn day25_part1() {
        let actual = twentythree::day25::create("real.txt").solve_part1();

        assert_eq!(actual, "");
    }

    // TODO: bring this back when i figure it out
    #[ignore]
    #[test]
    fn day25_part2() {
        let actual = twentythree::day25::create("real.txt").solve_part2();

        assert_eq!(actual, "");
    }
}
