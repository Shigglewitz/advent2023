use crate::create_advent_day;

create_advent_day!("2024", "04");

fn part1_with_input(input: &str) -> i64 {
    let chars = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let length = chars[0].len();
    let height = chars.len();

    let mut num_occurrences = 0;

    for i in 0..length {
        for j in 0..height {
            if left(&chars, i, j) {
                num_occurrences += 1;
            }
            if right(&chars, i, j) {
                num_occurrences += 1;
            }
            if up(&chars, i, j) {
                num_occurrences += 1;
            }
            if down(&chars, i, j) {
                num_occurrences += 1;
            }
            if up_left(&chars, i, j) {
                num_occurrences += 1;
            }
            if up_right(&chars, i, j) {
                num_occurrences += 1;
            }
            if down_left(&chars, i, j) {
                num_occurrences += 1;
            }
            if down_right(&chars, i, j) {
                num_occurrences += 1;
            }
        }
    }
    return num_occurrences;
}

fn left(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 3 {
        return false;
    }
    if chars[j][i] == 'X'
        && chars[j][i - 1] == 'M'
        && chars[j][i - 2] == 'A'
        && chars[j][i - 3] == 'S'
    {
        return true;
    }
    return false;
}

fn right(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i > chars[0].len() - 4 {
        return false;
    }
    if chars[j][i] == 'X'
        && chars[j][i + 1] == 'M'
        && chars[j][i + 2] == 'A'
        && chars[j][i + 3] == 'S'
    {
        return true;
    }
    return false;
}

fn up(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j < 3 {
        return false;
    }
    if chars[j][i] == 'X'
        && chars[j - 1][i] == 'M'
        && chars[j - 2][i] == 'A'
        && chars[j - 3][i] == 'S'
    {
        return true;
    }
    return false;
}
fn down(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j > chars.len() - 4 {
        return false;
    }
    if chars[j][i] == 'X'
        && chars[j + 1][i] == 'M'
        && chars[j + 2][i] == 'A'
        && chars[j + 3][i] == 'S'
    {
        return true;
    }
    return false;
}

fn up_left(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j < 3 {
        return false;
    }
    if i < 3 {
        return false;
    }
    if chars[j][i] == 'X'
        && chars[j - 1][i - 1] == 'M'
        && chars[j - 2][i - 2] == 'A'
        && chars[j - 3][i - 3] == 'S'
    {
        return true;
    }
    return false;
}

fn up_right(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j < 3 {
        return false;
    }
    if i > chars[0].len() - 4 {
        return false;
    }
    if chars[j][i] == 'X'
        && chars[j - 1][i + 1] == 'M'
        && chars[j - 2][i + 2] == 'A'
        && chars[j - 3][i + 3] == 'S'
    {
        return true;
    }
    return false;
}

fn down_left(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j > chars.len() - 4 {
        return false;
    }
    if i < 3 {
        return false;
    }
    if chars[j][i] == 'X'
        && chars[j + 1][i - 1] == 'M'
        && chars[j + 2][i - 2] == 'A'
        && chars[j + 3][i - 3] == 'S'
    {
        return true;
    }
    return false;
}

fn down_right(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j > chars.len() - 4 {
        return false;
    }
    if i > chars[0].len() - 4 {
        return false;
    }
    if chars[j][i] == 'X'
        && chars[j + 1][i + 1] == 'M'
        && chars[j + 2][i + 2] == 'A'
        && chars[j + 3][i + 3] == 'S'
    {
        return true;
    }
    return false;
}

fn part2_with_input(input: &str) -> i64 {
    let chars = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let length = chars[0].len();
    let height = chars.len();

    let mut num_occurrences = 0;

    for i in 1..length - 1 {
        for j in 1..height - 1 {
            if chars[j][i] != 'A' {
                continue;
            }
            if !((chars[j - 1][i - 1] == 'M' && chars[j + 1][i + 1] == 'S')
                || (chars[j - 1][i - 1] == 'S' && chars[j + 1][i + 1] == 'M'))
            {
                continue;
            }
            if !((chars[j - 1][i + 1] == 'M' && chars[j + 1][i - 1] == 'S')
                || (chars[j - 1][i + 1] == 'S' && chars[j + 1][i - 1] == 'M'))
            {
                continue;
            }
            num_occurrences += 1;
        }
    }

    return num_occurrences;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("18", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("9", &actual);
    }
}
