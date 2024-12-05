use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn p1(text: String) -> usize {
    let line_re = Regex::new(r"^(?<rule>(?<left>\d+)\|(?<right>\d+))|(?<line>(:?\d,?)+)$").unwrap();
    let mut rules_map: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut sum = 0;

    for line in text.lines() {
        let Some(cap) = line_re.captures(line) else {
            continue;
        };

        if cap.name("rule").is_some() {
            let left = cap["left"].parse::<usize>().unwrap();
            let right = cap["right"].parse::<usize>().unwrap();

            rules_map
                .entry(left)
                .and_modify(|v| v.push(right))
                .or_insert(vec![right]);
        }

        if cap.name("line").is_some() {
            let list: Vec<_> = cap["line"]
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            if list.is_sorted_by(|a, b| {
                if rules_map.contains_key(a) {
                    return rules_map.get(a).unwrap().contains(b);
                }
                !rules_map.get(b).unwrap().contains(a)
            }) {
                sum += list[list.len() / 2];
            }
        }
    }

    sum
}

fn main() {
    let example = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let text = fs::read_to_string("./5/input.txt").expect("couldn't read file");
    let s_example = p1(example.to_string());
    let s = p1(text);

    println!("example: {s_example}");
    println!("s: {s}");
}
