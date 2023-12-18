use crate::create_advent_day;

create_advent_day!("15");

fn part1_with_input(input: &str) -> usize {
    return input.split(",").map(holiday_ascii_string_helper).sum();
}

fn part2_with_input(input: &str) -> usize {
    let lens_boxes = LensBox::manual_arrangement_procedure(&input);
    return lens_boxes
        .iter()
        .enumerate()
        .map(|(index, lens_box)| lens_box.calculate_focusing_power(index))
        .sum();
}

fn holiday_ascii_string_helper(input: &str) -> usize {
    let mut total = 0;
    for byte in input.bytes() {
        total += byte as usize;
        total *= 17;
        total %= 256;
    }
    return total;
}

#[derive(Clone)]
struct LensBox {
    lenses: Vec<Lens>,
}

impl Default for LensBox {
    fn default() -> LensBox {
        return LensBox { lenses: Vec::new() };
    }
}

impl LensBox {
    fn manual_arrangement_procedure(input: &str) -> Vec<LensBox> {
        let mut boxes: Vec<LensBox> = vec![LensBox::default(); 256];
        let commands = input.split(",");
        for command in commands {
            if command.ends_with("-") {
                // drop the '-' at the end
                let label = &command[0..(command.len() - 1)];
                let box_index = holiday_ascii_string_helper(label);
                boxes[box_index].remove_lens(label);
            } else {
                let (label, focal_length_str) = command.split_once("=").unwrap();
                let focal_length = focal_length_str.parse::<usize>().unwrap();
                let box_index = holiday_ascii_string_helper(label);
                boxes[box_index].add_lens(label, focal_length);
            }
        }
        return boxes;
    }

    fn remove_lens(&mut self, label: &str) {
        let lens_index_option = self
            .lenses
            .iter()
            .enumerate()
            .filter(|(_, lens)| &lens.label == label)
            .map(|(index, _)| index)
            .next();
        if let Some(lens_index) = lens_index_option {
            self.lenses.remove(lens_index);
        }
    }

    fn add_lens(&mut self, label: &str, focal_length: usize) {
        let lens_index_option = self
            .lenses
            .iter()
            .enumerate()
            .filter(|(_, lens)| &lens.label == label)
            .map(|(index, _)| index)
            .next();
        match lens_index_option {
            Some(lens_index) => self.lenses[lens_index].focal_length = focal_length,
            None => self.lenses.push(Lens {
                label: label.to_string(),
                focal_length,
            }),
        };
    }

    fn calculate_focusing_power(&self, index: usize) -> usize {
        return self
            .lenses
            .iter()
            .enumerate()
            .map(|(lens_index, lens)| (index + 1) * (lens_index + 1) * lens.focal_length)
            .sum();
    }
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn test_lens_box() -> Vec<LensBox> {
        return LensBox::manual_arrangement_procedure(&utils::read_file("day15", "test.txt"));
    }

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!(&actual, "1320");
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(&actual, "145");
    }

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn holiday_ascii_string_helper_tests(#[case] input: &str, #[case] expected: usize) {
        let actual = holiday_ascii_string_helper(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn lens_box_initialization_sequence_works() {
        let lens_boxes = test_lens_box();

        assert_eq!(lens_boxes[0].lenses.len(), 2);
        assert_eq!(lens_boxes[0].lenses[0].label, "rn");
        assert_eq!(lens_boxes[0].lenses[0].focal_length, 1);
        assert_eq!(lens_boxes[0].lenses[1].label, "cm");
        assert_eq!(lens_boxes[0].lenses[1].focal_length, 2);
        assert_eq!(lens_boxes[3].lenses.len(), 3);
        assert_eq!(lens_boxes[3].lenses[0].label, "ot");
        assert_eq!(lens_boxes[3].lenses[0].focal_length, 7);
        assert_eq!(lens_boxes[3].lenses[1].label, "ab");
        assert_eq!(lens_boxes[3].lenses[1].focal_length, 5);
        assert_eq!(lens_boxes[3].lenses[2].label, "pc");
        assert_eq!(lens_boxes[3].lenses[2].focal_length, 6);
    }

    #[rstest]
    #[case(0, 5)]
    #[case(3, 140)]
    fn lens_box_calculate_focusing_power_tests(#[case] box_index: usize, #[case] expected: usize) {
        let lens_boxes = test_lens_box();
        let actual = lens_boxes[box_index].calculate_focusing_power(box_index);

        assert_eq!(actual, expected);
    }
}
