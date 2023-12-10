const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Farthest: {}", farthest(INPUT));
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

enum Pixel {
    Pipe([Dir; 2]),
    Start,
    Ground,
}

impl Pixel {
    fn connects(&self, dir: Dir) -> Option<Dir> {
        match self {
            Pixel::Pipe(cons) => {
                if cons[0] == dir {
                    Some(cons[1])
                } else if cons[1] == dir {
                    Some(cons[0])
                } else {
                    None
                }
            }
            Pixel::Ground | Pixel::Start => None,
        }
    }
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
    let mut starting_point: Option<Point> = None;
    let map = map
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, c)| match c {
                    '|' => Pixel::Pipe([Dir::North, Dir::South]),
                    '-' => Pixel::Pipe([Dir::West, Dir::East]),
                    'L' => Pixel::Pipe([Dir::North, Dir::East]),
                    'J' => Pixel::Pipe([Dir::North, Dir::West]),
                    '7' => Pixel::Pipe([Dir::West, Dir::South]),
                    'F' => Pixel::Pipe([Dir::East, Dir::South]),
                    '.' => Pixel::Ground,
                    'S' => {
                        starting_point = Some(Point::new(x as _, y as _));
                        Pixel::Start
                    }
                    c => panic!("invalid pixel {c:?}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut cur = [Dir::North, Dir::East, Dir::South, Dir::West]
        .into_iter()
        .find_map(|dir| walk(&map, starting_point.unwrap(), dir))
        .unwrap();
    let mut distance = 1;

    while let Some(next) = walk(&map, cur.0, cur.1) {
        cur = next;
        distance += 1;
    }
    (distance + 1) / 2
}

fn walk(map: &[Vec<Pixel>], point: Point, dir: Dir) -> Option<(Point, Dir)> {
    let point = point.to(dir);
    let dir = map
        .get(point.y as usize)?
        .get(point.x as usize)?
        .connects(dir.opposite())?;
    Some((point, dir))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_0() {
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
    fn test_1() {
        let map = concat! {
            "..F7.\n",
            ".FJ|.\n",
            "SJ.L7\n",
            "|F--J\n",
            "LJ...\n"
        };

        assert_eq!(super::farthest(map), 8)
    }
}
