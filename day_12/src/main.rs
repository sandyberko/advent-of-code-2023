use std::iter::once;

use day_12::{parse_row, try_match, State};
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

fn arrangements(line: &str) -> usize {
    // eprintln!("{}", line.blue());
    let (record, groups) = parse_row(line);

    let result = try_match(record.into_iter(), groups.into_iter(), &[]);
    // eprintln!("{}", result.red());
    result
}

fn arrangements_x5(line: &str) -> usize {
    eprintln!("{}", line.blue());
    let (record, groups) = parse_row(line);

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
    #[ignore = "too slow"]
    fn too_slow() {
        super::arrangements_x5("?????#???????????#?# 1,4,1,2,1,1");
    }
}
