use lib::read_lines_from_file;
use lib::read_lines_from_string;
use std::env;
use std::ops::RangeInclusive;
use std::time::Instant;

const TEST_INPUT: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

fn main() {
    let timer = Instant::now();
    let args: Vec<String> = env::args().collect();
    let lines = if args.len() < 2 {
        read_lines_from_string(TEST_INPUT)
    } else {
        read_lines_from_file(&args[1]).unwrap()
    };

    let ranges: Vec<_> = lines
        .iter()
        .take_while(|l| l.contains("-"))
        .map(|l| {
            let parts = l.split("-").collect::<Vec<&str>>();
            parts[0].parse::<usize>().unwrap()..=parts[1].parse::<usize>().unwrap()
        })
        .collect();
    let ingredients: Vec<_> = lines
        .iter()
        .skip_while(|l| l.contains("-") || l.is_empty())
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    let part1 = ingredients
        .iter()
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count();

    println!("Part 1: {}", part1);

    let mut ranges_sorted_by_start = ranges.clone();
    ranges_sorted_by_start.sort_by_key(|r| *r.start());

    let mut collapsed_ranges: Vec<RangeInclusive<usize>> = vec![];

    let mut i = 0;
    while i < ranges_sorted_by_start.len() {
        let cur = &ranges_sorted_by_start[i];
        let cur_start = cur.start();
        let mut cur_end = cur.end();
        let mut j = i + 1;

        // println!("{:?} {:?}", i, cur);
        while j < ranges_sorted_by_start.len() {
            if ranges_sorted_by_start[j].start() <= &(cur_end + 1) {
                // println!("{} collapsing {:?} <- {:?}", i, cur, sorted_ranges[j]);
                if ranges_sorted_by_start[j].end() > cur_end {
                    cur_end = ranges_sorted_by_start[j].end();
                }
            } else {
                break;
            }
            j += 1;
        }
        i = j;
        collapsed_ranges.push(*cur_start..=*cur_end);
    }

    let part2: usize = collapsed_ranges
        .iter()
        .map(|r| (r.end() - r.start()) + 1)
        .sum();

    println!("Part 2: {}", part2);
    println!(
        "Time: {}ms ({}µs)",
        timer.elapsed().as_millis(),
        timer.elapsed().as_micros()
    );
}
