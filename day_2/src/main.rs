use std::collections::HashMap;

fn main() {
    const GAMES: &str = include_str!("input.txt");

    let constraints = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    let result = sum_possible_games(GAMES, &constraints);
    println!("Result: {result}");

    let result = sum_pow_of_min_cubes(GAMES);
    println!("Sum of power of minimum possible color-cubes: {result}");
}

type CubeSet = HashMap<Color, u32>;

#[derive(PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

#[allow(clippy::needless_lifetimes)]
fn parse_games<'s>(
    games: &'s str,
) -> impl Iterator<
    Item = (
        u32,
        impl Iterator<Item = impl Iterator<Item = (u32, Color)> + 's> + 's,
    ),
> + 's {
    games.lines().map(|line| {
        let (tag, sets) = line.split_once(':').expect("line has a ':' delimited tag");

        let (tag_game, tag_id) = tag.trim().split_once(' ').expect("tag has a space");
        assert_eq!(tag_game, "Game");

        let id = tag_id.parse::<u32>().expect("tag_id is a number");

        let sets = sets.split(';').map(|set| {
            set.split(',').map(|group| {
                let (amount, color) = group
                    .trim()
                    .split_once(' ')
                    .expect("group is space delimited");
                let amount = amount.parse::<u32>().expect("amount is a number");
                let color = match color {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    color => panic!("unknown color: '{color}'"),
                };
                (amount, color)
            })
        });

        (id, sets)
    })
}

fn sum_possible_games(games: &str, constraints: &CubeSet) -> u32 {
    parse_games(games)
        .filter_map(|(id, sets)| {
            for (amount, color) in sets.flatten() {
                if amount > constraints.get(&color).copied().unwrap_or_default() {
                    return None;
                }
            }
            Some(id)
        })
        .sum()
}

fn sum_pow_of_min_cubes(games: &str) -> u32 {
    parse_games(games)
        .map(|(_, sets)| {
            let mut min_cubes = CubeSet::new();
            for (amount, color) in sets.flatten() {
                min_cubes
                    .entry(color)
                    .and_modify(|prev_min| {
                        if amount > *prev_min {
                            *prev_min = amount;
                        }
                    })
                    .or_insert(amount);
            }
            min_cubes.values().product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAMES: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_possible_games() {
        let constraints = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
        let result = sum_possible_games(GAMES, &constraints);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_min_cubes() {
        let result = sum_pow_of_min_cubes(GAMES);

        assert_eq!(result, 2286);
    }
}
