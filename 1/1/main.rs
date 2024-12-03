use std::fs;

fn main() {
    let lists = fs::read_to_string("./1/input.txt").expect("couldn't read file");
    let pairs = lists.trim().lines();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for pair in pairs {
        if let Some((a, b)) = pair.split_once("   ") {
            left.push(a.parse().unwrap());
            right.push(b.parse().unwrap());
        }
    }

    left.sort();
    right.sort();

    let distance: i32 = (0..left.len()).fold(0, |acc, cur| acc + (left[cur] - right[cur]).abs());
    println!("{distance}");
}
