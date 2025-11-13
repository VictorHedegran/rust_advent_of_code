use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_value(token: &str, wires: &HashMap<String, u16>) -> Option<u16> {
    token
        .parse::<u16>()
        .ok()
        .or_else(|| wires.get(token).copied())
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let mut instructions: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut wires: HashMap<String, u16> = HashMap::new();

    while !instructions.is_empty() {
        let mut pending = Vec::new();
        for line in instructions {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let instruction = parts[0];
            let wire = parts[1];

            if let Some(value) = if instruction.contains("AND") {
                let ops: Vec<&str> = instruction.split(" AND ").collect();
                Some(get_value(ops[0], &wires)? & get_value(ops[1], &wires)?)
            } else if instruction.contains("OR") {
                let ops: Vec<&str> = instruction.split(" OR ").collect();
                Some(get_value(ops[0], &wires)? | get_value(ops[1], &wires)?)
            } else if instruction.contains("LSHIFT") {
                let ops: Vec<&str> = instruction.split(" LSHIFT ").collect();
                Some(get_value(ops[0], &wires)? << get_value(ops[1], &wires)?)
            } else if instruction.contains("RSHIFT") {
                let ops: Vec<&str> = instruction.split(" RSHIFT ").collect();
                Some(get_value(ops[0], &wires)? >> get_value(ops[1], &wires)?)
            } else if instruction.starts_with("NOT") {
                Some(!get_value(
                    instruction.strip_prefix("NOT ").unwrap(),
                    &wires,
                )?)
            } else {
                get_value(instruction, &wires)
            } {
                wires.insert(wire.to_string(), value);
            } else {
                pending.push(line);
            }
        }
        instructions = pending;
    }

    println!("Wire 'a' has value: {}", wires["a"]);
    Ok(())
}
