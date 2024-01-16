use crate::create_advent_day;

create_advent_day!("2022", "15");

fn part1_with_input(input: &str) -> i32 {
    let field = Field::parse(input);
    return field.cannot_contain_beacon(2_000_000);
}

fn part2_with_input(input: &str) -> u32 {
    return 0;
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

    fn cannot_contain_beacon(&self, y_value: i32) -> i32 {
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
        let merged = Self::merge_ranges(ranges);
        return merged.iter().map(|tuple| tuple.1 - tuple.0).sum();
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
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "0");
    }
}
