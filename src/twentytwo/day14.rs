use crate::create_advent_day;

create_advent_day!("2022", "14");

fn part1_with_input(input: &str) -> u32 {
    let mut cavern = Cavern::parse(input, false);
    cavern.fill();
    return cavern.grains_of_sand;
}

fn part2_with_input(input: &str) -> u32 {
    let mut cavern = Cavern::parse(input, true);
    cavern.fill();
    return cavern.grains_of_sand;
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(input: &str) -> Self {
        let (x_str, y_str) = input.split_once(",").unwrap();
        return Self {
            x: x_str.parse::<usize>().unwrap(),
            y: y_str.parse::<usize>().unwrap(),
        };
    }

    fn point_range(&self, other: &Point) -> Vec<Point> {
        let mut start = self;
        let mut end = other;

        if self.x + self.y > other.x + other.y {
            start = other;
            end = self;
        }

        let mut points = Vec::new();
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                points.push(Point { x, y })
            }
        }

        return points;
    }
}

struct Cavern {
    space: Vec<Vec<u8>>,
    grains_of_sand: u32,
    height: usize,
    _width: usize,
}

impl Cavern {
    fn parse(input: &str, include_false_floor: bool) -> Self {
        let mut points: Vec<Vec<Point>> = input
            .lines()
            .map(|line| line.split(" -> ").map(Point::parse).collect())
            .collect();
        let max_y = points
            .iter()
            .map(|vec| vec.iter().map(|point| point.y).max().unwrap())
            .max()
            .unwrap()
            + 3;
        let max_x = points
            .iter()
            .map(|vec| vec.iter().map(|point| point.x).max().unwrap())
            .max()
            .unwrap()
            + max_y;
        if include_false_floor {
            points.push(vec![
                Point { x: 0, y: max_y - 1 },
                Point {
                    x: max_x - 1,
                    y: max_y - 1,
                },
            ]);
        }

        let mut space = vec![vec![b'.'; max_x]; max_y];
        points.iter().for_each(|row| {
            row.windows(2).for_each(|window| {
                let range = window[0].point_range(&window[1]);
                for point in range {
                    space[point.y][point.x] = b'#';
                }
            })
        });
        return Self {
            space: space,
            grains_of_sand: 0,
            height: max_y,
            _width: max_x,
        };
    }

    fn fill(&mut self) {
        loop {
            let mut location = Point { x: 500, y: 0 };
            let mut landed = false;
            while !landed {
                if location.y == self.height - 1 {
                    return;
                }
                if self.space[location.y + 1][location.x] == b'.' {
                    location = Point {
                        x: location.x,
                        y: location.y + 1,
                    };
                    continue;
                }
                if self.space[location.y + 1][location.x - 1] == b'.' {
                    location = Point {
                        x: location.x - 1,
                        y: location.y + 1,
                    };
                    continue;
                }
                if self.space[location.y + 1][location.x + 1] == b'.' {
                    location = Point {
                        x: location.x + 1,
                        y: location.y + 1,
                    };
                    continue;
                }
                landed = true;
                self.grains_of_sand += 1;
                self.space[location.y][location.x] = b'o';
                if location.y == 0 {
                    return;
                }
            }
        }
    }

    fn _debug(&self) -> String {
        return self
            .space
            .iter()
            .map(|row| std::str::from_utf8(row).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "24");
    }

    #[test]
    fn part2_works() {
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "93");
    }
}
