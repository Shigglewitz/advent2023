use std::collections::{hash_map::Entry, HashMap, HashSet};

use crate::create_advent_day;

create_advent_day!("22");

fn part1_with_input(input: &str) -> i32 {
    let mut bricks = input.lines().map(Brick::parse).collect::<Vec<_>>();
    apply_gravity(&mut bricks);
    let relationships = BrickRelationships::find_connections(&bricks);
    return relationships.count_removable(&bricks);
}

fn part2_with_input(input: &str) -> i32 {
    let mut bricks = input.lines().map(Brick::parse).collect::<Vec<_>>();
    apply_gravity(&mut bricks);
    let relationships = BrickRelationships::find_connections(&bricks);
    return relationships.fall_potential(&bricks);
}

fn apply_gravity(bricks: &mut Vec<Brick>) {
    bricks.sort_by_key(|brick| brick.min_z());
    let num_bricks = bricks.len();
    for index in 0..num_bricks {
        let new_height_opt = (0..index)
            .filter(|&second_index| bricks[index].intercepts(&bricks[second_index]))
            .map(|second_index| bricks[second_index].max_z())
            .max();
        if let Some(new_height) = new_height_opt {
            bricks[index].fall_to(new_height + 1);
        } else {
            bricks[index].fall_to(1);
        }
    }
}

struct BrickRelationships {
    indexes_above: Vec<HashSet<usize>>,
    indexes_below: Vec<HashSet<usize>>,
}

impl BrickRelationships {
    fn find_connections(bricks: &Vec<Brick>) -> Self {
        let mut max_height_map: HashMap<u32, Vec<usize>> = HashMap::new();
        for (index, brick) in bricks.iter().enumerate() {
            let height = brick.max_z();
            let array_at_height = match max_height_map.entry(height) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(Vec::new()),
            };
            array_at_height.push(index);
        }

        let mut indexes_above: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];
        let mut indexes_below: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];
        for (index, brick) in bricks.iter().enumerate() {
            let min_height = brick.min_z();
            let bricks_below_opt = &max_height_map.get(&(min_height - 1));
            if bricks_below_opt.is_none() {
                continue;
            }
            let bricks_below = bricks_below_opt.unwrap();
            for &brick_below_index in bricks_below {
                if brick.intercepts(&bricks[brick_below_index]) {
                    indexes_above[brick_below_index].insert(index);
                    indexes_below[index].insert(brick_below_index);
                }
            }
        }

        return BrickRelationships {
            indexes_above,
            indexes_below,
        };
    }

    fn count_removable(&self, bricks: &Vec<Brick>) -> i32 {
        let mut num_removable = 0;
        for index in 0..bricks.len() {
            let above_indexes = &self.indexes_above[index];
            let count = above_indexes
                .iter()
                .filter(|&&index| self.indexes_below[index].len() == 1)
                .count();
            if count == 0 {
                num_removable += 1;
            }
        }
        return num_removable;
    }

    fn fall_potential(&self, bricks: &Vec<Brick>) -> i32 {
        let mut count = 0;
        for (index, brick) in bricks.iter().enumerate() {
            let mut check_me: HashMap<u32, HashSet<usize>> = HashMap::new();
            let mut set = HashSet::new();
            set.insert(index);
            check_me.insert(brick.max_z(), set);
            let mut analyze_this_height = brick.max_z() - 1;
            while !check_me.is_empty() {
                analyze_this_height += 1;
                let analyze_this_opt = check_me.remove(&analyze_this_height);
                if analyze_this_opt.is_none() {
                    continue;
                }
                let analyze_this = analyze_this_opt.unwrap();
                let mut bricks_to_check: HashSet<usize> = HashSet::new();
                analyze_this.iter().for_each(|&ele| {
                    bricks_to_check.extend(&self.indexes_above[ele]);
                });
                for brick_above_index in bricks_to_check {
                    let remaining_support = self.indexes_below[brick_above_index]
                        .difference(&analyze_this)
                        .next();
                    if remaining_support.is_none() {
                        let set_at_height = match check_me.entry(bricks[brick_above_index].max_z())
                        {
                            Entry::Occupied(o) => o.into_mut(),
                            Entry::Vacant(v) => v.insert(HashSet::new()),
                        };
                        set_at_height.insert(brick_above_index);
                        count += 1;
                    }
                }
            }
        }

        return count;
    }
}

struct ThreeDPoint {
    x: u32,
    y: u32,
    z: u32,
}

struct Brick {
    point1: ThreeDPoint,
    point2: ThreeDPoint,
}

