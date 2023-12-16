const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Energized: {}", energized(INPUT));
}

enum Pixel {
    Empty,
    MirrorRightward,
    MirrorLeftward,
    SplitterHorizontal,
    SplitterVertical,
}

impl std::fmt::Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::MirrorRightward => write!(f, "/"),
            Self::MirrorLeftward => write!(f, "\\"),
            Self::SplitterHorizontal => write!(f, "-"),
            Self::SplitterVertical => write!(f, "|"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn energized(grid: &str) -> usize {
    let mut grid = grid
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    let p = match c {
                        '.' => Pixel::Empty,
                        '/' => Pixel::MirrorRightward,
                        '\\' => Pixel::MirrorLeftward,
                        '-' => Pixel::SplitterHorizontal,
                        '|' => Pixel::SplitterVertical,
                        c => panic!("invalid pixel {c:?}"),
                    };
                    (p, [false; 4])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    trace_ray(&mut grid, 0, 0, Dir::Right);

    // for line in &grid {
    //     for (p, is_energized) in line {
    //         match p {
    //             Pixel::Empty => {
    //                 if let count @ 2.. = is_energized.iter().filter(|x| **x).count() {
    //                     eprint!("{count}");
    //                 } else if let Some(dir) = is_energized
    //                     .iter()
    //                     .enumerate()
    //                     .find_map(|(i, is_energized)| is_energized.then_some(i))
    //                 {
    //                     match dir {
    //                         0 => eprint!("^"),
    //                         1 => eprint!("v"),
    //                         2 => eprint!("<"),
    //                         3 => eprint!(">"),
    //                         _ => panic!("{dir:?}"),
    //                     }
    //                 } else {
    //                     eprint!(".");
    //                 }
    //             }
    //             p => eprint!("{p:?}"),
    //         }
    //     }
    //     eprintln!()
    // }

    grid.into_iter()
        .flat_map(|line| {
            line.into_iter()
                .filter(|(_, is_energized)| is_energized.contains(&true))
        })
        .count()
}

fn trace_ray(grid: &mut [Vec<(Pixel, [bool; 4])>], mut y: usize, mut x: usize, dir: Dir) {
    while let Some((p, is_energized)) = grid
        .get_mut(y)
        .and_then(|line| line.get_mut(x))
        .and_then(|(p, is_energized)| (!is_energized[dir as usize]).then_some((p, is_energized)))
    {
        is_energized[dir as usize] = true;
        match (dir, p) {
            (_, Pixel::Empty)
            | (Dir::Up | Dir::Down, Pixel::SplitterVertical)
            | (Dir::Left | Dir::Right, Pixel::SplitterHorizontal) => match dir {
                Dir::Up => {
                    if let Some(next_y) = y.checked_sub(1) {
                        y = next_y;
                    } else {
                        break;
                    }
                }
                Dir::Down => y += 1,
                Dir::Left => {
                    if let Some(next_x) = x.checked_sub(1) {
                        x = next_x;
                    } else {
                        break;
                    }
                }
                Dir::Right => x += 1,
            },
            (Dir::Right, Pixel::MirrorRightward) | (Dir::Left, Pixel::MirrorLeftward) => {
                // up
                if let Some(y) = y.checked_sub(1) {
                    trace_ray(grid, y, x, Dir::Up);
                }
            }
            (Dir::Down, Pixel::MirrorLeftward) | (Dir::Up, Pixel::MirrorRightward) => {
                // right
                trace_ray(grid, y, x + 1, Dir::Right);
            }
            (Dir::Right, Pixel::MirrorLeftward) | (Dir::Left, Pixel::MirrorRightward) => {
                // down
                trace_ray(grid, y + 1, x, Dir::Down);
            }
            (Dir::Down, Pixel::MirrorRightward) | (Dir::Up, Pixel::MirrorLeftward) => {
                // left
                if let Some(x) = x.checked_sub(1) {
                    trace_ray(grid, y, x, Dir::Left);
                }
            }
            (Dir::Left | Dir::Right, Pixel::SplitterVertical) => {
                // up
                if let Some(y) = y.checked_sub(1) {
                    trace_ray(grid, y, x, Dir::Up);
                }
                // down
                trace_ray(grid, y + 1, x, Dir::Down);
            }
            (Dir::Up | Dir::Down, Pixel::SplitterHorizontal) => {
                // left
                if let Some(x) = x.checked_sub(1) {
                    trace_ray(grid, y, x, Dir::Left);
                }
                // right
                trace_ray(grid, y, x + 1, Dir::Right);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    const GRID: &str = concat! {
        r".|...\....", "\n",
        r"|.-.\.....", "\n",
        r".....|-...", "\n",
        r"........|.", "\n",
        r"..........", "\n",
        r".........\", "\n",
        r"..../.\\..", "\n",
        r".-.-/..|..", "\n",
        r".|....-|.\", "\n",
        r"..//.|....",
    };

    #[test]
    fn energized() {
        let energized = super::energized(GRID);
        assert_eq!(energized, 46);
    }
}
