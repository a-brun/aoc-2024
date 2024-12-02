use std::fmt::Display;
use std::time::Instant;
use itertools::Itertools;

use aoc_2024::get_input_as_string;

fn calculate_difference(input: &str) -> u32 {
    let mut difference: u32 = 0;
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        };

        let location_ids: Vec<u32> = line.split(" ").filter(|lid| !lid.is_empty()).map(|lid| lid.parse().unwrap_or(0)).collect::<Vec<u32>>();

        left.push(*location_ids.first().unwrap());
        right.push(*location_ids.last().unwrap());
    }

    left.sort();
    right.sort();

    for i in 0..left.len() {
        if left[i] > right[i] {
            difference += left[i] - right[i];
        } else {
            difference += right[i] - left[i];
        }
    }

    difference
}

fn calculate_similarity_code(input: &str) -> u32 {
    let mut similarity: u32 = 0;
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }

        let location_ids: Vec<u32> = line.split(" ").filter(|lid| !lid.is_empty()).map(|lid| lid.parse().unwrap_or(0)).collect::<Vec<u32>>();

        left.push(*location_ids.first().unwrap());
        right.push(*location_ids.last().unwrap());
    }

    let counts = right.into_iter().counts();

    for element in left {
        similarity += element * u32::try_from(*counts.get(&element).unwrap_or(&0)).unwrap_or(0);
    }

    similarity
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = calculate_difference(input);
    let p2 = calculate_similarity_code(input);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day01.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::calculate_difference;
    use crate::calculate_similarity_code;

    #[test]
    fn test_p1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let res = calculate_difference(&input);

        assert_eq!(11, res);
    }

    #[test]
    fn test_p2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let res = calculate_similarity_code(&input);

        assert_eq!(31, res);
    }

}

