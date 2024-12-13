use std::{collections::HashMap, usize};

use crate::create_advent_day;

create_advent_day!("2024", "12");

fn part1_with_input(input: &str) -> i64 {
    let plots: Vec<Vec<Plot>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|letter| Plot {
                    crop: letter,
                    region: usize::MAX,
                })
                .collect()
        })
        .collect();

    let height = plots.len();
    let width = plots[0].len();

    let mut garden = Garden {
        plots,
        height,
        width,
    };

    let mut regions: HashMap<usize, Region> = HashMap::new();
    let mut next_region: usize = 0;
    for x in 0..width {
        for y in 0..height {
            if garden.plots[y][x].region != usize::MAX {
                continue;
            }
            regions.insert(
                next_region,
                Region {
                    area: 0,
                    crop: garden.plots[y][x].crop,
                    perimeter: 0,
                },
            );
            define_region(&mut garden, x, y, next_region, &mut regions);
            next_region += 1;
        }
    }

    for x in 0..width {
        for y in 0..height {
            if x == 0 || garden.plots[y][x - 1].crop != garden.plots[y][x].crop {
                regions
                    .get_mut(&garden.plots[y][x].region)
                    .unwrap()
                    .perimeter += 1;
            }
            if x == garden.width - 1 || garden.plots[y][x + 1].crop != garden.plots[y][x].crop {
                regions
                    .get_mut(&garden.plots[y][x].region)
                    .unwrap()
                    .perimeter += 1;
            }
            if y == 0 || garden.plots[y - 1][x].crop != garden.plots[y][x].crop {
                regions
                    .get_mut(&garden.plots[y][x].region)
                    .unwrap()
                    .perimeter += 1;
            }
            if y == garden.height - 1 || garden.plots[y + 1][x].crop != garden.plots[y][x].crop {
                regions
                    .get_mut(&garden.plots[y][x].region)
                    .unwrap()
                    .perimeter += 1;
            }
        }
    }

    return regions
        .values()
        .map(|region| region.perimeter * region.area)
        .sum::<usize>() as i64;
}

fn define_region(
    garden: &mut Garden,
    x: usize,
    y: usize,
    region: usize,
    regions: &mut HashMap<usize, Region>,
) {
    if garden.plots[y][x].region != usize::MAX {
        return;
    }
    garden.plots[y][x].region = region;
    regions.get_mut(&region).unwrap().area += 1;
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < garden.width - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < garden.height - 1 {
        neighbors.push((x, y + 1));
    }
    let real_neighbors: Vec<(usize, usize)> = neighbors
        .into_iter()
        .filter(|(neighbor_x, neighbor_y)| {
            garden.plots[y][x].crop == garden.plots[*neighbor_y][*neighbor_x].crop
        })
        .collect();
    real_neighbors
        .iter()
        .for_each(|(x, y)| define_region(garden, *x, *y, region, regions));
}

struct Garden {
    plots: Vec<Vec<Plot>>,
    height: usize,
    width: usize,
}

struct Plot {
    crop: char,
    region: usize,
}

struct Region {
    crop: char,
    perimeter: usize,
    area: usize,
}

fn part2_with_input(_input: &str) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("1930", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("1206", &actual);
    }
}
