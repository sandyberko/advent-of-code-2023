const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Farthest: {}", farthest(INPUT));
    println!("Enclosed: {}", enclosed(INPUT));
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn opposite(self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }
}

enum Tile {
    Pipe([Dir; 2], bool),
    Start,
    Ground,
}

#[derive(Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn to(self, dir: Dir) -> Self {
        match dir {
            Dir::North => Point::new(self.x, self.y.saturating_sub(1)),
            Dir::East => Point::new(self.x + 1, self.y),
            Dir::South => Point::new(self.x, self.y + 1),
            Dir::West => Point::new(self.x.saturating_sub(1), self.y),
        }
    }
}

fn farthest(map: &str) -> u32 {
    let (mut map, starting_point) = parse_map(map);
    let mut cur = [Dir::North, Dir::East, Dir::South, Dir::West]
        .into_iter()
        .find_map(|dir| walk(&mut map, starting_point, dir))
        .unwrap();
    let mut distance = 1;

    while let Some(next) = walk(&mut map, cur.0, cur.1) {
        cur = next;
        distance += 1;
    }
    (distance + 1) / 2
}

fn enclosed(map: &str) -> u32 {
    let (mut map, starting_point) = parse_map(map);
    let mut cur = [Dir::North, Dir::East, Dir::South, Dir::West]
        .into_iter()
        .find_map(|dir| walk(&mut map, starting_point, dir))
        .unwrap();
    while let Some(next) = walk(&mut map, cur.0, cur.1) {
        cur = next;
    }

    // eprintln!("----------------");
    let mut enclosed = 0;
    for row in map {
        let mut is_inside_loop = false;
        for tile in row {
            match tile {
                Tile::Start => {
                    is_inside_loop = !is_inside_loop;
                    // if is_inside_loop {
                    //     eprint!("|");
                    // } else {
                    //     eprint!(":")
                    // }
                }
                Tile::Pipe(cons, true) => {
                    if cons.contains(&Dir::South) {
                        is_inside_loop = !is_inside_loop;
                    }
                    // let c = match cons {
                    //     [Dir::East, Dir::North] | [Dir::North, Dir::East] => 'L',
                    //     [Dir::South, Dir::North] | [Dir::North, Dir::South] => '|',
                    //     [Dir::West, Dir::North] | [Dir::North, Dir::West] => 'J',
                    //     [Dir::South, Dir::East] | [Dir::East, Dir::South] => 'F',
                    //     [Dir::West, Dir::East] | [Dir::East, Dir::West] => '-',
                    //     [Dir::West, Dir::South] | [Dir::South, Dir::West] => '7',
                    //     cons => panic!("{cons:?}"),
                    // };
                    // eprint!("{c}");
                }
                Tile::Pipe(_, false) | Tile::Ground => {
                    if is_inside_loop {
                        enclosed += 1;
                        // eprint!("I");
                    }
                    // else {
                    //     eprint!("O");
                    // }
                }
            }
        }
        // eprintln!();
    }
    enclosed
}

fn parse_map(map: &str) -> (Vec<Vec<Tile>>, Point) {
    let mut starting_point: Option<Point> = None;

    let map = map
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, c)| match c {
                    '|' => Tile::Pipe([Dir::North, Dir::South], false),
                    '-' => Tile::Pipe([Dir::West, Dir::East], false),
                    'L' => Tile::Pipe([Dir::North, Dir::East], false),
                    'J' => Tile::Pipe([Dir::North, Dir::West], false),
                    '7' => Tile::Pipe([Dir::West, Dir::South], false),
                    'F' => Tile::Pipe([Dir::East, Dir::South], false),
                    '.' => Tile::Ground,
                    'S' => {
                        starting_point = Some(Point::new(x as _, y as _));
                        Tile::Start
                    }
                    c => panic!("invalid pixel {c:?}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (map, starting_point.unwrap())
}

fn walk(map: &mut [Vec<Tile>], point: Point, dir: Dir) -> Option<(Point, Dir)> {
    let point = point.to(dir);
    let tile = map.get_mut(point.y as usize)?.get_mut(point.x as usize)?;
    let from_dir = dir.opposite();
    let to_dir = match tile {
        Tile::Pipe(cons, is_main_loop) => {
            if cons[0] == from_dir {
                *is_main_loop = true;
                cons[1]
            } else if cons[1] == from_dir {
                *is_main_loop = true;
                cons[0]
            } else {
                return None;
            }
        }
        Tile::Ground | Tile::Start => return None,
    };

    Some((point, to_dir))
}

#[cfg(test)]
mod tests {
    #[test]
    fn farthest_0() {
        let map = concat! {
            ".....\n",
            ".S-7.\n",
            ".|.|.\n",
            ".L-J.\n",
            ".....\n"
        };

        assert_eq!(super::farthest(map), 4)
    }

    #[test]
    fn farthest_1() {
        let map = concat! {
            "..F7.\n",
            ".FJ|.\n",
            "SJ.L7\n",
            "|F--J\n",
            "LJ...\n"
        };

        assert_eq!(super::farthest(map), 8)
    }

    #[test]
    fn enclosed_0() {
        let map = concat! {
            "...........\n",
            ".S-------7.\n",
            ".|F-----7|.\n",
            ".||.....||.\n",
            ".||.....||.\n",
            ".|L-7.F-J|.\n",
            ".|..|.|..|.\n",
            ".L--J.L--J.\n",
            "...........\n",
        };

        assert_eq!(super::enclosed(map), 4)
    }

    #[test]
    fn enclosed_1() {
        let map = concat! {
            ".F----7F7F7F7F-7....\n",
            ".|F--7||||||||FJ....\n",
            ".||.FJ||||||||L7....\n",
            "FJL7L7LJLJ||LJ.L-7..\n",
            "L--J.L7...LJS7F-7L7.\n",
            "....F-J..F7FJ|L7L7L7\n",
            "....L7.F7||L7|.L7L7|\n",
            ".....|FJLJ|FJ|F7|.LJ\n",
            "....FJL-7.||.||||...\n",
            "....L---J.LJ.LJLJ...\n",
        };

        assert_eq!(super::enclosed(map), 8)
    }

    #[test]
    fn enclosed_3() {
        let map = concat! {
            "FF7FSF7F7F7F7F7F---7\n",
            "L|LJ||||||||||||F--J\n",
            "FL-7LJLJ||||||LJL-77\n",
            "F--JF--7||LJLJ7F7FJ-\n",
            "L---JF-JLJ.||-FJLJJ7\n",
            "|F|F-JF---7F7-L7L|7|\n",
            "|FFJF7L7F-JF7|JL---7\n",
            "7-L-JL7||F7|L7F-7F7|\n",
            "L.L7LFJ|||||FJL7||LJ\n",
            "L7JLJL-JLJLJL--JLJ.L\n",
        };

        assert_eq!(super::enclosed(map), 10)
    }
}
