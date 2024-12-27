use std::collections::HashMap;

use crate::create_advent_day;

create_advent_day!("2024", "21");

fn part1_with_input(_input: &str) -> i64 {
    let directions = directions();
    return _input
        .lines()
        .map(|line| complexity(line, &directions))
        .sum::<usize>() as i64;
}

fn first_robot(output: &str, directions: &HashMap<(char, char), String>) -> String {
    // println!("translating {output}");
    let mut output = output.chars().collect::<Vec<_>>();
    output.insert(0, 'A');
    let strings = output
        .windows(2)
        .map(|tuple| directions.get(&(tuple[0], tuple[1])).unwrap().clone())
        .collect::<Vec<_>>();
    // let debug = strings.join(",");
    // println!("first sequence {debug}");
    return strings.concat();
}

fn second_robot(output: &str, directions: &HashMap<(char, char), String>) -> String {
    let first_sequence = first_robot(output, directions);
    let mut output = first_sequence.chars().collect::<Vec<_>>();
    output.insert(0, 'A');
    let strings = output
        .windows(2)
        .map(|tuple| directions.get(&(tuple[0], tuple[1])).unwrap().clone())
        .collect::<Vec<_>>();
    // let debug = strings.join(",");
    // println!("second sequence {debug}");
    return strings.concat();
}

fn third_robot(output: &str, directions: &HashMap<(char, char), String>) -> String {
    let second_sequence = second_robot(output, directions);
    let mut output = second_sequence.chars().collect::<Vec<_>>();
    output.insert(0, 'A');
    let strings = output
        .windows(2)
        .map(|tuple| directions.get(&(tuple[0], tuple[1])).unwrap().clone())
        .collect::<Vec<_>>();
    // let debug = strings.join(",");
    // println!("third sequence {debug}");
    return strings.concat();
}

fn complexity(output: &str, directions: &HashMap<(char, char), String>) -> usize {
    let code = third_robot(output, directions);
    let numeric = output.trim_end_matches("A").parse::<usize>().unwrap();
    return code.len() * numeric;
}

fn directions() -> HashMap<(char, char), String> {
    let mut numerical = numerical_directions();
    let arrows = arrow_directions();

    numerical.extend(arrows);
    return numerical;
}

