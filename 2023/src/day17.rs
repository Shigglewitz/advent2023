use std::collections::HashMap;

use crate::create_advent_day;

create_advent_day!("17");

fn part1_with_input(input: &str) -> u32 {
    let city_map = CityMap::parse(input);
    let _shortest_path = city_map.shortest_path();

    return input.len() as u32;
}

fn part2_with_input(input: &str) -> i32 {
    return input.len() as i32;
}

struct CityMap {
    heat_loss: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl CityMap {
    fn parse(input: &str) -> CityMap {
        let heat_loss = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let height = heat_loss.len();
        let width = heat_loss[0].len();

        return CityMap {
            heat_loss,
            width,
            height,
        };
    }

    // based on psuedo code here https://brilliant.org/wiki/dijkstras-short-path-finder/
    fn shortest_path(&self) -> (u32, String) {
        let mut solutions: HashMap<(usize, usize), (u32, String)> = HashMap::new();
        let mut to_solve: Vec<(usize, usize)> = Vec::new();
        // let mut visited: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..self.width {
            for j in 0..self.height {
                solutions.insert((i, j), (u32::MAX, "".to_string()));
                to_solve.push((i, j));
            }
        }

        solutions.insert((0, 0), (0, "".to_string()));

        while !to_solve.is_empty() {
            let mut next_city_index = (0 as usize, 0 as usize);
            let mut next_index = 0;
            let mut next_shortest_distance = u32::MAX;
            let mut next_shortest_path = "".to_string();
            to_solve
                .iter()
                .enumerate()
                .for_each(|(index, &city_index)| {
                    // if visited.contains(&city_index) {
                    //     return;
                    // }
                    if let Some(distance) = solutions.get(&city_index) {
                        if distance.0 < next_shortest_distance {
                            next_city_index = city_index;
                            next_shortest_distance = distance.0;
                            next_index = index;
                            next_shortest_path = distance.1.clone();
                        }
                    }
                });
            to_solve.remove(next_index);
            for neighbor in self.get_neighbors(&next_city_index) {
                let heat_loss = self.heat_loss[neighbor.1][neighbor.0];
                let alternative = next_shortest_distance + heat_loss;
                let solution = solutions.get(&neighbor).unwrap();
                if alternative < solution.0 {
                    let next_path = format!("{}{}", next_shortest_path, heat_loss.to_string());
                    solutions.insert(neighbor, (alternative, next_path));
                }
            }
        }

        let answer = solutions.get(&(self.width - 1, self.height - 1)).unwrap();

        return (answer.0, answer.1.clone());
    }

    fn get_neighbors(&self, index: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if index.0 != 0 {
            neighbors.push((index.0 - 1, index.1));
        }
        if index.0 < self.width - 1 {
            neighbors.push((index.0 + 1, index.1));
        }
        if index.1 != 0 {
            neighbors.push((index.0, index.1 - 1));
        }
        if index.1 < self.height - 1 {
            neighbors.push((index.0, index.1 + 1));
        }
        return neighbors;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn test_city_map() -> CityMap {
        let input = utils::read_file("day17", "test.txt");
        return CityMap::parse(&input);
    }

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!(&actual, "193");
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(&actual, "193");
    }

    #[rstest]
    #[case(0, "2413432311323")]
    #[case(1, "3215453535623")]
    #[case(11, "2546548887735")]
    #[case(12, "4322674655533")]
    fn city_map_parse_works(#[case] index: usize, #[case] expected: &str) {
        let city_map = test_city_map();

        let line = city_map.heat_loss[index]
            .iter()
            .map(|num| num.to_string())
            .collect::<String>();
        assert_eq!(&line, expected);
    }

    #[test]
    fn city_map_shortest_path_works() {
        let city_map = test_city_map();

        let shortest_path = city_map.shortest_path();
        assert_eq!(shortest_path.0, 78);
        assert_eq!(&shortest_path.1, "413432311322342646373353")
    }
}
