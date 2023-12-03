use day_3::Schematic;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let schematic = Schematic::from(INPUT);
    println!("Sum of Part Numbers: {}", schematic.sum_part_no());
    // 556057
    println!("Sum of Gear Ratios: {}", schematic.sum_gear_ratios());
    // 82824352
}
