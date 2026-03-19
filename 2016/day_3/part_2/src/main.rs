use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("no input file provided");

    let mut valid_triangle_count = 0;

    let mut columns: [Vec<&str>; 3] = [Vec::new(), Vec::new(), Vec::new()];

    for line in input.lines() {
        let sides: Vec<&str> = line.split_whitespace().collect();

        for number in 0..3 {
            columns[number].push(sides[number]);
        }
    }

    for column in &mut columns {
        for number in 0..column.len() / 3 {
            let mut triangle: Vec<i32> = column[number * 3..number * 3 + 3]
                .iter()
                .map(|x| x.parse().unwrap())
                .collect();

            triangle.sort();

            if triangle[0] + triangle[1] > triangle[2] {
                valid_triangle_count += 1;
            }
        }
    }

    println!("{}", valid_triangle_count)
}
