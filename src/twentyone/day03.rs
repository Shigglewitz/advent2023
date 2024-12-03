use std::collections::HashMap;

use crate::create_advent_day;

create_advent_day!("2021", "03");

fn part1_with_input(input: &str) -> i64 {
    let mut map: HashMap<usize, BitCount> = HashMap::new();
    for line in input.lines() {
        for (index, letter) in line.chars().enumerate() {
            if !map.contains_key(&index) {
                map.insert(index, BitCount { zeroes: 0, ones: 0 });
            }
            let count = map.get_mut(&index).unwrap();
            if letter == '0' {
                count.zeroes += 1;
            } else {
                count.ones += 1;
            }
        }
    }

    let mut gamma_arr = vec![0; map.len()];
    let mut epsilon_arr = vec![0; map.len()];
    for index in 0..map.len() {
        let count = map.get(&index).unwrap();
        if count.ones > count.zeroes {
            gamma_arr[index] = 1;
            epsilon_arr[index] = 0;
        } else if count.ones < count.zeroes {
            gamma_arr[index] = 0;
            epsilon_arr[index] = 1;
        } else {
            panic!("ones and zeroes are equal at index {index}")
        }
    }

    let gamma = binary_arr_to_int(gamma_arr);
    let epsilon = binary_arr_to_int(epsilon_arr);

    return gamma * epsilon;
}

fn part2_with_input(input: &str) -> i64 {
    let mut starts_with_one: Vec<Vec<i32>> = Vec::new();
    let mut starts_with_zero: Vec<Vec<i32>> = Vec::new();

    let all_letters = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|letter| letter.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    for word in all_letters {
        if word[0] == 0 {
            starts_with_zero.push(word);
        } else if word[0] == 1 {
            starts_with_one.push(word);
        }
    }

    let (oxygen, co) = if starts_with_one.len() > starts_with_zero.len() {
        (starts_with_one, starts_with_zero)
    } else {
        (starts_with_zero, starts_with_one)
    };

    let oxygen = filter_oxygen(oxygen, 1);
    let co = filter_co(co, 1);

    let oxygen = binary_arr_to_int(oxygen);
    let co = binary_arr_to_int(co);

    return oxygen * co;
}

fn binary_arr_to_int(input: Vec<i32>) -> i64 {
    let mut sum = 0;
    for (index, &number) in input.iter().rev().enumerate() {
        let base: i64 = 2;
        sum += base.pow(index as u32) * (number as i64);
    }
    return sum;
}

fn filter_oxygen(input: Vec<Vec<i32>>, index: usize) -> Vec<i32> {
    if input.len() == 1 {
        return input[0].clone();
    }
    let mut index_is_one: Vec<Vec<i32>> = Vec::new();
    let mut index_is_zero: Vec<Vec<i32>> = Vec::new();

    for word in input {
        if word[index] == 0 {
            index_is_zero.push(word);
        } else if word[index] == 1 {
            index_is_one.push(word);
        }
    }

    return if index_is_one.len() >= index_is_zero.len() {
        filter_oxygen(index_is_one, index + 1)
    } else {
        filter_oxygen(index_is_zero, index + 1)
    };
}

fn filter_co(input: Vec<Vec<i32>>, index: usize) -> Vec<i32> {
    if input.len() == 1 {
        return input[0].clone();
    }
    let mut index_is_one: Vec<Vec<i32>> = Vec::new();
    let mut index_is_zero: Vec<Vec<i32>> = Vec::new();

    for word in input {
        if word[index] == 0 {
            index_is_zero.push(word);
        } else if word[index] == 1 {
            index_is_one.push(word);
        }
    }

    return if index_is_one.len() >= index_is_zero.len() {
        filter_co(index_is_zero, index + 1)
    } else {
        filter_co(index_is_one, index + 1)
    };
}

struct BitCount {
    zeroes: i64,
    ones: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("198", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("230", &actual);
    }
}
