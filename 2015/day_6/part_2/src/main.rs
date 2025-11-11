use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let mut lights: [[u8; 1000]; 1000] = [[0; 1000]; 1000];

    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;

        let instruction = if line.starts_with("turn on") {
            "turn on"
        } else if line.starts_with("turn off") {
            "turn off"
        } else {
            "toggle"
        };

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

        for x in x1..=x2 {
            for y in y1..=y2 {
                lights[x][y] = match instruction {
                    "turn on" => lights[x][y] + 1,
                    "turn off" => lights[x][y].saturating_sub(1),
                    "toggle" => lights[x][y] + 2,
                    _ => lights[x][y],
                };
            }
        }
    }

    let total_on: usize = lights.iter().flatten().map(|&e| e as usize).sum();
    println!("Total amount of lights on: {}", total_on);

    Ok(())
}
