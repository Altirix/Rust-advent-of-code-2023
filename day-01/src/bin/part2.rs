fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    static WORDS: [(&str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let result: u32 = input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = (0..line.len())
                .filter_map(|index| {
                    let sub_line = &line[index..];

                    return WORDS.iter().find_map(|word| {
                        if sub_line.starts_with(word.0) {
                            return Some(word.1);
                        } else {
                            return sub_line.chars().next().unwrap().to_digit(10);
                        }
                    });
                })
                .collect();

            let line_result: u32 =
                digits.first().unwrap_or_else(|| &0) * 10 + digits.last().unwrap_or_else(|| &0);

            return line_result;
        })
        .sum::<u32>();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_answer() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let result = part2(input);

        assert_eq!(result, 281)
    }
}
