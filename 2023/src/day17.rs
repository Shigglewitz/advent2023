use crate::utils;

pub fn part1(file_name: &str) -> i32 {
    let input = utils::read_file("day17", file_name);

    let city_map = CityMap::parse(&input);
    let _heat_loss_len = city_map.heat_loss.len();

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

struct CityMap {
    heat_loss: Vec<Vec<u8>>,
}

impl CityMap {
    fn parse(input: &str) -> CityMap {
        let heat_loss = input
            .lines()
            .map(|line| line.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        return CityMap { heat_loss };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 193);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 193);
    }

    #[rstest]
    #[case(0, "2413432311323")]
    #[case(1, "3215453535623")]
    #[case(11, "2546548887735")]
    #[case(12, "4322674655533")]
    fn city_map_parse_works(#[case] index: usize, #[case] expected: &str) {
        let input = utils::read_file("day17", "test.txt");
        let city_map = CityMap::parse(&input);

        let line = String::from_utf8(city_map.heat_loss[index].clone()).unwrap();
        assert_eq!(line, expected);
    }
}
