use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader.lines().collect()
}

const TEST_INPUT: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines: Vec<String> = if args.len() < 2 {
        TEST_INPUT.lines().map(|s| s.to_string()).filter(|x| !x.is_empty()).collect()
    } else {
        let input_path = &args[1];
        read_lines(input_path).unwrap_or_else(|e| {
            println!("Error reading file: {}", e);
            std::process::exit(1)
        })
    };
    let levels: Vec<Vec<i32>> = lines.iter()
        .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();

    let mut safe_count = 0;
    let mut unsafe_levels: Vec<Vec<i32>> = Vec::new();
    for level in &levels {
        let diffs = level_diffs(level);
        if is_safe(&diffs) {
            safe_count += 1;
        } else {
            unsafe_levels.push(level.to_vec());
        }
    }
    println!("Part 1: {}", safe_count);

    let mut can_be_made_safe_count = 0;
    for level in &unsafe_levels {
        for i in 0..level.len() {
            let mut new_level = level.clone();
            new_level.remove(i);
            let diffs = level_diffs(&new_level);
            if is_safe(&diffs) {
                can_be_made_safe_count += 1;
                break;
            }
        }
    }

    println!("Part 2: {}", safe_count + can_be_made_safe_count);
}

fn level_diffs(level: &Vec<i32>) -> Vec<i32> {
    level.windows(2).map(|w| w[0] - w[1]).collect()
}

fn is_safe(diffs: &Vec<i32>) -> bool {
    diffs.iter().all(|x| *x >= 1 && *x <= 3) || diffs.iter().all(|x| *x <= -1 && *x >= -3)
}
