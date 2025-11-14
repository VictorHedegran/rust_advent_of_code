use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn decode_ascii_characters(s: &str) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < s.len() {
        if i + 3 < s.len() && &s[i..i + 2] == "\\x" {
            if let Ok(val) = u8::from_str_radix(&s[i + 2..i + 4], 16) {
                result.push(val as char);
                i += 4;
                continue;
            }
        }
        let ch = s[i..].chars().next().unwrap();
        result.push(ch);
        i += ch.len_utf8();
    }

    result
}

fn decode_string(encoded_string: &str) -> String {
    decode_ascii_characters(
        &encoded_string[1..encoded_string.len() - 1]
            .replace("\\\\", "\\")
            .replace("\\\"", "\""),
    )
}

fn main() -> io::Result<()> {
    let mut character_diff: u32 = 0;

    let file: File = File::open("src/input.txt")?;
    let reader: BufReader<File> = BufReader::new(file);

    for line_result in reader.lines() {
        let line: String = line_result?;

        let decoded_line: String = decode_string(&line);

        character_diff += line.chars().count() as u32 - decoded_line.chars().count() as u32;

        println!(
            "Line: {} Decoded Line: {} Character_diff: {}",
            line,
            decoded_line,
            line.len() as u32 - decoded_line.len() as u32
        )
    }

    println!("After decoding, we use {} less characters!", character_diff);

    Ok(())
}
