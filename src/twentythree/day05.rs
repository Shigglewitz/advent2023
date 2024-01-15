use std::collections::HashSet;

use crate::create_advent_day;

create_advent_day!("2023", "05");

fn part1_with_input(input: &str) -> i64 {
    let almanac = Almanac::from(&input);
    return almanac
        .seeds
        .iter()
        .map(|seed| almanac.seed_to_location(*seed))
        .min()
        .unwrap();
}

fn part2_with_input(input: &str) -> i64 {
    let almanac = Almanac::from(&input);
    let seed_ranges = almanac.seed_ranges();

    let seed_to_fertilizer = almanac
        .seed_to_soil_map
        .merge(&almanac.soil_to_fertilizer_map);
    let seed_to_water = seed_to_fertilizer.merge(&almanac.fertilizer_to_water_map);
    let seed_to_light = seed_to_water.merge(&almanac.water_to_light_map);
    let seed_to_temperature = seed_to_light.merge(&almanac.light_to_temperature_map);
    let seed_to_humidity = seed_to_temperature.merge(&almanac.temperature_to_humidity_map);
    let seed_to_location = seed_to_humidity.merge(&almanac.humidity_to_location_map);

    return seed_to_location
        .ranges
        .iter()
        .filter(|range| any_seed_range_contains(&seed_ranges, range.source_range_start))
        .map(|range| range.destination_range_start)
        .min()
        .unwrap();
}

struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil_map: Mapping,
    soil_to_fertilizer_map: Mapping,
    fertilizer_to_water_map: Mapping,
    water_to_light_map: Mapping,
    light_to_temperature_map: Mapping,
    temperature_to_humidity_map: Mapping,
    humidity_to_location_map: Mapping,
}

impl Almanac {
    fn from(input: &str) -> Almanac {
        let mut lines = input.lines();
        let line = lines.nth(0).unwrap();
        let seeds: Vec<i64> = line
            .split(":")
            .nth(1)
            .unwrap()
            .split(" ")
            .filter(|string| !string.is_empty())
            .map(|string| string.parse::<i64>().unwrap())
            .collect();
        let mut mappings: Vec<Mapping> = Vec::new();
        let mut ranges: Vec<AlmanacRange> = Vec::new();
        for line in lines.skip(1) {
            if line.ends_with("map:") {
                continue;
            }
            if line.len() == 0 {
                mappings.push(Mapping { ranges });
                ranges = Vec::new();
                continue;
            }
            let nums: Vec<i64> = line
                .split(" ")
                .map(|value| value.parse::<i64>().unwrap())
                .collect();
            ranges.push(AlmanacRange {
                source_range_start: nums[1],
                destination_range_start: nums[0],
                range_length: nums[2],
            });
        }

        mappings.push(Mapping { ranges });

        let almanac = Almanac {
            seeds,
            seed_to_soil_map: Mapping::from(&mappings[0].ranges),
            soil_to_fertilizer_map: Mapping::from(&mappings[1].ranges),
            fertilizer_to_water_map: Mapping::from(&mappings[2].ranges),
            water_to_light_map: Mapping::from(&mappings[3].ranges),
            light_to_temperature_map: Mapping::from(&mappings[4].ranges),
            temperature_to_humidity_map: Mapping::from(&mappings[5].ranges),
            humidity_to_location_map: Mapping::from(&mappings[6].ranges),
        };
        return almanac;
    }

    fn seed_to_location(&self, seed: i64) -> i64 {
        let soil = self.seed_to_soil_map.translate(seed, true);
        let fertilizer = self.soil_to_fertilizer_map.translate(soil, true);
        let water = self.fertilizer_to_water_map.translate(fertilizer, true);
        let light = self.water_to_light_map.translate(water, true);
        let temperature = self.light_to_temperature_map.translate(light, true);
        let humidity = self
            .temperature_to_humidity_map
            .translate(temperature, true);
        let location = self.humidity_to_location_map.translate(humidity, true);

        return location;
    }

    fn seed_ranges(&self) -> Vec<SeedRange> {
        return self
            .seeds
            .chunks(2)
            .map(|chunk| SeedRange {
                range_start: chunk[0],
                range_length: chunk[1],
            })
            .collect();
    }
}

struct Mapping {
    ranges: Vec<AlmanacRange>,
}

impl Mapping {
    fn from(ranges: &Vec<AlmanacRange>) -> Mapping {
        let mut mapping = Mapping { ranges: Vec::new() };
        mapping.ranges.extend(ranges);
        return mapping;
    }

