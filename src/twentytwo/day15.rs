use crate::create_advent_day;
use rayon::prelude::*;

create_advent_day!("2022", "15");

fn part1_with_input(input: &str) -> i32 {
    let field = Field::parse(input);
    return field.cannot_contain_beacon(2_000_000);
}

fn part2_with_input(input: &str) -> u64 {
    let field = Field::parse(input);
    return field.find_beacon(4_000_000);
}

struct Point {
    x: i32,
    y: i32,
}

struct Field {
    sensors: Vec<Point>,
    beacons: Vec<Point>,
}

impl Field {
    fn parse(input: &str) -> Self {
        let mut sensors = Vec::new();
        let mut beacons = Vec::new();

        input.lines().for_each(|line| {
            let (raw_sensor, raw_beacon) = line.split_once(":").unwrap();

            let (_, raw_sensor_x) = raw_sensor.split_once("x=").unwrap();
            let (sensor_x_str, _) = raw_sensor_x.split_once(",").unwrap();
            let (_, sensor_y_str) = raw_sensor.split_once("y=").unwrap();
            sensors.push(Point {
                x: sensor_x_str.parse().unwrap(),
                y: sensor_y_str.parse().unwrap(),
            });

            let (_, raw_beacon_x) = raw_beacon.split_once("x=").unwrap();
            let (beacon_x_str, _) = raw_beacon_x.split_once(",").unwrap();
            let (_, beacon_y_str) = raw_beacon.split_once("y=").unwrap();
            beacons.push(Point {
                x: beacon_x_str.parse().unwrap(),
                y: beacon_y_str.parse().unwrap(),
            });
        });

        return Self { sensors, beacons };
    }

    fn affected_ranges(&self, y_value: i32) -> Vec<(i32, i32)> {
        let num_inputs = self.beacons.len();
        let mut ranges = (0..num_inputs)
            .map(|index| {
                let manhattan = (self.sensors[index].x - self.beacons[index].x).abs()
                    + (self.sensors[index].y - self.beacons[index].y).abs();
                let sensor_to_y_value = (y_value - self.sensors[index].y).abs();
                if manhattan < sensor_to_y_value {
                    None
                } else {
                    let spread = manhattan - sensor_to_y_value;
                    let min_x = self.sensors[index].x - spread;
                    let max_x = self.sensors[index].x + spread;
                    Some((min_x, max_x))
                }
            })
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect::<Vec<_>>();
        ranges.sort_by(|a, b| a.0.cmp(&b.0));
        return Self::merge_ranges(ranges);
    }

    fn cannot_contain_beacon(&self, y_value: i32) -> i32 {
        let merged = self.affected_ranges(y_value);
        return merged.iter().map(|tuple| tuple.1 - tuple.0).sum();
    }

    fn find_beacon(&self, upper: i32) -> u64 {
        let location = &(0..=upper)
            .into_par_iter()
            .map(|y_value| self.has_free_space_between(y_value, 0, upper))
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect::<Vec<_>>()[0];
        return location.x as u64 * 4_000_000 + location.y as u64;
    }

    fn has_free_space_between(&self, y_value: i32, lower: i32, upper: i32) -> Option<Point> {
        let merged = self.affected_ranges(y_value);
        // TODO: if the answer is on the edge, this may miss one, but we'll see how likely that is
        let filtered = merged
            .iter()
            .filter(|tuple| tuple.0 > lower && tuple.0 < upper)
            .collect::<Vec<_>>();
        if filtered.is_empty() {
            return None;
        }
        return Some(Point {
            x: filtered[0].0 - 1,
            y: y_value,
        });
    }

    fn merge_ranges(input: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let mut index = 1;
        let mut merged = Vec::new();
        merged.push(input[0]);

        while index < input.len() {
            let next = input[index];
            let num_merged = merged.len();
            let prev = &mut merged[num_merged - 1];
            if prev.1 >= next.0 {
                prev.1 = prev.1.max(next.1);
            } else {
                merged.push(next);
            }
            index += 1;
        }

        return merged;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = &utils::read_file("2022/day15", "test.txt");
        let field = Field::parse(input);
        let actual = field.cannot_contain_beacon(10);

        assert_eq!(actual, 26);
    }

    #[test]
    fn part2_works() {
        let input = &utils::read_file("2022/day15", "test.txt");
        let field = Field::parse(input);
        let actual = field.find_beacon(20);

        assert_eq!(actual, 56_000_011);
    }
}
