use std::collections::{HashMap, VecDeque};

use crate::create_advent_day;

create_advent_day!("20");

fn part1_with_input(input: &str) -> u64 {
    let mut module_configuration = ModuleConfiguration::parse(&input);
    let thousand_presses = (0..1_000)
        .map(|_| module_configuration.press_button())
        .fold((0, 0), |acc, tuple| (acc.0 + tuple.0, acc.1 + tuple.1));
    return thousand_presses.0 * thousand_presses.1;
}

fn part2_with_input(input: &str) -> usize {
    return input.lines().count();
}

struct ModuleConfiguration {
    modules: HashMap<String, Box<dyn PulseHandler>>,
}

impl ModuleConfiguration {
    fn parse(input: &str) -> ModuleConfiguration {
        let mut modules: HashMap<String, Box<dyn PulseHandler>> = HashMap::new();
        for line in input.lines() {
            let (name, destinations) = line.split_once(" -> ").unwrap();
            let destination_names = destinations
                .split(", ")
                .map(|str| str.to_owned())
                .collect::<Vec<_>>();
            if name == "broadcaster" {
                modules.insert(
                    name.to_owned(),
                    Box::new(Broadcast {
                        destination_modules: destination_names,
                    }),
                );
            } else if name.starts_with("%") {
                modules.insert(
                    name[1..].to_owned(),
                    Box::new(FlipFlop::new(destination_names)),
                );
            } else if name.starts_with("&") {
                modules.insert(
                    name[1..].to_owned(),
                    Box::new(Conjunction::new(destination_names)),
                );
            }
        }

        let tuples = modules
            .iter()
            .map(|(name, module)| {
                module
                    .destination_modules()
                    .iter()
                    // TODO: does this need to clone?
                    .map(|dest| (name.clone(), dest.clone()))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        for tuple in tuples {
            let module_opt = modules.get_mut(&tuple.1);
            if module_opt.is_none() {
                modules.insert(
                    tuple.1.to_owned(),
                    Box::new(Output {
                        destination_modules: Vec::new(),
                    }),
                );
            } else {
                module_opt.map(|module| module.add_input(&tuple.0));
            }
        }

        return ModuleConfiguration { modules };
    }

    fn press_button(&mut self) -> (u64, u64) {
        // pre-counting first_pulse
        let mut num_low_pulses = 1;
        let mut num_high_pulses = 0;

        let mut pulses: VecDeque<Pulse> = VecDeque::new();
        let first_pulse = Pulse {
            source: "".to_owned(),
            strength: PulseStrength::LOW,
            destination: "broadcaster".to_owned(),
        };
        pulses.push_back(first_pulse);
        while !pulses.is_empty() {
            let pulse = pulses.pop_front().unwrap();
            let new_pulses = self
                .modules
                .get_mut(&pulse.destination)
                .unwrap()
                .handle_pulse(&pulse);
            for new_pulse in new_pulses {
                match new_pulse.strength {
                    PulseStrength::LOW => num_low_pulses += 1,
                    PulseStrength::HIGH => num_high_pulses += 1,
                };
                pulses.push_back(new_pulse);
            }
        }

        return (num_low_pulses, num_high_pulses);
    }
}

trait PulseHandler {
    fn handle_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse>;
    fn destination_modules(&self) -> &Vec<String>;
    fn add_input(&mut self, input_name: &str);
}

struct Pulse {
    source: String,
    strength: PulseStrength,
    destination: String,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum PulseStrength {
    HIGH,
    LOW,
}

struct FlipFlop {
    on: bool,
    destination_modules: Vec<String>,
}

impl PulseHandler for FlipFlop {
    fn handle_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        if pulse.strength == PulseStrength::HIGH {
            return Vec::new();
        }
        self.on = !self.on;
        let strength = if self.on {
            PulseStrength::HIGH
        } else {
            PulseStrength::LOW
        };
        return self
            .destination_modules
            .iter()
            .map(|dest| {
                Pulse {
                    // TODO: does this have to clone?
                    source: pulse.destination.clone(),
                    strength,
                    // TODO: does this have to clone?
                    destination: dest.clone(),
                }
            })
            .collect::<Vec<_>>();
    }
    fn destination_modules(&self) -> &Vec<String> {
        return &self.destination_modules;
    }
    fn add_input(&mut self, _: &str) {}
}

impl FlipFlop {
    fn new(destination_modules: Vec<String>) -> FlipFlop {
        return FlipFlop {
            on: false,
            destination_modules,
        };
    }
}

struct Conjunction {
    remembered_pulses: HashMap<String, PulseStrength>,
    destination_modules: Vec<String>,
}

impl PulseHandler for Conjunction {
    fn handle_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        self.remembered_pulses
            .insert(pulse.source.clone(), pulse.strength);
        let low_opt = self
            .remembered_pulses
            .values()
            .into_iter()
            .filter(|&&strength| strength == PulseStrength::LOW)
            .next();
        let strength = match low_opt {
            None => PulseStrength::LOW,
            Some(_) => PulseStrength::HIGH,
        };
        return self
            .destination_modules
            .iter()
            .map(|dest| Pulse {
                source: pulse.destination.clone(),
                strength,
                destination: dest.clone(),
            })
            .collect::<Vec<_>>();
    }
    fn destination_modules(&self) -> &Vec<String> {
        return &self.destination_modules;
    }
    fn add_input(&mut self, input_name: &str) {
        self.remembered_pulses
            .insert(input_name.to_owned(), PulseStrength::LOW);
    }
}

impl Conjunction {
    fn new(destination_modules: Vec<String>) -> Conjunction {
        return Conjunction {
            remembered_pulses: HashMap::new(),
            destination_modules,
        };
    }
}

struct Broadcast {
    destination_modules: Vec<String>,
}

impl PulseHandler for Broadcast {
    fn handle_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        return self
            .destination_modules
            .iter()
            .map(|dest| {
                Pulse {
                    // TODO: can this be a const?
                    source: "broadcaster".to_owned(),
                    strength: pulse.strength,
                    // TODO: can this avoid cloning?
                    destination: dest.clone(),
                }
            })
            .collect::<Vec<_>>();
    }
    fn destination_modules(&self) -> &Vec<String> {
        return &self.destination_modules;
    }
    fn add_input(&mut self, _: &str) {}
}

