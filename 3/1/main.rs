use regex::Regex;
use std::fs;

fn main() {
    let text = fs::read_to_string("./3/input.txt").expect("couldn't read file");
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: i32 = mul_re
        .captures_iter(&text)
        .map(|caps| {
            let (_, [l, r]) = caps.extract();
            (l, r)
        })
        .map(|pair| {
            (
                pair.0.to_string().parse::<i32>().unwrap(),
                pair.1.to_string().parse::<i32>().unwrap(),
            )
        })
        .fold(0, |acc, cur| acc + (cur.0 * cur.1));
    println!("{sum}");
}
