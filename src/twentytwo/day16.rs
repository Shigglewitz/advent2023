use std::collections::{HashMap, HashSet};

use crate::create_advent_day;

create_advent_day!("2022", "16");

fn part1_with_input(input: &str) -> u32 {
    let volcano = Volcano::parse(input);
    return volcano.navigate(&"AA".to_owned(), 0, 30, HashSet::new());
}

fn part2_with_input(input: &str) -> u32 {
    return 0;
}

struct Volcano {
    valves: HashMap<String, Valve>,
}

impl Volcano {
    fn parse(input: &str) -> Self {
        let mut valves = HashMap::new();
        input.lines().map(Valve::parse).for_each(|valve| {
            valves.insert(valve.label.clone(), valve);
        });
        let mut volcano = Self { valves };
        volcano.find_valve_connections();
        return volcano;
    }

    fn find_valve_connections(&mut self) {
        let all_labels = self
            .valves
            .values()
            .map(|valve| valve.label.clone())
            .collect::<Vec<_>>();
        for valve_label in all_labels {
            let mut seen_valves = HashSet::new();
            let mut distance = 1;
            let mut analyze_me = Vec::new();
            let mut shortest_paths = HashMap::new();
            seen_valves.insert(&valve_label);
            analyze_me.push(&valve_label);
            while !analyze_me.is_empty() {
                let limit = analyze_me.len();
                let mut index = 0;
                while index < limit {
                    let connection_limit = self
                        .valves
                        .get(analyze_me[index])
                        .unwrap()
                        .connections
                        .len();
                    let mut connection_index = 0;
                    while connection_index < connection_limit {
                        let current = self.valves.get(analyze_me[index]).unwrap();
                        if !seen_valves.contains(&current.connections[connection_index]) {
                            analyze_me.push(&current.connections[connection_index]);
                            seen_valves.insert(&current.connections[connection_index]);
                            if self
                                .valves
                                .get(&current.connections[connection_index])
                                .unwrap()
                                .flow_rate
                                > 0
                            {
                                shortest_paths.insert(
                                    current.connections[connection_index].clone(),
                                    distance,
                                );
                            }
                        }
                        connection_index += 1;
                    }
                    index += 1;
                }
                analyze_me.drain(0..limit);
                distance += 1;
            }
            self.valves.get_mut(&valve_label).unwrap().shortest_paths = shortest_paths;
        }
    }

    fn navigate(
        &self,
        location: &String,
        total_flow: u32,
        time_left: u32,
        opened_valves: HashSet<String>,
    ) -> u32 {
        let current = self.valves.get(location).unwrap();
        let mut options = current
            .shortest_paths
            .iter()
            .filter(|(label, _)| !opened_valves.contains(&(*label).clone()))
            .filter(|(_, &distance)| distance + 1 < time_left)
            .map(|(label, distance)| {
                let mut new_opened = opened_valves.clone();
                new_opened.insert(label.clone());
                let this_valve = self.valves.get(label).unwrap();
                let remaining_time = time_left - distance - 1;
                self.navigate(
                    label,
                    total_flow + (remaining_time * this_valve.flow_rate),
                    remaining_time,
                    new_opened,
                )
            })
            .collect::<Vec<_>>();
        options.push(total_flow);
        return *options.iter().max().unwrap();
    }
}

struct Valve {
    label: String,
    flow_rate: u32,
    connections: Vec<String>,
    shortest_paths: HashMap<String, u32>,
}

impl Valve {
    fn parse(input: &str) -> Self {
        let (raw_label, rest) = input.split_once(" has ").unwrap();
        let (_, label) = raw_label.split_once(" ").unwrap();
        let (_, rest) = rest.split_once("=").unwrap();
        let (flow_rate, rest) = rest.split_once("; ").unwrap();
        let rest = rest.split(" ").collect::<Vec<_>>();
        let connections = rest[4..]
            .iter()
            .map(|str| str.trim_matches(','))
            .map(|str| str.to_owned())
            .collect::<Vec<_>>();
        return Self {
            label: label.to_owned(),
            flow_rate: flow_rate.parse().unwrap(),
            connections,
            shortest_paths: HashMap::new(),
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "1651");
    }

    #[test]
    fn part2_works() {
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "0");
    }

    #[test]
    fn volcano_parse_works() {
        let input = &utils::read_file("2022/day16", "test.txt");
        let volcano = Volcano::parse(input);

        assert_eq!(volcano.valves.len(), 10);
        assert_eq!(volcano.valves.get("AA").unwrap().connections.len(), 3);
        assert_eq!(volcano.valves.get("AA").unwrap().flow_rate, 0);
        assert_connected(&volcano, "AA", "DD");
        assert_connected(&volcano, "AA", "II");
        assert_connected(&volcano, "AA", "BB");
        assert_eq!(volcano.valves.get("JJ").unwrap().connections.len(), 1);
        assert_eq!(volcano.valves.get("JJ").unwrap().flow_rate, 21);
        assert_connected(&volcano, "JJ", "II");

        assert_eq!(volcano.valves.get("AA").unwrap().shortest_paths.len(), 6);
        assert_eq!(volcano.valves.get("BB").unwrap().shortest_paths.len(), 5);
    }

    fn assert_connected(volcano: &Volcano, valve1: &str, valve2: &str) {
        assert!(volcano
            .valves
            .get(valve1)
            .unwrap()
            .connections
            .contains(&valve2.to_owned()));
        assert!(volcano
            .valves
            .get(valve2)
            .unwrap()
            .connections
            .contains(&valve1.to_owned()));
    }
}
