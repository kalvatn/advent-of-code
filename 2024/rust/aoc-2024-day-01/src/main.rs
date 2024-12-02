use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader.lines().collect()
}

const TEST_INPUT: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines: Vec<String> = if args.len() < 2 {
        TEST_INPUT.lines().map(|s| s.to_string()).collect()
    } else {
        let input_path = &args[1];
        read_lines(input_path).unwrap_or_else(|e| {
            println!("Error reading file: {}", e);
            std::process::exit(1)
        })
    };

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in &lines {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if numbers.len() == 2 {
            left.push(numbers[0]);
            right.push(numbers[1]);
        }
    }
    left.sort();
    right.sort();

    let mut distance: u64 = 0;
    for i in 0..left.len() {
        let diff = (left[i] - right[i]).abs();
        distance += diff as u64
    }
    println!("Part 1: {}", distance);

    let mut right_number_counts: HashMap<i32, usize> = HashMap::new();
    for &number in &right {
        *right_number_counts.entry(number).or_insert(0) += 1;
    }

    let mut similarity: u64 = 0;
    for value in &left {
        if let Some(value2) = right_number_counts.get(value) {
            similarity += (*value as u64) * (*value2 as u64);
        }
    }
    println!("Part 2: {}", similarity);
}
