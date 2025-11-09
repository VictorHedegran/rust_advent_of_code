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

    let char_vector: Vec<char> = contents.chars().collect();

    let mut houses_checked: Vec<(i32, i32)> = char_vector
        .iter()
        .map(|c: &char| convert_input_to_coordinate(*c))
        .fold(vec![(0, 0)], |mut acc, x| {
            let (last_x, last_y) = acc.last().copied().unwrap_or((0, 0));
            acc.push((last_x + x.0, last_y + x.1));
            acc
        });

    houses_checked.sort();
    houses_checked.dedup();

    println!("Unique houses checked: {:?}", houses_checked.len());
}

// chat gpt (I like this solution way more, but want to keep my own code as well :p)
/*
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

    let mut current = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(current);

    for c in contents.chars() {
        let delta = convert_input_to_coordinate(c);
        current = (current.0 + delta.0, current.1 + delta.1);
        visited.insert(current);
    }

    println!("Unique houses checked: {}", visited.len());
}
 */
