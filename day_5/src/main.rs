use std::collections::BTreeMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("lowest location: {}", lowest_location(INPUT));
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

fn lowest_location(input: &str) -> u32 {
    let mut sections = input.split("\n\n");

    let seeds = {
        let seeds_section = sections.next().unwrap();
        let (_tag, seeds) = seeds_section.split_once(':').unwrap();
        seeds
            .split_whitespace()
            .map(str::parse::<u32>)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    };

    sections
        .fold(seeds, |prev, mapping| {
            let mapping = mapping
                .split('\n')
                .skip(1)
                .map(|sect_line| {
                    let mut sect_line = sect_line
                        .split_whitespace()
                        .map(str::parse::<u32>)
                        .map(Result::unwrap);

                    let dest = sect_line.next().unwrap();
                    let source = sect_line.next().unwrap();
                    let length = sect_line.next().unwrap();

                    assert!(sect_line.next().is_none());

                    (source, (dest, length))
                })
                .collect::<BTreeMap<_, _>>();

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

#[cfg(test)]
pub mod tests {
    #[rustfmt::skip]
    const INPUT: &str = 
"seeds: 79 14 55 13

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
56 93 4";

    #[test]
    fn lowest_location() {
        let lowest_location = super::lowest_location(INPUT);
        assert_eq!(lowest_location, 35);
    }
}
