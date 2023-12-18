use crate::create_advent_day;

create_advent_day!("10");

fn part1_with_input(input: &str) -> i32 {
    let pipe_maze = PipeMaze::parse(&input);
    let path = pipe_maze.find_path();

    let length = path.len() as i32;
    return (length / 2) + 1;
}

fn part2_with_input(input: &str) -> i32 {
    let pipe_maze = PipeMaze::parse(&input);

    return pipe_maze.shoe_lace();
}

struct PipeMaze {
    layout: Vec<Vec<Tile>>,
}

struct Tile {
    pipe: char,
    point: Point,
}

struct Point {
    x: i32,
    y: i32,
}

impl PipeMaze {
    fn parse(input: &str) -> PipeMaze {
        let mut layout: Vec<Vec<Tile>> = Vec::new();
        for (i, line) in input.lines().enumerate() {
            let mut tiles: Vec<Tile> = Vec::new();
            for (j, letter) in line.chars().enumerate() {
                tiles.push(Tile {
                    pipe: letter,
                    point: Point {
                        x: j as i32,
                        y: i as i32,
                    },
                });
            }
            layout.push(tiles);
        }
        return PipeMaze { layout };
    }

    fn get_tile_at(&self, x: i32, y: i32) -> Option<&Tile> {
        let x_size = x as usize;
        let y_size = y as usize;
        if y_size >= self.layout.len() {
            return None;
        }
        let line = &self.layout[y_size];
        if x_size >= line.len() {
            return None;
        }
        return Some(&line[x_size]);
    }

    fn is_point_valid(&self, point: &Point) -> bool {
        let tile = match self.get_tile_at(point.x, point.y) {
            Some(value) => value,
            None => return false,
        };
        if tile.pipe == '.' {
            return false;
        }
        return true;
    }

    fn trace_to_animal(&self, animal_tile: &Tile, start: &Tile) -> Option<Vec<&Tile>> {
        let mut path: Vec<&Tile> = Vec::new();
        let mut finished = false;
        let mut curr_tile = start;
        let mut prev_tile = animal_tile;
        while !finished {
            path.push(
                self.get_tile_at(curr_tile.point.x, curr_tile.point.y)
                    .unwrap(),
            );
            let (point_1, point_2) = curr_tile.points_at();
            if !self.is_point_valid(&point_1) {
                return None;
            }
            if !(self.is_point_valid(&point_2)) {
                return None;
            }
            if point_1.x == prev_tile.point.x && point_1.y == prev_tile.point.y {
                prev_tile = curr_tile;
                curr_tile = self.get_tile_at(point_2.x, point_2.y).unwrap();
            } else if point_2.x == prev_tile.point.x && point_2.y == prev_tile.point.y {
                prev_tile = curr_tile;
                curr_tile = self.get_tile_at(point_1.x, point_1.y).unwrap();
            } else {
                return None;
            }
            finished = curr_tile.pipe == 'S';
        }

        return Some(path);
    }

    fn find_animal_tile(&self) -> &Tile {
        for line in &self.layout {
            for tile in line {
                if tile.pipe == 'S' {
                    return &tile;
                }
            }
        }

        panic!("no animal found in maze!");
    }

    fn find_path(&self) -> Vec<&Tile> {
        let animal_tile: &Tile = self.find_animal_tile();
        let mut start_points: Vec<Option<&Tile>> = Vec::new();
        start_points.push(self.get_tile_at(animal_tile.point.x, animal_tile.point.y + 1));
        start_points.push(self.get_tile_at(animal_tile.point.x, animal_tile.point.y - 1));
        start_points.push(self.get_tile_at(animal_tile.point.x + 1, animal_tile.point.y));
        start_points.push(self.get_tile_at(animal_tile.point.x - 1, animal_tile.point.y));

        let path = start_points
            .iter()
            .filter(|option| option.is_some())
            .map(|option| option.unwrap())
            .map(|tile| self.trace_to_animal(animal_tile, tile))
            .filter(|option| option.is_some())
            .map(|option| option.unwrap())
            .next()
            .unwrap();

        return path;
    }

