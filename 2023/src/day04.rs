use crate::create_advent_day;

create_advent_day!("04");

fn part1_with_input(input: &str) -> i32 {
    return input
        .lines()
        .map(Card::from)
        .map(|card| card.power_score())
        .sum();
}

fn part2_with_input(input: &str) -> i32 {
    let mut cards: Vec<Card> = input.lines().map(Card::from).collect();

    let num_cards = cards.len();
    for card_index in 0..num_cards {
        let score = cards[card_index].score();
        let count_of_this_card = cards[card_index].num_copies;
        for i in 0..score {
            cards[card_index + i as usize + 1].increment_copies(count_of_this_card);
        }
    }

    return cards.iter().map(|card| card.num_copies).sum();
}

struct Card {
    num_copies: i32,
    winning_numbers: Vec<i32>,
    your_numbers: Vec<i32>,
}

impl Card {
    fn from(input: &str) -> Card {
        let mut colon_split = input.split(":");
        let mut line_split = colon_split.nth(1).unwrap().split("|");
        let winning = line_split.nth(0).unwrap();
        let yours = line_split.nth(0).unwrap();

        let winning_numbers: Vec<i32> = winning
            .split(" ")
            .filter(|value| !value.is_empty())
            .map(|value| value.parse::<i32>().unwrap())
            .collect();

        let your_numbers: Vec<i32> = yours
            .split(" ")
            .filter(|value| !value.is_empty())
            .map(|value| value.parse::<i32>().unwrap())
            .collect();

        return Card {
            num_copies: 1,
            winning_numbers: winning_numbers,
            your_numbers: your_numbers,
        };
    }

    fn score(&self) -> i32 {
        let mut num_matches = 0;
        for num in &self.your_numbers {
            if self.winning_numbers.contains(&num) {
                num_matches = num_matches + 1;
            }
        }

        return num_matches;
    }

    fn increment_copies(&mut self, value: i32) {
        self.num_copies = self.num_copies + value;
    }

    fn power_score(&self) -> i32 {
        let num_matches = self.score();

        if num_matches == 0 {
            return 0;
        } else {
            return (2 as i32).pow(num_matches as u32 - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!(&actual, "13");
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(&actual, "30");
    }

    #[test]
    fn card_from_tests() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let actual = Card::from(input);

        assert_eq!(actual.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(actual.your_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53])
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn power_score_tests(#[case] input: &str, #[case] expected: i32) {
        let card = Card::from(input);
        let actual = card.power_score();

        assert_eq!(actual, expected);
    }
}
