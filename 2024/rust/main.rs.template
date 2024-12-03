use lib::read_lines_from_file;
use lib::read_lines_from_string;
use std::env;
use std::time::Instant;

const TEST_INPUT: &str = r#"
"#;

fn main() {
    let timer = Instant::now();
    let args: Vec<String> = env::args().collect();
    let lines = if args.len() < 2 {
        read_lines_from_string(TEST_INPUT)
    } else {
        read_lines_from_file(&args[1]).unwrap()
    };

    println!("Part 1: {}", 0);

    println!("Part 2: {}", 0);
    println!("Time: {}ms ({}µs)", timer.elapsed().as_millis(), timer.elapsed().as_micros());
}