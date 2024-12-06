use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::time::Instant;

use aoc_2024::{format_duration, get_input_as_string};

fn mean_page(input: &str) -> (u32, u32) {
    let v = input
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
    let [p1, p2] = v.as_slice() else {
        panic!("Invalid input")
    };

    let mut before_reqs: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut sum: u32 = 0;
    let mut unordered_sum: u32 = 0;

    for (left, right) in p1.split("\n").map(|l| {
        l.split("|")
            .map(|v| v.parse::<u32>().expect(&format!("Invalid integer {}", v)))
            .collect_tuple::<(u32, u32)>()
            .unwrap()
    }) {
        before_reqs
            .entry(right)
            .and_modify(|v| {
                v.insert(left);
            })
            .or_insert(HashSet::from([left]));
    }

    for instructions in p2
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(",")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
    {
        if is_ordered(&instructions, &before_reqs) {
            sum += instructions[(instructions.len() - 1) / 2];
        } else {
            let mut valid: Vec<u32> = Vec::new();
            let mut remainings = Vec::from(instructions.clone());

            while valid.len() < instructions.len() {
                for i in 0..remainings.len() {
                    let instruction: u32 = remainings[i];
                    let mut rest: HashSet<u32> = HashSet::from_iter(remainings.iter().cloned());
                    rest.remove(&instruction);

                    if is_position_valid(&rest, &before_reqs, instruction) {
                        remainings.remove(i);
                        valid.push(instruction);
                        break;
                    }
                }
            }

            unordered_sum += valid[(valid.len() - 1) / 2];
        }
    }

    (sum, unordered_sum)
}

fn is_ordered(instructions: &Vec<u32>, before_reqs: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut rest: HashSet<u32> = HashSet::from_iter(instructions.iter().cloned());

    for i in 0..instructions.len() - 1 {
        let instruction = instructions[i];
        rest.remove(&instruction);

        if !is_position_valid(&rest, before_reqs, instruction) {
            return false;
        }
    }

    true
}

fn is_position_valid(
    rest: &HashSet<u32>,
    before_reqs: &HashMap<u32, HashSet<u32>>,
    instruction: u32,
) -> bool {
    rest.intersection(before_reqs.get(&instruction).unwrap_or(&HashSet::new()))
        .count()
        == 0
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let (p1, p2) = mean_page(input);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day05.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[cfg(test)]
mod tests {
    use crate::mean_page;

    #[test]
    fn test_p1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        let (res, _) = mean_page(&input);
        assert_eq!(143, res);
    }

    #[test]
    fn test_p2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

        let (_, res) = mean_page(&input);

        assert_eq!(123, res);
    }
}
