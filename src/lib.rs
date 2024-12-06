use std::fmt::Debug;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

pub fn get_input(filename: &str) -> Vec<String> {
    let file = match File::open(format!("input/{}", filename)) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open file {}: {}", filename, error),
    };

    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|a| !a.is_empty())
        .collect()
}

pub fn get_input_as_string(filename: &str) -> String {
    let reader = match read_to_string(format!("input/{}", filename)) {
        Ok(r) => r,
        Err(error) => panic!("Unable to open file {}: {}", filename, error),
    };

    reader.parse().unwrap()
}

pub fn get_input_as_int<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Ord + FromStr>(
    filename: &str,
) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    get_input(filename)
        .iter()
        .map(|i| i.parse().unwrap())
        .collect()
}

pub fn format_duration(nanos: u128) -> String {
    let elapsed = nanos as f64 / 1000.0;

    if elapsed > 1_000_000.0 {
        format!("{:.03}s", elapsed / 1_000_000.0)
    } else if elapsed > 1000.0 {
        format!("{:.03}ms", elapsed / 1000.0)
    } else {
        format!("{:.03}μs", elapsed)
    }
}
