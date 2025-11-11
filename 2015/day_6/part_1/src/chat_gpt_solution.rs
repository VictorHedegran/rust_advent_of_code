use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // 2D array faster than HashMap
    let mut lights = [[false; 1000]; 1000];

    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;

        // Determine the instruction type
        let instruction = if line.starts_with("turn on") {
            "turn on"
        } else if line.starts_with("turn off") {
            "turn off"
        } else {
            "toggle"
        };

        // Extract the coordinates part and split into start and end
        let coords_part = line.strip_prefix(instruction).unwrap().trim();
        let (start_str, end_str) = coords_part.split_once(" through ").unwrap();

        let (x1, y1) = start_str
            .split_once(',')
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .unwrap();
        let (x2, y2) = end_str
            .split_once(',')
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .unwrap();

        // Apply the instruction to all lights in the rectangle
        for x in x1..=x2 {
            for y in y1..=y2 {
                lights[x][y] = match instruction {
                    "turn on" => true,
                    "turn off" => false,
                    "toggle" => !lights[x][y],
                    _ => lights[x][y],
                };
            }
        }
    }

    // Count all lights that are on
    let total_on = lights.iter().flatten().filter(|&&b| b).count();
    println!("Total amount of lights on: {}", total_on);

    Ok(())
}