fn numerical_directions() -> HashMap<(char, char), String> {
    let mut map = HashMap::new();

    // A
    map.insert(('A', 'A'), "A".to_owned());
    map.insert(('A', '0'), "<A".to_owned());
    map.insert(('A', '1'), "^<<A".to_owned());
    map.insert(('A', '2'), "^<A".to_owned());
    map.insert(('A', '3'), "^A".to_owned());
    map.insert(('A', '4'), "^^<<A".to_owned());
    map.insert(('A', '5'), "^^<A".to_owned());
    map.insert(('A', '6'), "^^A".to_owned());
    map.insert(('A', '7'), "^^^<<A".to_owned());
    map.insert(('A', '8'), "^^^<A".to_owned());
    map.insert(('A', '9'), "^^^A".to_owned());
    // 0
    map.insert(('0', 'A'), ">A".to_owned());
    map.insert(('0', '0'), "A".to_owned());
    map.insert(('0', '1'), "^<A".to_owned());
    map.insert(('0', '2'), "^A".to_owned());
    map.insert(('0', '3'), "^>A".to_owned());
    map.insert(('0', '4'), "^^<A".to_owned());
    map.insert(('0', '5'), "^^A".to_owned());
    map.insert(('0', '6'), "^^>A".to_owned());
    map.insert(('0', '7'), "^^^<A".to_owned());
    map.insert(('0', '8'), "^^^A".to_owned());
    map.insert(('0', '9'), "^^^>A".to_owned());
    // 1
    map.insert(('1', 'A'), ">>vA".to_owned());
    map.insert(('1', '0'), ">vA".to_owned());
    map.insert(('1', '1'), "A".to_owned());
    map.insert(('1', '2'), ">A".to_owned());
    map.insert(('1', '3'), ">>A".to_owned());
    map.insert(('1', '4'), "^A".to_owned());
    map.insert(('1', '5'), ">^A".to_owned());
    map.insert(('1', '6'), ">>^A".to_owned());
    map.insert(('1', '7'), "^^A".to_owned());
    map.insert(('1', '8'), ">^^A".to_owned());
    map.insert(('1', '9'), ">>^^A".to_owned());
    // 2
    map.insert(('2', 'A'), ">vA".to_owned());
    map.insert(('2', '0'), "vA".to_owned());
    map.insert(('2', '1'), "<A".to_owned());
    map.insert(('2', '2'), "A".to_owned());
    map.insert(('2', '3'), ">A".to_owned());
    map.insert(('2', '4'), "<^A".to_owned());
    map.insert(('2', '5'), "^A".to_owned());
    map.insert(('2', '6'), ">^A".to_owned());
    map.insert(('2', '7'), "<^^A".to_owned());
    map.insert(('2', '8'), "^^A".to_owned());
    map.insert(('2', '9'), ">^^A".to_owned());
    // 3
    map.insert(('3', 'A'), "vA".to_owned());
    map.insert(('3', '0'), "<vA".to_owned());
    map.insert(('3', '1'), "<<A".to_owned());
    map.insert(('3', '2'), "<A".to_owned());
    map.insert(('3', '3'), "A".to_owned());
    map.insert(('3', '4'), "<<^A".to_owned());
    map.insert(('3', '5'), "<^A".to_owned());
    map.insert(('3', '6'), "^A".to_owned());
    map.insert(('3', '7'), "<<^^A".to_owned());
    map.insert(('3', '8'), "<^^A".to_owned());
    map.insert(('3', '9'), "^^A".to_owned());
    // 4
    map.insert(('4', 'A'), ">>vvA".to_owned());
    map.insert(('4', '0'), ">vvA".to_owned());
    map.insert(('4', '1'), "vA".to_owned());
    map.insert(('4', '2'), ">vA".to_owned());
    map.insert(('4', '3'), ">>vA".to_owned());
    map.insert(('4', '4'), "A".to_owned());
    map.insert(('4', '5'), ">A".to_owned());
    map.insert(('4', '6'), ">>A".to_owned());
    map.insert(('4', '7'), "^A".to_owned());
    map.insert(('4', '8'), ">^A".to_owned());
    map.insert(('4', '9'), ">>^A".to_owned());
    // 5
    map.insert(('5', 'A'), ">vvA".to_owned());
    map.insert(('5', '0'), "vvA".to_owned());
    map.insert(('5', '1'), "<vA".to_owned());
    map.insert(('5', '2'), "vA".to_owned());
    map.insert(('5', '3'), ">vA".to_owned());
    map.insert(('5', '4'), "<A".to_owned());
    map.insert(('5', '5'), "A".to_owned());
    map.insert(('5', '6'), ">A".to_owned());
    map.insert(('5', '7'), "<^A".to_owned());
    map.insert(('5', '8'), "^A".to_owned());
    map.insert(('5', '9'), ">^A".to_owned());
    // 6
    map.insert(('6', 'A'), "vvA".to_owned());
    map.insert(('6', '0'), "<vvA".to_owned());
    map.insert(('6', '1'), "<<vA".to_owned());
    map.insert(('6', '2'), "<vA".to_owned());
    map.insert(('6', '3'), "vA".to_owned());
    map.insert(('6', '4'), "<<A".to_owned());
    map.insert(('6', '5'), "<A".to_owned());
    map.insert(('6', '6'), "A".to_owned());
    map.insert(('6', '7'), "<<^A".to_owned());
    map.insert(('6', '8'), "<^A".to_owned());
    map.insert(('6', '9'), "^A".to_owned());
    // 7
    map.insert(('7', 'A'), ">>vvvA".to_owned());
    map.insert(('7', '0'), ">vvvA".to_owned());
    map.insert(('7', '1'), "vvA".to_owned());
    map.insert(('7', '2'), ">vvA".to_owned());
    map.insert(('7', '3'), ">>vvA".to_owned());
    map.insert(('7', '4'), "vA".to_owned());
    map.insert(('7', '5'), ">vA".to_owned());
    map.insert(('7', '6'), ">>vA".to_owned());
    map.insert(('7', '7'), "A".to_owned());
    map.insert(('7', '8'), ">A".to_owned());
    map.insert(('7', '9'), ">>A".to_owned());
    // 8
    map.insert(('8', 'A'), ">vvvA".to_owned());
    map.insert(('8', '0'), "vvvA".to_owned());
    map.insert(('8', '1'), "<vvA".to_owned());
    map.insert(('8', '2'), "vvA".to_owned());
    map.insert(('8', '3'), "<vvA".to_owned());
    map.insert(('8', '4'), "<vA".to_owned());
    map.insert(('8', '5'), "vA".to_owned());
    map.insert(('8', '6'), ">vA".to_owned());
    map.insert(('8', '7'), "<A".to_owned());
    map.insert(('8', '8'), "A".to_owned());
    map.insert(('8', '9'), ">A".to_owned());
    // 9
    map.insert(('9', 'A'), "vvvA".to_owned());
    map.insert(('9', '0'), "<vvvA".to_owned());
    map.insert(('9', '1'), "vv<<A".to_owned());
    map.insert(('9', '2'), "vv<A".to_owned());
    map.insert(('9', '3'), "vvA".to_owned());
    map.insert(('9', '4'), "v<<A".to_owned());
    map.insert(('9', '5'), "v<A".to_owned());
    map.insert(('9', '6'), "vA".to_owned());
    map.insert(('9', '7'), "<<A".to_owned());
    map.insert(('9', '8'), "<A".to_owned());
    map.insert(('9', '9'), "A".to_owned());

    return map;
}

