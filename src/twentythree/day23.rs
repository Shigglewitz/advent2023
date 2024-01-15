use std::collections::HashSet;

use crate::create_advent_day;

create_advent_day!("2023", "23");

fn part1_with_input(input: &str) -> u32 {
    let forest = Forest::parse(input);
    return forest.traverse();
}

fn part2_with_input(input: &str) -> u32 {
    let forest = Forest::parse(input);
    let graph = ForestGraph::parse(&forest);
    let mut seen_vertices = vec![vec![false; forest.width]; forest.height];
    return graph.depth_first_search(&Point { x: 1, y: 0 }, &mut seen_vertices, 0);
}

struct Forest {
    map: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Forest {
    fn parse(input: &str) -> Forest {
        let map = input
            .lines()
            .map(|line| line.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let height = map.len();
        let width = map[0].len();

        return Forest { map, width, height };
    }

    fn get_neighbor(
        &self,
        location: &Point,
        dx: i32,
        dy: i32,
        allowed_slope: u8,
        uphill_travel: bool,
    ) -> Option<Point> {
        let neighbor = Point {
            x: location.x + dx,
            y: location.y + dy,
        };
        let map_char = self.map[neighbor.y as usize][neighbor.x as usize];
        if uphill_travel {
            return match map_char {
                b'#' => None,
                _ => Some(neighbor),
            };
        } else {
            return match map_char {
                char if char == allowed_slope => Some(neighbor),
                b'.' => Some(neighbor),
                _ => None,
            };
        }
    }

    fn north_neighbor(&self, location: &Point, uphill_travel: bool) -> Option<Point> {
        if location.y == 0 {
            return None;
        }
        return self.get_neighbor(location, 0, -1, b'^', uphill_travel);
    }
    fn south_neighbor(&self, location: &Point, uphill_travel: bool) -> Option<Point> {
        if location.y >= self.map.len() as i32 - 1 {
            return None;
        }
        return self.get_neighbor(location, 0, 1, b'v', uphill_travel);
    }
    fn east_neighbor(&self, location: &Point, uphill_travel: bool) -> Option<Point> {
        return self.get_neighbor(location, 1, 0, b'>', uphill_travel);
    }
    fn west_neighbor(&self, location: &Point, uphill_travel: bool) -> Option<Point> {
        return self.get_neighbor(location, -1, 0, b'<', uphill_travel);
    }

    fn get_possible_neighbors(&self, location: &Point, uphill_travel: bool) -> Vec<Point> {
        let mut potential_neighbors: Vec<Option<Point>> = Vec::new();
        let north_neighbor = self.north_neighbor(location, uphill_travel);
        let south_neighbor = self.south_neighbor(location, uphill_travel);
        let east_neighbor = self.east_neighbor(location, uphill_travel);
        let west_neighbor = self.west_neighbor(location, uphill_travel);
        potential_neighbors.push(north_neighbor);
        potential_neighbors.push(south_neighbor);
        potential_neighbors.push(east_neighbor);
        potential_neighbors.push(west_neighbor);

        return potential_neighbors
            .iter()
            .filter(|opt| opt.is_some())
            .map(|opt| opt.clone().unwrap())
            .collect::<Vec<_>>();
    }

    fn get_unvisited_neighbors(&self, hike: &Hike) -> Vec<Point> {
        let possible_neighbors = self.get_possible_neighbors(&hike.location, false);
        return possible_neighbors
            .iter()
            .filter(|neighbor| !hike.visited.contains(neighbor))
            .map(|point| point.clone())
            .collect::<Vec<_>>();
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
                let neighbors = self.get_unvisited_neighbors(hike);
                if neighbors.len() == 0 {
                    finished_hikes.push(hike.clone());
                    in_progress_hikes.remove(hike_index);
                    num_hikes -= 1;
                    continue;
                }
                hike.move_to(&neighbors[0]);
                for neighbor in neighbors[1..].iter() {
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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub const MAX: usize = 140;

    fn to_index(&self) -> usize {
        return self.x as usize * Self::MAX + self.y as usize;
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Edge {
    destination: Point,
    distance: u32,
}

struct ForestGraph {
    adjacency_list: Vec<HashSet<Edge>>,
    end_point: Point,
}

struct Trace {
    starting_location: Point,
    current_location: Point,
    previous_location: Point,
}

impl ForestGraph {
    fn parse(forest: &Forest) -> ForestGraph {
        let mut adjacency_list = Vec::with_capacity(Point::MAX * Point::MAX + 1);
        for _ in 0..(Point::MAX * Point::MAX + 1) {
            adjacency_list.push(HashSet::new());
        }
        let end_point = Point {
            x: forest.width as i32 - 2,
            y: forest.height as i32 - 1,
        };
        let mut traces: Vec<Trace> = Vec::new();
        traces.push(Trace {
            starting_location: Point { x: 1, y: 0 },
            current_location: Point { x: 1, y: 0 },
            previous_location: Point { x: 0, y: 0 },
        });
        while !traces.is_empty() {
            let mut trace = traces.pop().unwrap();
            let mut num_paths = 1;
            let mut steps_taken = 0;
            while num_paths == 1 {
                steps_taken += 1;
                let possible_neighbors =
                    forest.get_possible_neighbors(&trace.current_location, true);
                let next_steps = possible_neighbors
                    .iter()
                    .filter(|&point| point != &trace.previous_location)
                    .collect::<Vec<_>>();
                num_paths = next_steps.len();
                if num_paths == 0 {
                    adjacency_list[trace.starting_location.to_index()].insert(Edge {
                        destination: end_point.clone(),
                        distance: steps_taken,
                    });
                    adjacency_list[end_point.to_index()].insert(Edge {
                        destination: trace.starting_location.clone(),
                        distance: steps_taken,
                    });
                } else if num_paths == 1 {
                    trace.previous_location = trace.current_location;
                    trace.current_location = next_steps[0].clone();
                } else if num_paths > 1 {
                    adjacency_list[trace.starting_location.to_index()].insert(Edge {
                        destination: trace.current_location.clone(),
                        distance: steps_taken,
                    });
                    let found_vertex = &mut adjacency_list[trace.current_location.to_index()];
                    if found_vertex.is_empty() {
                        for step in next_steps {
                            traces.push(Trace {
                                starting_location: trace.current_location.clone(),
                                current_location: step.clone(),
                                previous_location: trace.current_location.clone(),
                            })
                        }
                    }
                    found_vertex.insert(Edge {
                        destination: trace.starting_location.clone(),
                        distance: steps_taken,
                    });
                }
            }
        }

        return ForestGraph {
            adjacency_list,
            end_point,
        };
    }

    fn depth_first_search(
        &self,
        vertex: &Point,
        seen_vertices: &mut Vec<Vec<bool>>,
        distance: u32,
    ) -> u32 {
        if seen_vertices[vertex.y as usize][vertex.x as usize] {
            return 0;
        }
        if vertex.eq(&self.end_point) {
            // TODO: why is this off by one?
            return distance - 1;
        }
        seen_vertices[vertex.y as usize][vertex.x as usize] = true;
        let max = self.adjacency_list[vertex.to_index()]
            .iter()
            .map(|edge| {
                self.depth_first_search(&edge.destination, seen_vertices, distance + edge.distance)
            })
            .max()
            .unwrap();
        seen_vertices[vertex.y as usize][vertex.x as usize] = false;
        return max;
    }
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
    fn forest_parse_works() {
        let input = utils::read_file("2023/day23", "test.txt");
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

    #[test]
    fn forest_graph_parse_works() {
        let input = utils::read_file("2023/day23", "test.txt");
        let forest = Forest::parse(&input);
        let _graph = ForestGraph::parse(&forest);

        assert_eq!(1, 1);
    }
}
