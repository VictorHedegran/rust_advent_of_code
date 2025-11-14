use std::fs;

fn encode_string(s: &str) -> String {
    format!("\"{}\"", s.replace("\\", "\\\\").replace("\"", "\\\""))
}

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("src/input.txt")?;

    let character_diff: usize = content
        .lines()
        .map(|line| encode_string(line).len() - line.len())
        .sum();

    println!("After encoding, we use {} more characters!", character_diff);

    Ok(())
}
