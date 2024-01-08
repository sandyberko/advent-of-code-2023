use std::{
    array,
    cell::Cell,
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    fs,
    io::{self, BufWriter, Write},
    iter::Sum,
    mem,
    ops::{Add, AddAssign, Not},
};

use num::Integer;

#[derive(Debug, PartialEq, Eq)]
pub struct Module<'s> {
    tag: &'s str,
    ty: ModuleType,
    state: Cell<Pulse>,
    dest: Vec<usize>,

    pulses: BTreeMap<Wave, Pulse>,
}

impl<'s> Module<'s> {
    fn new(tag: &'s str, ty: ModuleType) -> Self {
        Self {
            tag,
            ty,
            state: Cell::default(),
            dest: Vec::new(),
            pulses: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    Flip,
    Conj(Vec<usize>),
    Broadcaster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Wave {
    freq: usize,
    phase: usize,
}

impl Wave {
    fn new(freq: usize, phase: usize) -> Self {
        assert!(phase < freq);
        Self { freq, phase }
    }

    fn intersect(&self, other_n: Self) -> Option<Self> {
        (other_n.phase == self.phase).then_some(Self::new(self.freq.lcm(&other_n.freq), self.phase))
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pulse {
    High,
    #[default]
    Low,
}
impl Not for Pulse {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
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
                self.poll_pulses(usize::MAX, &mut queues).0
            })
            .sum::<Pulses>();
        pulses.high * pulses.low
    }

    pub fn poll_pulses(
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
                    // eprintln!(
                    //     "{} -{:?}-> {}",
                    //     src.tag,
                    //     src.state.get(),
                    //     self.modules[*dest].tag
                    // );

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
        let mut presses = 0;

        let mut queues = array::from_fn(|_| Vec::with_capacity(self.modules.len()));

        loop {
            eprint!("Press any key...");
            io::stderr().flush().unwrap();
            io::stdin().read_line(&mut String::new()).unwrap();

            presses += 1;
            eprintln!("\x1B[2J\x1B[1;1H{presses}");

            queues[0].push(self.ids[BROADCASTER]);
            if self.poll_pulses(rx, &mut queues).1 {
                return presses;
            }
            if presses % 2usize.pow(32) == 0 {
                eprint!("\x1B[2J\x1B[1;1H{presses}");
            }

            // for (i, module) in self.modules.iter().enumerate() {
            //     if i > 0 && i % 10 == 0 {
            //         eprintln!()
            //     }
            //     match module.state.get() {
            //         Pulse::High => eprint!("{} ", module.tag.green()),
            //         Pulse::Low => eprint!("{} ", module.tag.red()),
            //     }
            // }
            // eprintln!();
            // eprintln!("============");
        }
    }

    pub fn calc_to_rx(&mut self) -> usize {
        let rx = self.ids["rx"];
        self.calc_pulses();
        self.modules[rx]
            .pulses
            .iter()
            .find_map(|(n, pulse)| (*pulse == Pulse::Low).then_some(n.freq + n.phase))
            .unwrap()
    }

    pub fn calc_pulses(&mut self) {
        let mut queue = Vec::with_capacity(self.modules.len());
        queue.push((self.ids[BROADCASTER], Wave::new(1, 0), Pulse::Low));

        let mut counter = 0;

        while !queue.is_empty() {
            counter += 1;
            if counter > 6 {
                break;
            }
            // cn -Low-> sh -High-> mf -Low-> rx
            eprintln!("=====================");
            for (src, n, pulse) in std::mem::take(&mut queue) {
                for dest in self.modules[src].dest.clone() {
                    eprintln!(
                        "{} -{pulse:?}-> {}",
                        self.modules[src].tag, self.modules[dest].tag
                    );

                    match self.modules[dest].ty.clone() {
                        ModuleType::Flip => {
                            if pulse == Pulse::Low {
                                let last_state = self.modules[dest]
                                    .pulses
                                    .values()
                                    .last()
                                    .copied()
                                    .unwrap_or_default();

                                self.modules[dest]
                                    .pulses
                                    .entry(Wave::new(n.freq * 2, n.phase))
                                    .or_insert_with_key(|n| {
                                        queue.push((dest, *n, !last_state));
                                        Pulse::High
                                    });
                                self.modules[dest]
                                    .pulses
                                    .entry(Wave::new(n.freq * 2, n.phase + 1))
                                    .or_insert_with_key(|n| {
                                        queue.push((dest, *n, last_state));
                                        Pulse::Low
                                    });
                            }
                        }
                        ModuleType::Conj(inputs) => {
                            // on each `input` wave:
                            //     if everyone is High put Low,
                            //     otherwise High.
                            for &input in &inputs {
                                match pulse {
                                    Pulse::High => {
                                        if inputs.len() == 1 {
                                            self.modules[dest].pulses.entry(n).or_insert_with_key(
                                                |n| {
                                                    queue.push((dest, *n, Pulse::Low));
                                                    Pulse::Low
                                                },
                                            );
                                        } else {
                                            inputs.iter().try_fold(n, |acc, &other_in| {
                                                if input == other_in {
                                                    return Some(acc);
                                                }
                                                self.modules[other_in].pulses.iter().find_map(
                                                    |(&other_n, &other_pulse)| {
                                                        (pulse == other_pulse)
                                                            .then(|| acc.intersect(other_n))
                                                            .flatten()
                                                    },
                                                )
                                            });
                                            for &other_in in &inputs {
                                                if other_in == input {
                                                    continue;
                                                }
                                                for (other_n, pulse) in
                                                    self.modules[other_in].pulses.clone()
                                                {
                                                    if pulse != Pulse::High {
                                                        continue;
                                                    }
                                                    if let Some(intersection) = n.intersect(other_n)
                                                    {
                                                        // eprintln!("{n:?} & {other_n:?} = {intersection:?}");
                                                        self.modules[dest]
                                                            .pulses
                                                            .entry(intersection)
                                                            .or_insert_with_key(|n| {
                                                                queue.push((dest, *n, Pulse::Low));
                                                                Pulse::Low
                                                            });
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Pulse::Low => {
                                        self.modules[dest].pulses.entry(n).or_insert_with_key(
                                            |n| {
                                                queue.push((dest, *n, Pulse::High));
                                                Pulse::High
                                            },
                                        );
                                    }
                                }
                            }
                        }
                        ModuleType::Broadcaster => {
                            self.modules[dest].pulses = self.modules[src].pulses.clone();
                        }
                    };
                }
            }
        }

        let mut f = BufWriter::new(
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open("target/to_rx.dot")
                .unwrap(),
        );

        writeln!(
            &mut f,
            r##"digraph {{
    bgcolor="#0d1117"
    node [fontcolor="white" color="white" fontname="Monaspace Argon Var"]
    edge [color="white"]
    forcelabels=true
    "##
        )
        .unwrap();

        for Module {
            tag,
            ty,
            dest,
            pulses,
            ..
        } in &self.modules
        {
            let shape = match ty {
                ModuleType::Flip => "diamond",
                ModuleType::Conj(_) => "cube",
                ModuleType::Broadcaster => "circle",
            };
            write!(&mut f, "\t{tag} [shape={shape} xlabel=\"").unwrap();
            for (Wave { freq, phase }, pulse) in pulses {
                let pulse = match pulse {
                    Pulse::High => "H",
                    Pulse::Low => "L",
                };
                write!(&mut f, "{pulse}*{freq}+{phase}\\n").unwrap();
            }
            write!(&mut f, "\"]").unwrap();
            for (i, dest) in dest.iter().enumerate() {
                match i {
                    0 => write!(&mut f, "{tag} -> ").unwrap(),
                    _ => write!(&mut f, ", ").unwrap(),
                }
                write!(&mut f, "{}", self.modules[*dest].tag).unwrap();
            }
            writeln!(&mut f, ";").unwrap();
        }
        writeln!(&mut f, "}}").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::Schema;

    #[test]
    fn test() {
        Schema::parse(concat! {
            "broadcaster -> a\n",
            "&a -> b\n",
            "&b -> rx\n",
        })
        .calc_pulses();
    }
}
