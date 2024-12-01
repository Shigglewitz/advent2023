use crate::create_advent_day;

create_advent_day!("2021", "02");

fn part1_with_input(input: &str) -> i64 {
    let position = &mut SubmarinePosition {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        let word = split.next().unwrap();
        let magnitude = split.next().unwrap().parse::<i64>().unwrap();

        match word {
            "forward" => Forward { magnitude }.adjust_position1(position),
            "up" => Up { magnitude }.adjust_position1(position),
            "down" => Down { magnitude }.adjust_position1(position),
            _ => panic!("unexpected command $command"),
        };
    });
    return position.horizontal * position.depth;
}

fn part2_with_input(input: &str) -> i64 {
    let position = &mut SubmarinePosition {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        let word = split.next().unwrap();
        let magnitude = split.next().unwrap().parse::<i64>().unwrap();

        match word {
            "forward" => Forward { magnitude }.adjust_position2(position),
            "up" => Up { magnitude }.adjust_position2(position),
            "down" => Down { magnitude }.adjust_position2(position),
            _ => panic!("unexpected command $command"),
        };
    });
    return position.horizontal * position.depth;
}

trait Command {
    fn adjust_position1(&self, location: &mut SubmarinePosition);
    fn adjust_position2(&self, location: &mut SubmarinePosition);
}

struct SubmarinePosition {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

struct Forward {
    magnitude: i64,
}

impl Command for Forward {
    fn adjust_position1(&self, location: &mut SubmarinePosition) {
        location.horizontal += self.magnitude;
    }

    fn adjust_position2(&self, location: &mut SubmarinePosition) {
        location.horizontal += self.magnitude;
        location.depth += self.magnitude * location.aim;
    }
}

struct Down {
    magnitude: i64,
}

impl Command for Down {
    fn adjust_position1(&self, location: &mut SubmarinePosition) {
        location.depth += self.magnitude;
    }

    fn adjust_position2(&self, location: &mut SubmarinePosition) {
        location.aim += self.magnitude;
    }
}

struct Up {
    magnitude: i64,
}

impl Command for Up {
    fn adjust_position1(&self, location: &mut SubmarinePosition) {
        location.depth -= self.magnitude;
    }

    fn adjust_position2(&self, location: &mut SubmarinePosition) {
        location.aim -= self.magnitude;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("150", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("900", &actual);
    }
}
