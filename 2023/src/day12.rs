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
        let num_unknowns = self
            .conditions
            .iter()
            .filter(|condition| **condition == SpringCondition::UNKNOWN)
            .count();
        let limit = 2_i64.pow(num_unknowns as u32);
        let mut count = 0;
        for i in 0..limit {
            let mut current = i;
            let test_me = self
                .conditions
                .iter()
                .map(|condition| match *condition {
                    SpringCondition::DAMAGED => '#',
                    SpringCondition::OPERATIONAL => '.',
                    SpringCondition::UNKNOWN => {
                        let this_char = if current % 2 == 0 { '#' } else { '.' };
                        current /= 2;
                        this_char
                    }
                })
                .collect::<String>();
            if self.is_valid(&test_me) {
                count += 1;
            }
        }

        return count;
    }

    fn is_valid(&self, test_me: &str) -> bool {
        let mut found_groups: Vec<i32> = Vec::new();
        let mut group_size = 0;
        let mut searching_for_damaged = true;
        for spring in test_me.chars() {
            if !searching_for_damaged {
                if spring == '#' {
                    group_size += 1;
                } else if spring == '.' {
                    found_groups.push(group_size);
                    group_size = 0;
                    searching_for_damaged = true;
                }
            } else if spring == '#' {
                group_size += 1;
                searching_for_damaged = false;
            }
        }
        if !searching_for_damaged && group_size != 0 {
            found_groups.push(group_size)
        }
        return found_groups == self.damaged_groups;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn test_records() -> Vec<ConditionRecord> {
        let input = utils::read_file("day12", "test.txt");
        let records = input
            .lines()
            .map(ConditionRecord::parse)
            .collect::<Vec<ConditionRecord>>();

        return records;
    }

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 21);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 21);
    }

    #[test]
    fn condition_record_parse_works() {
        let records = test_records();

        assert_eq!(records[0].conditions[0], SpringCondition::UNKNOWN);
        assert_eq!(records[0].conditions[1], SpringCondition::UNKNOWN);
        assert_eq!(records[0].conditions[2], SpringCondition::UNKNOWN);
        assert_eq!(records[0].conditions[3], SpringCondition::OPERATIONAL);
        assert_eq!(records[0].conditions[4], SpringCondition::DAMAGED);
        assert_eq!(records[0].conditions[5], SpringCondition::DAMAGED);
        assert_eq!(records[0].conditions[6], SpringCondition::DAMAGED);
        assert_eq!(records[0].damaged_groups, vec![1, 1, 3]);
    }

    #[rstest]
    #[case(0, "#.#.###", true)]
    #[case(0, ".#..###", false)]
    #[case(1, "..........###", false)]
    #[case(1, ".#...#....###", true)]
    #[case(1, ".#....#...###", true)]
    #[case(1, "..#..#....###", true)]
    #[case(1, "..#...#...###", true)]
    fn condition_record_is_valid_tests(
        #[case] index: usize,
        #[case] input: &str,
        #[case] expected: bool,
    ) {
        let records = test_records();
        let record = &records[index];
        let actual = record.is_valid(input);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 4)]
    #[case(2, 1)]
    #[case(3, 1)]
    #[case(4, 4)]
    #[case(5, 10)]
    fn condition_record_num_arrangements_tests(#[case] index: usize, #[case] expected: i32) {
        let records = test_records();
        let record = &records[index];
        let actual = record.num_different_arrangements();

        assert_eq!(actual, expected);
    }
}
