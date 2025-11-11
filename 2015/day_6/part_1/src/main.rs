use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn apply_instruction(instruction: &str, value: bool) -> bool {
    match instruction {
        "turn on" => true,
        "turn off" => false,
        "toggle" => !value,
        _ => value,
    }
}

fn main() -> io::Result<()> {
    let mut lights: HashMap<(u32, u32), bool> = HashMap::new();

    let file: File = File::open("src/input.txt")?;
    let reader: BufReader<File> = BufReader::new(file);

    for line_result in reader.lines() {
        let line: String = line_result?;

        let mut tokens: Vec<&str> = line.split(" ").collect();

        let end: Vec<&str> = tokens.pop().unwrap().split(",").collect();
        let end_coords: (u32, u32) = (
            end[0].parse().expect("number"),
            end[1].parse().expect("number"),
        );
        tokens.pop();
        let start: Vec<&str> = tokens.pop().unwrap().split(",").collect();
        let start_coords: (u32, u32) = (
            start[0].parse().expect("number"),
            start[1].parse().expect("number"),
        );
        let instruction: String = tokens.join(" ");

        for x in start_coords.0..=end_coords.0 {
            for y in start_coords.1..=end_coords.1 {
                let value: bool = lights.get(&(x, y)).copied().unwrap_or(false);
                lights.insert((x, y), apply_instruction(&instruction, value));
            }
        }
    }

    println!(
        "Total amount of lights on: {}",
        lights.iter().filter(|&x| *x.1).count()
    );

    Ok(())
}