struct Output {
    destination_modules: Vec<String>,
}

impl PulseHandler for Output {
    fn handle_pulse(&mut self, _: &Pulse) -> Vec<Pulse> {
        return Vec::new();
    }

    fn destination_modules(&self) -> &Vec<String> {
        return &self.destination_modules;
    }

    fn add_input(&mut self, _: &str) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn test_module_configuration() -> ModuleConfiguration {
        let input = utils::read_file("day20", "test_1.txt");
        return ModuleConfiguration::parse(&input);
    }

    #[rstest]
    #[case("test_1.txt", 32000000)]
    #[case("test_2.txt", 11687500)]
    fn part1_works(#[case] file_name: &str, #[case] expected: u64) {
        let actual = create(file_name).solve_part1();

        assert_eq!(actual, expected.to_string());
    }

    #[test]
    fn part2_works() {
        let actual = create("test_1.txt").solve_part2();

        assert_eq!(actual, "5".to_owned());
    }

    #[test]
    fn module_configuration_parse_works() {
        let module_configuration = test_module_configuration();

        assert!(module_configuration.modules.contains_key("broadcaster"));
        assert!(module_configuration.modules.contains_key("a"));
        assert!(module_configuration.modules.contains_key("b"));
        assert!(module_configuration.modules.contains_key("c"));
        assert!(module_configuration.modules.contains_key("inv"));
    }

    #[test]
    fn module_configuration_press_button_works() {
        let mut module_configuration = test_module_configuration();

        let actual = module_configuration.press_button();

        assert_eq!(actual.0, 8);
        assert_eq!(actual.1, 4);
    }
}
