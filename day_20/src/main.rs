use std::{
    cell::Cell,
    collections::HashMap,
    iter::Sum,
    ops::{Add, AddAssign},
};

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Pulse Propagation: {}", pulse_propogation(INPUT));
    // 896998430
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module<'s> {
    ty: ModuleType<'s>,
    dest: Vec<&'s str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType<'s> {
    Flip(Cell<Pulse>),
    Conj(HashMap<&'s str, Cell<Pulse>>),
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

const BROADCASTER: &str = "broadcaster";

fn pulse_propogation(input: &str) -> usize {
    let mut state = input
        .lines()
        .map(|line| {
            let (module, dest) = line.split_once("->").unwrap();
            let dest = dest.split(',').map(str::trim).collect();

            let module = module.trim();
            let (module, ty) = if module == BROADCASTER {
                (module, ModuleType::Broadcaster)
            } else if let Some(module) = module.strip_prefix('%') {
                (module, ModuleType::Flip(Cell::new(Pulse::Low)))
            } else if let Some(module) = module.strip_prefix('&') {
                (module, ModuleType::Conj(HashMap::new()))
            } else {
                panic!("invalid module {module:?}")
            };

            (module, Module { ty, dest })
        })
        .collect::<HashMap<_, _>>();

    for (tag, module) in state.clone() {
        for dest in module.dest {
            if let Some(Module {
                ty: ModuleType::Conj(inputs),
                ..
            }) = state.get_mut(dest)
            {
                inputs.insert(tag, Cell::new(Pulse::Low));
            }
        }
    }

    let pulses = (1..=1000)
        .map(|_i| {
            let mut queue = vec![("button", Pulse::Low, &[BROADCASTER] as &[&str])];
            let mut pulses = Pulses { low: 0, high: 0 };

            while !queue.is_empty() {
                let state = &state;
                queue = queue
                    .into_iter()
                    .flat_map(|(src, pulse, dest)| {
                        match pulse {
                            Pulse::High => pulses.high += dest.len(),
                            Pulse::Low => pulses.low += dest.len(),
                        }

                        dest.iter().flat_map(move |dest| {
                            // eprintln!("{src} -{pulse:?}-> {dest}");
                            self::pulse(state, src, dest, pulse)
                        })
                    })
                    .collect();
            }
            // eprintln!("{pulses:?}");
            pulses
            // if let Some((j, ..)) = states
            //     .iter()
            //     .enumerate()
            //     .find(|(_, other)| **other == state)
            // {}
        })
        .sum::<Pulses>();
    pulses.high * pulses.low
}

fn pulse<'m, 's>(
    modules: &'m HashMap<&str, Module<'s>>,
    src: &'s str,
    dest: &'s str,
    pulse: Pulse,
) -> Option<(&'s str, Pulse, &'m [&'s str])> {
    let module = modules.get(dest)?;
    match &module.ty {
        ModuleType::Flip(state) => match pulse {
            Pulse::Low => {
                state.set(match state.get() {
                    Pulse::High => Pulse::Low,
                    Pulse::Low => Pulse::High,
                });
                Some((dest, state.get(), &module.dest))
            }
            Pulse::High => None,
        },
        ModuleType::Conj(inputs) => {
            inputs.get(src).unwrap().set(pulse);
            let pulse = if inputs.values().all(|pulse| pulse.get() == Pulse::High) {
                Pulse::Low
            } else {
                Pulse::High
            };
            Some((dest, pulse, &module.dest))
        }
        ModuleType::Broadcaster => Some((dest, pulse, &module.dest)),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn pulse_propogation() {
        const INPUT: &str = concat! {
            "broadcaster -> a, b, c\n",
            "%a -> b\n",
            "%b -> c\n",
            "%c -> inv\n",
            "&inv -> a",
        };
        let result = super::pulse_propogation(INPUT);
        assert_eq!(result, 32_000_000);
    }

    #[test]
    fn pulse_propogation_2() {
        const INPUT: &str = concat! {
            "broadcaster -> a\n",
            "%a -> inv, con\n",
            "&inv -> b\n",
            "%b -> con\n",
            "&con -> output",
        };
        let result = super::pulse_propogation(INPUT);
        assert_eq!(result, 11_687_500);
    }
}
