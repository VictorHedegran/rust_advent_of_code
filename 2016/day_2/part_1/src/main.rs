use std::fs;

fn main() {
    const GRID_SIZE: (i32, i32) = (2, 2); // 3x3 0 index

    let mut x = GRID_SIZE.0 / 2;
    let mut y = GRID_SIZE.1 / 2;
    let mut code: String = String::new();
    let input = fs::read_to_string("input.txt").expect("no input string provided");
    for line in input.lines() {
        for char in line.chars() {
            match char {
                'R' => {
                    if x < GRID_SIZE.0 {
                        x += 1
                    }
                }
                'L' => {
                    if x > 0 {
                        x -= 1
                    }
                }
                'U' => {
                    if y > 0 {
                        y -= 1
                    }
                }
                'D' => {
                    if y < GRID_SIZE.1 {
                        y += 1
                    }
                }
                _ => panic!("invalid direction"),
            }
        }
        code.push_str(&((x + 1) + (y * 3)).to_string());
    }
    println!("{}", code)
}
