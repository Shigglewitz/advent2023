use std::{cmp::Ordering, collections::HashMap};

use crate::utils;

pub fn part1(file_name: &str) -> i64 {
    let input = utils::read_file("day7", file_name);

    let mut hands: Vec<Hand> = input.lines().map(Hand::parse).collect();
    hands.sort_by(compare_hands);
    return hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as i64 + 1) * hand.bid)
        .sum();
}

pub fn part2(file_name: &str) -> i64 {
    let input = utils::read_file("day7", file_name);

    let mut hands: Vec<Hand> = input.lines().map(Hand::parse).collect();
    hands.sort_by(compare_hands_with_jokers);
    return hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as i64 + 1) * hand.bid)
        .sum();
}

#[derive(PartialEq, Debug, Ord, PartialOrd, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

struct Hand {
    cards: [char; 5],
    bid: i64,
}

impl Hand {
    fn parse(input: &str) -> Hand {
        let mut split = input.split(" ");
        let mut cards = [' '; 5];
        for (i, card) in split.next().unwrap().chars().take(5).enumerate() {
            cards[i] = card;
        }
        let bid = split.next().unwrap().parse::<i64>().unwrap();
        return Hand { cards, bid };
    }

    fn score_hand_with_jokers(&self) -> HandType {
        return self.score_hand_private(true);
    }

    fn score_hand(&self) -> HandType {
        return self.score_hand_private(false);
    }

    fn score_hand_private(&self, allow_jokers: bool) -> HandType {
        let mut map: HashMap<char, i32> = HashMap::new();

        let mut num_wilds = 0;
        for card in self.cards {
            if card == 'J' && allow_jokers {
                num_wilds = num_wilds + 1;
                continue;
            }
            match map.get(&card) {
                None => map.insert(card, 1),
                Some(i) => map.insert(card, i + 1),
            };
        }
        let mut values: Vec<&i32> = map.values().collect();
        values.sort_by(|a, b| b.partial_cmp(a).unwrap());
        if num_wilds == 5 {
            return HandType::FiveOfAKind;
        }
        let new_highest = values[0] + num_wilds;
        values[0] = &new_highest;

        if *values[0] == 5 {
            return HandType::FiveOfAKind;
        } else if *values[0] == 4 {
            return HandType::FourOfAKind;
        } else if *values[0] == 3 && *values[1] == 2 {
            return HandType::FullHouse;
        } else if *values[0] == 3 {
            return HandType::ThreeOfAKind;
        } else if *values[0] == 2 && *values[1] == 2 {
            return HandType::TwoPair;
        } else if *values[0] == 2 {
            return HandType::OnePair;
        } else {
            return HandType::HighCard;
        }
    }

    fn score_card(input: &char) -> i32 {
        return Self::score_card_private(&input, false);
    }

    fn score_card_with_jokers(input: &char) -> i32 {
        return Self::score_card_private(&input, true);
    }

    fn score_card_private(input: &char, allow_jokers: bool) -> i32 {
        if input == &'J' && allow_jokers {
            return 1;
        }
        match input {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => 0,
        }
    }
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    let order = b.score_hand().cmp(&a.score_hand());
    if order != Ordering::Equal {
        return order;
    }
    for i in 0..5 {
        let char_order = Hand::score_card(&a.cards[i]).cmp(&Hand::score_card(&b.cards[i]));
        if char_order != Ordering::Equal {
            return char_order;
        }
    }
    panic!("unable to compare");
}

fn compare_hands_with_jokers(a: &Hand, b: &Hand) -> Ordering {
    let order = b.score_hand_with_jokers().cmp(&a.score_hand_with_jokers());
    if order != Ordering::Equal {
        return order;
    }
    for i in 0..5 {
        let char_order = Hand::score_card_with_jokers(&a.cards[i])
            .cmp(&Hand::score_card_with_jokers(&b.cards[i]));
        if char_order != Ordering::Equal {
            return char_order;
        }
    }
    panic!("unable to compare");
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn test_hands() -> Vec<Hand> {
        let input = &utils::read_file("day7", "test.txt");
        return input.lines().map(Hand::parse).collect();
    }

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 6440);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test.txt");

        assert_eq!(actual, 5905);
    }

    #[test]
    fn hand_parse_works() {
        let hand = Hand::parse("32T3K 765");

        assert_eq!(hand.bid, 765);
        assert_eq!(hand.cards[0], '3');
        assert_eq!(hand.cards[1], '2');
        assert_eq!(hand.cards[2], 'T');
        assert_eq!(hand.cards[3], '3');
        assert_eq!(hand.cards[4], 'K');
    }

    #[rstest]
    #[case("AAAAA", HandType::FiveOfAKind)]
    #[case("AA8AA", HandType::FourOfAKind)]
    #[case("23332", HandType::FullHouse)]
    #[case("TTT98", HandType::ThreeOfAKind)]
    #[case("23432", HandType::TwoPair)]
    #[case("A23A4", HandType::OnePair)]
    #[case("23456", HandType::HighCard)]
    fn score_hand_tests(#[case] input: &str, #[case] expected: HandType) {
        let hand = Hand::parse(&format!("{} 0", input));
        let actual = hand.score_hand();

        assert_eq!(actual, expected);
    }

    #[test]
    fn sort_hands() {
        let mut hands = test_hands();

        hands.sort_by(compare_hands);

        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].bid, 220);
        assert_eq!(hands[2].bid, 28);
        assert_eq!(hands[3].bid, 684);
        assert_eq!(hands[4].bid, 483);
    }
}
