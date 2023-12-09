const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Sum of predictions: {}", predict(INPUT));
    println!("Back: {}", extrapolate_back(INPUT));
}

fn predict(input: &str) -> i32 {
    piramids(input)
        .map(|rows| {
            rows.into_iter()
                .rev()
                .fold(0, |acc, cur| cur.last().unwrap() + acc)
        })
        .sum()
}
fn extrapolate_back(input: &str) -> i32 {
    piramids(input)
        .map(|rows| {
            rows.into_iter()
                .rev()
                .fold(0, |acc, cur| cur.first().unwrap() - acc)
        })
        .sum()
}

fn piramids(input: &str) -> impl Iterator<Item = Vec<Vec<i32>>> + '_ {
    input.lines().map(|line| {
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
        rows
    })
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn predict() {
        let result = super::predict(INPUT);
        assert_eq!(result, 114);
    }

    #[test]
    fn extrapolate_back() {
        let result = super::extrapolate_back(INPUT);
        assert_eq!(result, 2);
    }
}
