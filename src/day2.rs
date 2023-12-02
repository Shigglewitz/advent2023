use rstest::rstest;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn part1(file_name: &str) -> i32 {
    return sum_possible_games(file_name.to_string());
}

pub fn part2(file_name: &str) -> i32 {
    return sum_game_powers(file_name.to_string());
}

struct Observation {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    observations: Vec<Observation>,
}

#[test]
fn part1_works() {
    let sum = part1("test.txt");

    assert_eq!(8, sum);
}

fn sum_game_powers(file_name: String) -> i32 {
    let input = read_file(file_name);
    return input
        .lines()
        .map(|line| line.to_string())
        .map(parse_game)
        .map(find_game_power)
        .sum();
}

#[test]
fn sum_game_powers_works() {
    let actual = sum_game_powers("test.txt".to_string());

    assert_eq!(2286, actual);
}

fn find_game_power(game: Game) -> i32 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for obs in &game.observations {
        if obs.red > max_red {
            max_red = obs.red;
        }
        if obs.green > max_green {
            max_green = obs.green;
        }
        if obs.blue > max_blue {
            max_blue = obs.blue;
        }
    }

    return max_red * max_green * max_blue;
}

#[rstest]
#[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
#[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
#[case(
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    1560
)]
#[case(
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    630
)]
#[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
fn find_game_power_works(#[case] input: &str, #[case] expected: i32) {
    let game = parse_game(input.to_string());

    let actual = find_game_power(game);

    assert_eq!(actual, expected);
}

fn sum_possible_games(file_name: String) -> i32 {
    let input = read_file(file_name);
    let example = Observation {
        red: 12,
        green: 13,
        blue: 14,
    };
    return input
        .lines()
        .map(|line| line.to_string())
        .map(parse_game)
        .filter(|game| is_possible(game, &example))
        .map(|game| game.id)
        .sum();
}

fn read_file(file_name: String) -> String {
    let path = format!("data/day2/{file_name}");
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    return contents;
}

fn is_possible(round: &Game, example: &Observation) -> bool {
    let mut possible = true;
    round.observations.iter().for_each(|obs| {
        if obs.red > example.red {
            possible = false;
        } else if obs.green > example.green {
            possible = false;
        } else if obs.blue > example.blue {
            possible = false;
        }
    });
    return possible;
}

#[rstest]
#[case("Game 1: 1 red, 2 blue, 3 green", true)]
#[case("Game 1: 1 red", true)]
#[case("Game 1: 12 red, 14 blue, 13 green", true)]
#[case("Game 1: 13 red", false)]
#[case("Game 1: 14 green", false)]
#[case("Game 1: 15 blue", false)]
fn is_possible_tests(#[case] input: &str, #[case] possible: bool) {
    let standard = Observation {
        red: 12,
        green: 13,
        blue: 14,
    };
    let game = parse_game(input.to_string());

    let actual = is_possible(&game, &standard);

    assert_eq!(actual, possible);
}

fn parse_game(input: String) -> Game {
    let mut colon_split = input.split(":");
    let id = colon_split
        .next()
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let observations = colon_split
        .next()
        .unwrap()
        .split(";")
        .map(|obs| parse_observation(obs.to_string()))
        .collect();
    return Game {
        id: id,
        observations: observations,
    };
}

#[test]
fn parse_game_sparse() {
    let input = "Game 1: 1 red, 2 green, 3 blue".to_string();

    let actual = parse_game(input);

    assert_eq!(actual.id, 1);
    assert_eq!(actual.observations.len(), 1);
    let first_obs = &actual.observations[0];
    assert_eq!(first_obs.red, 1);
    assert_eq!(first_obs.green, 2);
    assert_eq!(first_obs.blue, 3);
}

#[test]
fn parse_game_full() {
    let input = "Game 2: 3 red; 1 red, 2 blue; 4 red, 2 blue, 45 green".to_string();

    let actual = parse_game(input);

    assert_eq!(actual.id, 2);
    assert_eq!(actual.observations.len(), 3);
    let first_obs = &actual.observations[0];
    assert_eq!(first_obs.red, 3);
    assert_eq!(first_obs.green, 0);
    assert_eq!(first_obs.blue, 0);
    let second_obs = &actual.observations[1];
    assert_eq!(second_obs.red, 1);
    assert_eq!(second_obs.green, 0);
    assert_eq!(second_obs.blue, 2);
    let third_obs = &actual.observations[2];
    assert_eq!(third_obs.red, 4);
    assert_eq!(third_obs.green, 45);
    assert_eq!(third_obs.blue, 2);
}

fn parse_observation(input: String) -> Observation {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    input.split(",").for_each(|word| {
        let mut words = word.trim().split(" ");

        let num = words.next().unwrap().parse::<i32>().unwrap();
        let color = words.next().unwrap();
        if color == "red" {
            red = num
        } else if color == "green" {
            green = num
        } else {
            blue = num
        }
    });
    return Observation {
        red: red,
        green: green,
        blue: blue,
    };
}

#[test]
fn parse_observation_works_sparse() {
    let input = "3 red".to_string();

    let actual = parse_observation(input);

    assert_eq!(actual.red, 3);
    assert_eq!(actual.green, 0);
    assert_eq!(actual.blue, 0);
}

#[test]
fn parse_observation_works_full() {
    let input = "3 red, 2 blue, 1 green".to_string();

    let actual = parse_observation(input);

    assert_eq!(actual.red, 3);
    assert_eq!(actual.green, 1);
    assert_eq!(actual.blue, 2);
}
