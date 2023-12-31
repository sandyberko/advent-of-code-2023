use num::integer::lcm;
use std::{
    array,
    cell::Cell,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    default,
    iter::Sum,
    mem,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module<'s> {
    tag: &'s str,
    ty: ModuleType,
    state: Cell<Pulse>,
    dest: Vec<usize>,

    /// Number of pulses required to reach 'rx' with a `Pulse::Low`
    pulses: [usize; 2],
}

impl<'s> Module<'s> {
    fn new(tag: &'s str, ty: ModuleType) -> Self {
        Self {
            tag,
            ty,
            state: Cell::default(),
            dest: Vec::new(),
            pulses: [usize::MAX; 2],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    Flip,
    Conj(Vec<usize>),
    Broadcaster,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pulse {
    High,
    #[default]
    Low,
}

#[derive(Debug, Clone, Copy)]
pub struct Pulses {
    low: usize,
    high: usize,
}

impl Sum for Pulses {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut pulses = Pulses { high: 0, low: 0 };
        for pulse in iter {
            pulses.high += pulse.high;
            pulses.low += pulse.low;
        }
        pulses
    }
}

impl Add for Pulses {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pulses {
            high: self.high + rhs.high,
            low: self.low + rhs.low,
        }
    }
}

impl AddAssign for Pulses {
    fn add_assign(&mut self, rhs: Self) {
        self.high += rhs.high;
        self.low += rhs.low;
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    id: usize,
    pulse: Pulse,
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
            .then_with(|| self.id.cmp(&other.id))
            .then_with(|| self.pulse.cmp(&other.pulse))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub const BROADCASTER: &str = "broadcaster";

pub struct Schema<'s> {
    pub modules: Vec<Module<'s>>,
    pub ids: HashMap<&'s str, usize>,
}

impl<'s> Schema<'s> {
    pub fn pulse_propogation(&self) -> usize {
        let mut queues = array::from_fn(|_| Vec::with_capacity(self.modules.len()));

        let pulses = (1..=1000)
            .map(|_i| {
                queues[0].push(self.ids[BROADCASTER]);
                self.button_press(usize::MAX, &mut queues).0
            })
            .sum::<Pulses>();
        pulses.high * pulses.low
    }

    pub fn button_press(
        &self,
        rx: usize,
        [queue, next_queue]: &mut [Vec<usize>; 2],
    ) -> (Pulses, bool) {
        let mut pulses = Pulses { low: 1, high: 0 };

        while !queue.is_empty() {
            for src in queue.drain(0..) {
                let src = &self.modules[src];
                match src.state.get() {
                    Pulse::High => pulses.high += src.dest.len(),
                    Pulse::Low => pulses.low += src.dest.len(),
                }
                for dest in &src.dest {
                    // eprintln!("{src} -{pulse:?}-> {dest}");

                    if src.state.get() == Pulse::Low && *dest == rx {
                        return (pulses, true);
                    }
                    if self.pulse(*dest, src.state.get()) {
                        next_queue.push(*dest);
                    }
                }
            }
            mem::swap(queue, next_queue);
        }
        (pulses, false)
    }

    pub fn parse(input: &'s str) -> Self {
        let mut ids = HashMap::new();
        let mut next_id = 0usize;
        let (mut modules, dest): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|line| {
                let (tag, dest) = line.split_once("->").unwrap();
                let dest = dest.split(',').map(str::trim).collect::<Vec<_>>();

                let tag = tag.trim();
                let (tag, ty) = if tag == BROADCASTER {
                    (tag, ModuleType::Broadcaster)
                } else if let Some(tag) = tag.strip_prefix('%') {
                    (tag, ModuleType::Flip)
                } else if let Some(tag) = tag.strip_prefix('&') {
                    (tag, ModuleType::Conj(Vec::new()))
                } else {
                    panic!("invalid module {tag:?}")
                };

                ids.insert(tag, next_id);

                next_id += 1;

                (Module::new(tag, ty), dest)
            })
            .unzip();

        for (id, dest) in dest.into_iter().enumerate() {
            for dest in dest {
                let dest_id = ids.entry(dest).or_insert_with(|| {
                    let id = next_id;
                    modules.insert(id, Module::new(dest, ModuleType::Broadcaster));
                    next_id += 1;
                    id
                });
                modules[id].dest.push(*dest_id);
                if let ModuleType::Conj(inputs) = &mut modules[*dest_id].ty {
                    inputs.push(id);
                }
            }
        }
        Self { modules, ids }
    }

