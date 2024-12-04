use std::fs;

const CORNERS: [(i32, i32); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];

fn main() {
    let text = fs::read_to_string("./4/input.txt").expect("couldn't read file");
    let lines = text.trim().lines();

    let bytes: Vec<Vec<_>> = lines.map(|line| line.bytes().collect()).collect();
    let valid_y = 0..bytes.len() as i32;
    let valid_x = 0..bytes[0].len() as i32;
    let mut corners: [u8; 4];

    let mut count = 0;
    for y in 0..bytes.len() as i32 {
        for x in 0..bytes[0].len() as i32 {
            if bytes[y as usize][x as usize] == b'A' {
                corners = [b'!', b'!', b'!', b'!'];
                for (i, (inc_y, inc_x)) in CORNERS.into_iter().enumerate() {
                    let yy = y + inc_y;
                    let xx = x + inc_x;
                    corners[i] = if valid_y.contains(&yy) && valid_x.contains(&xx) {
                        bytes[yy as usize][xx as usize]
                    } else {
                        b'0'
                    };
                }
                match corners {
                    [b'M', b'M', b'S', b'S'] => count += 1,
                    [b'M', b'S', b'S', b'M'] => count += 1,
                    [b'S', b'S', b'M', b'M'] => count += 1,
                    [b'S', b'M', b'M', b'S'] => count += 1,
                    _ => {}
                }
            }
        }
    }

    println!("{count}")
}
