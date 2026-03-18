use std::fs;

fn main() {
    const DIRECTION_COORDINATE_CHANGES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    const RIGHT: char = 'R';
    const LEFT: char = 'L';

    let input: String = fs::read_to_string("input.txt").expect("Failed to parse input");
    let input: String = input.replace(",", "");
    let steps: Vec<&str> = input.split_whitespace().collect();

    let mut position = (0, 0);
    let mut current_direction = 0;

    for step in steps {
        let direction_char = step.chars().next().expect("no direction");
        let direction_length: i32 = step[1..].parse().expect("not a number");

        if direction_char == LEFT {
            if current_direction == 0 {
                current_direction = 3
            } else {
                current_direction -= 1
            }
        }

        if direction_char == RIGHT {
            if current_direction == 3 {
                current_direction = 0
            } else {
                current_direction += 1
            }
        }

        position = (
            position.0 + DIRECTION_COORDINATE_CHANGES[current_direction].0 * direction_length,
            position.1 + DIRECTION_COORDINATE_CHANGES[current_direction].1 * direction_length,
        );

        println!("{:?}", position)
    }

    let x = if position.0 < 0 {
        -position.0
    } else {
        position.0
    };
    let y = if position.1 < 0 {
        -position.1
    } else {
        position.1
    };

    println!("Position is x: {x}, y: {y} which is a length of {}", x + y)
}
