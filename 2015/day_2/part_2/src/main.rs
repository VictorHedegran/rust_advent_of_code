use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open the input file
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut total_ribbon: u32 = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let mut dims: Vec<u32> = line
            .split('x')
            .map(|s| s.parse::<u32>().expect("Invalid number"))
            .collect();

        dims.sort();

        if dims.len() == 3 {
            let (l, w, h) = (dims[0], dims[1], dims[2]);
            let wrap_length = l + l + w + w;
            let bow_length = l * w * h;
            let ribbon_length = wrap_length + bow_length;

            total_ribbon += ribbon_length;
        }
    }

    println!("Total ribbon required: {}", total_ribbon);

    Ok(())
}
