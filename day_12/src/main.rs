use std::{
    fmt::Display,
    iter::{self, once},
};

use owo_colors::OwoColorize;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let arrangement_count = INPUT.lines().map(arrangements).sum::<usize>();
    println!("Arrangement count: {arrangement_count}");
    // 7857

    let arrangement_count = INPUT.lines().map(arrangements_x5).sum::<usize>();
    println!("Arrangement count x5: {arrangement_count}");
    //
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

fn arrangements(line: &str) -> usize {
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

    let result = try_match(record.into_iter(), groups.into_iter(), &[]);
    // eprintln!("{}", result.red());
    result
}

fn arrangements_x5(line: &str) -> usize {
    eprintln!("{}", line.blue());
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

    let result = try_match(record.into_iter(), groups.into_iter(), &[]);
    return result.pow(5);

    let record = record
        .iter()
        .copied()
        .chain(once(State::Unknown))
        .chain(record.iter().copied())
        .chain(once(State::Unknown))
        .chain(record.iter().copied())
        .chain(once(State::Unknown))
        .chain(record.iter().copied())
        .chain(once(State::Unknown))
        .chain(record.iter().copied());

    let result = try_match(record, (0..5).flat_map(|_| groups.iter().copied()), &[]);
    eprintln!("{}", result.red());
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
) -> usize {
    let Some(group) = groups.next() else {
        let mut record_len = 0;
        if record.all(|s| {
            record_len += 1;
            s != State::Damaged
        }) {
            let dis_buf = dis_buf
                .iter()
                .copied()
                .chain(iter::repeat(State::Operational).take(record_len));
            fmt(dis_buf);
            // compare(og.iter().copied(), dis_buf);
            eprintln!(" ✅ end of groups");
            return 1;
        } else {
            return 0;
        }
    };

    let mut sum = 0;
    let mut i = 0;
    'record_loop: loop {
        let mut dis_buf = dis_buf
            .iter()
            .copied()
            .chain(iter::repeat(State::Operational).take(i))
            .collect::<Vec<_>>();

        'try_arr: {
            let mut record = record.clone();
            for _ in 0..group {
                match record.next() {
                    Some(State::Operational) => {
                        fmt(dis_buf.iter().copied());
                        eprintln!(" X invalid group");
                        break 'try_arr;
                    }
                    Some(State::Damaged | State::Unknown) => (),
                    None => {
                        fmt(dis_buf.iter().copied());
                        eprintln!(" X group end of record");
                        break 'record_loop;
                    }
                }
            }

            dis_buf.extend(iter::repeat(State::Damaged).take(group));

            match record.next() {
                None => {
                    if groups.next().is_some() {
                        fmt(dis_buf);
                        eprintln!(" X sep end of record, remaining groups");
                        break 'record_loop;
                    } else {
                        fmt(dis_buf.iter().copied());
                        // compare(record.iter().copied(), dis_buf);
                        eprintln!(" ✅ sep end of record, no groups");
                        sum += 1;
                    }
                }

                Some(State::Damaged) => {
                    fmt(dis_buf.iter().copied());
                    eprintln!(" X missing sep");
                }
                Some(State::Operational | State::Unknown) => {
                    dis_buf.push(State::Operational);

                    let arrangements = try_match(record, groups.clone(), &dis_buf);
                    sum += arrangements
                }
            }
        }

        match record.next() {
            Some(State::Damaged) | None => break,
            Some(State::Operational | State::Unknown) => {
                dis_buf.push(State::Operational);
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

    const RECORDS: &[&str] = &[
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];

    #[test]
    fn operational_arrangements() {
        let expected = [1, 4, 1, 1, 4, 10];

        for (line, expected) in RECORDS.iter().zip(expected.iter()) {
            let got = super::arrangements(line);
            assert_eq!(got, *expected, "{line}: expected {expected}, got {got}");
        }
    }

    #[test]
    fn operational_arrangements_x5() {
        let expected = [1, 16384, 1, 16, 2500, 506250];

        for (line, expected) in RECORDS.iter().zip(expected.iter()) {
            let got = super::arrangements_x5(line);
            assert_eq!(got, *expected, "{line}: expected {expected}, got {got}");
        }
    }

    #[test]
    fn too_slow() {
        super::arrangements_x5("?????#???????????#?# 1,4,1,2,1,1");
    }
}
