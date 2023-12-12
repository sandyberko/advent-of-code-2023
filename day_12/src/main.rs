use std::{
    fmt::Display,
    iter::{self, once},
};

use owo_colors::OwoColorize;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Arrangement count: {}", operational_arrangements(INPUT));
    // 7857
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Operational => write!(f, "."),
            State::Damaged => write!(f, "#"),
            State::Unknown => write!(f, "?"),
        }
    }
}

fn operational_arrangements(records: &str) -> usize {
    records.lines().map(op_arrangement_line).sum()
}

fn op_arrangement_line(line: &str) -> usize {
    // eprintln!("{}", line.blue());
    let (record, groups) = line.split_once(' ').unwrap();
    let record = record
        .chars()
        .map(|c| match c {
            '.' => State::Operational,
            '#' => State::Damaged,
            '?' => State::Unknown,
            c => panic!("unknown state: {c:?}"),
        })
        .collect::<Vec<_>>();
    let groups = groups
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let result = try_match(record.iter().copied(), groups.into_iter(), &[], &record);
    // eprintln!("{}", result.red());
    result
}

fn fmt(record: impl IntoIterator<Item = State>) {
    for s in record {
        eprint!("{s}");
    }
}

fn try_match(
    mut record: impl Iterator<Item = State> + Clone,
    mut groups: impl Iterator<Item = usize> + Clone,
    dis_buf: &[State],
    og: &[State],
) -> usize {
    let Some(group) = groups.next() else {
        // let mut record_len = 0;
        if record.all(|s| {
            // record_len += 1;
            s != State::Damaged
        }) {
            // let dis_buf = dis_buf
            //     .iter()
            //     .copied()
            //     .chain(iter::repeat(State::Operational).take(record_len));
            // fmt(dis_buf);
            // compare(og.iter().copied(), dis_buf);
            // eprintln!(" ✅ end of groups");
            return 1;
        } else {
            return 0;
        }
    };

    let mut sum = 0;
    let mut i = 0;
    'record_loop: loop {
        // let mut dis_buf = dis_buf
        //     .iter()
        //     .copied()
        //     .chain(iter::repeat(State::Operational).take(i))
        //     .collect::<Vec<_>>();

        'try_arr: {
            let mut record = record.clone();
            for _ in 0..group {
                match record.next() {
                    Some(State::Operational) => {
                        // fmt(dis_buf);
                        // eprintln!(" X invalid group");
                        break 'try_arr;
                    }
                    Some(State::Damaged | State::Unknown) => (),
                    None => {
                        // fmt(dis_buf.iter().copied());
                        // eprintln!(" X group end of record");
                        break 'record_loop;
                    }
                }
            }

            // eprint!("{}", "#".repeat(group));
            // dis_buf.extend(iter::repeat(State::Damaged).take(group));

            // separator
            match record.next() {
                None => {
                    if groups.next().is_some() {
                        // fmt(dis_buf);
                        // eprintln!(" X sep end of record, remaining groups");
                        break 'record_loop;
                    } else {
                        // fmt(dis_buf.iter().copied());
                        // compare(record.iter().copied(), dis_buf);
                        // eprintln!(" ✅ sep end of record, no groups");
                        sum += 1;
                    }
                }

                Some(State::Damaged) => {
                    // fmt(dis_buf);
                    // eprintln!(" X missing sep");
                }
                Some(State::Operational | State::Unknown) => {
                    // dis_buf.push(State::Operational);

                    let arrangements = try_match(record, groups.clone(), &dis_buf, og);
                    // TODO short-circuit
                    // if arrangements == 0 {
                    //     // fmt(dis_buf);
                    //     // eprintln!(" X no more matches {depth}");
                    //     None
                    // } else
                    {
                        sum += arrangements
                    }
                }
            }
        }

        match record.next() {
            Some(State::Damaged) | None => break,
            Some(State::Operational | State::Unknown) => {
                i += 1;
            }
        }
    }
    sum
}

fn compare(expected: impl IntoIterator<Item = State>, got: impl IntoIterator<Item = State>) {
    for (i, pair) in expected.into_iter().zip(got).enumerate() {
        match pair {
            (State::Unknown, _) => (),
            (_, State::Unknown) => (),
            (expected, got) => assert_eq!(expected, got, "#{i}: {expected} <--> {got}"),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn operational_arrangements() {
        let lines = [
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ];
        let expected = [1, 4, 1, 1, 4, 10];

        for (line, expected) in lines.iter().zip(expected.iter()) {
            let got = super::op_arrangement_line(line);
            assert_eq!(got, *expected, "{line}: expected {expected}, got {got}");
        }

        assert_eq!(super::operational_arrangements(&lines.join("\n")), 21);
    }
}
