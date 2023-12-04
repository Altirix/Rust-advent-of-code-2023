use std::{collections::BTreeMap, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Game {
    id: u32,
    rounds: Vec<BTreeMap<Colour, u32>>,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Colour {
    Red,
    Green,
    Blue,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(raw_game: &str) -> Result<Self, Self::Err> {
        let (game, data) = raw_game.split_once(": ").unwrap();

        let id = game
            .chars()
            .filter_map(|char| char.to_digit(10))
            .fold(0, |acc, x| acc * 10 + x);

        let rounds: Vec<BTreeMap<Colour, u32>> = data.split("; ").map(|set| get_set(set)).collect();

        Ok(Self { id, rounds })
    }
}
// should be with the from str impl, but idk how to do that
fn get_set(raw_set: &str) -> BTreeMap<Colour, u32> {
    return raw_set.split(", ").map(|cube| get_cube(cube)).collect();
}

fn get_cube(raw_cube: &str) -> (Colour, u32) {
    let (amount, colour) = raw_cube.split_once(" ").unwrap();

    return (
        Colour::from_str(colour).unwrap(),
        amount.parse::<u32>().unwrap(),
    );
}

impl FromStr for Colour {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "red" => Ok(Colour::Red),
            "green" => Ok(Colour::Green),
            "blue" => Ok(Colour::Blue),
            _ => Err(()),
        }
    }
}

fn part1(input: &str) -> u32 {

    let valid: BTreeMap<Colour, u32> =
        BTreeMap::from([(Colour::Red, 12), (Colour::Green, 13), (Colour::Blue, 14)]);

    let games: Vec<Game> = input
        .lines()
        .map(|game| Game::from_str(game).unwrap())
        .collect();

    return games
        .iter()
        .filter(|game| {
            game.rounds.iter().all(|set| {
                set.get(&Colour::Red) <= valid.get(&Colour::Red)
                    && set.get(&Colour::Green) <= valid.get(&Colour::Green)
                    && set.get(&Colour::Blue) <= valid.get(&Colour::Blue)
            })
        })
        .map(|valid_game| valid_game.id)
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_answer() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part1(input);

        assert_eq!(result, 8)
    }
}
