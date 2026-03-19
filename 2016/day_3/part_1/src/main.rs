use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("no input file provided");

    let mut valid_triangle_count = 0;

    for line in input.lines() {
        let mut sides: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("not a valid side"))
            .collect();

        sides.sort();

        if sides[0] + sides[1] > sides[2] {
            valid_triangle_count += 1
        };
    }

    println!("{}", valid_triangle_count);
}
