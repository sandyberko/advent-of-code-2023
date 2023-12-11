const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Shortest Paths: {}", shortest_paths(INPUT, 1));
    // 10154062
    println!("Shortest Paths x1M: {}", shortest_paths(INPUT, 999_999));
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    y: usize,
    x: usize,
}

fn shortest_paths(image: &str, expansion: usize) -> usize {
    let mut populated_y = Vec::new();
    let mut populated_x = Vec::new();

    let mut galaxies = Vec::new();
    for (y, line) in image.lines().enumerate() {
        for (x, c) in line.char_indices() {
            match c {
                '#' => {
                    if y + 1 > populated_y.len() {
                        populated_y.resize(y + 1, false);
                    }
                    populated_y[y] = true;

                    if x + 1 > populated_x.len() {
                        populated_x.resize(x + 1, false);
                    }
                    populated_x[x] = true;

                    galaxies.push(Point {
                        y: y as _,
                        x: x as _,
                    });
                }
                '.' => (),
                c => panic!("{c}"),
            }
        }
    }

    let mut y_offset = 0;
    let y_offsets = populated_y
        .into_iter()
        .enumerate()
        .map(|(_, is_populated)| {
            if !is_populated {
                y_offset += 1;
            }
            y_offset
        })
        .collect::<Vec<_>>();

    let mut x_offset = 0;
    let x_offsets = populated_x
        .into_iter()
        .enumerate()
        .map(|(_, is_populated)| {
            if !is_populated {
                x_offset += 1;
            }
            x_offset
        })
        .collect::<Vec<_>>();

    let galaxies = galaxies
        .into_iter()
        .map(|Point { y, x }| Point {
            y: y + (y_offsets.get(y).copied().unwrap_or_default() * expansion),
            x: x + (x_offsets.get(x).copied().unwrap_or_default() * expansion),
        })
        .collect::<Vec<_>>();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, galaxy_0)| {
            galaxies[i + 1..]
                .iter()
                .map(|galaxy_1| galaxy_0.y.abs_diff(galaxy_1.y) + galaxy_0.x.abs_diff(galaxy_1.x))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn shortest_paths() {
        let image = concat! {
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....\n",
        };

        assert_eq!(super::shortest_paths(image, 1), 374);
        assert_eq!(super::shortest_paths(image, 9), 1030);
        assert_eq!(super::shortest_paths(image, 99), 8410);
    }
}