    /// if returns true, `self.modules[dest]` sould pulse next.
    fn pulse(&self, dest: usize, pulse: Pulse) -> bool {
        let Some(module) = self.modules.get(dest) else {
            return false;
        };
        let state = match &module.ty {
            ModuleType::Flip => match pulse {
                Pulse::Low => match module.state.get() {
                    Pulse::High => Pulse::Low,
                    Pulse::Low => Pulse::High,
                },
                Pulse::High => return false,
            },
            ModuleType::Conj(inputs) => {
                if inputs
                    .iter()
                    .all(|id| self.modules[*id].state.get() == Pulse::High)
                {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
            ModuleType::Broadcaster => pulse,
        };
        module.state.set(state);

        true
    }

    pub fn pulse_to_rx(&self) -> usize {
        let rx = self.ids["rx"];
        // let (mut flips, mut conjs) = (Vec::new(), Vec::new());
        // for (tag, module) in &state {
        //     match &module.ty {
        //         ModuleType::Flip(_) => flips.push(tag),
        //         ModuleType::Conj(_) => conjs.push(tag),
        //         ModuleType::Broadcaster => (),
        //     }
        // }

        let mut presses = 0;

        let mut queues = array::from_fn(|_| Vec::with_capacity(self.modules.len()));
        queues[0].push(self.ids[BROADCASTER]);

        loop {
            presses += 1;
            if self.button_press(rx, &mut queues).1 {
                return presses;
            }
            if presses % 2usize.pow(32) == 0 {
                eprint!("\x1B[2J\x1B[1;1H{presses}");
            }

            // eprint!("Press any key...");
            // io::stderr().flush().unwrap();
            // io::stdin().read_line(&mut String::new()).unwrap();
            // eprint!("\x1B[2J\x1B[1;1H");

            // for (i, tag) in flips.iter().enumerate() {
            //     if let ModuleType::Flip(state) = &state.get(*tag).unwrap().ty {
            //         if i > 0 && i % 10 == 0 {
            //             eprintln!()
            //         }

            //         match state.get() {
            //             Pulse::High => eprint!("{} ", tag.green()),
            //             Pulse::Low => eprint!("{} ", tag.red()),
            //         }
            //     }
            // }
            // eprintln!();
            // for tag in &conjs {
            //     if let ModuleType::Conj(inputs) = &state.get(*tag).unwrap().ty {
            //         eprint!("{tag}: [");
            //         for (i, (input, state)) in inputs.iter().enumerate() {
            //             if i > 0 {
            //                 eprint!(" ");
            //             }
            //             match state.get() {
            //                 Pulse::High => eprint!("{}", input.green()),
            //                 Pulse::Low => eprint!("{}", input.red()),
            //             }
            //         }
            //         eprintln!("]");
            //     }
            // }
            // eprintln!("============");
        }
    }

    // Dijkstra's shortest path algorithm.

    // Start at `start` and use `dist` to track the current shortest distance
    // to each node. This implementation isn't memory-efficient as it may leave duplicate
    // nodes in the queue. It also uses `usize::MAX` as a sentinel value,
    // for a simpler implementation.
    // fn shortest_path(&mut self, start: usize, goal: usize) -> Option<usize> {
    //     let mut heap = BinaryHeap::new();

    //     // We're at `start`, with a zero cost
    //     self.modules[start].pulses[Pulse::Low as usize] = 0;
    //     heap.push(State {
    //         cost: 0,
    //         id: start,
    //         pulse: Pulse::Low,
    //     });

    //     // Examine the frontier with lower cost nodes first (min-heap)
    //     while let Some(State { cost, id, pulse }) = heap.pop() {
    //         // Alternatively we could have continued to find all shortest paths
    //         if id == goal && pulse == Pulse::Low {
    //             return Some(cost);
    //         }

    //         // Important as we may have already found a better way
    //         if cost > self.modules[id].pulses[pulse as usize] {
    //             continue;
    //         }

    //         // For each node we can reach, see if we can find a way with
    //         // a lower cost going through this node
    //         match &self.modules[id].ty {
    //             ModuleType::Flip(_) => {

    //             },
    //             ModuleType::Conj(_) => todo!(),
    //             ModuleType::Broadcaster => unreachable!(),
    //         }
    //         for edge in &adj_list[position] {
    //             let next = State {
    //                 cost: cost + edge.cost,
    //                 position: edge.node,
    //             };

    //             // If so, add it to the frontier and continue
    //             if next.cost < dist[next.position] {
    //                 heap.push(next);
    //                 // Relaxation, we have now found a better way
    //                 dist[next.position] = next.cost;
    //             }
    //         }
    //     }

    //     // Goal not reachable
    //     None
    // }
}
