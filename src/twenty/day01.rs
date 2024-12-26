use crate::create_advent_day;

create_advent_day!("2020", "01");

fn part1_with_input(_input: &str) -> i64 {
    let numbers: Vec<u32> = _input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    let count = numbers.len();
    for i in 0..count {
        for j in (i + 1)..count {
            if numbers[i] + numbers[j] == 2020 {
                // let first = numbers[i].to_string();
                // let second = numbers[j].to_string();
                // println!("{first} {second}");
                return (numbers[i] * numbers[j]) as i64;
            }
        }
    }
    return 0;
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

        assert_eq!("514579", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("0", &actual);
    }
}
