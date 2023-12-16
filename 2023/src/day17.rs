use crate::utils;

pub fn part1(file_name: &str) -> i32 {
    let input = utils::read_file("day17", file_name);

    return part1_with_input(&input);
}

pub fn part1_with_input(input: &str) -> i32 {
    return input.len() as i32;
}

pub fn part2(file_name: &str) -> i32 {
    let input = utils::read_file("day17", file_name);

    return part2_with_input(&input);
}

pub fn part2_with_input(input: &str) -> i32 {
    return input.len() as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 13);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 13);
    }
}
