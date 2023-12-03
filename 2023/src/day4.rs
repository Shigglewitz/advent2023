use crate::utils;

pub fn part1(file_name: &str) -> i32 {
    let input = utils::read_file("day4", file_name);

    return input.len() as i32;
}

pub fn part2(file_name: &str) -> i32 {
    let input = utils::read_file("day4", file_name);

    return input.len() as i32;
}

#[cfg(test)]
mod tests {
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
