use std::collections::BTreeMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("lowest location: {}", lowest_location(INPUT));
    println!("lowest location ranged: {}", lowest_location_ranged(INPUT));
}

// const CATEGORIES: &[&str] = &[
//     "soil",
//     "fertilizer",
//     "water",
//     "light",
//     "temperature",
//     "humidity",
//     "location",
// ];

fn lowest_location(input: &str) -> u64 {
    let mut sections = input.split("\n\n");

    let seeds = {
        let seeds_section = sections.next().unwrap();
        let (_tag, seeds) = seeds_section.split_once(':').unwrap();
        seeds
            .split_whitespace()
            .map(str::parse::<u64>)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    };

    sections
        .fold(seeds, |prev, mapping| {
            let mapping = parse_mapping(mapping);

            let result = prev
                .into_iter()
                .map(|seed| {
                    mapping
                        .range(..=seed)
                        .next_back()
                        .and_then(|(source, (dest, length))| {
                            let distance = seed - source;
                            // eprintln!("\t{distance} = {seed} - {source} < {length}");
                            (distance < *length).then_some(dest + distance)
                        })
                        .unwrap_or(seed)
                })
                .collect();

            // for (prev, result) in prev.iter().zip(&result) {
            //     eprintln!("Seed number {prev} corresponds to soil number {result}");
            // }
            // eprintln!("---");

            result
        })
        .into_iter()
        .min()
        .unwrap()
}

#[derive(PartialEq, Eq)]
struct Range {
    start: u64,
    length: u64,
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Range {
    fn new(start: u64, length: u64) -> Self {
        Self { start, length }
    }

    fn end(&self) -> u64 {
        self.start + self.length
    }
}

fn lowest_location_ranged(input: &str) -> u64 {
    let mut sections = input.split("\n\n");

    let mut seed_ranges = Vec::new();
    {
        let seeds_section = sections.next().unwrap();
        let (_tag, seeds) = seeds_section.split_once(':').unwrap();
        let mut seeds = seeds
            .split_whitespace()
            .map(str::parse::<u64>)
            .map(Result::unwrap);

        while let (Some(start), Some(length)) = (seeds.next(), seeds.next()) {
            seed_ranges.push(Range::new(start, length));
        }
    };

    sections
        .fold(seed_ranges, |prev, mapping| {
            let mapping = parse_mapping(mapping);

            prev.into_iter()
                .flat_map(|seed| {
                    let mut cur_seed_end = seed.end();
                    let mut result = Vec::new();
                    for (&map_start, (dest, length)) in mapping
                        .range(..=seed.end())
                        .rev()
                        .take_while(|(&map_start, (_, length))| seed.start < map_start + length)
                    {
                        let shift = *dest as i64 - map_start as i64;
                        let map_end = map_start + length;

                        if map_end < cur_seed_end {
                            Range::new(map_end, cur_seed_end - map_end);
                        }

                        let start = map_start.max(seed.start);
                        let length = cur_seed_end.min(map_end) - map_start.max(seed.start);
                        result.push(Range::new(start.checked_add_signed(shift).unwrap(), length));
                        cur_seed_end = start;
                    }
                    if cur_seed_end > seed.start {
                        result.push(Range::new(seed.start, cur_seed_end - seed.start));
                    }
                    result
                })
                .collect()
        })
        .into_iter()
        .min()
        .unwrap()
        .start
}

fn parse_mapping(mapping: &str) -> BTreeMap<u64, (u64, u64)> {
    mapping
        .split('\n')
        .skip(1)
        .map(|sect_line| {
            let mut sect_line = sect_line
                .split_whitespace()
                .map(str::parse::<u64>)
                .map(Result::unwrap);

            let dest = sect_line.next().unwrap();
            let source = sect_line.next().unwrap();
            let length = sect_line.next().unwrap();

            assert!(sect_line.next().is_none());

            (source, (dest, length))
        })
        .collect::<BTreeMap<_, _>>()
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn lowest_location() {
        let lowest_location = super::lowest_location(INPUT);
        assert_eq!(lowest_location, 35);
    }

    #[test]
    fn lowest_location_ranged() {
        let lowest_location = super::lowest_location_ranged(INPUT);
        assert_eq!(lowest_location, 46);
    }
}