    // https://en.wikipedia.org/wiki/Shoelace_formula
    fn shoe_lace(&self) -> i32 {
        let path = self.find_path();
        let first = path.first().unwrap();
        let last = path.last().unwrap();
        let animal_tile_is_vertex = first.point.x != last.point.x && first.point.y != last.point.y;
        let vertex_pipes = vec!['F', '7', 'J', 'L'];

        let mut verticies: Vec<&Tile> = path
            .iter()
            .filter(|tile| vertex_pipes.contains(&tile.pipe))
            .map(|tile| *tile)
            .collect();
        if animal_tile_is_vertex {
            verticies.push(self.find_animal_tile());
        }
        let raw_area = verticies
            .windows(2)
            .map(|arr| (arr[0].point.y + arr[1].point.y) * (arr[0].point.x - arr[1].point.x))
            .sum::<i32>()
            .abs();

        let divide_me_by_two = raw_area - (path.len() as i32) + 1;

        return divide_me_by_two / 2;
    }
}

impl Tile {
    fn points_at(&self) -> (Point, Point) {
        return match self.pipe {
            'F' => (
                Point {
                    x: self.point.x + 1,
                    y: self.point.y,
                },
                Point {
                    x: self.point.x,
                    y: self.point.y + 1,
                },
            ),
            '-' => (
                Point {
                    x: self.point.x + 1,
                    y: self.point.y,
                },
                Point {
                    x: self.point.x - 1,
                    y: self.point.y,
                },
            ),
            '7' => (
                Point {
                    x: self.point.x,
                    y: self.point.y + 1,
                },
                Point {
                    x: self.point.x - 1,
                    y: self.point.y,
                },
            ),
            '|' => (
                Point {
                    x: self.point.x,
                    y: self.point.y + 1,
                },
                Point {
                    x: self.point.x,
                    y: self.point.y - 1,
                },
            ),
            'J' => (
                Point {
                    x: self.point.x - 1,
                    y: self.point.y,
                },
                Point {
                    x: self.point.x,
                    y: self.point.y - 1,
                },
            ),
            'L' => (
                Point {
                    x: self.point.x,
                    y: self.point.y - 1,
                },
                Point {
                    x: self.point.x + 1,
                    y: self.point.y,
                },
            ),
            _ => (Point { x: -1, y: -1 }, Point { x: -1, y: -1 }),
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn test_maze() -> PipeMaze {
        return PipeMaze::parse(&utils::read_file("day10", "test1.txt"));
    }

    fn test_maze_2() -> PipeMaze {
        return PipeMaze::parse(&utils::read_file("day10", "test2.txt"));
    }

    #[test]
    fn part1_1_works() {
        let actual = create("test1.txt").solve_part1();

        assert_eq!(&actual, "4");
    }

    #[test]
    fn part1_2_works() {
        let actual = create("test2.txt").solve_part1();

        assert_eq!(&actual, "8");
    }

    #[test]
    fn part2_works() {
        let actual = create("test3.txt").solve_part2();

        assert_eq!(&actual, "4");
    }

    #[test]
    fn pipe_maze_pase_works() {
        let actual = test_maze();

        let line1: String = actual.layout[0].iter().map(|tile| tile.pipe).collect();
        assert_eq!(&line1, ".....");
        let line2: String = actual.layout[1].iter().map(|tile| tile.pipe).collect();
        assert_eq!(&line2, ".S-7.");
        assert_eq!(actual.layout.len(), 5);
        let tile = actual.get_tile_at(1, 3).unwrap();
        assert_eq!(tile.point.x, 1);
        assert_eq!(tile.point.y, 3);
        assert_eq!(tile.pipe, 'L');
    }

    #[rstest]
    #[case(2, 1, '-', 3, 1, '7', 1, 1, 'S')]
    #[case(3, 1, '7', 3, 2, '|', 2, 1, '-')]
    #[case(3, 2, '|', 3, 3, 'J', 3, 1, '7')]
    #[case(3, 3, 'J', 2, 3, '-', 3, 2, '|')]
    #[case(2, 3, '-', 3, 3, 'J', 1, 3, 'L')]
    #[case(1, 3, 'L', 1, 2, '|', 2, 3, '-')]
    #[case(1, 2, '|', 1, 3, 'L', 1, 1, 'S')]
    fn tile_points_at_tests(
        #[case] tile_x: i32,
        #[case] tile_y: i32,
        #[case] tile_char: char,
        #[case] tile_x_1: i32,
        #[case] tile_y_1: i32,
        #[case] tile_char_1: char,
        #[case] tile_x_2: i32,
        #[case] tile_y_2: i32,
        #[case] tile_char_2: char,
    ) {
        let pipe_maze = test_maze();
        let tile = pipe_maze.get_tile_at(tile_x, tile_y).unwrap();
        let (points_at_1, points_at_2) = tile.points_at();

        assert_eq!(tile.pipe, tile_char);
        assert_eq!(points_at_1.x, tile_x_1);
        assert_eq!(points_at_1.y, tile_y_1);
        assert_eq!(
            pipe_maze
                .get_tile_at(points_at_1.x, points_at_1.y)
                .unwrap()
                .pipe,
            tile_char_1
        );
        assert_eq!(points_at_2.x, tile_x_2);
        assert_eq!(points_at_2.y, tile_y_2);
        assert_eq!(
            pipe_maze
                .get_tile_at(points_at_2.x, points_at_2.y)
                .unwrap()
                .pipe,
            tile_char_2
        );
    }

    #[test]
    fn tile_points_at_f_test() {
        let pipe_maze = test_maze_2();
        let tile = pipe_maze.get_tile_at(2, 0).unwrap();
        let (points_at_1, points_at_2) = tile.points_at();

        assert_eq!(tile.pipe, 'F');
        assert_eq!(
            pipe_maze
                .get_tile_at(points_at_1.x, points_at_1.y)
                .unwrap()
                .pipe,
            '7'
        );
        assert_eq!(
            pipe_maze
                .get_tile_at(points_at_2.x, points_at_2.y)
                .unwrap()
                .pipe,
            'J'
        );
    }

    #[rstest]
    #[case(2, 1)]
    #[case(1, 2)]
    fn pipe_maze_trace_to_animal_tests_1(#[case] start_x: i32, #[case] start_y: i32) {
        let pipe_maze = test_maze();
        let animal_tile = pipe_maze.get_tile_at(1, 1).unwrap();
        let start_tile = pipe_maze.get_tile_at(start_x, start_y).unwrap();
        let path_option: Option<Vec<&Tile>> = pipe_maze.trace_to_animal(animal_tile, start_tile);
        let distance = path_option.unwrap().len();

        assert_eq!(distance, 7);
    }

    #[rstest]
    #[case(1, 2)]
    #[case(0, 3)]
    fn pipe_maze_trace_to_animal_tests_2(#[case] start_x: i32, #[case] start_y: i32) {
        let pipe_maze = test_maze_2();
        let animal_tile = pipe_maze.get_tile_at(0, 2).unwrap();
        let start_tile = pipe_maze.get_tile_at(start_x, start_y).unwrap();
        let path_option = pipe_maze.trace_to_animal(animal_tile, start_tile);
        let distance = path_option.unwrap().len();

        assert_eq!(distance, 15);
    }

    #[test]
    fn pipe_maze_find_animal_works_1() {
        let pipe_maze = test_maze();
        let actual = pipe_maze.find_animal_tile();

        assert_eq!(actual.point.x, 1);
        assert_eq!(actual.point.y, 1);
    }

    #[test]
    fn pipe_maze_find_animal_works_2() {
        let pipe_maze = test_maze_2();
        let actual = pipe_maze.find_animal_tile();

        assert_eq!(actual.point.x, 0);
        assert_eq!(actual.point.y, 2);
    }

    #[rstest]
    #[case("test1.txt", 1)]
    #[case("test2.txt", 1)]
    #[case("test3.txt", 4)]
    #[case("test4.txt", 4)]
    #[case("test5.txt", 8)]
    #[case("test6.txt", 10)]
    fn pipe_maze_shoe_lace_tests(#[case] file_name: &str, #[case] expected: i32) {
        let pipe_maze = PipeMaze::parse(&utils::read_file("day10", file_name));
        let actual = pipe_maze.shoe_lace();

        assert_eq!(actual, expected);
    }
}
