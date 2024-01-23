use crate::create_advent_day;
use rayon::prelude::*;
use std::{collections::HashSet, ops::Sub};

create_advent_day!("2022", "19");

fn part1_with_input(input: &str) -> u32 {
    let mut blueprints = input.lines().map(Blueprint::parse).collect::<Vec<_>>();
    return blueprints
        .iter_mut()
        .map(|blueprint| {
            blueprint.find_max_geodes(24);
            blueprint.quality_level()
        })
        .sum();
}

fn part2_with_input(input: &str) -> u32 {
    let mut blueprints = input.lines().map(Blueprint::parse).collect::<Vec<_>>();
    return blueprints
        .iter_mut()
        .take(3)
        .map(|blueprint| {
            blueprint.find_max_geodes(32);
            blueprint.max_geodes
        })
        .product();
}

#[derive(Clone, Debug, Copy)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Resources {
    fn empty() -> Self {
        return Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
    }

    fn can_build(&self, other: &Self) -> bool {
        return self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode;
    }

    fn get(&self, robot_type: &RobotType) -> u32 {
        return match robot_type {
            RobotType::ORE => self.ore,
            RobotType::CLAY => self.clay,
            RobotType::OBSIDIAN => self.obsidian,
            RobotType::GEODE => self.geode,
        };
    }
}

impl Sub for Resources {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        return Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        };
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum RobotType {
    ORE,
    CLAY,
    OBSIDIAN,
    GEODE,
}

struct Blueprint {
    id: u32,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
    max_costs: Resources,
    max_geodes: u32,
}

impl Blueprint {
    fn parse(input: &str) -> Self {
        let (left, right) = input.split_once(":").unwrap();
        let (_, id) = left.split_once(" ").unwrap();
        let sentences = right.trim().split(".");
        let robots = sentences
            .filter(|split| !split.is_empty())
            .map(|sentence| {
                let words = sentence.trim().split(" ").collect::<Vec<_>>();
                let robot_type = words[1];
                let ore_cost = words[4].parse::<u32>().unwrap();
                let mut cost = Resources {
                    ore: ore_cost,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                };
                if words.len() > 6 {
                    let second_cost = words[7].parse::<u32>().unwrap();
                    let second_cost_type = words[8];
                    match second_cost_type {
                        "clay" => cost.clay = second_cost,
                        "obsidian" => cost.obsidian = second_cost,
                        _ => unreachable!("unexpected second cost type"),
                    };
                }
                (robot_type, cost)
            })
            .collect::<Vec<_>>();

        let mut blueprint = Self {
            id: id.parse().unwrap(),
            ore_robot_cost: Resources::empty(),
            clay_robot_cost: Resources::empty(),
            obsidian_robot_cost: Resources::empty(),
            geode_robot_cost: Resources::empty(),
            max_costs: Resources::empty(),
            max_geodes: 0,
        };

        robots.iter().for_each(|robot| match robot.0 {
            "ore" => blueprint.ore_robot_cost = robot.1.clone(),
            "clay" => blueprint.clay_robot_cost = robot.1.clone(),
            "obsidian" => blueprint.obsidian_robot_cost = robot.1.clone(),
            "geode" => blueprint.geode_robot_cost = robot.1.clone(),
            _ => unreachable!("unexpected robot type"),
        });

        let all_costs = vec![
            &blueprint.ore_robot_cost,
            &blueprint.clay_robot_cost,
            &blueprint.obsidian_robot_cost,
            &blueprint.geode_robot_cost,
        ];
        blueprint.max_costs.ore = all_costs.iter().map(|cost| cost.ore).max().unwrap();
        blueprint.max_costs.clay = all_costs.iter().map(|cost| cost.clay).max().unwrap();
        blueprint.max_costs.obsidian = all_costs.iter().map(|cost| cost.obsidian).max().unwrap();
        blueprint.max_costs.geode = u32::MAX;

        return blueprint;
    }

    fn possible_robots(&self, current_resources: &Resources) -> Vec<RobotType> {
        let mut possibilites = Vec::new();
        if current_resources.can_build(&self.ore_robot_cost) {
            possibilites.push(RobotType::ORE);
        }
        if current_resources.can_build(&self.clay_robot_cost) {
            possibilites.push(RobotType::CLAY);
        }
        if current_resources.can_build(&self.obsidian_robot_cost) {
            possibilites.push(RobotType::OBSIDIAN);
        }
        if current_resources.can_build(&self.geode_robot_cost) {
            possibilites.push(RobotType::GEODE);
        }
        return possibilites;
    }

    fn find_max_geodes(&mut self, minutes_remaining: u32) {
        if self.max_geodes == 0 {
            self.max_geodes = self.simulate_minute(GameState::new(), minutes_remaining);
        }
    }

