use std::fs;

#[derive(Eq, PartialEq)]
enum Direction {
    Decreasing, // safely decreasing
    Increasing, // safely increasing
    Unsafe,     // unsafe
}

fn get_direction(previous_level: &i32, current_level: &i32) -> Direction {
    let change = current_level - previous_level;

    match change {
        1..=3 => Direction::Increasing,
        -3..=-1 => Direction::Decreasing,
        _ => Direction::Unsafe,
    }
}

fn are_levels_safe(levels: Vec<i32>) -> bool {
    let mut iter = levels.iter();
    // perform first comparison to seed fold
    let initial = iter.next().expect("must be 2 digits to compare");
    let next = iter.next().expect("must be 2 digits to compare");
    let initial_direction = get_direction(initial, next);

    let result = iter.fold(
        (next, initial_direction),
        |(previous_level, previous_direction): (&i32, Direction), current_level: &i32| {
            // always unsafe if determined unsafe
            if previous_direction == Direction::Unsafe {
                return (current_level, previous_direction);
            }

            let next_direction = get_direction(previous_level, current_level);

            if previous_direction != next_direction {
                return (current_level, Direction::Unsafe);
            }

            (current_level, next_direction)
        },
    );
    result.1 != Direction::Unsafe
}

fn main() {
    let text = fs::read_to_string("./2/input.txt").expect("couldn't read file");
    let levels = text.trim().lines();

    let mut safe_count: i32 = 0;

    for level_strings in levels {
        let split_levels: Vec<i32> = level_strings
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if are_levels_safe(split_levels) {
            safe_count += 1;
        }
    }

    println!("{safe_count}")
}
