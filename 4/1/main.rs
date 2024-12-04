use std::fs;

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];

fn main() {
    let text = fs::read_to_string("./4/input.txt").expect("couldn't read file");
    let lines = text.trim().lines();

    let bytes: Vec<Vec<_>> = lines.map(|line| line.bytes().collect()).collect();
    let valid_y = 0..bytes.len() as i32;
    let valid_x = 0..bytes[0].len() as i32;

    let mut count = 0;
    for y in 0..bytes.len() as i32 {
        for x in 0..bytes[0].len() as i32 {
            if bytes[y as usize][x as usize] == XMAS[0] {
                for (y_inc, x_inc) in DIRECTIONS {
                    let mut i = 0;
                    let mut xx = x;
                    let mut yy = y;

                    while i <= 3
                        && valid_x.contains(&xx)
                        && valid_y.contains(&yy)
                        && bytes[yy as usize][xx as usize] == XMAS[i]
                    {
                        yy += y_inc;
                        xx += x_inc;
                        i += 1;
                    }
                    if i == 4 {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{count}")
}
