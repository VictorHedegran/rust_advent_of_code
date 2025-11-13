use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let mut known_wires: Vec<(String, u16)> = Vec::new();
    known_wires.push(("b".to_string(), 46065));
    let mut call_stack: Vec<String> = Vec::new();

    let file: File = File::open("src/input.txt")?;
    let reader: BufReader<File> = BufReader::new(file);

    for line_result in reader.lines() {
        let line: String = line_result?;

        call_stack.push(line);
    }

    while call_stack.len() > 0 {
        let current_stack = call_stack.clone();
        call_stack.clear();
        for line in current_stack {
            let wire = line.split(" -> ").nth(1).unwrap().to_string();

            if wire == "b" {
                continue;
            }

            let instruction = line.split(" -> ").nth(0).unwrap().to_string();

            if instruction.contains("OR") {
                let first = instruction.split(" OR ").nth(0).unwrap();
                let second = instruction.split(" OR ").nth(1).unwrap();

                let first_value: u16 = match first.parse::<u16>() {
                    Ok(num) => num,
                    Err(_) => {
                        if let Some((_, num)) =
                            known_wires.iter().find(|(wire_name, _)| wire_name == first)
                        {
                            *num
                        } else {
                            call_stack.push(line.clone());
                            continue;
                        }
                    }
                };

                let second_value: u16 = match second.parse::<u16>() {
                    Ok(num) => num,
                    Err(_) => {
                        if let Some((_, num)) = known_wires.iter().find(|(wire, _)| *wire == second)
                        {
                            *num
                        } else {
                            call_stack.push(line.clone());
                            continue;
                        }
                    }
                };

                known_wires.push((wire.clone(), first_value | second_value));
                continue;
            }

            if instruction.contains("AND") {
                let first = instruction.split(" AND ").nth(0).unwrap();
                let second = instruction.split(" AND ").nth(1).unwrap();

                let first_value: u16 = match first.parse::<u16>() {
                    Ok(num) => num,
                    Err(_) => {
                        if let Some((_, num)) =
                            known_wires.iter().find(|(wire_name, _)| wire_name == first)
                        {
                            *num
                        } else {
                            call_stack.push(line.clone());
                            continue;
                        }
                    }
                };

                let second_value: u16 = match second.parse::<u16>() {
                    Ok(num) => num,
                    Err(_) => {
                        if let Some((_, num)) = known_wires.iter().find(|(wire, _)| *wire == second)
                        {
                            *num
                        } else {
                            call_stack.push(line.clone());
                            continue;
                        }
                    }
                };

                known_wires.push((wire.clone(), first_value & second_value));
                continue;
            }

            if instruction.contains("RSHIFT") {
                let first = instruction.split(" RSHIFT ").nth(0).unwrap();
                let second = instruction.split(" RSHIFT ").nth(1).unwrap();

                let first_value: u16 = match first.parse::<u16>() {
                    Ok(num) => num,
                    Err(_) => {
                        if let Some((_, num)) =
                            known_wires.iter().find(|(wire_name, _)| wire_name == first)
                        {
                            *num
                        } else {
                            call_stack.push(line.clone());
                            continue;
                        }
                    }
                };

                let second_value: u16 = match second.parse::<u16>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid RSHIFT value: {}", second),
                        ));
                    }
                };

                known_wires.push((wire.clone(), first_value >> second_value));
                continue;
            }

            if instruction.contains("LSHIFT") {
                let first = instruction.split(" LSHIFT ").nth(0).unwrap();
                let second = instruction.split(" LSHIFT ").nth(1).unwrap();

                let first_value: u16 = match first.parse::<u16>() {
                    Ok(num) => num,
                    Err(_) => {
                        if let Some((_, num)) =
                            known_wires.iter().find(|(wire_name, _)| wire_name == first)
                        {
                            *num
                        } else {
                            call_stack.push(line.clone());
                            continue;
                        }
                    }
                };

                let second_value: u16 = match second.parse::<u16>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid LSHIFT value: {}", second),
                        ));
                    }
                };

                known_wires.push((wire.clone(), first_value << second_value));
                continue;
            }

            if instruction.contains("NOT") {
                let first = instruction.strip_prefix("NOT ").unwrap();

                let first_value: u16 = match first.parse::<u16>() {
                    Ok(num) => num,
                    Err(_) => {
                        if let Some((_, num)) =
                            known_wires.iter().find(|(wire_name, _)| wire_name == first)
                        {
                            *num
                        } else {
                            call_stack.push(line.clone());
                            continue;
                        }
                    }
                };

                known_wires.push((wire.clone(), !first_value));
                continue;
            }

            let first = instruction.trim();

            let first_value: u16 = match first.parse::<u16>() {
                Ok(num) => num,
                Err(_) => {
                    if let Some((_, num)) =
                        known_wires.iter().find(|(wire_name, _)| wire_name == first)
                    {
                        *num
                    } else {
                        call_stack.push(line.clone());
                        continue;
                    }
                }
            };

            known_wires.push((wire.clone(), first_value));
            continue;
        }
    }

    if let Some((_, value)) = known_wires.iter().find(|(wire_name, _)| wire_name == "a") {
        println!("Wire 'a' has the value of: {}", value);
    } else {
        println!("Wire 'a' not found.");
    }

    Ok(())
}
