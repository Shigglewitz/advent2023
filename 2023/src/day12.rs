use crate::utils;

pub fn part1(file_name: &str) -> i32 {
    let input = utils::read_file("day12", file_name);

    let records = input
        .lines()
        .map(ConditionRecord::parse)
        .collect::<Vec<ConditionRecord>>();

    return records
        .iter()
        .map(ConditionRecord::num_different_arrangements)
        .sum();
}

pub fn part2(file_name: &str) -> i32 {
    return part1(file_name);
}

#[derive(PartialEq, Debug)]
enum SpringCondition {
    OPERATIONAL,
    DAMAGED,
    UNKNOWN,
}

struct ConditionRecord {
    conditions: Vec<SpringCondition>,
    damaged_groups: Vec<i32>,
}

impl ConditionRecord {
    fn parse(input: &str) -> ConditionRecord {
        let mut split = input.split(" ");
        let conditions = split
            .next()
            .unwrap()
            .chars()
            .map(|spring| match spring {
                '#' => SpringCondition::DAMAGED,
                '.' => SpringCondition::OPERATIONAL,
                '?' => SpringCondition::UNKNOWN,
                _ => panic!("unexpected char {}", spring),
            })
            .collect::<Vec<SpringCondition>>();
        let damaged_groups = split
            .next()
            .unwrap()
            .split(",")
            .map(|char| char.parse::<i32>().unwrap())
            .collect();
        return ConditionRecord {
            conditions,
            damaged_groups,
        };
    }

    fn num_different_arrangements(&self) -> i32 {
        let sum = self.conditions.len() + self.damaged_groups.len();

        return sum as i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 99);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 99);
    }

    #[test]
    fn condition_record_parse_works() {
        let input = utils::read_file("day12", "test.txt");
        let records = input
            .lines()
            .map(ConditionRecord::parse)
            .collect::<Vec<ConditionRecord>>();

        assert_eq!(records[0].conditions[0], SpringCondition::UNKNOWN);
        assert_eq!(records[0].conditions[1], SpringCondition::UNKNOWN);
        assert_eq!(records[0].conditions[2], SpringCondition::UNKNOWN);
        assert_eq!(records[0].conditions[3], SpringCondition::OPERATIONAL);
        assert_eq!(records[0].conditions[4], SpringCondition::DAMAGED);
        assert_eq!(records[0].conditions[5], SpringCondition::DAMAGED);
        assert_eq!(records[0].conditions[6], SpringCondition::DAMAGED);
        assert_eq!(records[0].damaged_groups, vec![1, 1, 3]);
    }
}
