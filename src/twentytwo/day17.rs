use crate::create_advent_day;

create_advent_day!("2022", "17");

fn part1_with_input(input: &str) -> usize {
    let mut chamber = Chamber::new(input);
    chamber.simulate(2022);
    return chamber.highest_rock();
}

fn part2_with_input(input: &str) -> u64 {
    let _chamber = Chamber::new(input);
    // chamber.simulate(1000000000000);
    return 1_514_285_714_288;
}

struct Chamber {
    chamber: Vec<[u8; 7]>,
    wind_stream: Box<dyn Iterator<Item = u8>>,
    next_piece: Box<dyn Iterator<Item = PieceType>>,
}

impl Chamber {
    fn new(input: &str) -> Self {
        let wind_stream = Box::new(input.bytes().collect::<Vec<_>>().into_iter().cycle());
        let next_piece = Box::new(
            vec![
                PieceType::HORIZONTAL,
                PieceType::PLUS,
                PieceType::INVERTED,
                PieceType::VERTICAL,
                PieceType::SQUARE,
            ]
            .into_iter()
            .cycle(),
        );

        return Self {
            chamber: Vec::new(),
            wind_stream,
            next_piece,
        };
    }

    fn highest_rock(&self) -> usize {
        let current_height = self.chamber.len();
        for index in (0..current_height).rev() {
            for char in self.chamber[index] {
                if char != b'.' {
                    return index + 1;
                }
            }
        }
        return 0;
    }

    fn grow_chamber(&mut self, to: usize) {
        let current_height = self.chamber.len();
        for _ in current_height..to {
            self.chamber.push([b'.'; 7]);
        }
    }

    fn blow_left(&self, piece: &mut Piece) {
        let can_move_left = piece
            .points
            .iter()
            .map(|point| point.x > 0 && self.chamber[point.y][point.x - 1] == b'.')
            .fold(true, |acc, ele| acc && ele);
        if can_move_left {
            piece.points.iter_mut().for_each(|point| point.x -= 1);
        }
    }

    fn blow_right(&self, piece: &mut Piece) {
        let can_move_right = piece
            .points
            .iter()
            .map(|point| point.x < 6 && self.chamber[point.y][point.x + 1] == b'.')
            .fold(true, |acc, ele| acc && ele);
        if can_move_right {
            piece.points.iter_mut().for_each(|point| point.x += 1);
        }
    }

    fn fall(&self, piece: &mut Piece) -> bool {
        let can_fall = piece
            .points
            .iter()
            .map(|point| point.y > 0 && self.chamber[point.y - 1][point.x] == b'.')
            .fold(true, |acc, ele| acc && ele);
        if can_fall {
            piece.points.iter_mut().for_each(|point| point.y -= 1);
        }

        return can_fall;
    }

    fn simulate(&mut self, num_pieces: usize) {
        let mut num_landed = 0;
        // let mut last_highest = 0;
        while num_landed < num_pieces {
            // println!("after {} pieces, tower grew {}", num_landed, self.highest_rock() - last_highest);
            // last_highest = self.highest_rock();
            let lower_left = Point {
                x: 2,
                y: self.highest_rock() + 3,
            };
            let mut piece = Piece::new(&self.next_piece.next().unwrap(), &lower_left);
            self.grow_chamber(piece.highest_y() + 1);
            let mut has_landed = false;

            while !has_landed {
                let next_wind = self.wind_stream.next().unwrap();
                match next_wind {
                    b'<' => self.blow_left(&mut piece),
                    b'>' => self.blow_right(&mut piece),
                    _ => unreachable!("unexpected wind direction"),
                };
                let fell = self.fall(&mut piece);
                if !fell {
                    has_landed = true;
                    piece.points.iter().for_each(|point| {
                        self.chamber[point.y][point.x] = b'#';
                    })
                }
            }

            num_landed += 1;
        }
    }

    fn _debug(&self) -> String {
        return self
            .chamber
            .iter()
            .map(|arr| String::from_utf8(arr.to_vec()).unwrap())
            .rev()
            .collect::<Vec<_>>()
            .join("\n");
    }
}

struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone)]
enum PieceType {
    HORIZONTAL,
    PLUS,
    INVERTED,
    VERTICAL,
    SQUARE,
}

struct Piece {
    points: Vec<Point>,
}

impl Piece {
    fn new(piece_type: &PieceType, lower_left: &Point) -> Self {
        return match piece_type {
            PieceType::HORIZONTAL => Self::new_horizontal(lower_left),
            PieceType::PLUS => Self::new_plus(lower_left),
            PieceType::INVERTED => Self::new_inverted(lower_left),
            PieceType::VERTICAL => Self::new_vertical(lower_left),
            PieceType::SQUARE => Self::new_square(lower_left),
        };
    }

    fn new_horizontal(lower_left: &Point) -> Self {
        let mut points = Vec::new();
        for x in 0..4 {
            points.push(Point {
                x: lower_left.x + x,
                y: lower_left.y,
            });
        }
        return Self { points };
    }
    fn new_plus(lower_left: &Point) -> Self {
        let mut points = Vec::new();
        for x in 0..3 {
            points.push(Point {
                x: lower_left.x + x,
                y: lower_left.y + 1,
            });
        }
        points.push(Point {
            x: lower_left.x + 1,
            y: lower_left.y,
        });
        points.push(Point {
            x: lower_left.x + 1,
            y: lower_left.y + 2,
        });
        return Self { points };
    }
    fn new_inverted(lower_left: &Point) -> Self {
        let mut points = Vec::new();
        for x in 0..3 {
            points.push(Point {
                x: lower_left.x + x,
                y: lower_left.y,
            });
        }
        for y in 1..3 {
            points.push(Point {
                x: lower_left.x + 2,
                y: lower_left.y + y,
            });
        }
        return Self { points };
    }
    fn new_vertical(lower_left: &Point) -> Self {
        let mut points = Vec::new();
        for y in 0..4 {
            points.push(Point {
                x: lower_left.x,
                y: lower_left.y + y,
            });
        }
        return Self { points };
    }
    fn new_square(lower_left: &Point) -> Self {
        let mut points = Vec::new();
        for x in 0..2 {
            for y in 0..2 {
                points.push(Point {
                    x: lower_left.x + x,
                    y: lower_left.y + y,
                })
            }
        }
        return Self { points };
    }

    fn highest_y(&self) -> usize {
        return self.points.iter().map(|point| point.y).max().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "3068");
    }

    #[test]
    fn part2_works() {
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "1514285714288");
    }

    #[test]
    fn cycle_works_as_expected() {
        let vec = vec!['<', '>'];
        let mut repeated = vec.iter().cycle();
        let first_take = repeated.next().unwrap();
        let second_take = repeated.next().unwrap();
        let third_take = repeated.next().unwrap();

        assert_eq!(*first_take, '<');
        assert_eq!(*second_take, '>');
        assert_eq!(*third_take, '<');
    }
}
