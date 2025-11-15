use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

fn main() {
    let content = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");

    let mut distances = HashMap::new();
    let mut cities = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(" = ").collect();
        let distance = parts[1].parse::<usize>().unwrap();

        let pair: Vec<&str> = parts[0].split(" to ").collect();
        let a = pair[0];
        let b = pair[1];

        distances.insert((a, b), distance);
        distances.insert((b, a), distance);

        if !cities.contains(&a) {
            cities.push(a);
        }
        if !cities.contains(&b) {
            cities.push(b);
        }
    }

    let mut max_distance = usize::MAX;

    // Generate all permutations recursively
    fn permute<T: Copy + Eq + Hash>(
        cities: &mut Vec<T>,
        start: usize,
        distances: &HashMap<(T, T), usize>,
        max_distance: &mut usize,
    ) {
        if start == cities.len() {
            let mut sum = 0;
            for w in cities.windows(2) {
                sum += distances[&(w[0], w[1])];
            }
            if sum > *max_distance {
                *max_distance = sum;
            }
        } else {
            for i in start..cities.len() {
                cities.swap(start, i);
                permute(cities, start + 1, distances, max_distance);
                cities.swap(start, i);
            }
        }
    }

    permute(&mut cities, 0, &distances, &mut max_distance);

    println!("Shortest route: {}", max_distance);
}
