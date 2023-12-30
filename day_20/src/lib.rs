use num::integer::lcm;
use owo_colors::OwoColorize;
use std::{
    array,
    cell::Cell,
    collections::HashMap,
    io::{self, Write},
    iter::Sum,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module<'s> {
    tag: &'s str,
    ty: ModuleType,
    dest: Vec<usize>,
    send_period: [Cell<Option<usize>>; 2],
}

impl<'s> Module<'s> {
    fn new(tag: &'s str, ty: ModuleType) -> Self {
        Self {
            tag,
            ty,
            dest: Vec::new(),
            send_period: array::from_fn(|_| Cell::new(None)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    Flip(Cell<Pulse>),
    Conj(HashMap<usize, Cell<Pulse>>),
    Broadcaster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, Copy)]
struct Pulses {
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

pub const BROADCASTER: &str = "broadcaster";

pub struct Schema<'s> {
    modules: Vec<Module<'s>>,
    pub ids: HashMap<&'s str, usize>,
}

impl<'s> Schema<'s> {
    pub fn pulse_propogation(&self) -> usize {
        let initial_dest = [self.ids[BROADCASTER]];

        let pulses = (1..=1000)
            .map(|_i| {
                let mut queue = vec![(usize::MAX, Pulse::Low, &initial_dest as &[usize])];
                let mut pulses = Pulses { low: 0, high: 0 };

                while !queue.is_empty() {
                    queue = queue
                        .into_iter()
                        .flat_map(|(src, pulse, dest)| {
                            match pulse {
                                Pulse::High => pulses.high += dest.len(),
                                Pulse::Low => pulses.low += dest.len(),
                            }

                            dest.iter().flat_map(move |dest| {
                                // eprintln!("{src} -{pulse:?}-> {dest}");
                                self.pulse(src, *dest, pulse)
                                    .map(|(pulse, dests)| (*dest, pulse, dests))
                            })
                        })
                        .collect();
                }
                // eprintln!("{pulses:?}");
                pulses
            })
            .sum::<Pulses>();
        pulses.high * pulses.low
    }

    pub fn pulse_to_rx(&self) -> usize {
        let rx = self.ids["rx"];
        let inital_dest = [self.ids[BROADCASTER]];

        self.receive_period(Pulse::Low, rx)

        // let (mut flips, mut conjs) = (Vec::new(), Vec::new());
        // for (tag, module) in &state {
        //     match &module.ty {
        //         ModuleType::Flip(_) => flips.push(tag),
        //         ModuleType::Conj(_) => conjs.push(tag),
        //         ModuleType::Broadcaster => (),
        //     }
        // }

        // let mut presses = 0;

        // loop {
        //     presses += 1;
        //     if self.button_press(inital_dest, rx) {
        //         return presses;
        //     }
        //     if presses % 2usize.pow(17) == 0 {
        //         eprint!("\x1B[2J\x1B[1;1H{presses}");
        //     }

        //     // eprint!("Press any key...");
        //     // io::stderr().flush().unwrap();
        //     // io::stdin().read_line(&mut String::new()).unwrap();
        //     // eprint!("\x1B[2J\x1B[1;1H");

        //     // for (i, tag) in flips.iter().enumerate() {
        //     //     if let ModuleType::Flip(state) = &state.get(*tag).unwrap().ty {
        //     //         if i > 0 && i % 10 == 0 {
        //     //             eprintln!()
        //     //         }

        //     //         match state.get() {
        //     //             Pulse::High => eprint!("{} ", tag.green()),
        //     //             Pulse::Low => eprint!("{} ", tag.red()),
        //     //         }
        //     //     }
        //     // }
        //     // eprintln!();
        //     // for tag in &conjs {
        //     //     if let ModuleType::Conj(inputs) = &state.get(*tag).unwrap().ty {
        //     //         eprint!("{tag}: [");
        //     //         for (i, (input, state)) in inputs.iter().enumerate() {
        //     //             if i > 0 {
        //     //                 eprint!(" ");
        //     //             }
        //     //             match state.get() {
        //     //                 Pulse::High => eprint!("{}", input.green()),
        //     //                 Pulse::Low => eprint!("{}", input.red()),
        //     //             }
        //     //         }
        //     //         eprintln!("]");
        //     //     }
        //     // }
        //     // eprintln!("============");
        // }
    }

    pub fn button_press(&self, inital_dest: [usize; 1], rx: usize) -> bool {
        let mut queue = vec![(usize::MAX, Pulse::Low, &inital_dest as &[usize])];
        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            for (src, pulse, dest) in queue {
                for dest in dest.iter() {
                    // eprintln!("{src} -{pulse:?}-> {dest}");
                    if pulse == Pulse::Low && *dest == rx {
                        return true;
                    }
                    if let Some((pulse, dests)) = self.pulse(src, *dest, pulse) {
                        next_queue.push((*dest, pulse, dests));
                    }
                }
            }
            queue = next_queue;
        }
        false
    }

    fn receive_period(&self, pulse: Pulse, target: usize) -> usize {
        eprintln!("receive {} {pulse:?}", self.modules[target].tag);
        if let Some(period) = self
            .modules
            .iter()
            .enumerate()
            .filter_map(|(src_id, src_module)| {
                if !src_module.dest.contains(&target) {
                    return None;
                }

                Some(self.send_period(src_id, pulse))
            })
            // FIXME that's not true?
            .min()
        {
            period
        } else {
            panic!("{target:?} will never receive a {pulse:?}")
        }
    }

    fn send_period(&self, id: usize, pulse: Pulse) -> usize {
        let module = &self.modules[id];
        if let Some(period) = module.send_period[pulse as usize].get() {
            return period;
        }
        eprintln!("send {} {pulse:?}", module.tag);
        let period = match &module.ty {
            ModuleType::Flip(_) => {
                self.receive_period(Pulse::Low, id)
                    * match pulse {
                        // the first pulse a flip receives, flips it to high and sends a high
                        Pulse::High => 1,
                        // to send a low, it neesd one more pulse
                        Pulse::Low => 2,
                    }
            }
            ModuleType::Conj(inputs) => match pulse {
                Pulse::High => inputs
                    .keys()
                    .map(|input| self.send_period(*input, Pulse::Low))
                    // FIXME that's not true either, it doesn't consider the case when high is sent but not all inputs are high
                    .min()
                    .expect("conj to have at least one input"),
                Pulse::Low => inputs
                    .keys()
                    .map(|input| self.send_period(*input, Pulse::High))
                    .fold(1, lcm),
            },
            ModuleType::Broadcaster => match pulse {
                Pulse::High => panic!("broadcaster never sends high"),
                Pulse::Low => 1,
            },
        };
        module.send_period[pulse as usize].set(Some(period));
        period
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
                    (tag, ModuleType::Flip(Cell::new(Pulse::Low)))
                } else if let Some(tag) = tag.strip_prefix('&') {
                    (tag, ModuleType::Conj(HashMap::new()))
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
                    inputs.insert(id, Cell::new(Pulse::Low));
                }
            }
        }
        Self { modules, ids }
    }

    fn pulse(&self, src: usize, dest: usize, pulse: Pulse) -> Option<(Pulse, &[usize])> {
        let module = self.modules.get(dest)?;
        match &module.ty {
            ModuleType::Flip(state) => match pulse {
                Pulse::Low => {
                    state.set(match state.get() {
                        Pulse::High => Pulse::Low,
                        Pulse::Low => Pulse::High,
                    });
                    Some((state.get(), &module.dest))
                }
                Pulse::High => None,
            },
            ModuleType::Conj(inputs) => {
                inputs[&src].set(pulse);
                let pulse = if inputs.values().all(|pulse| pulse.get() == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                Some((pulse, &module.dest))
            }
            ModuleType::Broadcaster => Some((pulse, &module.dest)),
        }
    }
}