    fn calculate_max_possible_geodes(
        current_geodes: u32,
        current_geode_robots: u32,
        minutes_remaining: u32,
    ) -> u32 {
        let last_number = minutes_remaining + current_geode_robots;
        let first_number = current_geode_robots;
        let quadratic = (last_number - first_number + 1) * (first_number + last_number) / 2;
        return quadratic + current_geodes;
    }

    fn simulate_minute(&self, input_state: GameState, minutes_remaining: u32) -> u32 {
        if minutes_remaining == 0 {
            return input_state.current_resources.geode;
        }
        if Self::calculate_max_possible_geodes(
            input_state.current_resources.geode,
            input_state.current_robots.geode,
            minutes_remaining,
        ) < input_state.highest_so_far
        {
            return input_state.highest_so_far;
        }
        let mut state = input_state.clone();
        let possible_robots = self.possible_robots(&state.current_resources);
        state.current_resources.ore += state.current_robots.ore;
        state.current_resources.clay += state.current_robots.clay;
        state.current_resources.obsidian += state.current_robots.obsidian;
        state.current_resources.geode += state.current_robots.geode;
        let mut possible_current_resources = possible_robots
            .iter()
            .rev()
            .filter(|robot| {
                let current_robots_of_type = state.current_robots.get(robot);
                let max_usage_of_type = self.max_costs.get(robot);
                current_robots_of_type < max_usage_of_type
            })
            .filter(|robot| !state.ignored_builds.contains(robot))
            .map(|possibility| match possibility {
                RobotType::ORE => {
                    let new_resources =
                        state.current_resources.clone() - self.ore_robot_cost.clone();
                    let mut new_robots = state.current_robots.clone();
                    new_robots.ore += 1;
                    GameState {
                        current_resources: new_resources,
                        current_robots: new_robots,
                        ignored_builds: HashSet::new(),
                        highest_so_far: state.highest_so_far,
                    }
                }
                RobotType::CLAY => {
                    let new_resources =
                        state.current_resources.clone() - self.clay_robot_cost.clone();
                    let mut new_robots = state.current_robots.clone();
                    new_robots.clay += 1;
                    GameState {
                        current_resources: new_resources,
                        current_robots: new_robots,
                        ignored_builds: HashSet::new(),
                        highest_so_far: state.highest_so_far,
                    }
                }
                RobotType::OBSIDIAN => {
                    let new_resources =
                        state.current_resources.clone() - self.obsidian_robot_cost.clone();
                    let mut new_robots = state.current_robots.clone();
                    new_robots.obsidian += 1;
                    GameState {
                        current_resources: new_resources,
                        current_robots: new_robots,
                        ignored_builds: HashSet::new(),
                        highest_so_far: state.highest_so_far,
                    }
                }
                RobotType::GEODE => {
                    let new_resources =
                        state.current_resources.clone() - self.geode_robot_cost.clone();
                    let mut new_robots = state.current_robots.clone();
                    new_robots.geode += 1;
                    GameState {
                        current_resources: new_resources,
                        current_robots: new_robots,
                        ignored_builds: HashSet::new(),
                        highest_so_far: state.highest_so_far,
                    }
                }
            })
            .collect::<Vec<_>>();
        let mut do_not_build = state.clone();
        possible_robots.iter().for_each(|robot| {
            do_not_build.ignored_builds.insert(robot.clone());
        });
        possible_current_resources.push(do_not_build);
        return possible_current_resources
            .par_iter_mut()
            .fold(
                || state.highest_so_far,
                |acc, ele| {
                    ele.highest_so_far = acc;
                    acc.max(self.simulate_minute(ele.clone(), minutes_remaining - 1))
                },
            )
            // .map(|game_state| self.simulate_minute(game_state.clone(), minutes_remaining - 1))
            .max()
            .unwrap();
    }

    fn quality_level(&self) -> u32 {
        return self.id * self.max_geodes;
    }
}

#[derive(Clone)]
struct GameState {
    current_resources: Resources,
    current_robots: Resources,
    ignored_builds: HashSet<RobotType>,
    highest_so_far: u32,
}

impl GameState {
    fn new() -> Self {
        return Self {
            current_resources: Resources::empty(),
            current_robots: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            ignored_builds: HashSet::new(),
            highest_so_far: 0,
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "33");
    }

    #[test]
    fn part2_works() {
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "3472");
    }

    #[test]
    fn blueprint_parse_works() {
        let actual = Blueprint::parse("Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 2 ore and 12 obsidian.");

        assert_eq!(actual.id, 1);
        assert_eq!(actual.ore_robot_cost.ore, 3);
        assert_eq!(actual.clay_robot_cost.ore, 3);
        assert_eq!(actual.obsidian_robot_cost.ore, 2);
        assert_eq!(actual.obsidian_robot_cost.clay, 19);
        assert_eq!(actual.geode_robot_cost.ore, 2);
        assert_eq!(actual.geode_robot_cost.obsidian, 12);
    }
}
