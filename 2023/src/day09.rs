use crate::utils;

pub fn part1(file_name: &str) -> i32 {
    let input = utils::read_file("day09", file_name);

    let histories: Vec<History> = input.lines().map(History::parse).collect();

    return histories.iter().map(History::next_value).sum();
}

pub fn part2(file_name: &str) -> i32 {
    let input = utils::read_file("day09", file_name);

    let histories: Vec<History> = input.lines().map(History::parse).collect();

    return histories.iter().map(History::previous_value).sum();
}

struct History {
    sequences: Vec<Vec<i32>>,
}

impl History {
    fn parse(input: &str) -> History {
        let numbers: Vec<i32> = input
            .split(" ")
            .filter(|value| !value.is_empty())
            .map(|value| value.parse::<i32>().unwrap())
            .collect();
        let mut sequences: Vec<Vec<i32>> = Vec::new();
        sequences.push(numbers);
        let mut finished = false;
        while !finished {
            let next_history: Vec<i32> = sequences
                .last()
                .unwrap()
                .windows(2)
                .map(|arr| arr[1] - arr[0])
                .collect();
            sequences.push(next_history);
            finished = is_all_zeroes(&sequences.last().unwrap());
        }

        return History { sequences };
    }

    fn next_value(&self) -> i32 {
        return self.sequences.iter().map(|vec| vec.last().unwrap()).sum();
    }

    fn previous_value(&self) -> i32 {
        return self
            .sequences
            .iter()
            .rev()
            .map(|vec| vec.first().unwrap())
            .fold(0, |a, b| b - a);
    }
}

fn is_all_zeroes(input: &Vec<i32>) -> bool {
    for num in input {
        if num != &0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 114);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 2);
    }

    #[test]
    fn history_parse_works() {
        let actual = History::parse("0 3 6 9 12 15");

        assert_eq!(actual.sequences[0], vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(actual.sequences[1], vec![3, 3, 3, 3, 3]);
        assert_eq!(actual.sequences[2], vec![0, 0, 0, 0]);
    }

    #[test]
    fn history_next_value_works() {
        let history = History::parse("0 3 6 9 12 15");
        let actual = history.next_value();

        assert_eq!(actual, 18)
    }

    #[test]
    fn history_previous_value_works() {
        let history = History::parse("10 13 16 21 30 45");
        let actual = history.previous_value();

        assert_eq!(actual, 5);
    }

    #[test]
    fn history_previous_value_thorough() {
        let histories: Vec<History> = utils::read_file("day09", "test.txt")
            .lines()
            .map(History::parse)
            .collect();

        assert_eq!(histories[0].previous_value(), -3);
        assert_eq!(histories[1].previous_value(), 0);
        assert_eq!(histories[2].previous_value(), 5);
    }
}
