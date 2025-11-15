/*
so my rust skills are improving! yay... but my programming and logic is not quite there yet... makes me quite sad!
this way of greedily picking the lowest routes seemed fine on paper, but doesn't work and I seem to have misunderstood the assignment. again.
the correct solution in main, generates all permutations and picks the lowest one. which apparently is the correct solution since the dataset is only eight cities.

... which is okay, I've learned some stuff, line Some, None match flows for optional types! and also the difference between unwrap and expect. which also getting more comfortable with Vector methods
*/

use ::std::fs;

fn main() {
    let content = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");

    let mut shortest_path_to_destination: Vec<(&str, &str, usize)> = Vec::new();

    let mut a: Vec<(usize, &str)> = content
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" = ").collect();
            (
                parts[1].parse::<usize>().expect("invalid distance"),
                parts[0],
            )
        })
        .collect();
    a.sort();

    a.iter().for_each(|data_point| {
        let distance = data_point.0;
        let to_from: Vec<&str> = data_point
            .1
            .split(" to ")
            .collect::<Vec<&str>>()
            .into_iter()
            .rev()
            .collect();

        let entry: Option<&(&str, &str, usize)> = shortest_path_to_destination
            .iter()
            .find(|&&(to, from, _)| to == to_from[0] || from == to_from[1]);

        let should_update = match entry {
            Some(_) => false,
            None => true,
        };

        if should_update {
            shortest_path_to_destination.push((to_from[0], to_from[1], distance))
        }
    });

    println!(
        "Total distance {}",
        shortest_path_to_destination
            .iter()
            .map(|x| { x.2 })
            .sum::<usize>()
    );
}
