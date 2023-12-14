const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Load: {}", load(INPUT));
    println!("Load 1m cycles: {}", load_cycles(INPUT));
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

const CYCLE_COUNT: usize = 1_000_000_000;
fn load_cycles(input: &str) -> usize {
    let og_input = input
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

    let mut cycles = vec![og_input];

    for i in 0..CYCLE_COUNT {
        // eprintln!("cycle {i}");
        let mut input = cycles.last().unwrap().clone();
        cycle(&mut input);

        if let Some(j) = cycles
            .iter()
            .enumerate()
            .find_map(|(i, cycle)| (cycle == &input).then_some(i))
        {
            let period = i - (j - 1);
            let remainder = (CYCLE_COUNT - j) % period;
            // eprintln!("period = {i} - {j} = {period}; remainder = {remainder}");
            let final_arrangement = &cycles[j + remainder];
            // eprintln!("final = {}", j + remainder);

            // for row in final_arrangement {
            //     for p in row {
            //         match p {
            //             Pixel::Round => eprint!("O"),
            //             Pixel::Cube => eprint!("#"),
            //             Pixel::Empty => eprint!("."),
            //         }
            //     }
            //     eprintln!()
            // }
            // eprintln!();

            let input_len = final_arrangement.len();
            return final_arrangement
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .map(|p| if *p == Pixel::Round { input_len - y } else { 0 })
                        .sum::<usize>()
                })
                .sum();
        } else {
            cycles.push(input);
        }
    }
    assert!(cycles.last().unwrap() == &cycles[114]);
    panic!("no periodicity")
}

fn cycle(input: &mut [Vec<Pixel>]) {
    // north
    for x in 0..input[0].len() {
        let mut empty_span = 0;
        for y in 0..input.len() {
            match input[y][x] {
                Pixel::Round => {
                    input[y][x] = Pixel::Empty;
                    input[y - empty_span][x] = Pixel::Round;
                }
                Pixel::Cube => empty_span = 0,
                Pixel::Empty => empty_span += 1,
            }
        }
    }

    // west
    for row in input.iter_mut() {
        let mut empty_span = 0;
        for x in 0..row.len() {
            match row[x] {
                Pixel::Round => {
                    row[x] = Pixel::Empty;
                    row[x - empty_span] = Pixel::Round;
                }
                Pixel::Cube => empty_span = 0,
                Pixel::Empty => empty_span += 1,
            }
        }
    }

    // south
    for x in 0..input[0].len() {
        let mut empty_span = 0;
        for y in (0..input.len()).rev() {
            match input[y][x] {
                Pixel::Round => {
                    input[y][x] = Pixel::Empty;
                    input[y + empty_span][x] = Pixel::Round;
                }
                Pixel::Cube => empty_span = 0,
                Pixel::Empty => empty_span += 1,
            }
        }
    }

    // east
    for row in input.iter_mut() {
        let mut empty_span = 0;
        for x in (0..row.len()).rev() {
            match row[x] {
                Pixel::Round => {
                    row[x] = Pixel::Empty;
                    row[x + empty_span] = Pixel::Round;
                }
                Pixel::Cube => empty_span = 0,
                Pixel::Empty => empty_span += 1,
            }
        }
    }
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

    #[test]
    fn load_cycles() {
        let load = super::load_cycles(INPUT);
        assert_eq!(load, 64);
    }
}
