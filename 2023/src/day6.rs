use crate::utils;

pub fn part1(file_name: &str) -> i64 {
    let input = utils::read_file("day6", file_name);

    let races = Race::parse(&input);
    return races.iter().map(Race::num_ways_to_beat).product();
}

pub fn part2(file_name: &str) -> i64 {
    let input = utils::read_file("day6", file_name);

    let race = Race::parse_ignore_spaces(&input);
    return race.num_ways_to_beat();
}

struct Race {
    time: i64,
    distance_record: i64,
}

impl Race {
    fn parse(input: &str) -> Vec<Race> {
        let mut lines: std::str::Lines<'_> = input.lines();
        let times: Vec<i64> = lines
            .next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split(" ")
            .filter(|value| value.len() > 0)
            .map(|value| value.parse::<i64>().unwrap())
            .collect();
        let distances: Vec<i64> = lines
            .next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split(" ")
            .filter(|value| value.len() > 0)
            .map(|value| value.parse::<i64>().unwrap())
            .collect();
        let num_races = times.len();

        return (0..num_races)
            .into_iter()
            .map(|index| Race {
                time: times[index],
                distance_record: distances[index],
            })
            .collect();
    }

    fn parse_ignore_spaces(input: &str) -> Race {
        let mut lines = input.lines();
        let time_str: String = lines
            .next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split(" ")
            .filter(|value| value.len() > 0)
            .collect();
        let distance_str: String = lines
            .next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split(" ")
            .filter(|value| value.len() > 0)
            .collect();

        return Race {
            time: time_str.parse::<i64>().unwrap(),
            distance_record: distance_str.parse::<i64>().unwrap(),
        };
    }

    fn num_ways_to_beat(&self) -> i64 {
        let mut num_ways = 0;
        for time_held in 1..self.time {
            if (self.time - time_held) * time_held > self.distance_record {
                num_ways = num_ways + 1;
            }
        }
        return num_ways;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_races() -> Vec<Race> {
        return Race::parse(&utils::read_file("day6", "test.txt"));
    }

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 288);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 71503);
    }

    #[test]
    fn race_parse_works() {
        let actual = test_races();

        assert_eq!(actual.len(), 3);
        assert_eq!(actual[1].time, 15);
        assert_eq!(actual[2].distance_record, 200);
    }

    #[test]
    fn race_num_ways_to_beat_works() {
        let actual = test_races();

        assert_eq!(actual[0].num_ways_to_beat(), 4);
        assert_eq!(actual[1].num_ways_to_beat(), 8);
        assert_eq!(actual[2].num_ways_to_beat(), 9);
    }

    #[test]
    fn race_parse_ignore_spaces_works() {
        let actual = Race::parse_ignore_spaces(&utils::read_file("day6", "test.txt"));

        assert_eq!(actual.time, 71530);
        assert_eq!(actual.distance_record, 940200);
    }
}