fn arrow_directions() -> HashMap<(char, char), String> {
    let mut map = HashMap::new();

    // >
    map.insert(('>', '>'), "A".to_owned());
    map.insert(('>', 'v'), "<A".to_owned());
    map.insert(('>', '<'), "<<A".to_owned());
    map.insert(('>', '^'), "<^A".to_owned());
    map.insert(('>', 'A'), "^A".to_owned());
    // v
    map.insert(('v', '>'), ">A".to_owned());
    map.insert(('v', 'v'), "A".to_owned());
    map.insert(('v', '<'), "<A".to_owned());
    map.insert(('v', '^'), "^A".to_owned());
    map.insert(('v', 'A'), ">^A".to_owned());
    // <
    map.insert(('<', '>'), ">>A".to_owned());
    map.insert(('<', 'v'), ">A".to_owned());
    map.insert(('<', '<'), "A".to_owned());
    map.insert(('<', '^'), ">^A".to_owned());
    map.insert(('<', 'A'), ">>^A".to_owned());
    // ^
    map.insert(('^', '>'), ">vA".to_owned());
    map.insert(('^', 'v'), "vA".to_owned());
    map.insert(('^', '<'), "v<A".to_owned());
    map.insert(('^', '^'), "A".to_owned());
    map.insert(('^', 'A'), ">A".to_owned());
    // A
    map.insert(('A', '>'), "vA".to_owned());
    map.insert(('A', 'v'), "<vA".to_owned());
    map.insert(('A', '<'), "v<<A".to_owned());
    map.insert(('A', '^'), "<A".to_owned());
    map.insert(('A', 'A'), "A".to_owned());

    return map;
}

fn part2_with_input(_input: &str) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("126384", &actual);
    }

    #[test]
    fn first_robot_works() {
        let actual = first_robot("029A", &directions());

        assert_eq!("<A^A>^^AvvvA", actual);
    }

    #[test]
    fn second_robot_works() {
        let actual = second_robot("029A", &directions());

        assert_eq!("v<<A>>^A<A>AvA<^AA>A<vAAA>^A", actual);
    }

    // TODO: these might need to be looked at again
    #[rstest]
    #[case(
        "029A",
        "<vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A"
    )]
    #[case("980A", "v<<A>>^AAAvA^A<vA<AA>>^AvAA<^A>Av<<A>A>^AAAvA<^A>A<vA>^A<A>A")]
    #[case(
        "179A",
        "v<<A>>^A<vA<A>>^AAvAA<^A>Av<<A>>^AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A"
    )]
    #[case(
        "456A",
        "v<<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>Av<<A>A>^AAvA<^A>A"
    )]
    #[case(
        "379A",
        "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A"
    )]
    fn third_robot_works(#[case] output: &str, #[case] expected: &str) {
        let actual = third_robot(output, &directions());

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("029A", 1972)]
    #[case("980A", 58800)]
    #[case("179A", 12172)]
    #[case("456A", 29184)]
    #[case("379A", 24256)]
    fn complexity_works(#[case] output: &str, #[case] expected: usize) {
        let actual = complexity(output, &directions());

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("0", &actual);
    }
}
