use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut nice_string_counter = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let chars: Vec<char> = line.chars().collect();

        let mut has_repeated_pair = false;
        let mut has_repeat_with_gap = false;

        // Check for repeated pairs without overlap
        'outer: for i in 0..chars.len().saturating_sub(1) {
            let pair = (chars[i], chars[i + 1]);
            for j in i + 2..chars.len().saturating_sub(1) {
                if (chars[j], chars[j + 1]) == pair {
                    has_repeated_pair = true;
                    break 'outer;
                }
            }
        }

        // Check for letter repeating with one between
        for i in 0..chars.len().saturating_sub(2) {
            if chars[i] == chars[i + 2] {
                has_repeat_with_gap = true;
                break;
            }
        }

        if has_repeated_pair && has_repeat_with_gap {
            nice_string_counter += 1;
        }
    }

    println!("Total amount of nice strings: {}", nice_string_counter);
    Ok(())
}
