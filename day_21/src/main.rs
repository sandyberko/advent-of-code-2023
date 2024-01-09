use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Destinations: {}", reachable_plots(INPUT, 64));
}

#[derive(Clone, Copy)]
enum Pixel {
    Plot,
    Rock,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    y: usize,
    x: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point,
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
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn reachable_plots(input: &str, dist: usize) -> usize {
    let mut start = Option::<Point>::None;
    let mut garden = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, c)| {
                    let pixel = match c {
                        '#' => Pixel::Rock,
                        '.' => Pixel::Plot,
                        'S' => {
                            start = Some(Point { y, x });
                            Pixel::Plot
                        }
                        c => panic!("invalid pixel {c:?}"),
                    };
                    (pixel, usize::MAX)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let start = start.unwrap();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    garden[start.y][start.x].1 = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        position: Point { y, x },
    }) = heap.pop()
    {
        // Important as we may have already found a better way
        if cost > garden[y][x].1 {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        let adj_list = y
            .checked_sub(1)
            .map(|y| Point { y, x })
            .into_iter()
            .chain(x.checked_sub(1).map(|x| Point { y, x }))
            .chain([Point { y, x: x + 1 }, Point { y: y + 1, x }]);
        for position @ Point { y, x } in adj_list {
            let Some((Pixel::Plot, next_cost)) = garden.get_mut(y).and_then(|line| line.get_mut(x))
            else {
                continue;
            };
            let next = State {
                cost: cost + 1,
                position,
            };

            // If so, add it to the frontier and continue
            if next.cost < *next_cost {
                // Relaxation, we have now found a better way
                *next_cost = next.cost;

                if next.cost < dist {
                    heap.push(next);
                }
            }
        }
    }

    let oddness = dist % 2;
    garden
        .into_iter()
        .flatten()
        .filter(|&(_, steps)| steps <= dist && steps % 2 == oddness)
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = concat! {
            "...........\n",
            ".....###.#.\n",
            ".###.##..#.\n",
            "..#.#...#..\n",
            "....#.#....\n",
            ".##..S####.\n",
            ".##..#...#.\n",
            ".......##..\n",
            ".##.#.####.\n",
            ".##..##.##.\n",
            "...........\n",
        };
        let result = super::reachable_plots(input, 6);
        assert_eq!(16, result);
    }
}
