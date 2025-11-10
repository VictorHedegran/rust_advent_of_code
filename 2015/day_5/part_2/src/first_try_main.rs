use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut nice_string_counter = 0;

    for line_result in reader.lines() {
        let line: String = line_result?;
        let chars: Vec<char> = line.chars().collect();

        let mut has_matched_adjacent_chars: bool = false;
        let mut has_matched_skipped_chars: bool = false;

        let mut adjacent_char_tokens: Vec<String> = Vec::new();
        let mut skipped_char_tokens: Vec<(char, char)> = Vec::new();
        
        for i in 0..chars.len() - 1 {
            adjacent_char_tokens.push(format!("{}{}",chars[i], chars[i + 1]))
        }

        for i in 0..chars.len() - 2 {
            skipped_char_tokens.push((chars[i], chars[i + 2]))
        }

        for token in adjacent_char_tokens {
            let matches: Vec<&str> = line.matches(&token).collect();
            if matches.len() >= 2 {
                has_matched_adjacent_chars = true;
            }
        }

        for token in skipped_char_tokens {
            if token.0 == token.1 {
                has_matched_skipped_chars = true;
            }
        }

    

        if has_matched_adjacent_chars && has_matched_skipped_chars {
            nice_string_counter += 1;
        }
    }

    println!("Total amount of nice strings: {}", nice_string_counter);

    Ok(())
}
