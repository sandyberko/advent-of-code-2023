use std::{array, collections::BTreeMap, mem};

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Total Winnings: {}", total_winnings(INPUT));
    // 250232501
    println!("Total Winnings, Joker: {}", total_winnings_j(INPUT));
    // 249138943
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Joker,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    ty: HandType,
    hand: [Card; 5],
}

fn total_winnings(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let mut hand = hand.chars().map(|c| match c {
                '2' => Card::C2,
                '3' => Card::C3,
                '4' => Card::C4,
                '5' => Card::C5,
                '6' => Card::C6,
                '7' => Card::C7,
                '8' => Card::C8,
                '9' => Card::C9,
                'T' => Card::T,
                'J' => Card::J,
                'Q' => Card::Q,
                'K' => Card::K,
                'A' => Card::A,
                c => panic!("unknown card {c}"),
            });
            let hand = array::from_fn(|_| hand.next().unwrap());
            let mut card_count = [0usize; 14];
            for &card in &hand {
                card_count[card as usize] += 1;
            }
            card_count.sort_unstable();
            let ty = match card_count {
                [.., 5] => HandType::FiveOAK,
                [.., 4] => HandType::FourOAK,
                [.., 2, 3] => HandType::FullHouse,
                [.., 1, 1, 3] => HandType::ThreeOAK,
                [.., 1, 2, 2] => HandType::TwoPair,
                [.., 1, 1, 1, 2] => HandType::OnePair,
                [.., 1, 1, 1, 1, 1] => HandType::HighCard,
                card_count => panic!("unknown card count {card_count:?}"),
            };

            let bid = bid.parse::<usize>().unwrap();
            (Hand { ty, hand }, bid)
        })
        .collect::<BTreeMap<_, _>>()
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum()
}

fn total_winnings_j(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let mut hand = hand.chars().map(|c| match c {
                '2' => Card::C2,
                '3' => Card::C3,
                '4' => Card::C4,
                '5' => Card::C5,
                '6' => Card::C6,
                '7' => Card::C7,
                '8' => Card::C8,
                '9' => Card::C9,
                'T' => Card::T,
                'J' => Card::Joker,
                'Q' => Card::Q,
                'K' => Card::K,
                'A' => Card::A,
                c => panic!("unknown card {c}"),
            });
            let hand = array::from_fn(|_| hand.next().unwrap());
            let mut card_count = [0usize; 14];
            for &card in &hand {
                card_count[card as usize] += 1;
            }
            let joker_count = mem::take(&mut card_count[Card::Joker as usize]);
            *card_count.iter_mut().max().unwrap() += joker_count;
            card_count.sort_unstable();
            let ty = match card_count {
                [.., 5] => HandType::FiveOAK,
                [.., 4] => HandType::FourOAK,
                [.., 2, 3] => HandType::FullHouse,
                [.., 1, 1, 3] => HandType::ThreeOAK,
                [.., 1, 2, 2] => HandType::TwoPair,
                [.., 1, 1, 1, 2] => HandType::OnePair,
                [.., 1, 1, 1, 1, 1] => HandType::HighCard,
                card_count => panic!("unknown card count {card_count:?}"),
            };

            let bid = bid.parse::<usize>().unwrap();
            (Hand { ty, hand }, bid)
        })
        .collect::<BTreeMap<_, _>>()
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn total_winnings() {
        let result = super::total_winnings(INPUT);
        assert_eq!(result, 6440);
    }

    #[test]
    fn total_winnings_j() {
        let result = super::total_winnings_j(INPUT);
        assert_eq!(result, 5905);
    }
}
