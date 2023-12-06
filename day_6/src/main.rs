const INPUT: &str = include_str!("input.txt");

fn main() {
    println!(
        "Product of winning strategy count: {}",
        winning_start_count(INPUT)
    );
    //1108800

    println!("Single race: {}", single_race(INPUT));
    // 36919753
}

fn winning_start_count(input: &str) -> u64 {
    let mut lines = input.lines().map(|line| {
        let (_tag, values) = line.split_once(':').unwrap();
        values
            .split_whitespace()
            .map(str::parse::<u64>)
            .map(Result::unwrap)
    });
    lines
        .next()
        .unwrap()
        .zip(lines.next().unwrap())
        .map(|(time, best_distance)| winning_strategy_count(time, best_distance))
        .product()
}

fn single_race(input: &str) -> u64 {
    let mut lines = input.lines().map(|line| {
        let (_tag, values) = line.split_once(':').unwrap();
        values.replace(' ', "").parse::<u64>().unwrap()
    });
    let (time, best_distance) = (lines.next().unwrap(), lines.next().unwrap());

    winning_strategy_count(time, best_distance)
}

fn winning_strategy_count(time: u64, best_distance: u64) -> u64 {
    // t + sqrt(t^2 - 4 b) - ceil(t - sqrt(t^2 - 4 b)) + 1
    let b = f64::sqrt((time.pow(2) - 4 * (best_distance + 1)) as f64);
    let x_0 = (time as f64 - b) / 2.0;
    let x_1 = (time as f64 + b) / 2.0;
    let result = x_1 - x_0.ceil() + 1.0;
    result as _
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn product_of_winning_strategy_count() {
        let result = super::winning_start_count(INPUT);
        assert_eq!(result, 288)
    }

    #[test]
    fn single_race() {
        let result = super::single_race(INPUT);
        assert_eq!(result, 71503)
    }
}
