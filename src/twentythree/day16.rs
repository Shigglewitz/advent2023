use rayon::prelude::*;
use std::collections::HashSet;

use crate::create_advent_day;

create_advent_day!("2023", "16");

fn part1_with_input(input: &str) -> i32 {
    let mut contraption = Contraption::parse(input);
    contraption.add_default_start();
    contraption.trace_light();
    return contraption.count_energized_tiles();
}

fn part2_with_input(input: &str) -> i32 {
    let contraption = Contraption::parse(input);
    let mut starts: Vec<(usize, usize, Direction)> = Vec::new();
    for index in 0..contraption.width {
        starts.push((index, 0, Direction::DOWN));
        starts.push((index, contraption.height - 1, Direction::UP));
    }
    for index in 0..contraption.height {
        starts.push((contraption.width - 1, index, Direction::LEFT));
        starts.push((0, index, Direction::RIGHT));
    }

    return starts
        .par_iter()
        .map(|start| {
            let mut this_contraption = contraption.clone();
            this_contraption.add_start(start.0, start.1, start.2.clone());
            this_contraption.trace_light();
            this_contraption.count_energized_tiles()
        })
        .max()
        .unwrap();
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN,
}

#[derive(Clone)]
struct RayOfLight {
    direction: Direction,
    x: usize,
    y: usize,
    completed: bool,
}

#[derive(Clone)]
struct Contraption {
    tiles: Vec<Vec<u8>>,
    energized_tiles: Vec<Vec<bool>>,
    rays: Vec<RayOfLight>,
    width: usize,
    height: usize,
}

impl Contraption {
    fn parse(input: &str) -> Contraption {
        let mut tiles: Vec<Vec<u8>> = Vec::new();
        for line in input.lines() {
            tiles.push(line.bytes().collect::<Vec<u8>>());
        }
        let num_cols = tiles[0].len();
        let num_rows = tiles.len();
        return Contraption {
            tiles,
            energized_tiles: vec![vec![false; num_cols]; num_rows],
            rays: Vec::new(),
            width: num_cols,
            height: num_rows,
        };
    }

    fn add_default_start(&mut self) {
        self.add_start(0, 0, Direction::RIGHT);
    }

    fn add_start(&mut self, x: usize, y: usize, direction: Direction) {
        self.rays.push(RayOfLight {
            direction,
            x,
            y,
            completed: false,
        });
    }

    fn trace_light(&mut self) {
        let mut processed_light: HashSet<(usize, usize, Direction)> = HashSet::new();
        while !self.rays.is_empty() {
            let num_rays = self.rays.len();
            for index in 0..num_rays {
                let ray_x = self.rays[index].x;
                let ray_y = self.rays[index].y;
                if self.tiles[ray_y][ray_x] != b'.' {
                    let key = (ray_x, ray_y, self.rays[index].direction.clone());
                    if processed_light.get(&key).is_some() {
                        self.rays[index].completed = true;
                        continue;
                    } else {
                        processed_light.insert(key);
                    }
                }
                self.energized_tiles[ray_y][ray_x] = true;

                match self.tiles[ray_y][ray_x] {
                    b'.' => (),
                    b'/' => match self.rays[index].direction {
                        Direction::DOWN => self.rays[index].direction = Direction::LEFT,
                        Direction::LEFT => self.rays[index].direction = Direction::DOWN,
                        Direction::RIGHT => self.rays[index].direction = Direction::UP,
                        Direction::UP => self.rays[index].direction = Direction::RIGHT,
                    },
                    b'\\' => match self.rays[index].direction {
                        Direction::DOWN => self.rays[index].direction = Direction::RIGHT,
                        Direction::LEFT => self.rays[index].direction = Direction::UP,
                        Direction::RIGHT => self.rays[index].direction = Direction::DOWN,
                        Direction::UP => self.rays[index].direction = Direction::LEFT,
                    },
                    b'|' => match self.rays[index].direction {
                        Direction::DOWN => (),
                        Direction::UP => (),
                        Direction::LEFT | Direction::RIGHT => {
                            self.rays[index].direction = Direction::DOWN;
                            if ray_y > 0 {
                                self.rays.push(RayOfLight {
                                    direction: Direction::UP,
                                    x: self.rays[index].x,
                                    y: self.rays[index].y - 1,
                                    completed: false,
                                })
                            }
                        }
                    },
                    b'-' => match self.rays[index].direction {
                        Direction::DOWN | Direction::UP => {
                            self.rays[index].direction = Direction::RIGHT;
                            if ray_x > 0 {
                                self.rays.push(RayOfLight {
                                    direction: Direction::LEFT,
                                    x: self.rays[index].x - 1,
                                    y: self.rays[index].y,
                                    completed: false,
                                })
                            }
                        }
                        Direction::LEFT => (),
                        Direction::RIGHT => (),
                    },
                    _ => unreachable!("unexpected condition while tracing light!"),
                }

                let next_coords_opt: Option<(usize, usize)> = match self.rays[index].direction {
                    Direction::LEFT => {
                        if ray_x == 0 {
                            None
                        } else {
                            Some((ray_x - 1, ray_y))
                        }
                    }
                    Direction::RIGHT => {
                        if ray_x >= self.width - 1 {
                            None
                        } else {
                            Some((ray_x + 1, ray_y))
                        }
                    }
                    Direction::UP => {
                        if ray_y == 0 {
                            None
                        } else {
                            Some((ray_x, ray_y - 1))
                        }
                    }
                    Direction::DOWN => {
                        if ray_y >= self.height - 1 {
                            None
                        } else {
                            Some((ray_x, ray_y + 1))
                        }
                    }
                };
                if next_coords_opt.is_none() {
                    self.rays[index].completed = true;
                    continue;
                }
                let (next_x, next_y) = next_coords_opt.unwrap();
                self.rays[index].x = next_x;
                self.rays[index].y = next_y;
            }
            let new_num_rays = self.rays.len();
            for index in (0..new_num_rays).rev() {
                if self.rays[index].completed {
                    self.rays.remove(index);
                }
            }
        }
    }

    fn count_energized_tiles(&self) -> i32 {
        let mut sum = 0;

        self.energized_tiles.iter().for_each(|row| {
            row.iter().for_each(|&tile| {
                if tile {
                    sum += 1;
                }
            })
        });

        return sum;
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!(&actual, "46");
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(&actual, "51");
    }

    #[test]
    fn contraption_parse_works() {
        let input = utils::read_file("2023/day16", "test.txt");
        let contraption = Contraption::parse(&input);

        let picture: String = contraption
            .tiles
            .iter()
            .map(|line| String::from_utf8(line.clone()).unwrap())
            .collect::<Vec<String>>()
            .join(
                "
",
            );

        assert_eq!(
            picture,
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
        );
    }
}
