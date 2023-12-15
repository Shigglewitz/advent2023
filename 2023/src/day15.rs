use crate::utils;

pub fn part1(file_name: &str) -> usize {
    let input = utils::read_file("day15", file_name);
    return input.split(",").map(|step| hash(step)).sum();
}

pub fn part2(file_name: &str) -> usize {
    let input = utils::read_file("day15", file_name);
    return initialization_sequence(&input);
}

fn hash(input: &str) -> usize {
    let mut total = 0;
    for byte in input.bytes() {
        total += byte as usize;
        total *= 17;
        total %= 256;
    }
    return total;
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn initialization_sequence(input: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    let commands = input.split(",");
    for command in commands {
        if command.ends_with("-") {
            let label = command.trim_end_matches("-");
            let box_index = hash(label);
            let bx = &mut boxes[box_index];
            let mut lens_index = usize::MAX;
            let num_lenses = bx.len();
            let label_string = label.to_string();
            for index in 0..num_lenses {
                if bx[index].label == label_string {
                    lens_index = index;
                    break;
                }
            }
            if lens_index != usize::MAX {
                bx.remove(lens_index);
            }
        } else {
            let (label, focal_length_str) = command.split_once("=").unwrap();
            let focal_length = focal_length_str.parse::<usize>().unwrap();
            let box_index = hash(label);
            let bx = &mut boxes[box_index];
            let mut lens_index = usize::MAX;
            let num_lenses = bx.len();
            let label_string = label.to_string();
            for index in 0..num_lenses {
                if bx[index].label == label_string {
                    lens_index = index;
                    break;
                }
            }
            if lens_index == usize::MAX {
                bx.push(Lens {
                    label: label_string,
                    focal_length,
                });
            } else {
                bx[lens_index].focal_length = focal_length;
            }
        }
    }

    let mut sum = 0;
    for (box_index, bx) in boxes.iter().enumerate() {
        for (lens_index, lens) in bx.iter().enumerate() {
            let focusing_power = (box_index + 1) * (lens_index + 1) * lens.focal_length;
            sum += focusing_power;
        }
    }
    return sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 1320);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 145);
    }

    #[test]
    fn hash_works() {
        let actual = hash("HASH");

        assert_eq!(actual, 52);
    }
}