impl Brick {
    fn parse(input: &str) -> Brick {
        let (start, end) = input.split_once("~").unwrap();
        let mut start_split = start.split(",");
        let point1 = ThreeDPoint {
            x: start_split.next().unwrap().parse::<u32>().unwrap(),
            y: start_split.next().unwrap().parse::<u32>().unwrap(),
            z: start_split.next().unwrap().parse::<u32>().unwrap(),
        };
        let mut end_split = end.split(",");
        let point2 = ThreeDPoint {
            x: end_split.next().unwrap().parse::<u32>().unwrap(),
            y: end_split.next().unwrap().parse::<u32>().unwrap(),
            z: end_split.next().unwrap().parse::<u32>().unwrap(),
        };
        return Brick { point1, point2 };
    }

    fn intercepts(&self, other: &Brick) -> bool {
        let x_overlap = (self.point1.x <= other.point1.x && self.point2.x >= other.point1.x)
            || (other.point1.x <= self.point1.x && other.point2.x >= self.point1.x);
        let y_overlap = (self.point1.y <= other.point1.y && self.point2.y >= other.point1.y)
            || (other.point1.y <= self.point1.y && other.point2.y >= self.point1.y);
        return x_overlap && y_overlap;
    }

    fn fall_to(&mut self, new_z: u32) {
        let diff = self.point1.z - new_z;
        self.point1.z -= diff;
        self.point2.z -= diff;
    }

    fn max_z(&self) -> u32 {
        return self.point1.z.max(self.point2.z);
    }

    fn min_z(&self) -> u32 {
        return self.point1.z.min(self.point2.z);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn test_bricks() -> Vec<Brick> {
        let input = utils::read_file("day22", "test.txt");
        return input.lines().map(Brick::parse).collect::<Vec<_>>();
    }

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!(actual, "5");
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(actual, "7");
    }

    #[test]
    fn brick_parse_works() {
        let bricks = test_bricks();

        assert_eq!(bricks.len(), 7);
        assert_eq!(bricks[0].point1.y, 0);
        assert_eq!(bricks[0].point2.y, 2);
    }

    #[rstest]
    #[case("test.txt")]
    #[case("real.txt")]
    fn brick_points_in_right_order(#[case] file_name: &str) {
        let input = utils::read_file("day22", file_name);
        let bricks = input.lines().map(Brick::parse).collect::<Vec<_>>();

        bricks.iter().for_each(|brick| {
            assert!(
                brick.point1.x + brick.point1.y + brick.point1.z
                    <= brick.point2.x + brick.point2.y + brick.point2.z
            );
        });
    }

    #[rstest]
    #[case(0, 1, true)]
    #[case(0, 2, true)]
    #[case(1, 2, false)]
    #[case(1, 6, false)]
    #[case(2, 6, false)]
    fn brick_intercept_tests(
        #[case] first_index: usize,
        #[case] second_index: usize,
        #[case] expected: bool,
    ) {
        let bricks = test_bricks();
        let actual = bricks[first_index].intercepts(&bricks[second_index]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn brick_intercept_test() {
        let brick1 = Brick::parse("9,6,6~9,7,6");
        let brick2 = Brick::parse("6,6,6~9,6,6");

        assert!(brick1.intercepts(&brick2));
    }

    #[test]
    fn fall_to_works() {
        let mut bricks = test_bricks();
        bricks[1].fall_to(1);
        assert_eq!(bricks[1].point1.z, 1);
        assert_eq!(bricks[1].point2.z, 1);

        bricks[6].fall_to(1);
        assert_eq!(bricks[6].point1.z, 1);
        assert_eq!(bricks[6].point2.z, 2);
    }

    #[test]
    fn apply_gravity_works() {
        let mut bricks = test_bricks();
        apply_gravity(&mut bricks);
        assert_eq!(bricks[2].point1.z, 2);
    }

    #[test]
    fn reddit_test_case_1() {
        let input = "0,0,1~0,1,1
1,1,1~1,1,1
0,0,2~0,0,2
0,1,2~1,1,2";
        let actual = part1_with_input(input);

        assert_eq!(actual, 3);
    }

    #[test]
    fn reddit_test_case_2() {
        let input = "0,0,1~1,0,1
0,1,1~0,1,2
0,0,5~0,0,5
0,0,4~0,1,4";
        let actual = part1_with_input(input);

        assert_eq!(actual, 2);
    }

    #[test]
    fn reddit_test_case_3() {
        let input = "0,0,1~5,0,1
1,0,2~6,0,2";
        let actual = part1_with_input(input);

        assert_eq!(actual, 1);
    }

    #[test]
    fn reddit_test_fcase_5() {
        let input = "0,0,1~0,0,2
1,0,1~2,0,1
1,0,2~1,0,2
0,0,3~1,0,3";
        let actual = part1_with_input(input);

        assert_eq!(actual, 3);
    }

    #[test]
    fn vertical_test_case() {
        let input = "0,0,5~0,0,9
0,1,20~0,1,29
0,1,31~0,1,34";

        let actual = part1_with_input(input);

        assert_eq!(actual, 2);
    }
}
