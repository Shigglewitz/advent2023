use crate::create_advent_day;

create_advent_day!("13");

fn part1_with_input(input: &str) -> i32 {
    let patterns = Pattern::parse(&input);
    return patterns.iter().map(|pattern| pattern.score(false)).sum();
}

fn part2_with_input(input: &str) -> i32 {
    let patterns = Pattern::parse(&input);
    return patterns.iter().map(|pattern| pattern.score(true)).sum();
}

enum Orientation {
    HORIZONTAL,
    VERTICAL,
}

struct Pattern {
    lines: Vec<String>,
    cols: Vec<String>,
}

impl Pattern {
    fn parse(input: &str) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = Vec::new();
        let mut current_pattern: Vec<String> = Vec::new();
        for line in input.lines() {
            if line.is_empty() {
                patterns.push(Pattern {
                    lines: current_pattern,
                    cols: Vec::new(),
                });
                current_pattern = Vec::new();
            } else {
                current_pattern.push(line.to_string());
            }
        }
        if !current_pattern.is_empty() {
            patterns.push(Pattern {
                lines: current_pattern,
                cols: Vec::new(),
            })
        }

        let num_patterns = patterns.len();
        for i in 0..num_patterns {
            let num_cols = patterns[i].lines[0].len();
            let line_vecs: Vec<Vec<char>> = patterns[i]
                .lines
                .iter()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect();
            for j in 0..num_cols {
                let new_col = line_vecs
                    .iter()
                    .map(|line_vec| line_vec[j])
                    .collect::<String>();
                patterns[i].cols.push(new_col);
            }
        }

        return patterns;
    }

    fn find_mirror(&self, orientation: Orientation, allow_smudge: bool) -> Option<i32> {
        let mut possible_indices: Vec<usize> = Vec::new();
        let array = match orientation {
            Orientation::HORIZONTAL => &self.lines,
            Orientation::VERTICAL => &self.cols,
        };
        for (index, arr) in array.windows(2).enumerate() {
            if allow_smudge {
                possible_indices.push(index)
            } else if arr[0] == arr[1] {
                possible_indices.push(index);
            }
        }
        let allowed_differences = if allow_smudge { 1 } else { 0 };

        return possible_indices
            .iter()
            .filter(|index| self.is_mirror(index, array, allowed_differences))
            .map(|index| *index as i32)
            .collect::<Vec<i32>>()
            .first()
            .copied();
    }

    fn is_mirror(
        &self,
        possible_index: &usize,
        array: &Vec<String>,
        allowed_differences: i32,
    ) -> bool {
        let mut top_index = *possible_index as i32;
        let mut bottom_index = possible_index + 1;
        let mut differences = 0;

        while top_index >= 0 && bottom_index < array.len() {
            let num_chars = array[bottom_index].len();
            let mut top_iter = array[top_index as usize].chars();
            let mut bottom_iter = array[bottom_index].chars();
            for _ in 0..num_chars {
                if top_iter.next().unwrap() != bottom_iter.next().unwrap() {
                    differences += 1;
                }
            }
            top_index -= 1;
            bottom_index += 1;
        }

        return differences == allowed_differences;
    }

    fn score(&self, allow_smudge: bool) -> i32 {
        let vertical_mirror = self.find_mirror(Orientation::VERTICAL, allow_smudge);
        let horizontal_mirror = self.find_mirror(Orientation::HORIZONTAL, allow_smudge);

        if vertical_mirror.is_some() {
            return vertical_mirror.unwrap() + 1;
        }
        if horizontal_mirror.is_some() {
            return (horizontal_mirror.unwrap() + 1) * 100;
        }

        panic!("no mirror found!");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn test_patterns() -> Vec<Pattern> {
        let input = utils::read_file("day13", "test.txt");
        return Pattern::parse(&input);
    }

    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "405");
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(&actual, "400");
    }

    #[test]
    fn patterns_parse_works() {
        let patterns = test_patterns();

        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].lines[0], "#.##..##.");
        assert_eq!(patterns[0].lines[4], "..#.##.#.");
        assert_eq!(patterns[1].lines[1], "#....#..#");
        assert_eq!(patterns[1].lines[6], "#....#..#");

        assert_eq!(patterns[0].cols[0], "#.##..#");
        assert_eq!(patterns[0].cols[8], "..##...");
    }

    #[rstest]
    #[case(0, Orientation::HORIZONTAL, None)]
    #[case(1, Orientation::HORIZONTAL, Some(3))]
    #[case(0, Orientation::VERTICAL, Some(4))]
    #[case(1, Orientation::VERTICAL, None)]
    fn patterns_find_mirror_tests(
        #[case] pattern_index: usize,
        #[case] orientation: Orientation,
        #[case] expected: Option<i32>,
    ) {
        let patterns = test_patterns();
        let actual = (&patterns[pattern_index]).find_mirror(orientation, false);

        assert_eq!(actual, expected);
    }

    #[test]
    fn patterns_score_works() {
        let patterns = test_patterns();

        assert_eq!(patterns[0].score(false), 5);
        assert_eq!(patterns[1].score(false), 400);
    }
}
