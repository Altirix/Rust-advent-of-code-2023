use std::{collections::BTreeMap, str::FromStr, char, ops::Index};

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {

    let grid: Vec<char> = input
    .lines().flat_map( | line| line.chars() ).collect();


   let parts = grid.iter().enumerate().filter_map( | (n , char)|)

    dbg!(grid);
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_answer() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let result = part1(input);

        assert_eq!(result, 4361)
    }
}
