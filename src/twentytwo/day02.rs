use crate::create_advent_day;

create_advent_day!("2022", "02");

fn part1_with_input(input: &str) -> i32 {
    return input.lines().map(parse_round).map(score_round).sum();
}

fn part2_with_input(input: &str) -> i32 {
    return input.lines().map(parse_new_strategy).map(score_round).sum();
}

#[derive(Debug, PartialEq, Eq)]
enum Shape {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    WIN = 6,
    LOSS = 0,
    TIE = 3,
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    your_choice: Shape,
    opponent_choice: Shape,
}

fn score_round(round: Round) -> i32 {
    let outcome = decide_outcome(&round);

    return round.your_choice as i32 + outcome as i32;
}

fn decide_outcome(round: &Round) -> Outcome {
    match round.your_choice {
        Shape::ROCK => match round.opponent_choice {
            Shape::ROCK => Outcome::TIE,
            Shape::PAPER => Outcome::LOSS,
            Shape::SCISSORS => Outcome::WIN,
        },
        Shape::PAPER => match round.opponent_choice {
            Shape::ROCK => Outcome::WIN,
            Shape::PAPER => Outcome::TIE,
            Shape::SCISSORS => Outcome::LOSS,
        },
        Shape::SCISSORS => match round.opponent_choice {
            Shape::ROCK => Outcome::LOSS,
            Shape::PAPER => Outcome::WIN,
            Shape::SCISSORS => Outcome::TIE,
        },
    }
}

fn parse_new_strategy(line: &str) -> Round {
    let mut space_split = line.split(" ");
    let opponent_choice = match space_split.next().unwrap() {
        "A" => Shape::ROCK,
        "B" => Shape::PAPER,
        "C" => Shape::SCISSORS,
        _ => panic!("Invalid input"),
    };
    let your_choice = match space_split.next().unwrap() {
        // lose
        "X" => match opponent_choice {
            Shape::ROCK => Shape::SCISSORS,
            Shape::PAPER => Shape::ROCK,
            Shape::SCISSORS => Shape::PAPER,
        },
        // tie
        "Y" => match opponent_choice {
            Shape::ROCK => Shape::ROCK,
            Shape::PAPER => Shape::PAPER,
            Shape::SCISSORS => Shape::SCISSORS,
        },
        // win
        "Z" => match opponent_choice {
            Shape::ROCK => Shape::PAPER,
            Shape::PAPER => Shape::SCISSORS,
            Shape::SCISSORS => Shape::ROCK,
        },
        _ => panic!("Invalid input"),
    };

    return Round {
        your_choice: your_choice,
        opponent_choice: opponent_choice,
    };
}

fn parse_round(line: &str) -> Round {
    let mut space_split = line.split(" ");
    let opponent_choice = match space_split.next().unwrap() {
        "A" => Shape::ROCK,
        "B" => Shape::PAPER,
        "C" => Shape::SCISSORS,
        _ => panic!("Invalid input"),
    };
    let your_choice = match space_split.next().unwrap() {
        "X" => Shape::ROCK,
        "Y" => Shape::PAPER,
        "Z" => Shape::SCISSORS,
        _ => panic!("Invalid input"),
    };

    return Round {
        your_choice: your_choice,
        opponent_choice: opponent_choice,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "15");
    }

    #[test]
    fn part2_works() {
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "12");
    }

    #[rstest]
    #[case("A Y", 8)]
    #[case("B X", 1)]
    #[case("C Z", 6)]
    fn score_round_tests(#[case] input: &str, #[case] expected: i32) {
        let round = parse_round(input);
        let actual = score_round(round);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("A X", Outcome::TIE)]
    #[case("B Y", Outcome::TIE)]
    #[case("C Z", Outcome::TIE)]
    #[case("A Y", Outcome::WIN)]
    #[case("B Z", Outcome::WIN)]
    #[case("C X", Outcome::WIN)]
    #[case("A Z", Outcome::LOSS)]
    #[case("B X", Outcome::LOSS)]
    #[case("C Y", Outcome::LOSS)]
    fn decide_outcome_tests(#[case] input: &str, #[case] expected: Outcome) {
        let round = parse_round(input);
        let actual = decide_outcome(&round);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("A Y", Round { your_choice: Shape::ROCK, opponent_choice: Shape::ROCK})]
    #[case("B X", Round { your_choice: Shape::ROCK, opponent_choice: Shape::PAPER})]
    #[case("C Z", Round { your_choice: Shape::ROCK, opponent_choice: Shape::SCISSORS})]
    fn parse_new_strategy_tests(#[case] input: &str, #[case] expected: Round) {
        let actual = parse_new_strategy(input);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("C X", Round { your_choice: Shape::ROCK, opponent_choice: Shape::SCISSORS})]
    #[case("B Y", Round { your_choice: Shape::PAPER, opponent_choice: Shape::PAPER})]
    #[case("A Z", Round { your_choice: Shape::SCISSORS, opponent_choice: Shape::ROCK})]
    fn parse_round_tests(#[case] input: &str, #[case] expected: Round) {
        let actual = parse_round(input);

        assert_eq!(actual, expected);
    }
}
