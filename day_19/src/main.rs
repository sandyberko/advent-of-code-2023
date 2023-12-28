use std::{array, collections::HashMap, ops::Range};

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Accepted: {}", workflows(INPUT));
}

#[derive(Clone, Copy)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

enum Op {
    Lt,
    Gt,
}

struct Cond {
    cat: Category,
    op: Op,
    arg: usize,
}

struct Rule<'s> {
    cond: Option<Cond>,
    dest: &'s str,
}

fn workflows(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = parse_workflows(workflows);

    parts
        .lines()
        .filter_map(|part| {
            let mut part = part
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|cat_val| {
                    cat_val
                        .split_once('=')
                        .map(|(_, val)| val.parse::<usize>().unwrap())
                        .unwrap()
                });
            let (x, m, a, s) = (
                part.next().unwrap(),
                part.next().unwrap(),
                part.next().unwrap(),
                part.next().unwrap(),
            );

            let mut tag = "in";
            'workflows: loop {
                if tag == "A" {
                    return Some(x + m + a + s);
                }
                if tag == "R" {
                    return None;
                }
                let workflow = workflows.get(tag).unwrap();
                'rules: for rule in workflow {
                    if let Some(cond) = &rule.cond {
                        let lhs = match cond.cat {
                            Category::ExtremelyCoolLooking => x,
                            Category::Musical => m,
                            Category::Aerodynamic => a,
                            Category::Shiny => s,
                        };
                        let matched = match cond.op {
                            Op::Lt => lhs < cond.arg,
                            Op::Gt => lhs > cond.arg,
                        };

                        if !matched {
                            continue 'rules;
                        }
                    }

                    tag = rule.dest;
                    continue 'workflows;
                }
                panic!("no rule matched");
            }
        })
        .sum()
}

fn parse_workflows(workflows: &str) -> HashMap<&str, Vec<Rule<'_>>> {
    workflows
        .lines()
        .map(|line| {
            let (tag, rules) = line.split_once('{').unwrap();
            let rules = rules
                .trim_end_matches('}')
                .split(',')
                .map(|rule| {
                    if let Some((cond, dest)) = rule.split_once(':') {
                        let mut cond = cond.chars();
                        let cat = match cond.next().unwrap() {
                            'x' => Category::ExtremelyCoolLooking,
                            'm' => Category::Musical,
                            'a' => Category::Aerodynamic,
                            's' => Category::Shiny,
                            cat => panic!("invalid category {cat}"),
                        };
                        let op = match cond.next().unwrap() {
                            '<' => Op::Lt,
                            '>' => Op::Gt,
                            op => panic!("invalid op {op}"),
                        };
                        let arg = cond.as_str().parse().unwrap();
                        Rule {
                            cond: Some(Cond { cat, op, arg }),
                            dest,
                        }
                    } else {
                        Rule {
                            cond: None,
                            dest: rule,
                        }
                    }
                })
                .collect::<Vec<_>>();
            (tag, rules)
        })
        .collect::<HashMap<_, _>>()
}

fn combinations(input: &str) -> usize {
    let (workflows, _parts) = input.split_once("\n\n").unwrap();

    let workflows = parse_workflows(workflows);

    combs(&workflows, "A", array::from_fn(|_| 1..4001))
}

fn combs(workflows: &HashMap<&str, Vec<Rule<'_>>>, dest: &str, ranges: [Range<usize>; 4]) -> usize {
    if dest == "in" {
        return ranges.iter().map(|range| range.len()).product();
    }
    workflows
        .iter()
        .filter_map(|(tag, rules)| {
            let (i, rule) = rules
                .iter()
                .enumerate()
                .rfind(|(_, rule)| rule.dest == dest)?;

            let mut ranges = ranges.clone();

            if let Some(cond) = &rule.cond {
                let range = &mut ranges[cond.cat as usize];
                match cond.op {
                    Op::Lt => range.end = range.end.min(cond.arg),
                    Op::Gt => range.start = range.start.max(cond.arg + 1),
                }
            }

            for rule in &rules[..i] {
                let cond = rule.cond.as_ref().unwrap();
                let range = &mut ranges[cond.cat as usize];
                match cond.op {
                    Op::Gt => range.end = range.end.min(cond.arg - 1),
                    Op::Lt => range.start = range.start.max(cond.arg),
                }
            }

            if ranges.iter().any(|range| range.is_empty()) {
                return None;
            }

            Some(combs(workflows, tag, ranges))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat! {
        "px{a<2006:qkq,m>2090:A,rfg}\n",
        "pv{a>1716:R,A}\n",
        "lnx{m>1548:A,A}\n",
        "rfg{s<537:gd,x>2440:R,A}\n",
        "qs{s>3448:A,lnx}\n",
        "qkq{x<1416:A,crn}\n",
        "crn{x>2662:A,R}\n",
        "in{s<1351:px,qqz}\n",
        "qqz{s>2770:qs,m<1801:hdj,R}\n",
        "gd{a>3333:R,R}\n",
        "hdj{m>838:A,pv}\n",
        "\n",
        "{x=787,m=2655,a=1222,s=2876}\n",
        "{x=1679,m=44,a=2067,s=496}\n",
        "{x=2036,m=264,a=79,s=2244}\n",
        "{x=2461,m=1339,a=466,s=291}\n",
        "{x=2127,m=1623,a=2188,s=1013}\n",
    };

    #[test]
    fn workflows() {
        let result = super::workflows(INPUT);
        assert_eq!(result, 19114);
    }

    #[test]
    fn combinations() {
        let result = super::combinations(INPUT);
        assert_eq!(result, 167_409_079_868_000);
    }
}
