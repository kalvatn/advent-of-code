use lib::read_lines_from_file;
use lib::read_lines_from_string;
use std::env;
use std::time::Instant;

const TEST_INPUT: &str = r#"
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"#;

fn main() {
    let timer = Instant::now();
    let args: Vec<String> = env::args().collect();
    let lines = if args.len() < 2 {
        read_lines_from_string(TEST_INPUT)
    } else {
        read_lines_from_file(&args[1]).unwrap()
    };

    let numbers: Vec<_> = lines
        .iter()
        .take(lines.len() - 1)
        .map(|s| s.to_owned() + "  ")
        .collect();
    let mut operators: Vec<char> = lines.last().unwrap().chars().collect();
    operators.push(' ');
    operators.push(' ');
    operators.push(' ');

    let ops: Vec<&char> = operators.iter().filter(|c| !c.is_whitespace()).collect();
    let mut positions: Vec<(usize, usize)> = (0..ops.len()).map(|_| (0, 0)).collect();
    let mut i = 1;
    let mut start = 0;
    for x in 1..operators.len() {
        if operators[x] != ' ' {
            positions[i - 1] = (start, x - 1);
            start = x;
            i += 1;
        }
        if x == operators.len() - 1 {
            positions[i - 1] = (start, x);
        }
    }

    let mut part1: usize = 0;
    for (i, pos) in positions.iter().enumerate() {
        let nums: Vec<_> = (0..numbers.len())
            .map(|row| numbers[row][pos.0..pos.1].trim().parse::<usize>().unwrap())
            .collect();

        let operator = ops[i];
        if operator.eq(&'*') {
            part1 += nums.iter().product::<usize>();
        } else if operator.eq(&'+') {
            part1 += nums.iter().sum::<usize>();
        }
    }

    println!("Part 1: {}", part1);

    let mut part2: usize = 0;
    for (i, pos) in positions.iter().enumerate() {
        let nums: Vec<_> = (pos.0..pos.1)
            .map(|col| {
                let s: String = (0..numbers.len())
                    .map(|row| numbers[row].chars().nth(col).unwrap())
                    .collect();
                s.trim().to_owned()
            })
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let operator = ops[i];
        if operator.eq(&'*') {
            part2 += nums.iter().product::<usize>();
        } else if operator.eq(&'+') {
            part2 += nums.iter().sum::<usize>();
        }
    }

    println!("Part 2: {}", part2);
    println!(
        "Time: {}ms ({}µs)",
        timer.elapsed().as_millis(),
        timer.elapsed().as_micros()
    );
}
