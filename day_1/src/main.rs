pub fn first_last_digit(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(char::is_ascii_digit)
                .expect("contains digit");

            let last = line
                .chars()
                .rev()
                .find(char::is_ascii_digit)
                .expect("contains digit");

            [first, last]
                .into_iter()
                .collect::<String>()
                .parse::<u64>()
                .expect("valid number")
        })
        .sum::<u64>()
}

fn extract_spelled(input: &str) -> u32 {
    const SPELLINGS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .lines()
        .map(|line| {
            let (first, ..) = SPELLINGS
                .iter()
                .enumerate()
                .filter_map(|(i, spelling)| {
                    line.find(*spelling).map(|match_i| (i as u32 + 1, match_i))
                })
                .chain(
                    line.char_indices()
                        .find_map(|(match_i, c)| c.to_digit(10).map(|digit| (digit, match_i))),
                )
                .min_by_key(|(_, match_i)| *match_i)
                .expect("has digit");

            let (last, ..) = SPELLINGS
                .iter()
                .enumerate()
                .filter_map(|(i, spelling)| {
                    line.rfind(*spelling).map(|match_i| (i as u32 + 1, match_i))
                })
                .chain(
                    line.char_indices()
                        .rev()
                        .find_map(|(match_i, c)| c.to_digit(10).map(|digit| (digit, match_i))),
                )
                .max_by_key(|(_, match_i)| *match_i)
                .expect("has digit");

            first * 10 + last
        })
        .sum()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let result = first_last_digit(INPUT);
    println!("Result: {result}");

    let result = extract_spelled(INPUT);
    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const INPUT: &str = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let result = first_last_digit(INPUT);
        assert_eq!(result, 142);
    }

    #[test]
    fn spelled_out() {
        const INPUT: &str = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let result = extract_spelled(INPUT);
        assert_eq!(result, 281);
    }
}
