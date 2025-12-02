use lib::read_lines_from_file;
use lib::read_lines_from_string;
use std::env;
use std::time::Instant;

const TEST_INPUT: &str = r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"#;

fn has_repeating_numbers(s: &str) -> bool {
    let length = s.len();
    if length % 2 == 0 {
        if s[..length / 2] == s[length / 2..] {
            return true;
        }
    }
    false
}

fn has_repeating_numbers_two(s: &str) -> bool {
    let len = s.len();
    if len < 2 {
        return false;
    }

    let first = s.chars().next().unwrap();
    let all_same = s.chars().all(|c| c == first);

    if all_same {
        return true;
    }
    for window_size in 2..(len / 2) {
        let w = windows(s, window_size);
        if w.len() > 1
            && w.iter()
                .all(|x| x.eq(w.first().unwrap()) && w.join("").eq(s))
        {
            return true;
        }
    }
    false
}

fn windows(s: &str, window_size: usize) -> Vec<String> {
    let mut result = Vec::new();
    let len = s.len();
    for start in (0..=len - window_size).step_by(window_size) {
        result.push(s[start..start + window_size].to_string());
    }
    result
}

fn main() {
    let timer = Instant::now();
    let args: Vec<String> = env::args().collect();
    let lines = if args.len() < 2 {
        read_lines_from_string(TEST_INPUT)
    } else {
        read_lines_from_file(&args[1]).unwrap()
    };

    let ranges = lines
        .first()
        .unwrap()
        .split(',')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    let mut invalid_ids_one: Vec<String> = vec![];
    let mut invalid_ids_two: Vec<String> = vec![];
    for range in &ranges {
        let bounds = range.split('-').collect::<Vec<&str>>();
        let start: u64 = bounds[0].parse().unwrap();
        let end: u64 = bounds[1].parse().unwrap();
        for num in start..=end {
            let s = num.to_string();
            if has_repeating_numbers(&s) {
                invalid_ids_one.push(s.clone());
                invalid_ids_two.push(s);
            } else if has_repeating_numbers_two(&s) {
                invalid_ids_two.push(s);
            }
        }
    }

    let part1 = invalid_ids_one
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .sum::<u64>();
    println!("Part 1: {}", part1);

    let part2 = invalid_ids_two
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .sum::<u64>();
    println!("Part 2: {}", part2);
    println!(
        "Time: {}ms ({}µs)",
        timer.elapsed().as_millis(),
        timer.elapsed().as_micros()
    );
}
