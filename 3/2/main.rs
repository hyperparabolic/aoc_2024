use regex::Regex;
use std::fs;

enum Instruction {
    Do(bool),
    Mul { left: isize, right: isize },
}

fn main() {
    let text = fs::read_to_string("./3/input.txt").expect("couldn't read file");
    let instruction_re = Regex::new(
        r"(?:mul\((?<left>\d{1,3}),(?<right>\d{1,3})\))|(?<enable>do\(\))|(?<disable>don't\(\))",
    )
    .unwrap();

    let captures_iter = instruction_re.captures_iter(&text);

    let result = captures_iter
        .map(|cap| {
            if cap.name("enable").is_some() {
                return Instruction::Do(true);
            }
            if cap.name("disable").is_some() {
                return Instruction::Do(false);
            }
            let left = &cap["left"].parse::<isize>().unwrap();
            let right = &cap["right"].parse::<isize>().unwrap();
            Instruction::Mul {
                left: *left,
                right: *right,
            }
        })
        .fold(
            (0, true),
            |(sum, enable): (isize, bool), instruction: Instruction| match instruction {
                Instruction::Do(e) => (sum, e),
                Instruction::Mul { left, right } => {
                    if !enable {
                        return (sum, enable);
                    }
                    (sum + (left * right), enable)
                }
            },
        );
    let sum = result.0;
    println!("{sum}");
}
