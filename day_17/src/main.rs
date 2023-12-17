use std::{cmp::Ordering, collections::BinaryHeap};

fn main() {
    println!("Hello, world!");
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

struct Node {
    heat_loss: u8,
    cost: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Span {
    from: Dir,
    dir: u8,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point,
    span: Option<Span>,
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

fn shortest_path(map: &str) -> usize {
    let mut map = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Node {
                    heat_loss: c.to_digit(10).unwrap() as _,
                    cost: usize::MAX,
                })
                .collect::<Box<[_]>>()
        })
        .collect::<Box<[_]>>();

    let mut unvisited = BinaryHeap::new();

    map[0][0].cost = 0;
    unvisited.push(State {
        cost: 0,
        position: Point { y: 0, x: 0 },
        span: None,
    });

    while let Some(State {
        cost,
        position: Point { y, x },
        span,
    }) = unvisited.pop()
    {
        if y == map.len() - 1 && x == map[0].len() - 1 {
            // for row in map.iter() {
            //     for node in row.iter() {
            //         // if node.cost == usize::MAX {
            //         //     eprint!("--- ");
            //         // } else {
            //         //     eprint!("{:3} ", node.cost);
            //         // }
            //         match node.span {
            //             Some(Span {
            //                 from: Dir::Down,
            //                 dir: span,
            //             }) => eprint!("v{span}",),
            //             Some(Span {
            //                 from: Dir::Left,
            //                 dir: span,
            //             }) => eprint!(">{span}"),
            //             Some(Span {
            //                 from: Dir::Right,
            //                 dir: span,
            //             }) => eprint!("<{span}"),
            //             Some(Span {
            //                 from: Dir::Up,
            //                 dir: span,
            //             }) => eprint!("^{span}"),
            //             None => eprint!("  "),
            //         }
            //     }
            //     eprintln!()
            // }

            return cost;
        }

        // Important as we may have already found a better way
        if cost > map[y][x].cost {
            continue;
        }

        for (nbor, dir) in [
            y.checked_sub(1).map(|y| (Point { y, x }, Dir::Up)),
            x.checked_sub(1).map(|x| (Point { y, x }, Dir::Left)),
        ]
        .into_iter()
        .flatten()
        .chain([
            (Point { y: y + 1, x }, Dir::Down),
            (Point { y, x: x + 1 }, Dir::Right),
        ]) {
            let Some(node) = map.get(nbor.y).and_then(|row| row.get(nbor.x)) else {
                continue;
            };

            let span = if let Some(span) = span.and_then(|span| (span.from == dir).then_some(span))
            {
                if span.dir >= 2 {
                    continue;
                } else {
                    Some(Span {
                        from: dir,
                        dir: span.dir + 1,
                    })
                }
            } else {
                Some(Span { from: dir, dir: 0 })
            };

            let next = State {
                cost: cost + node.heat_loss as usize,
                position: nbor,
                span,
            };

            if next.cost < map[nbor.y][nbor.x].cost {
                unvisited.push(next);

                map[nbor.y][nbor.x].cost = next.cost;
            }
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
        let shortest_path = super::shortest_path(MAP);
        assert_eq!(shortest_path, 102);
    }
}
