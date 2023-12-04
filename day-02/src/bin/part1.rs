use std::{str::FromStr, collections::HashMap};

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Game {
    id: u32,
    rounds: Vec<Set>,
}

//struct Set (HashMap<Colour,u32>);

struct Set {
    set: HashMap<Colour,u32>
}

// struct Cube {
//     colour: Colour,
//     amount: u32,
// }

#[derive(PartialEq, Eq, Hash,Debug)]
enum Colour {Red,Green,Blue}


impl FromStr for Game {

    type Err = ();

    fn from_str(raw_game: &str) -> Result<Self, Self::Err> {
        let (game, data) = raw_game.split_once(": ").unwrap();

        let id = game.chars().filter_map( |char| char.to_digit(10)).fold(0, |acc, x| acc * 10 + x);

        let rounds: Vec<Set> = data.split("; ").map( |set| Set::from_str(set).unwrap()).collect();

        Ok(Self {id, rounds})
    }
}

impl FromStr for Set {

    type Err = ();

    fn from_str(raw_set: &str) -> Result<Self, Self::Err> {

        let data = raw_set.split(", ").map( |cube| {
            let (amount, colour) = cube.split_once( " ").unwrap(); 
            return (Colour::from_str(colour).unwrap(), amount.parse::<u32>().unwrap())}
        ).collect();

        Ok(Self { set: data })
    }
}

impl FromStr for Colour {

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "red"  => Ok(Colour::Red),
            "green"  => Ok(Colour::Green),
            "blue"  => Ok(Colour::Blue),
            _      => Err(()),
        }
    }
}


fn part1(input: &str) -> u32 {

    // static VALID: [Cube; 3] = [
    //     Cube{colour: Colour::Red, amount: 12},
    //     Cube{colour: Colour::Green, amount: 13},
    //     Cube{colour: Colour::Blue, amount: 14}];
    let valid: HashMap<Colour,u32> = HashMap::from([
        (Colour::Red, 12),
        (Colour::Green, 13),
        (Colour::Blue, 14),
    ]);

    let games: Vec<Game> = input.lines().map( |game| Game::from_str(game).unwrap()).collect();

    return games.iter()
                .filter( |game| game.rounds.iter()
                                                    .all( |set| set.set.get(&Colour::Red) <= valid.get(&Colour::Red)
                                                                   && set.set.get(&Colour::Green) <= valid.get(&Colour::Green)
                                                                   && set.set.get(&Colour::Blue) <= valid.get(&Colour::Blue)
                                                        )
                        ).map(|valid_game| valid_game.id).sum();
    
    
}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn valid_answer(){
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part1(input);

        assert_eq!(result, 8)
    }
}