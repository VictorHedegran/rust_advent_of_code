use std::fs;

fn convert_input_to_integer(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("file not parsed");

    let mut floor: i32 = 0;

    for (i, val) in contents.chars().enumerate() {
        floor += convert_input_to_integer(val);
        if floor < 0 {
            println!("basement {}", i + 1);
            return;
        }
    }
}
