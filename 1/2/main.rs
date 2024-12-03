use std::collections::HashMap;
use std::fs;

fn main() {
    let lists = fs::read_to_string("./1/input.txt").expect("couldn't read file");
    let pairs = lists.trim().lines();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    for pair in pairs {
        if let Some((a, b)) = pair.split_once("   ") {
            left_list.push(a.parse().unwrap());
            let right: i32 = b.parse().unwrap();
            right_counts
                .entry(right)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    left_list.sort();

    let similarity: i32 = (0..left_list.len()).fold(0, |acc, cur| {
        acc + (left_list[cur] * right_counts.get(&left_list[cur]).unwrap_or(&0))
    });
    println!("{similarity}")
}
