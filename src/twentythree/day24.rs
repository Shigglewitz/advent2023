use std::cmp::Ordering;

use crate::create_advent_day;

create_advent_day!("2023", "24");

fn part1_with_input(input: &str) -> u32 {
    let stones = input.lines().map(HailStone::parse).collect::<Vec<_>>();

    return count_future_intersections(stones, 200000000000000.0, 400000000000000.0);
}

fn part2_with_input(input: &str) -> u32 {
    return input.lines().count() as u32;
}

fn count_future_intersections(
    hailstones: Vec<HailStone>,
    lower_bound: f64,
    upper_bound: f64,
) -> u32 {
    let mut count = 0;

    for (first_index, hailstone) in hailstones.iter().enumerate() {
        for other_stone in hailstones[first_index + 1..].iter() {
            let (x, y) = hailstone.intersection(&other_stone);
            if x >= lower_bound
                && x <= upper_bound
                && y >= lower_bound
                && y <= upper_bound
                && hailstone.point_in_future(x, y)
                && other_stone.point_in_future(x, y)
            {
                count += 1;
            }
        }
    }

    return count;
}

struct HailStone {
    px: i64,
    py: i64,
    pz: i64,
    dx: i64,
    dy: i64,
    dz: i64,
}

impl HailStone {
    fn parse(input: &str) -> Self {
        let (positions, velocities) = input.split_once(" @ ").unwrap();
        let mut p_split = positions.split(", ");
        let mut v_split = velocities.split(", ");
        return Self {
            px: p_split.next().unwrap().trim().parse::<i64>().unwrap(),
            py: p_split.next().unwrap().trim().parse::<i64>().unwrap(),
            pz: p_split.next().unwrap().trim().parse::<i64>().unwrap(),
            dx: v_split.next().unwrap().trim().parse::<i64>().unwrap(),
            dy: v_split.next().unwrap().trim().parse::<i64>().unwrap(),
            dz: v_split.next().unwrap().trim().parse::<i64>().unwrap(),
        };
    }

    fn get_a_b_c(&self) -> (f64, f64, f64) {
        let a = -self.dy as f64;
        let b = self.dx as f64;
        let c = -((self.py * self.dx) - (self.dy * self.px)) as f64;

        if a < 0.0 {
            return (a * -1.0, b * -1.0, c * -1.0);
        } else {
            return (a, b, c);
        }
    }

    fn intersection(&self, other: &HailStone) -> (f64, f64) {
        let (a1, b1, c1) = other.get_a_b_c();
        let (a2, b2, c2) = self.get_a_b_c();
        let x = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
        let y = (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1);
        return (x, y);
    }

    fn point_in_future(&self, x: f64, y: f64) -> bool {
        let expected_x_dir = self.px.cmp(&(self.px + self.dx));
        let expected_y_dir = self.py.cmp(&(self.py + self.dy));
        let actual_x_dir = if (self.px as f64) < x {
            Ordering::Less
        } else {
            Ordering::Greater
        };
        let actual_y_dir = if (self.py as f64) < y {
            Ordering::Less
        } else {
            Ordering::Greater
        };

        return expected_x_dir == actual_x_dir && expected_y_dir == actual_y_dir;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = utils::read_file("2023/day24", "test.txt");
        let stones = input.lines().map(HailStone::parse).collect::<Vec<_>>();
        let actual = count_future_intersections(stones, 7.0, 27.0);

        assert_eq!(actual, 2);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(actual, "5");
    }

    #[test]
    fn hailstone_parse_works() {
        let hailstone = HailStone::parse("19, 13, 30 @ -2,  1, -2");

        assert_eq!(hailstone.px, 19);
        assert_eq!(hailstone.py, 13);
        assert_eq!(hailstone.pz, 30);
        assert_eq!(hailstone.dx, -2);
        assert_eq!(hailstone.dy, 1);
        assert_eq!(hailstone.dz, -2);
    }

    #[test]
    fn hailstone_intersection_works() {
        let input = utils::read_file("2023/day24", "test.txt");
        let stones = input.lines().map(HailStone::parse).collect::<Vec<_>>();

        let (x, y) = stones[0].intersection(&stones[1]);
        assert!(x > 14.0);
        assert!(x < 15.0);
        assert!(y > 15.0);
        assert!(y < 16.0);
        assert_eq!(stones[0].point_in_future(x, y), true);
        assert_eq!(stones[1].point_in_future(x, y), true);

        // this intersection happened in the past
        let (x, y) = stones[0].intersection(&stones[4]);
        assert!(x > 21.0);
        assert!(x < 22.0);
        assert!(y > 11.0);
        assert!(y < 12.0);
        assert_eq!(stones[0].point_in_future(x, y), false);
        assert_eq!(stones[4].point_in_future(x, y), true);

        let (x, y) = stones[1].intersection(&stones[2]);
        assert_eq!(x, f64::INFINITY);
        assert_eq!(y, f64::INFINITY);
    }
}
