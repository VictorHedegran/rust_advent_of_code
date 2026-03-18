use std::fs;

fn main() {
    const KEYPAD: [[char; 5]; 5] = [
        ['x', 'x', '1', 'x', 'x'],
        ['x', '2', '3', '4', 'x'],
        ['5', '6', '7', '8', '9'],
        ['x', 'A', 'B', 'C', 'x'],
        ['x', 'x', 'D', 'x', 'x'],
    ];

    let mut x: usize = 0;
    let mut y: usize = 2;

    let mut code = String::new();

    let input = fs::read_to_string("input.txt").expect("no input string provided");

    for line in input.lines() {
        for c in line.chars() {
            match c {
                'R' if x < 4 && KEYPAD[y][x + 1] != 'x' => x += 1,
                'L' if x > 0 && KEYPAD[y][x - 1] != 'x' => x -= 1,
                'U' if y > 0 && KEYPAD[y - 1][x] != 'x' => y -= 1,
                'D' if y < 4 && KEYPAD[y + 1][x] != 'x' => y += 1,
                _ => {}
            }
        }

        code.push(KEYPAD[y][x]);
    }

    println!("{code}");
}
