use rayon::prelude::*;
use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
};

use crate::create_advent_day;

create_advent_day!("2022", "16");

fn part1_with_input(input: &str) -> u32 {
    let volcano = Volcano::parse(input);
    return volcano.navigate(Volcano::id_for_label("AA"), 0, 30, HashSet::new());
}

fn part2_with_input(input: &str) -> u32 {
    let volcano = Volcano::parse(input);
    return volcano.navigate_with_elephant(
        Volcano::id_for_label("AA"),
        0,
        Volcano::id_for_label("AA"),
        0,
        0,
        26,
        HashSet::new(),
    );
}

struct Volcano {
    valves: HashMap<u64, Valve>,
    root_node_hash: u64,
}

impl Volcano {
    fn id_for_label(input: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        let hash = hasher.finish();
        return hash;
    }

    fn parse(input: &str) -> Self {
        let mut valves = HashMap::new();
        input.lines().map(Valve::parse).for_each(|valve| {
            valves.insert(valve.id, valve);
        });
        let mut volcano = Self {
            valves,
            root_node_hash: Self::id_for_label("AA"),
        };
        volcano.find_valve_connections();
        return volcano;
    }

    fn find_valve_connections(&mut self) {
        let all_ids = self
            .valves
            .values()
            .map(|valve| valve.id)
            .collect::<Vec<_>>();
        for valve_id in all_ids {
            let mut seen_valves = HashSet::new();
            let mut distance = 1;
            let mut analyze_me = Vec::new();
            let mut shortest_paths = HashMap::new();
            seen_valves.insert(&valve_id);
            analyze_me.push(&valve_id);
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
                                shortest_paths
                                    .insert(current.connections[connection_index], distance);
                            }
                        }
                        connection_index += 1;
                    }
                    index += 1;
                }
                analyze_me.drain(0..limit);
                distance += 1;
            }
            self.valves.get_mut(&valve_id).unwrap().shortest_paths = shortest_paths;
        }
    }

    fn navigate(
        &self,
        location_id: u64,
        total_flow: u32,
        time_left: u32,
        opened_valves: HashSet<u64>,
    ) -> u32 {
        let current = self.valves.get(&location_id).unwrap();
        let mut options = current
            .shortest_paths
            .par_iter()
            .filter(|(id, _)| !opened_valves.contains(id))
            .filter(|(_, &distance)| distance + 1 < time_left)
            .map(|(&id, distance)| {
                let mut new_opened = opened_valves.clone();
                new_opened.insert(id);
                let this_valve = self.valves.get(&id).unwrap();
                let remaining_time = time_left - distance - 1;
                self.navigate(
                    id,
                    total_flow + (remaining_time * this_valve.flow_rate),
                    remaining_time,
                    new_opened,
                )
            })
            .collect::<Vec<_>>();
        options.push(total_flow);
        return *options.iter().max().unwrap();
    }

    fn navigate_with_elephant(
        &self,
        elephant_location: u64,
        elephant_time_to_next_decision: u32,
        my_location: u64,
        my_time_to_next_decision: u32,
        total_flow: u32,
        time_left: u32,
        opened_valves: HashSet<u64>,
    ) -> u32 {
        let mut new_total_flow = total_flow;
        let mut elephant_options = if elephant_time_to_next_decision != 0 {
            vec![(elephant_location, elephant_time_to_next_decision)]
        } else {
            new_total_flow += self.valves.get(&elephant_location).unwrap().flow_rate * time_left;
            self.valves
                .get(&elephant_location)
                .unwrap()
                .shortest_paths
                .iter()
                .filter(|(id, _)| !opened_valves.contains(id))
                .filter(|(_, &distance)| distance + 1 < time_left)
                .map(|(&id, distance)| (id, distance + 1))
                .collect()
        };
        let mut my_options = if my_time_to_next_decision != 0 {
            vec![(my_location, my_time_to_next_decision)]
        } else {
            new_total_flow += self.valves.get(&my_location).unwrap().flow_rate * time_left;
            self.valves
                .get(&my_location)
                .unwrap()
                .shortest_paths
                .iter()
                .filter(|(id, _)| !opened_valves.contains(id))
                .filter(|(_, &distance)| distance + 1 < time_left)
                .map(|(&id, distance)| (id, distance + 1))
                .collect()
        };

        if elephant_options.len() + my_options.len() == 1 {
            if elephant_options.is_empty() {
                elephant_options.push((self.root_node_hash, time_left))
            } else {
                my_options.push((self.root_node_hash, time_left))
            }
        }

        let mut zipped_options = Vec::with_capacity(elephant_options.len() * my_options.len());
        for e_o in &elephant_options {
            for m_o in &my_options {
                zipped_options.push((e_o, m_o));
            }
        }

        let mut all_options = zipped_options
            .par_iter()
            .filter(|tuple| tuple.0 .0 != tuple.1 .0)
            .map(
                |(
                    &(elephant_next_location, elephant_next_distance),
                    &(my_next_location, my_next_distance),
                )| {
                    let delta = elephant_next_distance.min(my_next_distance);
                    let mut new_opened = opened_valves.clone();
                    new_opened.insert(elephant_next_location);
                    new_opened.insert(my_next_location);
                    self.navigate_with_elephant(
                        elephant_next_location,
                        elephant_next_distance - delta,
                        my_next_location,
                        my_next_distance - delta,
                        new_total_flow,
                        time_left - delta,
                        new_opened,
                    )
                },
            )
            .collect::<Vec<_>>();
        all_options.push(new_total_flow);
        return *all_options.iter().max().unwrap();
    }
}

struct Valve {
    _label: String,
    id: u64,
    flow_rate: u32,
    connections: Vec<u64>,
    shortest_paths: HashMap<u64, u32>,
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
            .map(|str| Volcano::id_for_label(str))
            .collect::<Vec<_>>();
        return Self {
            _label: label.to_owned(),
            id: Volcano::id_for_label(label),
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

        assert_eq!(actual, "1707");
    }

    #[test]
    fn volcano_parse_works() {
        let input = &utils::read_file("2022/day16", "test.txt");
        let volcano = Volcano::parse(input);

        let aa_id = &Volcano::id_for_label("AA");
        let bb_id = &Volcano::id_for_label("BB");
        let dd_id = &Volcano::id_for_label("DD");
        let ii_id = &Volcano::id_for_label("II");
        let jj_id = &Volcano::id_for_label("JJ");

        assert_eq!(volcano.valves.len(), 10);
        assert_eq!(volcano.valves.get(aa_id).unwrap().connections.len(), 3);
        assert_eq!(volcano.valves.get(aa_id).unwrap().flow_rate, 0);
        assert_connected(&volcano, aa_id, dd_id);
        assert_connected(&volcano, aa_id, ii_id);
        assert_connected(&volcano, aa_id, bb_id);
        assert_eq!(volcano.valves.get(jj_id).unwrap().connections.len(), 1);
        assert_eq!(volcano.valves.get(jj_id).unwrap().flow_rate, 21);
        assert_connected(&volcano, jj_id, ii_id);

        assert_eq!(volcano.valves.get(aa_id).unwrap().shortest_paths.len(), 6);
        assert_eq!(volcano.valves.get(bb_id).unwrap().shortest_paths.len(), 5);
    }

    fn assert_connected(volcano: &Volcano, valve1: &u64, valve2: &u64) {
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

    #[test]
    fn hashing() {
        let hash = Volcano::id_for_label("AA");

        assert_eq!(hash, 8039442397745286354);
    }
}
