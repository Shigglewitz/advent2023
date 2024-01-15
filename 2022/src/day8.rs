use crate::utils;

pub fn part1(file_name: &str) -> usize {
    let input = utils::read_file("day8", file_name);
    let mut forest = Forest::parse(&input);
    forest.find_visible();
    return forest.num_visible();
}

pub fn part2(file_name: &str) -> usize {
    let input = utils::read_file("day8", file_name);
    let mut forest = Forest::parse(&input);
    forest.calculate_scenic_scores();
    return forest.highest_scenic_score();
}

struct Forest {
    trees: Vec<Vec<Tree>>,
    width: usize,
    height: usize,
}

struct Tree {
    height: i32,
    visible: bool,
    scenic_score: usize,
}

impl Forest {
    fn parse(input: &str) -> Forest {
        let trees = input
            .lines()
            .map(|line| line.chars())
            .map(|chars| {
                chars
                    .map(|char| {
                        let height = char.to_digit(10).unwrap();
                        Tree {
                            height: height as i32,
                            visible: false,
                            scenic_score: 0,
                        }
                    })
                    .collect::<Vec<Tree>>()
            })
            .collect::<Vec<_>>();
        let width = trees[0].len();
        let height = trees.len();
        return Forest {
            trees,
            width,
            height,
        };
    }

    fn find_visible(&mut self) {
        for row in &mut self.trees {
            let mut max_height = -1;
            for index in 0..self.width {
                if row[index].height > max_height {
                    row[index].visible = true;
                    max_height = row[index].height;
                }
            }
            max_height = -1;
            for index in (0..self.width).rev() {
                if row[index].height > max_height {
                    row[index].visible = true;
                    max_height = row[index].height;
                }
            }
        }
        for col in 0..self.width {
            let mut max_height = -1;
            for index in 0..self.height {
                if self.trees[index][col].height > max_height {
                    self.trees[index][col].visible = true;
                    max_height = self.trees[index][col].height;
                }
            }
            max_height = -1;
            for index in (0..self.height).rev() {
                if self.trees[index][col].height > max_height {
                    self.trees[index][col].visible = true;
                    max_height = self.trees[index][col].height;
                }
            }
        }
    }
    fn calculate_scenic_scores(&mut self) {
        for tree_row in 0..self.height {
            for tree_col in 0..self.width {
                let current_height = self.trees[tree_row][tree_col].height;
                let mut up_visible = 0;
                for index in (0..tree_row).rev() {
                    up_visible += 1;
                    if self.trees[index][tree_col].height >= current_height {
                        break;
                    }
                }
                let mut left_visible = 0;
                for index in (0..tree_col).rev() {
                    left_visible += 1;
                    if self.trees[tree_row][index].height >= current_height {
                        break;
                    }
                }
                let mut right_visible = 0;
                for index in tree_col + 1..self.width {
                    right_visible += 1;
                    if self.trees[tree_row][index].height >= current_height {
                        break;
                    }
                }
                let mut down_visible = 0;
                for index in tree_row + 1..self.height {
                    down_visible += 1;
                    if self.trees[index][tree_col].height >= current_height {
                        break;
                    }
                }
                self.trees[tree_row][tree_col].scenic_score =
                    up_visible * left_visible * right_visible * down_visible;
            }
        }
    }

    fn num_visible(&self) -> usize {
        return self
            .trees
            .iter()
            .map(|row| row.iter().filter(|tree| tree.visible).count())
            .sum();
    }

    fn highest_scenic_score(&self) -> usize {
        return self
            .trees
            .iter()
            .map(|row| row.iter().map(|tree| tree.scenic_score).max().unwrap())
            .max()
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 21);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 8)
    }
}
