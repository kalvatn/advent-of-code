use lib::read_lines_from_file;
use lib::read_lines_from_string;
use regex::Regex;
use std::env;
use std::time::Instant;

const TEST_INPUT: &str = r#"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"#;

fn main() {
    let timer = Instant::now();
    let args: Vec<String> = env::args().collect();
    let lines = if args.len() < 2 {
        read_lines_from_string(TEST_INPUT)
    } else {
        read_lines_from_file(&args[1]).unwrap()
    };
    let memory = &lines[0];
    let sum = extract_multiply_sums(&memory);

    println!("Part 1: {}", sum);

    let re_exclude = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let replaced_line = re_exclude.replace_all(&memory, "").to_string();

    let sum: u64 = extract_multiply_sums(&replaced_line);
    println!("Part 2: {}", sum);

    println!("Time: {}ms ({}µs)", timer.elapsed().as_millis(), timer.elapsed().as_micros());
}

fn extract_multiply_sums(memory: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.captures_iter(&memory);
    let sum: u64 = matches
        .map(|cap| {
            let a: u64 = cap[1].parse().unwrap();
            let b: u64 = cap[2].parse().unwrap();
            a * b
        }).sum();
    sum
}