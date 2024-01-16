use std::cmp::Ordering;

use crate::create_advent_day;

create_advent_day!("2022", "13");

fn part1_with_input(input: &str) -> u32 {
    let pairs: Vec<(Packet, Packet)> = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| (Packet::parse(chunk[0]), Packet::parse(chunk[1])))
        .collect::<Vec<_>>();
    return pairs
        .iter()
        .enumerate()
        .filter(|tuple| tuple.1 .0.in_correct_order(&tuple.1 .1))
        .map(|tuple| tuple.0 as u32 + 1)
        .sum();
}

fn part2_with_input(input: &str) -> u32 {
    let mut packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Packet::parse)
        .collect::<Vec<_>>();
    packets.push(Packet::parse("[[2]]"));
    packets.push(Packet::parse("[[6]]"));
    packets.sort_by(|a, b| {
        if a.in_correct_order(b) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let mut index_of_2 = 0;
    let mut index_of_6 = 0;
    for (index, packet) in packets.iter().enumerate() {
        if &packet.raw_input == "[[2]]" {
            index_of_2 = index as u32 + 1;
        } else if &packet.raw_input == "[[6]]" {
            index_of_6 = index as u32 + 1;
        }
    }
    return index_of_2 * index_of_6;
}

struct Packet {
    data: Vec<Element>,
    raw_input: String,
}

impl Packet {
    fn parse(input: &str) -> Self {
        let bytes = input.as_bytes();
        let mut elements: Vec<Vec<Element>> = Vec::new();
        let mut data = Vec::new();
        let mut index = 0;
        let mut current_num = 0;
        let mut found_num = false;
        while index < bytes.len() {
            if bytes[index] == b'[' {
                elements.push(Vec::new());
            } else if bytes[index] == b',' {
                if found_num {
                    let num_elements = elements.len();
                    elements[num_elements - 1].push(Element::num_element(current_num));
                    current_num = 0;
                    found_num = false;
                }
            } else if bytes[index] == b']' {
                if found_num {
                    let num_elements = elements.len();
                    elements[num_elements - 1].push(Element::num_element(current_num));
                    current_num = 0;
                    found_num = false;
                }
                let add_me = elements.pop().unwrap();
                let new_num_elements = elements.len();
                if new_num_elements == 0 {
                    data = add_me;
                } else {
                    elements[new_num_elements - 1].push(Element::list_element(add_me));
                }
            } else {
                let num = bytes[index] - b'0';
                current_num *= 10;
                current_num += num as u32;
                found_num = true;
            }
            index += 1;
        }
        return Self {
            data,
            raw_input: input.to_owned(),
        };
    }

    fn in_correct_order(&self, other: &Packet) -> bool {
        let mut ordering = Ordering::Equal;
        let num_self_elements = self.data.len();
        let num_other_elements = other.data.len();
        let mut index = 0;
        while ordering == Ordering::Equal && index < num_self_elements && index < num_other_elements
        {
            ordering = self.data[index].compare(&other.data[index]);
            index += 1;
        }
        if ordering == Ordering::Greater {
            return false;
        } else if ordering == Ordering::Less {
            return true;
        }
        if num_self_elements < num_other_elements {
            return true;
        } else if num_self_elements > num_other_elements {
            return false;
        }
        unreachable!("should find a difference in packets")
    }
}

struct Element {
    list_data: Option<Vec<Element>>,
    num_data: Option<u32>,
}

impl Element {
    fn list_element(list: Vec<Element>) -> Self {
        return Self {
            list_data: Some(list),
            num_data: None,
        };
    }
    fn num_element(num: u32) -> Self {
        return Self {
            list_data: None,
            num_data: Some(num),
        };
    }

    fn compare(&self, other: &Element) -> Ordering {
        if self.num_data.is_some() && other.num_data.is_some() {
            return self.compare_numbers(other);
        } else if self.list_data.is_some() && other.list_data.is_some() {
            return self.compare_lists(other);
        } else {
            if self.list_data.is_none() {
                return self.to_list_element().compare_lists(other);
            } else {
                return self.compare_lists(&other.to_list_element());
            }
        }
    }

    fn compare_numbers(&self, other: &Element) -> Ordering {
        return self.num_data.unwrap().cmp(&other.num_data.unwrap());
    }

    fn compare_lists(&self, other: &Element) -> Ordering {
        let num_self_elements = self.list_data.as_ref().unwrap().len();
        let num_other_elements = other.list_data.as_ref().unwrap().len();
        let mut ordering = Ordering::Equal;
        let mut index = 0;
        while ordering == Ordering::Equal && index < num_self_elements && index < num_other_elements
        {
            ordering = self.list_data.as_ref().unwrap()[index]
                .compare(&other.list_data.as_ref().unwrap()[index]);
            index += 1;
        }
        if ordering != Ordering::Equal {
            return ordering;
        }
        return num_self_elements.cmp(&num_other_elements);
    }

    fn to_list_element(&self) -> Element {
        let mut list = Vec::new();
        let data = self.num_data.unwrap();
        let element = Self::num_element(data);
        list.push(element);
        return Self::list_element(list);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    impl Packet {
        fn unwrap_indexes(&self, indexes: Vec<usize>) -> &Element {
            let mut list = &self.data;
            let num_indexes = indexes.len();
            for index in &indexes[0..num_indexes - 1] {
                list = list[*index].list_data.as_ref().unwrap();
            }
            return &list[indexes[num_indexes - 1]];
        }
    }

    impl Element {
        fn get_len(&self) -> usize {
            return self.list_data.as_ref().unwrap().len();
        }

        fn get_val(&self) -> u32 {
            return self.num_data.unwrap();
        }
    }

    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "13");
    }

    #[test]
    fn part2_works() {
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "140");
    }

    #[test]
    fn parse_all_nums_works() {
        let packet = Packet::parse("[1,2,3,4]");

        assert_eq!(packet.data.len(), 4);
        for index in 0..4 {
            assert!(packet.data[index].list_data.is_none());
            assert_eq!(packet.data[index].num_data, Some(index as u32 + 1));
        }
    }

    #[test]
    fn parse_with_lists_works() {
        let packet = Packet::parse("[[1],[2,3,4]]");

        assert_eq!(packet.data.len(), 2);
        assert!(packet.data[0].list_data.is_some());
        assert_eq!(packet.unwrap_indexes(vec![0, 0]).get_val(), 1);
        assert!(packet.data[1].list_data.is_some());
        assert_eq!(packet.unwrap_indexes(vec![1, 0]).get_val(), 2);
        assert_eq!(packet.unwrap_indexes(vec![1, 1]).get_val(), 3);
        assert_eq!(packet.unwrap_indexes(vec![1, 2]).get_val(), 4);
    }

    #[test]
    fn parse_with_zero_works() {
        let packet = Packet::parse("[0,[0]]");

        assert_eq!(packet.data.len(), 2);
        assert_eq!(packet.unwrap_indexes(vec![0]).get_val(), 0);
        assert_eq!(packet.unwrap_indexes(vec![1]).get_len(), 1);
        assert_eq!(packet.unwrap_indexes(vec![1, 0]).get_val(), 0);
    }

    #[test]
    fn nested_parsing_works() {
        let packet = Packet::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]");

        assert_eq!(packet.data.len(), 4);
        assert_eq!(packet.data[0].get_val(), 1);
        assert_eq!(packet.data[1].get_len(), 2);
        assert_eq!(packet.unwrap_indexes(vec![1, 0]).get_val(), 2);
        assert_eq!(packet.unwrap_indexes(vec![1, 1]).get_len(), 2);
        assert_eq!(packet.unwrap_indexes(vec![1, 1, 0]).get_val(), 3);
        assert_eq!(packet.unwrap_indexes(vec![1, 1, 1]).get_len(), 2);
        assert_eq!(packet.unwrap_indexes(vec![1, 1, 1, 0]).get_val(), 4);
        assert_eq!(packet.unwrap_indexes(vec![1, 1, 1, 1]).get_len(), 3);
        assert_eq!(packet.unwrap_indexes(vec![1, 1, 1, 1, 0]).get_val(), 5);
        assert_eq!(packet.unwrap_indexes(vec![1, 1, 1, 1, 1]).get_val(), 6);
        assert_eq!(packet.unwrap_indexes(vec![1, 1, 1, 1, 2]).get_val(), 7);
        assert_eq!(packet.data[2].num_data.unwrap(), 8);
        assert_eq!(packet.data[3].num_data.unwrap(), 9);
    }

    #[rstest]
    #[case("[1,1,3,1,1]", "[1,1,5,1,1]", true)]
    #[case("[[1],[2,3,4]]", "[[1],4]", true)]
    #[case("[9]", "[[8,7,6]]", false)]
    #[case("[[4,4],4,4]", "[[4,4],4,4,4]", true)]
    #[case("[7,7,7,7]", "[7,7,7]", false)]
    #[case("[]", "[3]", true)]
    #[case("[[[]]]", "[[]]", false)]
    #[case("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]", false)]
    fn in_correct_order(#[case] packet1: &str, #[case] packet2: &str, #[case] expected: bool) {
        let packets = (Packet::parse(packet1), Packet::parse(packet2));
        let actual = packets.0.in_correct_order(&packets.1);

        assert_eq!(actual, expected);
    }
}
