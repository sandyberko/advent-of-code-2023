use std::array;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Hash sum: {}", hash_sum(INPUT));
    println!("HashMap: {}", hash_map(INPUT));
}

fn hash_sum(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn hash(s: &str) -> usize {
    let mut current_value = 0usize;
    for c in s.chars().filter(|c| *c != '\n') {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn hash_map(ops: &str) -> usize {
    let mut hash_map: [Vec<(&str, usize)>; 256] = array::from_fn(|_| Vec::new());

    for op in ops.split(',') {
        let op = op.trim();
        if let Some(key) = op.strip_suffix('-') {
            hash_map[hash(key)].retain(|(ex_key, _)| key != *ex_key);
        } else if let Some((key, value)) = op.split_once('=') {
            let r#box = &mut hash_map[hash(key)];
            if let Some((_, ex_value)) = r#box.iter_mut().find(|(ex_key, _)| *ex_key == key) {
                *ex_value = value.parse().unwrap();
            } else {
                r#box.push((key, value.parse().unwrap()));
            }
        } else {
            panic!("invalid operation {op:?}");
        }
    }

    hash_map
        .iter()
        .enumerate()
        .flat_map(|(i_box, r#box)| {
            r#box
                .iter()
                .enumerate()
                .map(move |(i_kv, (_, value))| (i_box + 1) * (i_kv + 1) * value)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash() {
        let result = super::hash_sum(INPUT);
        assert_eq!(result, 1320);
    }

    #[test]
    fn hashmap() {
        let result = super::hash_map(INPUT);
        assert_eq!(result, 145);
    }
}
