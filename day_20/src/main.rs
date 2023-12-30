use day_20::Schema;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!(
        "Pulse Propagation: {}",
        Schema::parse(INPUT).pulse_propogation()
    );
    // 896998430
    println!("Pulse to rx: {}", Schema::parse(INPUT).pulse_to_rx());
}
#[cfg(test)]
mod tests {

    #[test]
    fn pulse_propogation() {
        const INPUT: &str = concat! {
            "broadcaster -> a, b, c\n",
            "%a -> b\n",
            "%b -> c\n",
            "%c -> inv\n",
            "&inv -> a",
        };
        let result = super::Schema::parse(INPUT).pulse_propogation();
        assert_eq!(result, 32_000_000);
    }

    #[test]
    fn pulse_propogation_2() {
        const INPUT: &str = concat! {
            "broadcaster -> a\n",
            "%a -> inv, con\n",
            "&inv -> b\n",
            "%b -> con\n",
            "&con -> output",
        };
        let result = super::Schema::parse(INPUT).pulse_propogation();
        assert_eq!(result, 11_687_500);
    }
}
