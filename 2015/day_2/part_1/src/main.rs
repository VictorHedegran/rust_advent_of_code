use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open the input file
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut total_paper = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let dims: Vec<u32> = line
            .split('x')
            .map(|s| s.parse::<u32>().expect("Invalid number"))
            .collect();

        if dims.len() == 3 {
            let (l, w, h) = (dims[0], dims[1], dims[2]);
            let side1 = l * w;
            let side2 = w * h;
            let side3 = h * l;

            let smallest_side = *[side1, side2, side3].iter().min().unwrap();
            let surface_area = 2 * (side1 + side2 + side3);
            total_paper += surface_area + smallest_side;
        }
    }

    println!("Total wrapping paper: {}", total_paper);

    Ok(())
}
