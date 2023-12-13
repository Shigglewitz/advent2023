use std::collections::HashMap;

use crate::utils;

pub fn part1(file_name: &str) -> usize {
    let input = utils::read_file("day12", file_name);

    return input
        .lines()
        .map(ConditionRecord::parse)
        .map(|record| record.solve())
        .sum();
}

#[allow(dead_code)]
pub fn part2(file_name: &str) -> usize {
    let input = utils::read_file("day12", file_name);

    return input
        .lines()
        .map(ConditionRecord::parse)
        .map(|record| record.solve_expanded())
        .sum();
}

fn num_different_arrangements_recurse(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    input: &[u8],
    nums: &[usize],
    tracing: Option<usize>,
) -> usize {
    if input.is_empty() {
        return match (tracing, nums.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == nums[0] => 1,
            _ => 0,
        };
    }
    if tracing.is_some() && nums.is_empty() {
        return 0;
    }
    let key = (input.len(), nums.len(), tracing.unwrap_or(0));
    let cache_value = cache.get(&key);
    if cache_value.is_some() {
        return *cache_value.unwrap();
    }
    let num_ways = match (input[0], tracing) {
        (b'#', None) => num_different_arrangements_recurse(cache, &input[1..], nums, Some(1)),
        (b'#', Some(_)) => {
            num_different_arrangements_recurse(cache, &input[1..], nums, tracing.map(|x| x + 1))
        }
        (b'.', None) => num_different_arrangements_recurse(cache, &input[1..], nums, None),
        (b'.', Some(x)) if x == nums[0] => {
            num_different_arrangements_recurse(cache, &input[1..], &nums[1..], None)
        }
        (b'.', Some(_)) => 0,
        (b'?', None) => {
            let treat_as_hash =
                num_different_arrangements_recurse(cache, &input[1..], nums, Some(1));
            let treat_as_dot = num_different_arrangements_recurse(cache, &input[1..], nums, None);
            treat_as_hash + treat_as_dot
        }
        (b'?', Some(x)) => {
            let treat_as_hash = num_different_arrangements_recurse(
                cache,
                &input[1..],
                nums,
                tracing.map(|x| x + 1),
            );
            let treat_as_dot = if x == nums[0] {
                num_different_arrangements_recurse(cache, &input[1..], &nums[1..], None)
            } else {
                0
            };
            treat_as_hash + treat_as_dot
        }

        _ => unreachable!(),
    };
    cache.insert(key, num_ways);

    return num_ways;
}

struct ConditionRecord {
    conditions_str: String,
    groups_usize: Vec<usize>,
}

impl ConditionRecord {
    fn parse(input: &str) -> ConditionRecord {
        let (springs, nums) = input.split_once(" ").unwrap();
        let groups_usize: Vec<usize> = nums
            .split(",")
            .map(|char| char.parse::<usize>().unwrap())
            .collect();
        return ConditionRecord {
            conditions_str: springs.to_string(),
            groups_usize,
        };
    }

    fn expanded_input(&self) -> String {
        return (0..5)
            .map(|_| self.conditions_str.clone())
            .collect::<Vec<String>>()
            .join("?");
    }

    fn expanded_nums(&self) -> Vec<usize> {
        return (0..5)
            .map(|_| self.groups_usize.clone())
            .flatten()
            .collect::<Vec<usize>>();
    }

    fn solve(&self) -> usize {
        let mut cache = HashMap::new();
        return num_different_arrangements_recurse(
            &mut cache,
            self.conditions_str.as_bytes(),
            &self.groups_usize,
            None,
        );
    }

    fn solve_expanded(&self) -> usize {
        let mut cache = HashMap::new();
        return num_different_arrangements_recurse(
            &mut cache,
            self.expanded_input().as_bytes(),
            &self.expanded_nums(),
            None,
        );
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

        assert_eq!(actual, 525152);
    }

    #[test]
    fn condition_record_parse_works() {
        let records = test_records();

        assert_eq!(records[0].conditions_str, "???.###");
        assert_eq!(records[0].groups_usize, vec![1, 1, 3]);
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 4)]
    #[case(2, 1)]
    #[case(3, 1)]
    #[case(4, 4)]
    #[case(5, 10)]
    fn condition_record_num_arrangements_tests(#[case] index: usize, #[case] expected: usize) {
        let records = test_records();
        let record = &records[index];
        let actual = record.solve();

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("#.#.###", vec![1,1,3], 1)]
    #[case("#.#..###", vec![1,1,3],1)]
    #[case("#.#.#.#", vec![1,1,3],0)]
    #[case("#.#.##",vec![1,1,3], 0)]
    #[case("##.#.###", vec![1,1,3],0)]
    #[case("?.#.###", vec![1,1,3],1)]
    #[case("??#.###",vec![1,1,3], 1)]
    #[case("???.###", vec![1,1,3],1)]
    #[case(".??..??...?##.",vec![1,1,3], 4)]
    #[case("?###????????",vec![3,2,1], 10)]
    fn num_different_arrangements_recurse_tests(
        #[case] input: &str,
        #[case] nums: Vec<usize>,
        #[case] expected: usize,
    ) {
        let mut cache = HashMap::new();
        let actual = num_different_arrangements_recurse(&mut cache, input.as_bytes(), &nums, None);

        assert_eq!(actual, expected);
    }

    #[test]
    fn expand_tests() {
        let records = test_records();
        let record = &records[0];
        let expanded_input = record.expanded_input();
        let expanded_nums = record.expanded_nums();

        assert_eq!(expanded_input, "???.###????.###????.###????.###????.###");
        assert_eq!(
            expanded_nums,
            vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
        );
    }
}
