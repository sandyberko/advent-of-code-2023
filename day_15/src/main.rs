const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Hash sum: {}", hash_sum(INPUT));
}

fn hash_sum(input: &str) -> usize {
    input
        .split(',')
        .map(|step| {
            let mut current_value = 0usize;
            for c in step.chars().filter(|c| *c != '\n') {
                current_value += c as usize;
                current_value *= 17;
                current_value %= 256;
            }
            current_value
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    #[test]
    fn test() {
        let result = super::hash_sum(INPUT);
        assert_eq!(result, 1320);
    }
}
