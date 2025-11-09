use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("file not parsed");

    fn convert_input_to_integer(c: char) -> i32 {
        if c == '(' {
            return 1
        } else if c == ')' {
            return -1
        } else {
            return 0
        }
    }

    let char_vector: Vec<char> = contents.chars().collect();

    let int_vector: Vec<i32> = char_vector.iter().map(|c: &char| convert_input_to_integer(*c)).collect();

    let sum: i32 = int_vector.iter().fold(0, |acc: i32, &x| acc + x);
    
    println!("{:?}", sum);
}
