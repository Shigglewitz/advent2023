use std::collections::{HashMap, HashSet};

use crate::create_advent_day;

create_advent_day!("2024", "23");

fn part1_with_input(input: &str) -> i64 {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    input.lines().for_each(|line| {
        let mut split = line.split("-");
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        connections
            .entry(first.to_owned())
            .or_default()
            .insert(second.to_owned());
        connections
            .entry(second.to_owned())
            .or_default()
            .insert(first.to_owned());
    });

    let trios: HashSet<String> = connections
        .iter()
        .filter(|(key, _)| key.starts_with("t"))
        .map(|(t_label, t_label_connections)| {
            t_label_connections
                .iter()
                .map(|label| {
                    connections
                        .get(&label.clone())
                        .unwrap()
                        .iter()
                        .filter_map(|third_label| {
                            let contains_initial =
                                connections.get(third_label).unwrap().contains(t_label);
                            if contains_initial {
                                let mut list =
                                    vec![t_label.clone(), label.clone(), third_label.clone()];
                                list.sort();
                                Some(list.join(","))
                            } else {
                                None
                            }
                        })
                        .collect::<HashSet<_>>()
                })
                .flatten()
                .collect::<HashSet<_>>()
        })
        .flatten()
        .collect();

    return trios.len() as i64;
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

        assert_eq!("7", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("0", &actual);
    }
}
