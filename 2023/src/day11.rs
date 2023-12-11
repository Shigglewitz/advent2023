use crate::utils;

pub fn part1(file_name: &str) -> i64 {
    let input = utils::read_file("day11", file_name);

    let mut outer_space = OuterSpace::parse(&input);
    return outer_space.solve(1);
}

pub fn part2(file_name: &str) -> i64 {
    let input = utils::read_file("day11", file_name);

    let mut outer_space = OuterSpace::parse(&input);
    return outer_space.solve(999999);
}

struct OuterSpace {
    galaxies: Vec<Galaxy>,
    width: i64,
    depth: i64,
}

impl OuterSpace {
    fn parse(input: &str) -> OuterSpace {
        let mut galaxies: Vec<Galaxy> = Vec::new();
        let mut width = 0;
        let mut depth = 0;
        for (i, line) in input.lines().enumerate() {
            depth = depth + 1;
            width = line.len();
            for (j, char) in line.chars().enumerate() {
                if char == '#' {
                    galaxies.push(Galaxy {
                        x: j as i64,
                        y: i as i64,
                    });
                }
            }
        }
        return OuterSpace {
            galaxies,
            width: width as i64,
            depth: depth as i64,
        };
    }

    fn expand(&mut self, distance_to_expand: i64) {
        let num_galaxies = self.galaxies.len();
        for i in (0..self.width).rev() {
            if self
                .galaxies
                .iter()
                .filter(|galaxy| galaxy.x == i)
                .collect::<Vec<&Galaxy>>()
                .first()
                .is_none()
            {
                for j in 0..num_galaxies {
                    if self.galaxies[j].x > i {
                        self.galaxies[j].x = self.galaxies[j].x + distance_to_expand;
                    }
                }
            }
        }
        for i in (0..self.depth).rev() {
            if self
                .galaxies
                .iter()
                .filter(|galaxy| galaxy.y == i)
                .collect::<Vec<&Galaxy>>()
                .first()
                .is_none()
            {
                for j in 0..num_galaxies {
                    if self.galaxies[j].y > i {
                        self.galaxies[j].y = self.galaxies[j].y + distance_to_expand;
                    }
                }
            }
        }
    }

    fn distance_between_galaxies(&self, index_1: usize, index_2: usize) -> i64 {
        if index_1 == index_2 {
            return 0;
        }
        let galaxy_1 = &self.galaxies[index_1];
        let galaxy_2 = &self.galaxies[index_2];

        return (galaxy_1.x - galaxy_2.x).abs() + (galaxy_1.y - galaxy_2.y).abs();
    }

    fn solve(&mut self, distance_to_expand: i64) -> i64 {
        self.expand(distance_to_expand);
        let num_galaxies = self.galaxies.len();
        let mut sum = 0;
        for i in 0..(num_galaxies - 1) {
            for j in (i + 1)..num_galaxies {
                sum = sum + self.distance_between_galaxies(i, j);
            }
        }

        return sum;
    }
}

struct Galaxy {
    x: i64,
    y: i64,
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn test_outer_space() -> OuterSpace {
        return OuterSpace::parse(&utils::read_file("day11", "test.txt"));
    }

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 374)
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 82000210)
    }

    #[test]
    fn outer_space_parse_works() {
        let actual = test_outer_space();

        assert_eq!(actual.galaxies.len(), 9);
        assert_eq!(actual.galaxies[0].x, 3);
        assert_eq!(actual.galaxies[0].y, 0);
        assert_eq!(actual.galaxies[5].x, 9);
        assert_eq!(actual.galaxies[5].y, 6);
        assert_eq!(actual.galaxies[7].x, 0);
        assert_eq!(actual.galaxies[7].y, 9);
        assert_eq!(actual.galaxies[8].x, 4);
        assert_eq!(actual.galaxies[8].y, 9);

        assert_eq!(actual.depth, 10);
        assert_eq!(actual.width, 10);
    }

    #[test]
    fn outer_space_expand_works() {
        let mut outer_space = test_outer_space();
        outer_space.expand(1);
        let galaxies = outer_space.galaxies;

        assert_eq!(galaxies.len(), 9);
        assert_eq!(galaxies[0].x, 4);
        assert_eq!(galaxies[5].x, 12);
        assert_eq!(galaxies[5].y, 7);
    }

    #[rstest]
    #[case(4, 8, 9)]
    #[case(0, 6, 15)]
    #[case(2, 5, 17)]
    #[case(7, 8, 5)]
    fn distance_between_galaxies_tests(
        #[case] index_1: usize,
        #[case] index_2: usize,
        #[case] expected: i64,
    ) {
        let mut outer_space = test_outer_space();
        outer_space.expand(1);
        let actual = outer_space.distance_between_galaxies(index_1, index_2);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(1, 374)]
    #[case(9, 1030)]
    #[case(99, 8410)]
    fn solve_tests(#[case] distance_to_expand: i64, #[case] expected: i64) {
        let mut outer_space = test_outer_space();
        let actual = outer_space.solve(distance_to_expand);

        assert_eq!(actual, expected);
    }
}
