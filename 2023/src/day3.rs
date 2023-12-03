use crate::utils;

pub fn part1(file_name: &str) -> i32 {
    return do_stuff(file_name);
}

pub fn part2(file_name: &str) -> i32 {
    return do_stuff(file_name);
}

fn do_stuff(file_name: &str) -> i32 {
    let input = utils::read_file("day3", file_name);

    return input.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_stuff_works() {
        let file_length = do_stuff("test.txt");

        assert_eq!(13, file_length);
    }
}
