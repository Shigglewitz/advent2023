use crate::utils;
use std::cmp;

pub fn part1(file_name: &str) -> i32 {
    return do_stuff(file_name);
}

pub fn part2(file_name: &str) -> i32 {
    return do_stuff(file_name);
}

fn do_stuff(file_name: &str) -> i32 {
    let input = utils::read_file("day3", file_name);

    let schematic = get_schematic(input);

    return schematic
        .parts
        .iter()
        .filter(|part| borders_special_char(&schematic, &part))
        .map(|part| part.value)
        .sum();
}

struct Schematic {
    lines: Vec<String>,
    parts: Vec<PartNumber>,
}

struct PartNumber {
    line_number: usize,
    start_col: i32,
    end_col: i32,
    value: i32,
}

fn get_top_border(schematic: &Schematic, part: &PartNumber) -> String {
    if part.line_number == 0 {
        return "".to_string();
    }
    let left_col = cmp::max(part.start_col - 1, 0) as usize;
    let right_col = cmp::min(
        schematic.lines[part.line_number - 1].len(),
        part.end_col as usize + 1,
    );

    return schematic.lines[part.line_number - 1][left_col..right_col].to_string();
}

fn get_bottom_border(schematic: &Schematic, part: &PartNumber) -> String {
    if part.line_number == schematic.lines.len() - 1 {
        return "".to_string();
    }
    let left_col = cmp::max(part.start_col - 1, 0) as usize;
    let right_col = cmp::min(
        schematic.lines[part.line_number + 1].len(),
        part.end_col as usize + 1,
    );
    return schematic.lines[part.line_number + 1][left_col..right_col].to_string();
}

fn get_left_border(schematic: &Schematic, part: &PartNumber) -> String {
    if part.start_col == 0 {
        return "".to_string();
    }
    let left_col = part.start_col as usize - 1;
    let right_col = part.start_col as usize;
    return schematic.lines[part.line_number][left_col..right_col].to_string();
}

fn get_right_border(schematic: &Schematic, part: &PartNumber) -> String {
    if part.end_col == schematic.lines[part.line_number].len() as i32 {
        return "".to_string();
    }
    let left_col = part.end_col as usize;
    let right_col = part.end_col as usize + 1;
    return schematic.lines[part.line_number][left_col..right_col].to_string();
}

fn borders_special_char(schematic: &Schematic, part: &PartNumber) -> bool {
    let top = get_top_border(schematic, part);
    let bottom = get_bottom_border(schematic, part);
    let left = get_left_border(schematic, part);
    let right = get_right_border(schematic, part);

    return contains_special_char(&top)
        || contains_special_char(&bottom)
        || contains_special_char(&left)
        || contains_special_char(&right);
}

fn is_special_char(input: char) -> bool {
    return !input.is_digit(10) && input != '.';
}

fn contains_special_char(input: &str) -> bool {
    for letter in input.chars() {
        if is_special_char(letter) {
            return true;
        }
    }
    return false;
}

fn get_schematic(input: String) -> Schematic {
    let mut lines = Vec::new();
    let mut parts = Vec::new();

    for (line_num, line) in input.lines().enumerate() {
        lines.push(line.to_string());
        let this_line_parts: &mut Vec<PartNumber> = &mut get_part_numbers(line_num, line);
        parts.append(this_line_parts);
    }

    return Schematic {
        lines: lines,
        parts: parts,
    };
}

