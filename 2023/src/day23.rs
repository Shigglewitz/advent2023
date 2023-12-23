use std::collections::HashSet;

use crate::create_advent_day;

create_advent_day!("23");

fn part1_with_input(input: &str) -> u32 {
    let forest = Forest::parse(input);
    return forest.traverse();
}

fn part2_with_input(input: &str) -> u32 {
    let forest = Forest::parse(input);
    return forest.traverse();
}

struct Forest {
    map: Vec<Vec<u8>>,
}

impl Forest {
    fn parse(input: &str) -> Forest {
        let map = input
            .lines()
            .map(|line| line.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        return Forest { map };
    }

    fn north_neighbor(&self, location: &Point) -> Option<Point> {
        let neighbor = Point {
            x: location.x,
            y: location.y - 1,
        };
        return match self.map[neighbor.y as usize][neighbor.x as usize] {
            b'.' | b'^' => Some(neighbor),
            _ => None,
        };
    }
    fn south_neighbor(&self, location: &Point) -> Option<Point> {
        let neighbor = Point {
            x: location.x,
            y: location.y + 1,
        };
        if neighbor.y >= self.map.len() as u32 {
            return None;
        }
        return match self.map[neighbor.y as usize][neighbor.x as usize] {
            b'.' | b'v' => Some(neighbor),
            _ => None,
        };
    }
    fn east_neighbor(&self, location: &Point) -> Option<Point> {
        let neighbor = Point {
            x: location.x + 1,
            y: location.y,
        };
        return match self.map[neighbor.y as usize][neighbor.x as usize] {
            b'.' | b'>' => Some(neighbor),
            _ => None,
        };
    }
    fn west_neighbor(&self, location: &Point) -> Option<Point> {
        let neighbor = Point {
            x: location.x - 1,
            y: location.y,
        };
        return match self.map[neighbor.y as usize][neighbor.x as usize] {
            b'.' | b'<' => Some(neighbor),
            _ => None,
        };
    }

    fn traverse(&self) -> u32 {
        let mut finished_hikes: Vec<Hike> = Vec::new();
        let mut in_progress_hikes: Vec<Hike> = Vec::new();
        let initial_hike = Self::initial_hike();
        in_progress_hikes.push(initial_hike);
        while !in_progress_hikes.is_empty() {
            let mut num_hikes = in_progress_hikes.len();
            let mut hike_index = 0;
            let mut new_hikes: Vec<Hike> = Vec::new();
            while hike_index < num_hikes {
                let hike = in_progress_hikes.get_mut(hike_index).unwrap();
                hike.visit_self();
                let mut neighbors: Vec<Point> = Vec::new();
                let north_neighbor_opt = self.north_neighbor(&hike.location);
                if let Some(north_neighbor) = north_neighbor_opt {
                    neighbors.push(north_neighbor);
                }
                let south_neighbor_opt = self.south_neighbor(&hike.location);
                if let Some(south_neighbor) = south_neighbor_opt {
                    neighbors.push(south_neighbor);
                }
                let east_neighbor_opt = self.east_neighbor(&hike.location);
                if let Some(east_neighbor) = east_neighbor_opt {
                    neighbors.push(east_neighbor);
                }
                let west_neighbor_opt = self.west_neighbor(&hike.location);
                if let Some(west_neighbor) = west_neighbor_opt {
                    neighbors.push(west_neighbor);
                }
                let neighbors = neighbors
                    .iter()
                    .filter(|&neighbor| !hike.visited.contains(neighbor))
                    .collect::<Vec<_>>();
                let mut neighbors_iter = neighbors.iter();
                let first_neighbor_opt = neighbors_iter.next();
                if first_neighbor_opt.is_none() {
                    finished_hikes.push(hike.clone());
                    in_progress_hikes.remove(hike_index);
                    num_hikes -= 1;
                    continue;
                }
                let first_neighbor = first_neighbor_opt.unwrap();
                hike.move_to(first_neighbor);
                for neighbor in neighbors_iter {
                    let mut new_hike = hike.clone();
                    new_hike.move_to(neighbor);
                    new_hikes.push(new_hike);
                }
                hike_index += 1;
            }
            for hike in new_hikes {
                in_progress_hikes.push(hike);
            }
        }
        return finished_hikes
            .iter()
            .map(|hike| hike.visited.len())
            .max()
            .unwrap() as u32
            - 1;
    }

    fn initial_hike() -> Hike {
        let location = Point { x: 1, y: 1 };
        let mut visited = HashSet::new();
        visited.insert(Point { x: 1, y: 0 });

        return Hike { location, visited };
    }
}

#[derive(Clone)]
struct Hike {
    location: Point,
    visited: HashSet<Point>,
}

impl Hike {
    fn visit_self(&mut self) {
        self.visited.insert(self.location.clone());
    }

    fn move_to(&mut self, location: &Point) {
        self.location = location.clone();
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!(actual, "94");
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(actual, "154");
    }

    #[test]
    fn forst_parse_works() {
        let input = utils::read_file("day23", "test.txt");
        let forest = Forest::parse(&input);

        assert_eq!(
            String::from_utf8(forest.map[0].clone()).unwrap().as_str(),
            "#.#####################"
        );
        assert_eq!(
            String::from_utf8(forest.map[forest.map.len() - 1].clone())
                .unwrap()
                .as_str(),
            "#####################.#"
        );
    }
}
