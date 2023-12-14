const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Reflections: {}", reflection(INPUT));
    println!("Smudged: {}", reflection_smudged(INPUT));
}

#[derive(PartialEq, Eq)]
enum Pixel {
    Ash,
    Rocks,
}

fn reflection(patterns: &str) -> usize {
    patterns
        .split("\n\n")
        .enumerate()
        .map(|(pattern_i, pattern)| {
            let pattern = pattern
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Pixel::Ash,
                            '#' => Pixel::Rocks,
                            c => panic!("{c:?}"),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            // horizontal
            for reflected_y in 1..pattern.len() {
                // eprintln!("does {reflected_y} reflect?");
                if (0..reflected_y)
                    .rev()
                    .zip(reflected_y..pattern.len())
                    .all(|(y0, y1)| {
                        // eprintln!("checking {y0} == {y1}");
                        pattern[y0] == pattern[y1]
                    })
                {
                    // eprintln!("horizontal reflection after {}!", reflected_y);
                    return reflected_y * 100;
                }
            }

            // vertical
            for reflected_x in 1..pattern[0].len() {
                if (0..reflected_x)
                    .rev()
                    .zip(reflected_x..pattern[0].len())
                    .all(|(y0, y1)| pattern.iter().all(|row| row[y0] == row[y1]))
                {
                    return reflected_x;
                }
            }

            panic!("pattern {pattern_i} has no reflection!")
        })
        .sum()
}

fn reflection_smudged(patterns: &str) -> usize {
    patterns
        .split("\n\n")
        .enumerate()
        .map(|(pattern_i, pattern)| {
            let pattern = pattern
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Pixel::Ash,
                            '#' => Pixel::Rocks,
                            c => panic!("{c:?}"),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            // horizontal
            for reflected_y in 1..pattern.len() {
                let mut smudge_found = false;
                // eprintln!("does {reflected_y} reflect?");
                if (0..reflected_y)
                    .rev()
                    .zip(reflected_y..pattern.len())
                    .all(|(y0, y1)| {
                        let difference = pattern[y0]
                            .iter()
                            .zip(&pattern[y1])
                            .filter(|(p0, p1)| p0 != p1)
                            .count();

                        // eprintln!("checking {y0} == {y1}: {difference}; {smudge_found}");

                        if !smudge_found && difference == 1 {
                            smudge_found = true;
                            true
                        } else {
                            difference == 0
                        }
                    })
                    && smudge_found
                {
                    // eprintln!("horizontal reflection after {}!", reflected_y);
                    return reflected_y * 100;
                }
            }

            // vertical
            for reflected_x in 1..pattern[0].len() {
                let mut smudge_found = false;
                if (0..reflected_x)
                    .rev()
                    .zip(reflected_x..pattern[0].len())
                    .all(|(y0, y1)| {
                        let difference = pattern.iter().filter(|row| row[y0] != row[y1]).count();

                        if !smudge_found && difference == 1 {
                            smudge_found = true;
                            true
                        } else {
                            difference == 0
                        }
                    })
                    && smudge_found
                {
                    return reflected_x;
                }
            }

            panic!("pattern {pattern_i} has no reflection!")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const PATTERNS: &str = concat! {
        "#.##..##.\n",
        "..#.##.#.\n",
        "##......#\n",
        "##......#\n",
        "..#.##.#.\n",
        "..##..##.\n",
        "#.#.##.#.\n",
        "\n",
        "#...##..#\n",
        "#....#..#\n",
        "..##..###\n",
        "#####.##.\n",
        "#####.##.\n",
        "..##..###\n",
        "#....#..#\n",
    };

    #[test]
    fn reflection() {
        let result = super::reflection(PATTERNS);
        assert_eq!(result, 405);
    }

    #[test]
    fn reflection_smudged() {
        let result = super::reflection_smudged(PATTERNS);
        assert_eq!(result, 400);
    }
}
