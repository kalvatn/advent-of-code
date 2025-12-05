use lib::read_lines_from_file;
use lib::read_lines_from_string;
use std::env;
use std::time::Instant;

const TEST_INPUT: &str = r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;

fn find_first_battery_index(s: &str, cutoff: usize) -> (usize, u32) {
    let mut highest = (0, 0);

    for ic in s.chars().enumerate() {
        let digit = ic.1.to_digit(10).unwrap();
        if digit > highest.1 && ic.0 < s.len() - cutoff {
            highest = (ic.0, digit)
        }
    }

    highest
}

fn find_all(s: &str, r: String) -> String {
    if r.len() >= 12 {
        return r;
    }
    let mut result = r.clone();
    let ic = find_first_battery_index(s, 11 - r.len());
    result.push(s.chars().nth(ic.0).unwrap());
    find_all(&s[ic.0 + 1..s.len()], result).to_string()
}

fn main() {
    let timer = Instant::now();
    let args: Vec<String> = env::args().collect();
    let lines = if args.len() < 2 {
        read_lines_from_string(TEST_INPUT)
    } else {
        read_lines_from_file(&args[1]).unwrap()
    };

    let joltages: u32 = lines
        .iter()
        .map(|line| {
            let first = find_first_battery_index(&line, 1);
            let second = find_first_battery_index(&line[first.0 + 1..line.len()], 0);
            let joltage: u32 = format!("{}{}", first.1, second.1).parse().unwrap();
            joltage
        })
        .sum();

    println!("Part 1: {}", joltages);

    let joltages2: u64 = lines
        .iter()
        .map(|line| find_all(&line, "".to_string()).parse::<u64>().unwrap())
        .sum();
    println!("Part 2: {}", joltages2);
    println!(
        "Time: {}ms ({}µs)",
        timer.elapsed().as_millis(),
        timer.elapsed().as_micros()
    );
}