    fn translate(&self, input: i64, forward: bool) -> i64 {
        for range in self.ranges.iter() {
            if range.contains(input, forward) {
                return range.translate_forward(input, forward);
            }
        }
        return input;
    }

    fn merge(&self, mapping: &Mapping) -> Mapping {
        let mut boundaries: HashSet<i64> = HashSet::new();

        for range in &self.ranges {
            boundaries.insert(range.destination_range_start);
            boundaries.insert(range.destination_range_start + range.range_length);
        }
        for range in &mapping.ranges {
            boundaries.insert(range.source_range_start);
            boundaries.insert(range.source_range_start + range.range_length);
        }

        let mut sorted_boundaries: Vec<i64> = Vec::from_iter(boundaries);
        sorted_boundaries.sort();

        let mut start = *sorted_boundaries.first().unwrap();

        let new_ranges = sorted_boundaries
            .iter()
            .skip(1)
            .map(|boundary| {
                let source_range_start = self.translate(start, false);
                let destination_range_start = mapping.translate(start, true);
                let range_length = boundary - start;
                start = *boundary;
                AlmanacRange {
                    source_range_start,
                    destination_range_start,
                    range_length,
                }
            })
            .collect();

        return Mapping { ranges: new_ranges };
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct AlmanacRange {
    source_range_start: i64,
    destination_range_start: i64,
    range_length: i64,
}

impl AlmanacRange {
    fn contains(&self, input: i64, forward: bool) -> bool {
        let compare_me = if forward {
            self.source_range_start
        } else {
            self.destination_range_start
        };

        return input >= compare_me && input < compare_me + self.range_length;
    }

    fn translate_forward(&self, input: i64, forward: bool) -> i64 {
        if forward {
            return self.destination_range_start + (input - self.source_range_start);
        } else {
            return self.source_range_start + (input - self.destination_range_start);
        }
    }
}

struct SeedRange {
    range_start: i64,
    range_length: i64,
}

impl SeedRange {
    fn contains(&self, seed: i64) -> bool {
        return seed >= self.range_start && seed < self.range_start + self.range_length;
    }
}

fn any_seed_range_contains(seed_ranges: &Vec<SeedRange>, seed: i64) -> bool {
    for seed_range in seed_ranges {
        if seed_range.contains(seed) {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_almanac() -> Almanac {
        return Almanac::from(&utils::read_file("2023/day05", "test.txt"));
    }

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!(&actual, "35");
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(&actual, "46");
    }

    #[test]
    fn merge_works() {
        let almanac = test_almanac();

        let merged = almanac
            .temperature_to_humidity_map
            .merge(&almanac.humidity_to_location_map);

        assert_eq!(merged.translate(0, true), 1);
        assert_eq!(merged.translate(56, true), 61);
        assert_eq!(merged.translate(69, true), 0);
        assert_eq!(merged.translate(70, true), 74);
        assert_eq!(merged.translate(93, true), 56);
        assert_eq!(merged.translate(97, true), 97);
    }

    #[test]
    fn almanac_from_works() {
        let almanac = test_almanac();

        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(
            almanac.seed_to_soil_map.ranges[0],
            AlmanacRange {
                source_range_start: 98,
                destination_range_start: 50,
                range_length: 2
            }
        );
        assert_eq!(almanac.seed_to_soil_map.ranges.len(), 2);
        assert_eq!(
            almanac.soil_to_fertilizer_map.ranges[1],
            AlmanacRange {
                source_range_start: 52,
                destination_range_start: 37,
                range_length: 2,
            }
        );
        assert_eq!(almanac.soil_to_fertilizer_map.ranges.len(), 3);
        assert_eq!(almanac.fertilizer_to_water_map.ranges.len(), 4);
        assert_eq!(almanac.water_to_light_map.ranges.len(), 2);
        assert_eq!(almanac.light_to_temperature_map.ranges.len(), 3);
        assert_eq!(almanac.temperature_to_humidity_map.ranges.len(), 2);
        assert_eq!(almanac.humidity_to_location_map.ranges.len(), 2);
    }

    #[test]
    fn almanac_seed_ranges_works() {
        let almanac = test_almanac();
        let seed_ranges = almanac.seed_ranges();

        assert_eq!(seed_ranges.len(), 2);
        assert_eq!(seed_ranges[0].range_start, 79);
        assert_eq!(seed_ranges[0].range_length, 14);
        assert_eq!(seed_ranges[1].range_start, 55);
        assert_eq!(seed_ranges[1].range_length, 13);

        assert!(seed_ranges[0].contains(79));
        assert!(seed_ranges[0].contains(92));
        assert!(!seed_ranges[0].contains(78));
        assert!(!seed_ranges[0].contains(93));
    }
}
