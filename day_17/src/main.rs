use std::{cmp::Ordering, collections::BinaryHeap};

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Shortest Path: {}", shortest_path::<3>(INPUT, 1));
    // 1260
    println!("Shortest Path - Ultra: {}", shortest_path::<10>(INPUT, 4));
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    y: usize,
    x: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
        }
    }
}

struct Node<const MAX_STR: usize> {
    heat_loss: u8,
    cost: [[usize; MAX_STR]; 4],
    visited: [[bool; MAX_STR]; 4],
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Span {
    dir: Dir,
    span: u8,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point,
    span: Option<Span>,
    past: Vec<(Point, Span, usize)>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.span.cmp(&other.span))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path<const MAX_STR: usize>(map: &str, stride: u8) -> usize {
    let mut map = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Node {
                    heat_loss: c.to_digit(10).unwrap() as _,
                    cost: [[usize::MAX; MAX_STR]; 4],
                    visited: [[false; MAX_STR]; 4],
                })
                .collect::<Box<[_]>>()
        })
        .collect::<Box<[_]>>();

    let mut unvisited = BinaryHeap::new();

    map[0][0].cost = [[0; MAX_STR]; 4];
    unvisited.push(State {
        cost: 0,
        position: Point { y: 0, x: 0 },
        span: None,
        past: Vec::new(),
    });

    while let Some(State {
        cost,
        position: Point { y, x },
        span,
        past,
    }) = unvisited.pop()
    {
        // eprintln!("{}", unvisited.len());
        if let Some(span) = span {
            let visited = &mut map[y][x].visited[span.dir as usize][span.span as usize];
            if *visited {
                continue;
            } else {
                *visited = true;
            }
        }

        if y >= map.len() - 1 && x >= map[0].len() - 1 {
            // eprintln!("✅✅✅✅✅✅");
            // for (_, span, stride_cost) in &past {
            //     eprintln!("{}: {stride_cost}", span.span);
            // }
            // for (y, row) in map.iter().enumerate() {
            //     for (x, node) in row.iter().enumerate() {
            //         if past.iter().filter(|(p, ..)| p == &Point { y, x }).count() > 1 {
            //             eprint!("@");
            //         } else if let Some((_, dir, ..)) =
            //             past.iter().find(|(p, ..)| p == &Point { y, x })
            //         {
            //             eprint!("{:?}", dir.dir);
            //         } else {
            //             eprint!("{}", node.heat_loss);
            //         }
            //     }
            //     eprintln!()
            // }

            return cost;
        }

        for dir in [Dir::Down, Dir::Left, Dir::Right, Dir::Up] {
            let stride = if span.map_or(true, |span| dir != span.dir) {
                stride
            } else {
                1
            };

            let (nbor, stride_cost) = match dir {
                Dir::Up => {
                    if let Some(nbor_y) = y.checked_sub(stride as usize) {
                        (
                            Point { y: nbor_y, x },
                            (nbor_y..y)
                                .filter_map(|nbor_y| {
                                    Some(map.get(nbor_y)?.get(x)?.heat_loss as usize)
                                })
                                .sum::<usize>(),
                        )
                    } else {
                        continue;
                    }
                }
                Dir::Left => {
                    if let Some(nbor_x) = x.checked_sub(stride as usize) {
                        (
                            Point { y, x: nbor_x },
                            (nbor_x..x)
                                .filter_map(|nbor_x| {
                                    Some(map.get(y)?.get(nbor_x)?.heat_loss as usize)
                                })
                                .sum::<usize>(),
                        )
                    } else {
                        continue;
                    }
                }
                Dir::Down => (
                    Point {
                        y: y + stride as usize,
                        x,
                    },
                    (y + 1..=y + stride as usize)
                        .filter_map(|y| Some(map.get(y)?.get(x)?.heat_loss as usize))
                        .sum::<usize>(),
                ),
                Dir::Right => (
                    Point {
                        y,
                        x: x + stride as usize,
                    },
                    (x + 1..=x + stride as usize)
                        .filter_map(|x| Some(map.get(y)?.get(x)?.heat_loss as usize))
                        .sum::<usize>(),
                ),
            };

            let Some(nbor_node) = map.get_mut(nbor.y).and_then(|row| row.get_mut(nbor.x)) else {
                continue;
            };

            let next_span =
                if let Some(span) = span.and_then(|span| (span.dir == dir).then_some(span)) {
                    if ((span.span + stride) as usize) < MAX_STR {
                        Span {
                            dir,
                            span: span.span + stride,
                        }
                    } else {
                        continue;
                    }
                } else {
                    Span {
                        dir,
                        span: stride - 1,
                    }
                };

            if nbor_node.visited[next_span.dir as usize][next_span.span as usize] {
                continue;
            }

            let next = State {
                cost: cost + stride_cost,
                position: nbor,
                span: Some(next_span),
                past: [past.clone(), vec![(nbor, next_span, stride_cost)]].concat(),
            };

            if nbor_node.cost.iter().enumerate().any(|(dir_i, dirs)| {
                dirs.iter().enumerate().any(|(span_i, cost_i)| {
                    dir_i == dir as usize && span_i as u8 <= next_span.span && *cost_i < next.cost
                })
            }) {
                continue;
            }

            nbor_node.cost[dir as usize][next_span.span as usize] = next.cost;
            unvisited.push(next);
        }
    }

    panic!("unreachable goal")
}

#[cfg(test)]
mod tests {
    const MAP: &str = concat! {
        "2413432311323\n",
        "3215453535623\n",
        "3255245654254\n",
        "3446585845452\n",
        "4546657867536\n",
        "1438598798454\n",
        "4457876987766\n",
        "3637877979653\n",
        "4654967986887\n",
        "4564679986453\n",
        "1224686865563\n",
        "2546548887735\n",
        "4322674655533\n",
    };

    #[test]
    fn shortest_path() {
        // FIXME the path tries to go back on itself, thus this fails.
        // although the puzzle input somehow does work
        let shortest_path = super::shortest_path::<3>(MAP, 1);
        assert_eq!(shortest_path, 102);
    }

    #[test]
    fn shortest_path_ultra() {
        let shortest_path = super::shortest_path::<10>(MAP, 4);
        assert_eq!(shortest_path, 94);
    }

    #[test]
    fn shortest_path_ultra_2() {
        // FIXME the path tries to go back on itself, thus this fails.
        let map = concat! {
            "111111111111\n",
            "999999999991\n",
            "999999999991\n",
            "999999999991\n",
            "999999999991\n",
        };
        let shortest_path = super::shortest_path::<10>(map, 4);
        assert_eq!(shortest_path, 71);
    }
}
