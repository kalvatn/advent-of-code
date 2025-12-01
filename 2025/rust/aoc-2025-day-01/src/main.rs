use lib::read_lines_from_file;
use lib::read_lines_from_string;
use std::env;
use std::time::Instant;

const TEST_INPUT: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

struct Dial {
    position: usize,
    visited: Vec<usize>,
    endings: Vec<usize>,
}

impl Dial {
    fn new(initial: usize) -> Self {
        Self {
            position: initial % 100,
            visited: vec![initial % 100],
            endings: Vec::new(),
        }
    }

    fn spin_left(&mut self, steps: usize) {
        for _ in 0..steps {
            self.position = if self.position == 0 { 99 } else { self.position - 1 };
            self.visited.push(self.position);
        }
        self.endings.push(self.position);
    }

    fn spin_right(&mut self, steps: usize) {
        for _ in 0..steps {
            self.position = if self.position == 99 { 0 } else { self.position + 1 };
            self.visited.push(self.position);
        }
        self.endings.push(self.position);
    }

    #[allow(dead_code)]
    fn reset(&mut self, initial: usize) {
        self.position = initial % 100;
        self.visited.clear();
        self.visited.push(self.position);
        self.endings.clear();
    }

    fn get_visited(&self) -> &Vec<usize> {
        &self.visited
    }

    fn get_endings(&self) -> &Vec<usize> {
        &self.endings
    }
}

fn main() {
    let timer = Instant::now();
    let args: Vec<String> = env::args().collect();
    let lines = if args.len() < 2 {
        read_lines_from_string(TEST_INPUT)
    } else {
        read_lines_from_file(&args[1]).unwrap()
    };

    let mut dial = Dial::new(50);
    for line in lines {
        let line = line.trim();
        if line.is_empty() { continue; }
        match line.chars().next() {
            Some('L') => {
                if let Ok(n) = line[1..].parse::<usize>() {
                    dial.spin_left(n);
                } else {
                    eprintln!("Invalid left spin: {}", line);
                }
            }
            Some('R') => {
                if let Ok(n) = line[1..].parse::<usize>() {
                    dial.spin_right(n);
                } else {
                    eprintln!("Invalid right spin: {}", line);
                }
            }
            _ => {
                eprintln!("Unknown action: {}", line);
            }
        }
    }
    println!("Part 1: {}", dial.get_endings().iter().filter(|&&x| x == 0).count());
    println!("Part 2: {}", dial.get_visited().iter().filter(|&&x| x == 0).count());
    println!("Time: {}ms ({}µs)", timer.elapsed().as_millis(), timer.elapsed().as_micros());
}