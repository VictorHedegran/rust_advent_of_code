use std::collections::HashSet;
use std::fs;

fn main() {
    const DIRECTION_COORDINATE_CHANGES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let input: String = fs::read_to_string("input.txt").expect("Failed to parse input");
    let input: String = input.replace(",", "");
    let steps: Vec<&str> = input.split_whitespace().collect();

    let mut visited_coordinates = HashSet::new();
    let mut position = (0, 0);
    let mut current_direction = 0;

    'steps_loop: for step in steps {
        let direction_char = step.chars().next().expect("no direction");
        let direction_length: i32 = step[1..].parse().expect("not a number");

        match direction_char {
            'L' => current_direction = (current_direction + 3) % 4,
            'R' => current_direction = (current_direction + 1) % 4,
            _ => panic!("invalid direction"),
        }

        let mut i: i32 = 0;
        while i < direction_length {
            position = (
                position.0 + DIRECTION_COORDINATE_CHANGES[current_direction].0,
                position.1 + DIRECTION_COORDINATE_CHANGES[current_direction].1,
            );

            if visited_coordinates.contains(&position) {
                println!(
                    "The position visited twice is {:?}, length is: {}",
                    position,
                    position.0.abs() + position.1.abs()
                );
                break 'steps_loop;
            };

            visited_coordinates.insert(position);

            i += 1;
        }
    }
}
