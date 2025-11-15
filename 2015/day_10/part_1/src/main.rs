use ::std::fs;

const MAX_ITERATION: usize = 40;

fn main() {
    let content = fs::read_to_string("src/input.txt").unwrap();

    fn look_and_say(current_string: String, current_iteration: usize) -> usize {
        println!("DEBUG {}, {}", current_string, current_iteration);
        let mut previous_chars: Option<(char, usize)> = None;

        let mut next_string: String = String::new();

        current_string.chars().for_each(|c| match previous_chars {
            Some(prev) => {
                if prev.0 == c {
                    previous_chars = Some((prev.0, prev.1 + 1))
                } else {
                    next_string += &format!("{}{}", prev.1, prev.0);
                    previous_chars = Some((c, 1))
                }
            }
            None => previous_chars = Some((c, 1)),
        });

        match previous_chars {
            Some(prev) => {
                next_string += &format!("{}{}", prev.1, prev.0);
            }
            None => {}
        }

        if current_iteration < MAX_ITERATION {
            look_and_say(next_string, current_iteration + 1)
        } else {
            next_string.len()
        }
    }

    println!("{}", look_and_say(content, 1));
}
