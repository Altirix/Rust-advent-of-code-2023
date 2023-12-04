use std::{collections::BTreeMap, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
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

fn part2(input: &str) -> u32 {
    let games: Vec<Game> = input
        .lines()
        .map(|game| Game::from_str(game).unwrap())
        .collect();

    return games
        .iter()
        .map(|game| {
            game.rounds.iter().fold(BTreeMap::new(), |acc: BTreeMap<Colour, u32>, set| {
                let red: (Colour, u32) = (
                    Colour::Red,
                    *set.get(&Colour::Red)
                        .unwrap_or_else(|| &0)
                        .max(acc.get(&Colour::Red).unwrap_or_else(|| &0)),
                );
                let green: (Colour, u32) = (
                    Colour::Green,
                    *set.get(&Colour::Green)
                        .unwrap_or_else(|| &0)
                        .max(acc.get(&Colour::Green).unwrap_or_else(|| &0)),
                );
                let blue: (Colour, u32) = (
                    Colour::Blue,
                    *set.get(&Colour::Blue)
                        .unwrap_or_else(|| &0)
                        .max(acc.get(&Colour::Blue).unwrap_or_else(|| &0)),
                );
                return BTreeMap::from([red, green, blue]);
            })
        })
        .map(|set_max_cubes| {
            set_max_cubes.get(&Colour::Red).unwrap_or_else(|| &0)
                * set_max_cubes.get(&Colour::Green).unwrap_or_else(|| &0)
                * set_max_cubes.get(&Colour::Blue).unwrap_or_else(|| &0)
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_answer() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let result = part2(input);

        assert_eq!(result, 2285)
    }
}
