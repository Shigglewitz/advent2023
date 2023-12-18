use std::{cmp::Ordering, collections::HashMap};

use crate::create_advent_day;

create_advent_day!("07");

fn part1_with_input(input: &str) -> i64 {
    let mut hands: Vec<Hand> = input.lines().map(|line| Hand::parse(line, false)).collect();
    hands.sort_by(|a, b| compare_hands(a, b, false));
    return hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as i64 + 1) * hand.bid)
        .sum();
}

fn part2_with_input(input: &str) -> i64 {
    let mut hands: Vec<Hand> = input.lines().map(|line| Hand::parse(line, true)).collect();
    hands.sort_by(|a, b| compare_hands(a, b, true));
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
    hand_type: HandType,
}

impl Hand {
    fn parse(input: &str, allow_jokers: bool) -> Hand {
        let mut split = input.split(" ");
        let mut cards = [' '; 5];
        for (i, card) in split.next().unwrap().chars().take(5).enumerate() {
            cards[i] = card;
        }
        let bid = split.next().unwrap().parse::<i64>().unwrap();
        return Hand {
            cards,
            bid,
            hand_type: Hand::score_hand(cards, allow_jokers),
        };
    }

    fn score_hand(cards: [char; 5], allow_jokers: bool) -> HandType {
        let mut map: HashMap<char, i32> = HashMap::new();

        let mut num_wilds = 0;
        for card in cards {
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

    fn score_card(input: &char, allow_jokers: bool) -> i32 {
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

fn compare_hands(a: &Hand, b: &Hand, allow_jokers: bool) -> Ordering {
    let order = b.hand_type.cmp(&a.hand_type);
    if order != Ordering::Equal {
        return order;
    }
    for i in 0..5 {
        let char_order = Hand::score_card(&a.cards[i], allow_jokers)
            .cmp(&Hand::score_card(&b.cards[i], allow_jokers));
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
        let input = &utils::read_file("day07", "test.txt");
        return input.lines().map(|line| Hand::parse(line, false)).collect();
    }

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!(&actual, "6440");
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(&actual, "5905");
    }

    #[test]
    fn hand_parse_works() {
        let hand = Hand::parse("32T3K 765", false);

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
        let hand = Hand::parse(&format!("{} 0", input), false);
        let actual = hand.hand_type;

        assert_eq!(actual, expected);
    }

    #[test]
    fn sort_hands() {
        let mut hands = test_hands();

        hands.sort_by(|a, b| compare_hands(a, b, false));

        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].bid, 220);
        assert_eq!(hands[2].bid, 28);
        assert_eq!(hands[3].bid, 684);
        assert_eq!(hands[4].bid, 483);
    }
}
