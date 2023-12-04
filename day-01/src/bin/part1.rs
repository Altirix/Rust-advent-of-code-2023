fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let result: u32 = input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line
                .chars()
                .filter_map(|_char| _char.to_digit(10))
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = part1(input);

        assert_eq!(result, 142)
    }
}