fn get_part_numbers(line_num: usize, line: &str) -> Vec<PartNumber> {
    let mut parts = Vec::new();
    let mut value: i32 = 0;
    let mut start_col: i32 = 0;
    for (col, letter) in line.chars().enumerate() {
        if letter.is_digit(10) {
            if value == 0 {
                start_col = col as i32;
            }
            value = value * 10 + letter.to_digit(10).unwrap() as i32;
        } else {
            if value != 0 {
                parts.push(PartNumber {
                    line_number: line_num,
                    start_col: start_col,
                    end_col: col as i32,
                    value: value,
                });
                value = 0;
                start_col = 0;
            }
        }
    }
    if value != 0 {
        parts.push(PartNumber {
            line_number: line_num,
            start_col: start_col,
            end_col: line.len() as i32,
            value: value,
        });
    }
    return parts;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // static TEST_SCHEMATIC: Schematic = get_schematic(utils::read_file("day3", "test.txt"));

    fn test_schematic() -> Schematic {
        return get_schematic(utils::read_file("day3", "test.txt"));
    }

    #[rstest]
    #[case(0, "")]
    #[case(1, "")]
    #[case(2, "")]
    #[case(3, "..*.")]
    #[case(4, ".....")]
    #[case(5, "....")]
    #[case(6, "....")]
    #[case(7, "....+")]
    #[case(8, ".....")]
    #[case(9, "...$.")]
    #[case(10, ".*...")]
    fn top_border_tests(#[case] index: usize, #[case] expected: &str) {
        let schematic = test_schematic();
        let top_border = get_top_border(&schematic, &schematic.parts[index]);

        assert_eq!(top_border, expected.to_string());
    }

    #[rstest]
    #[case(0, "...*")]
    #[case(1, ".....")]
    #[case(2, "..")]
    #[case(3, "....")]
    #[case(4, ".#...")]
    #[case(5, "....")]
    #[case(6, "....")]
    #[case(7, ".....")]
    #[case(8, "*....")]
    #[case(9, "")]
    #[case(10, "")]
    fn bottom_border_tests(#[case] index: usize, #[case] expected: &str) {
        let schematic = test_schematic();
        let bottom_border = get_bottom_border(&schematic, &schematic.parts[index]);

        assert_eq!(bottom_border, expected.to_string());
    }

    #[rstest]
    #[case(0, "")]
    #[case(1, ".")]
    #[case(2, ".")]
    #[case(3, ".")]
    #[case(4, ".")]
    #[case(5, "")]
    #[case(6, ".")]
    #[case(7, ".")]
    #[case(8, ".")]
    #[case(9, ".")]
    #[case(10, ".")]
    fn left_border_tests(#[case] index: usize, #[case] expected: &str) {
        let schematic = test_schematic();
        let left_border = get_left_border(&schematic, &schematic.parts[index]);

        assert_eq!(left_border, expected.to_string());
    }

    #[rstest]
    #[case(0, ".")]
    #[case(1, ".")]
    #[case(2, "")]
    #[case(3, ".")]
    #[case(4, ".")]
    #[case(5, "*")]
    #[case(6, ".")]
    #[case(7, ".")]
    #[case(8, ".")]
    #[case(9, ".")]
    #[case(10, ".")]
    fn right_border_tests(#[case] index: usize, #[case] expected: &str) {
        let schematic = test_schematic();
        let right_border = get_right_border(&schematic, &schematic.parts[index]);

        assert_eq!(right_border, expected.to_string());
    }

    #[rstest]
    #[case(0)]
    #[case(3)]
    #[case(4)]
    #[case(5)]
    #[case(7)]
    #[case(8)]
    #[case(9)]
    #[case(10)]
    fn borders_special_char_true(#[case] index: usize) {
        let schematic = get_schematic(utils::read_file("day3", "test.txt"));
        let part = &schematic.parts[index];
        let actual = borders_special_char(&schematic, part);

        assert_eq!(actual, true);
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(6)]
    fn borders_special_char_false(#[case] index: usize) {
        let schematic = get_schematic(utils::read_file("day3", "test.txt"));
        let part = &schematic.parts[index];
        let actual = borders_special_char(&schematic, part);

        assert_eq!(actual, false);
    }

    #[rstest]
    #[case('1', false)]
    #[case('.', false)]
    #[case('#', true)]
    #[case('*', true)]
    fn is_special_char_test(#[case] input: char, #[case] expected: bool) {
        let actual = is_special_char(input);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("....", false)]
    #[case("", false)]
    #[case(".23.", false)]
    #[case("..*..", true)]
    fn contains_special_char_tests(#[case] input: &str, #[case] expected: bool) {
        let actual = contains_special_char(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn get_schematic_works() {
        let input = utils::read_file("day3", "test.txt");
        let actual = get_schematic(input);

        assert_eq!(actual.lines.len(), 10);
        assert_eq!(actual.parts.len(), 11);
        assert_eq!(actual.parts[9].value, 664);
    }

    #[test]
    fn get_part_numbers_works() {
        let actual = get_part_numbers(1, "467..114..");

        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0].line_number, 1);
        assert_eq!(actual[0].value, 467);
        assert_eq!(actual[0].start_col, 0);
        assert_eq!(actual[0].end_col, 3);
        assert_eq!(actual[1].line_number, 1);
        assert_eq!(actual[1].value, 114);
        assert_eq!(actual[1].start_col, 5);
        assert_eq!(actual[1].end_col, 8);
    }

    #[ignore]
    #[test]
    fn do_stuff_works() {
        let file_length = do_stuff("test.txt");

        assert_eq!(4361, file_length);
    }
}
