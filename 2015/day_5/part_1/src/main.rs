use std::fs::File;
use std::io::{self, BufRead, BufReader};

const BAD_SUBSTRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];
const VOWELS: &str = "aeiou";

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut nice_string_counter = 0;

    'lines:for line_result in reader.lines() {
        let line = line_result?;

        for bad_substring in BAD_SUBSTRINGS {
            if line.contains(bad_substring) {
                continue 'lines;
            }   
        }

        let mut vowel_count: u32 = 0;
        let mut previous_char: char = ' ';
        let mut double_char_found: bool = false;

        for char in line.chars() {
            if VOWELS.contains(char) {
                vowel_count += 1;
            }
            if char == previous_char {
                double_char_found = true;
            } 
            previous_char = char;
        }

        if vowel_count >= 3 && double_char_found {
            nice_string_counter += 1;
        }
    }

    println!("Total amount of nice strings: {}", nice_string_counter);

    Ok(())
}
