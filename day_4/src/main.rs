use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Points: {}", points(INPUT));
    println!("Card Copies: {}", copies(INPUT));
}

fn points(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_tag, numbers) = line.split_once(':').expect("line has a ':'");
            let (winning_nums, own_nums) = numbers.split_once('|').expect("numbers have '|'");
            // TODO optimize allocaion away
            let winning_nums = winning_nums
                .split_whitespace()
                .map(str::parse::<u32>)
                .collect::<Result<HashSet<_>, _>>()
                .expect("valid winning numbers");

            let win_count = own_nums
                .split_whitespace()
                .filter(|own_num| {
                    winning_nums.contains(&own_num.parse().expect("valid own number"))
                })
                .count() as u32;

            win_count
                .checked_sub(1)
                .map_or(0, |win_count| 2u32.pow(win_count))
        })
        .sum()
}

fn copies(input: &str) -> u32 {
    let mut card_count = 0;
    let mut earned_copies = VecDeque::new();
    for  line in input.lines() {
        let (_tag, numbers) = line.split_once(':').expect("line has a ':'");
        let (winning_nums, own_nums) = numbers.split_once('|').expect("numbers have '|'");

        // original card + copies you earned
        let cur_earned_copies = 1 + earned_copies.pop_front().unwrap_or(0);
        card_count += cur_earned_copies;

        // TODO optimize allocaion away
        let winning_nums = winning_nums
            .split_whitespace()
            .map(str::parse::<u32>)
            .collect::<Result<HashSet<_>, _>>()
            .expect("valid winning numbers");

        let win_count = own_nums
            .split_whitespace()
            .filter(|own_num| winning_nums.contains(&own_num.parse().expect("valid own number")))
            .count();

        if earned_copies.len() < win_count {
            earned_copies.resize(win_count, 0);
        }
        for card_copies in earned_copies.range_mut(..win_count) {
            *card_copies += cur_earned_copies;
        }
    }

    card_count
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn points() {
        let points = super::points(INPUT);
        assert_eq!(points, 13);
    }

    #[test]
    fn copies() {
        let points = super::copies(INPUT);
        assert_eq!(points, 30);
    }
}
