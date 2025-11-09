use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("file not parsed");

    fn convert_input_to_coordinate(c: char) -> (i32, i32) {
        match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => (0, 0),
        }
    }

    let mut santa_current = (0, 0);
    let mut robo_santa_current = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(santa_current);
    visited.insert(robo_santa_current);

    for (i, c) in contents.chars().enumerate() {
        let delta = convert_input_to_coordinate(c);
        if i % 2 == 0 {
            santa_current = (santa_current.0 + delta.0, santa_current.1 + delta.1);
            visited.insert(santa_current);
        } else {
            robo_santa_current = (
                robo_santa_current.0 + delta.0,
                robo_santa_current.1 + delta.1,
            );
            visited.insert(robo_santa_current);
        }
    }

    println!("Unique houses checked: {}", visited.len());
}
