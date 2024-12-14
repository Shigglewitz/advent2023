use std::u64;

use crate::create_advent_day;

create_advent_day!("2024", "13");

fn part1_with_input(input: &str) -> u64 {
    let lines = input.lines();
    let mut machines: Vec<ClawMachine> = Vec::new();
    parse_claw_machines(lines, &mut machines);
    return machines
        .iter()
        .filter_map(|machine| machine.cheapest_solution(Some(100)))
        .sum::<u64>() as u64;
}

fn parse_claw_machines(mut lines: std::str::Lines<'_>, machines: &mut Vec<ClawMachine>) {
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next())
    {
        let mut split = first.split("+");
        split.next();
        let a_x = split
            .next()
            .unwrap()
            .split(",")
            .next()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let a_y = split.next().unwrap().parse::<u64>().unwrap();
        let mut split = second.split("+");
        split.next();
        let b_x = split
            .next()
            .unwrap()
            .split(",")
            .next()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let b_y = split.next().unwrap().parse::<u64>().unwrap();
        let mut split = third.split("=");
        split.next();
        let prize_x = split
            .next()
            .unwrap()
            .split(",")
            .next()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let prize_y = split.next().unwrap().parse::<u64>().unwrap();

        machines.push(ClawMachine {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        });
        lines.next();
    }
}

struct ClawMachine {
    a_x: u64,
    a_y: u64,
    b_x: u64,
    b_y: u64,
    prize_x: u64,
    prize_y: u64,
}

impl ClawMachine {
    fn cheapest_solution(&self, press_limit: Option<u64>) -> Option<u64> {
        // println!("cheapest solutions");
        let x_combinations = find_combinations(self.a_x, self.b_x, self.prize_x);
        return x_combinations
            .iter()
            .filter(|(a, b)| match press_limit {
                Some(limit) => *a <= limit && *b <= limit,
                None => true,
            })
            .filter(|(a_presses, b_presses)| {
                (self.a_y * *a_presses) + (self.b_y * *b_presses) == self.prize_y
            })
            .map(|(a_presses, b_presses)| *a_presses * 3 + *b_presses)
            .min();
    }
}

fn find_combinations(first: u64, second: u64, goal: u64) -> Vec<(u64, u64)> {
    let (a, b, swap) = if first > second {
        (first, second, false)
    } else {
        (second, first, true)
    };
    let mut results = Vec::new();
    let mut initial_diff = u64::MAX;
    // println!("starting");
    for i in (0..((goal / a) + 1)).rev() {
        if results.len() == 2 {
            break;
        }
        let moving_target = i * a;
        let diff = goal - moving_target;
        if diff == 0 {
            results.push((i, 0));
            continue;
        }
        let remainder = diff % b;
        if remainder == 0 {
            results.push((i, diff / b));
        }
        // println!("** {remainder}");
        if remainder == initial_diff && results.is_empty() {
            break;
        }
        if initial_diff == u64::MAX {
            initial_diff = remainder;
        }
    }

    if results.len() == 2 {
        let a_diff = results[0].0 - results[1].0;
        let b_diff = results[1].1 - results[0].1;
        let a_starting = results[1].0;
        // println!("starting at {a_starting} with diff {a_diff}");
        let mut a_current = results[1].0;
        let mut b_current = results[1].1;
        for _ in (0..(a_starting / a_diff)).rev() {
            a_current -= a_diff;
            b_current += b_diff;
            results.push((a_current, b_current));
        }
    }

    if swap {
        return results.into_iter().map(|(a, b)| (b, a)).collect();
    } else {
        return results;
    }
}

fn part2_with_input(input: &str) -> u64 {
    let lines = input.lines();
    let mut machines: Vec<ClawMachine> = Vec::new();
    parse_claw_machines(lines, &mut machines);
    // println!("done parsing machines");
    let _adjusted_machines: Vec<ClawMachine> = machines
        .into_iter()
        .map(|mut machine| {
            machine.prize_x += 10000000000000;
            machine.prize_y += 10000000000000;
            machine
        })
        .collect();
    // println!("done adjusting machines");
    // let size = adjusted_machines.len();
    // println!("size {size}");

    // adjusted_machines[0].cheapest_solution(None);
    return 42;

    // return adjusted_machines
    //     .iter()
    //     .filter_map(|machine| machine.cheapest_solution(None))
    //     .sum::<u64>() as u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("480", &actual);
    }

    #[test]
    fn test_find_combinations1() {
        let result = find_combinations(94, 22, 8400);

        assert_eq!(result, vec![(80, 40), (69, 87)]);
    }

    #[test]
    fn test_find_combinations2() {
        let result = find_combinations(17, 84, 7870);

        assert_eq!(result, vec![(38, 86)]);
    }

    #[test]
    fn test_cheapest_solution() {
        let machine = ClawMachine {
            a_x: 94,
            a_y: 34,
            b_x: 22,
            b_y: 67,
            prize_x: 8400,
            prize_y: 5400,
        };
        let solution = machine.cheapest_solution(Some(100));

        assert_eq!(solution, Some(280));
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("1", &actual);
    }
}
