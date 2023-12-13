use crate::utils;

pub fn part1(file_name: &str) -> i32 {
    let input = utils::read_file("day13", file_name);

    let patterns = Pattern::parse(&input);
    return patterns.iter().map(Pattern::score).sum();
}

pub fn part2(file_name: &str) -> i32 {
    let input = utils::read_file("day13", file_name);

    let patterns = Pattern::parse(&input);
    return patterns.iter().map(Pattern::score_smudge).sum();
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
            let line_vecs: Vec<Vec<char>> = patterns[i].lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect();
            for j in 0..num_cols {
                let new_col = line_vecs.iter().map(|line_vec| line_vec[j]).collect::<String>();
                patterns[i].cols.push(new_col);
            }
        }

        return patterns;
    }

    fn find_horizontal_mirror(&self) -> Option<i32> {
        let mut possible_indices: Vec<usize> = Vec::new();
        for (index, arr) in self.lines.windows(2).enumerate() {
            if arr[0] == arr[1] {
                possible_indices.push(index);
            }
        }

        return possible_indices.iter().filter(|index| {
            self.is_horizontal_mirror(index)
        }).map(|index| *index as i32)
        .collect::<Vec<i32>>()
        .first()
        .copied();
    }

    fn is_horizontal_mirror(&self, possible_index: &usize) -> bool {
        let mut top_index = *possible_index as i32;
        let mut bottom_index = possible_index + 1;

        while top_index >= 0 && bottom_index < self.lines.len() {
            if self.lines[top_index as usize] != self.lines[bottom_index] {
                return false;
            }
            top_index -= 1;
            bottom_index += 1;
        }

        return true;
    }

    fn find_vertical_mirror(&self) -> Option<i32> {
        let mut possible_indices: Vec<usize> = Vec::new();
        for (index, arr) in self.cols.windows(2).enumerate() {
            if arr[0] == arr[1] {
                possible_indices.push(index);
            }
        }

        return possible_indices.iter().filter(|index| {
            self.is_vertical_mirror(index)
        }).map(|index| *index as i32)
        .collect::<Vec<i32>>()
        .first()
        .copied();
    }

    fn is_vertical_mirror(&self, possible_index: &usize) -> bool {
        let mut top_index = *possible_index as i32;
        let mut bottom_index = possible_index + 1;

        while top_index >= 0 && bottom_index < self.cols.len() {
            if self.cols[top_index as usize] != self.cols[bottom_index] {
                return false;
            }
            top_index -= 1;
            bottom_index += 1;
        }

        return true;
    }

    fn score(&self) -> i32 {
        let vertical_mirror = self.find_vertical_mirror();
        let horizontal_mirror = self.find_horizontal_mirror();
        let mut sum = 0;

        match vertical_mirror {
            None => (),
            Some(val) => sum = sum + val + 1,
        }
        match horizontal_mirror {
            None => (),
            Some(val) => sum = sum + ((val + 1) * 100),
        }

        return sum;
    }

    fn find_horizontal_mirror_smudge(&self) -> Option<i32> {
        let mut possible_indices: Vec<usize> = Vec::new();
        for (index, arr) in self.lines.windows(2).enumerate() {
            
                possible_indices.push(index);
        }

        return possible_indices.iter().filter(|index| {
            self.is_horizontal_mirror_smudge(index)
        }).map(|index| *index as i32)
        .collect::<Vec<i32>>()
        .first()
        .copied();
    }

    fn is_horizontal_mirror_smudge(&self, possible_index: &usize) -> bool {
        let mut top_index = *possible_index as i32;
        let mut bottom_index = possible_index + 1;
        let mut differences = 0;

        while top_index >= 0 && bottom_index < self.lines.len() {
            let num_chars = self.lines[bottom_index].len();
            let mut top_iter = self.lines[top_index as usize].chars();
            let mut bottom_iter = self.lines[bottom_index].chars();
            for _ in 0..num_chars {
                if top_iter.next().unwrap() != bottom_iter.next().unwrap() {
                    differences += 1;
                }
            }
            top_index -= 1;
            bottom_index += 1;
        }

        return differences == 1;
    }

    fn find_vertical_mirror_smudge(&self) -> Option<i32> {
        let mut possible_indices: Vec<usize> = Vec::new();
        for (index, arr) in self.cols.windows(2).enumerate() {
                possible_indices.push(index);
        }

        return possible_indices.iter().filter(|index| {
            self.is_vertical_mirror_smudge(index)
        }).map(|index| *index as i32)
        .collect::<Vec<i32>>()
        .first()
        .copied();
    }

    fn is_vertical_mirror_smudge(&self, possible_index: &usize) -> bool {
        let mut top_index = *possible_index as i32;
        let mut bottom_index = possible_index + 1;
        let mut differences = 0;

        while top_index >= 0 && bottom_index < self.cols.len() {
            let num_chars = self.cols[bottom_index].len();
            let mut top_iter = self.cols[top_index as usize].chars();
            let mut bottom_iter = self.cols[bottom_index].chars();
            for _ in 0..num_chars {
                if top_iter.next().unwrap() != bottom_iter.next().unwrap() {
                    differences += 1;
                }
            }
            top_index -= 1;
            bottom_index += 1;
        }

        return differences == 1;
    }

    fn score_smudge(&self) -> i32 {
        let vertical_mirror = self.find_vertical_mirror_smudge();
        let horizontal_mirror = self.find_horizontal_mirror_smudge();

        if vertical_mirror.is_some() {
            return vertical_mirror.unwrap() + 1;
        }
        if horizontal_mirror.is_some() {
            return (horizontal_mirror.unwrap() + 1) * 100;
        }

        panic!("did not find a score!")
    }
}

fn num_diffs_between_str(input1: &str, input2: &str) -> i32 {
    let mut iter1 = input1.chars();
    let mut iter2 = input2.chars();
    let num_chars = input1.len();
    let mut sum = 0;

    for _ in 0..num_chars {
        if iter1.next().unwrap() != iter2.next().unwrap() {
            sum += 1;
        }
    } 

    return sum;
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_patterns() -> Vec<Pattern> {
        let input = utils::read_file("day13", "test.txt");
        return Pattern::parse(&input);
    }

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 405);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 400);
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

    #[test]
    fn patterns_horizontal_mirror_works() {
        let patterns = test_patterns();
        let first_pattern_actual = (&patterns[0]).find_horizontal_mirror();
        let second_pattern_actual = (&patterns[1]).find_horizontal_mirror();

        assert_eq!(first_pattern_actual, None);
        assert_eq!(second_pattern_actual, Some(3));
    }

    #[test]
    fn patterns_vertical_mirror_works() {
        let patterns = test_patterns();
        let first_pattern_actual = (&patterns[0]).find_vertical_mirror();
        let second_pattern_actual = (&patterns[1]).find_vertical_mirror();

        assert_eq!(first_pattern_actual, Some(4));
        assert_eq!(second_pattern_actual, None);
    }

    #[test]
    fn patterns_score_works() {
        let patterns = test_patterns();
        
        assert_eq!(patterns[0].score(), 5);
        assert_eq!(patterns[1].score(), 400);
    }
}
