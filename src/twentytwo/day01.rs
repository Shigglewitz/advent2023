use crate::create_advent_day;

create_advent_day!("2022", "01");

fn part1_with_input(input: &str) -> i32 {
    let elves = parse_and_sort_elves(input);

    return elves[0].calories;
}

fn part2_with_input(input: &str) -> i32 {
    let elves = parse_and_sort_elves(input);

    return elves[0..3].iter().map(|elf| elf.calories).sum();
}

struct Elf {
    calories: i32,
}

fn parse_and_sort_elves(input: &str) -> Vec<Elf> {
    let mut sum: i32 = 0;
    let mut elves = Vec::new();

    input.lines().for_each(|line| {
        if line.is_empty() {
            elves.push(Elf { calories: sum });
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    });
    elves.push(Elf { calories: sum });

    elves.sort_by(|a, b| b.calories.cmp(&a.calories));

    return elves;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("24000", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("45000", &actual);
    }
}
