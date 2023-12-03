use crate::utils;

pub fn part1(file_name: &str) -> i32 {
    let elves = parse_and_sort_elves(file_name);

    return elves[0].calories;
}

pub fn part2(file_name: &str) -> i32 {
    let elves = parse_and_sort_elves(file_name);

    return elves[0..3].iter().map(|elf| elf.calories).sum();
}

struct Elf {
    calories: i32,
}

#[test]
fn part1_works() {
    let actual = part1("test.txt");

    assert_eq!(24000, actual);
}

#[test]
fn part2_works() {
    let actual = part2("test.txt");

    assert_eq!(45000, actual);
}

fn parse_and_sort_elves(file_name: &str) -> Vec<Elf> {
    let input = utils::read_file("day1", file_name);
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
