const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Sum of predictions: {}", predict(INPUT));
}

fn predict(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut rows = Vec::new();
            rows.push(
                line.split_whitespace()
                    .map(str::parse::<i32>)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap(),
            );

            while !rows.last().unwrap().iter().all(|val| *val == 0) {
                rows.push(
                    rows.last()
                        .unwrap()
                        .windows(2)
                        .map(|pair| {
                            let [x, y] = pair else { panic!() };
                            y - x
                        })
                        .collect(),
                )
            }

            rows.into_iter()
                .rev()
                .fold(0, |acc, cur| cur.last().unwrap() + acc)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn predict() {
        const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        let result = super::predict(INPUT);
        assert_eq!(result, 114);
    }
}
