use std::collections::{BTreeMap, HashMap};

const RADIX: u32 = 10;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    y: u32,
    x: u32,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn perimeter(self, width: u32) -> impl Iterator<Item = Self> {
        // left
        {
            self.x
                .checked_sub(1)
                .map(|x| Point::new(x, self.y))
                .into_iter()
        }
        // right
        .chain([Point::new(self.x + width, self.y)])
        // top
        .chain(
            self.y
                .checked_sub(1)
                .into_iter()
                .flat_map(move |y| self.perimeter_width(width).map(move |x| Point::new(x, y))),
        )
        // bottom
        .chain(
            self.perimeter_width(width)
                .map(move |x| Point::new(x, self.y + 1)),
        )
    }

    fn perimeter_width(self, width: u32) -> std::ops::RangeInclusive<u32> {
        if let Some(x) = self.x.checked_sub(1) {
            x..=self.x + width
        } else {
            self.x..=self.x + width
        }
    }
}

type PartNumber = (Point, u32);

#[derive(Default)]
pub struct Schematic {
    parts: HashMap<Point, char>,
    nums: BTreeMap<Point, u32>,
}

impl From<&'_ str> for Schematic {
    fn from(value: &'_ str) -> Self {
        let mut schematic = Schematic::default();
        for (y, line) in value.lines().enumerate() {
            let mut part_number_found: Option<PartNumber> = None;

            for (x, c) in line.chars().enumerate() {
                if let Some(digit) = c.to_digit(RADIX) {
                    if let Some((_, num)) = &mut part_number_found {
                        *num = *num * RADIX + digit;
                    } else {
                        part_number_found = Some((Point::new(x as _, y as _), digit));
                    }
                } else {
                    // number ended
                    schematic.nums.extend(part_number_found.take());

                    if c != '.' {
                        assert!(
                            schematic
                                .parts
                                .insert(Point::new(x as _, y as _), c)
                                .is_none(),
                            "Part alerady exists at {x},{y}"
                        );
                    }
                }
            }

            // number ended at end of line
            schematic.nums.extend(part_number_found.take());
        }
        schematic
    }
}

impl Schematic {
    fn has_adjacent_part(&self, (coord, num): PartNumber) -> bool {
        let num_length = num.checked_ilog10().unwrap_or(0) + 1;
        coord
            .perimeter(num_length)
            .any(|point| self.parts.contains_key(&point))
    }

    pub fn sum_part_no(&self) -> u32 {
        self.nums
            .iter()
            .filter_map(|(&coord, &num)| self.has_adjacent_part((coord, num)).then_some(num))
            .sum()
    }

    pub fn sum_gear_ratios(&self) -> u32 {
        self.parts
            .iter()
            .filter_map(|(coord, part)| self.gear_ratio(*part, *coord))
            .sum()
    }

    fn gear_ratio(&self, part: char, coord: Point) -> Option<u32> {
        if part != '*' {
            return None;
        }

        // eprintln!("Gear: {coord:?}");
        let mut gears = {
            self.nums
                .range(..coord)
                .rev()
                .take_while(|(&num_coord, ..)| num_coord.y >= coord.y - 1)
        }
        .chain(
            self.nums
                .range(coord..)
                .take_while(|(&num_coord, ..)| num_coord.y <= coord.y + 1),
        )
        .filter_map(|(num_coord, num)| {
            let num_length = num.checked_ilog10().unwrap_or(0);

            (num_coord.x <= coord.x + 1 && (num_coord.x + num_length) >= coord.x - 1).then_some(num)
        });

        if let (Some(r1), Some(r2), None) = (gears.next(), gears.next(), gears.next()) {
            Some(r1 * r2)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, BufWriter, Write};

    use owo_colors::OwoColorize;

    use super::*;

    impl Schematic {
        fn print(&self, width: u32, height: u32) {
            let mut w = BufWriter::new(io::stdout());

            for y in 0..height {
                let mut x_iter = 0..width;
                while let Some(x) = x_iter.next() {
                    let coord = Point::new(x, y);

                    // let (pivot, _) = self.nums.iter().find(|(_, &num)| num == 805).unwrap();

                    if let Some(num) = self.nums.get(&coord) {
                        // if self.has_adjacent_part((coord, *num)) {
                        //     write!(w, "{}", num.red()).ok();
                        // } else {
                        //      write!(w, "{}", num).ok();
                        // }
                        // match coord.cmp(pivot) {
                        //     std::cmp::Ordering::Less => {
                        //         write!(w, "{}", num.red()).ok();
                        //     }
                        //     std::cmp::Ordering::Equal => {
                        //         write!(w, "{}", num.bold().yellow()).ok();
                        //     }
                        //     std::cmp::Ordering::Greater => {
                        //         write!(w, "{}", num.blue()).ok();
                        //     }
                        // }
                        write!(w, "{}", num).ok();
                        for _ in 1..=(num.checked_ilog10().unwrap_or(0)) {
                            x_iter.next();
                        }
                    } else if let Some(c) = self.parts.get(&coord) {
                        if self.gear_ratio(*c, coord).is_some() {
                            write!(w, "{}", c.bright_yellow()).ok();
                        } else if *c == '*' {
                            write!(w, "{}", c.red()).ok();
                        } else {
                            write!(w, "{c}").ok();
                        }
                    } else {
                        write!(w, ".").ok();
                    }
                }
                writeln!(w).ok();
            }
            w.flush().ok();
        }
    }

    #[test]
    fn test_perimeter_too_long() {
        #[rustfmt::skip]
        const INPUT: &str = concat!(
            ".....&.\n",
            ".253...\n",
            ".......\n",
        );

        let schematic = Schematic::from(INPUT);
        assert_eq!(schematic.sum_part_no(), 0);
    }

    const INPUT: &str = concat!(
        "467..114..\n",
        "...*......\n",
        "..35..633.\n",
        "......#...\n",
        "617*......\n",
        ".....+.58.\n",
        "..592.....\n",
        "......755.\n",
        "...$.*....\n",
        ".664.598.."
    );

    #[test]
    fn identify_engine_parts_test() {
        let schematic = Schematic::from(INPUT);
        assert_eq!(schematic.sum_part_no(), 4361);
    }

    #[test]
    fn sum_gear_ratios() {
        let schematic = Schematic::from(INPUT);
        assert_eq!(schematic.sum_gear_ratios(), 467835);
    }

    #[test]
    fn sum_gear_ratios_2() {
        const INPUT: &str = concat!(
            "........897..\n",
            "........*....\n",
            "...350..847..\n",
            ".....&.......\n",
        );
        let schematic = Schematic::from(INPUT);
        assert_eq!(schematic.sum_gear_ratios(), 759759);
    }

    #[test]
    #[ignore]
    fn print() {
        const INPUT: &str = include_str!("input.txt");
        let schematic = Schematic::from(INPUT);
        schematic.print(140, 140);
    }
}
