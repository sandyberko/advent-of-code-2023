const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Load: {}", load(INPUT));
}

enum Pixel {
    Round,
    Cube,
    Empty,
}

fn load(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Pixel::Round,
                    '#' => Pixel::Cube,
                    '.' => Pixel::Empty,
                    c => panic!("{c:?}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (0..input[0].len())
        .map(|x| {
            let mut empty_span = 0;
            let mut load = 0;
            for (y, row) in input.iter().enumerate() {
                match row[x] {
                    Pixel::Round => load += input.len() - (y - empty_span),
                    Pixel::Cube => empty_span = 0,
                    Pixel::Empty => empty_span += 1,
                }
            }
            load
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat! {
        "O....#....\n",
        "O.OO#....#\n",
        ".....##...\n",
        "OO.#O....O\n",
        ".O.....O#.\n",
        "O.#..O.#.#\n",
        "..O..#O..O\n",
        ".......O..\n",
        "#....###..\n",
        "#OO..#....\n",
    };

    #[test]
    fn load() {
        let load = super::load(INPUT);
        assert_eq!(load, 136);
    }
}
