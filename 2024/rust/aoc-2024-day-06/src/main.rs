use lib::read_lines_from_file;
use lib::read_lines_from_string;
use std::collections::HashSet;
use std::env;
use std::time::Instant;
#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_diff(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

const TEST_INPUT: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

fn main() {
    let timer = Instant::now();
    let args: Vec<String> = env::args().collect();
    let lines = if args.len() < 2 {
        read_lines_from_string(TEST_INPUT)
    } else {
        read_lines_from_file(&args[1]).unwrap()
    };
    let grid: Vec<Vec<char>> = lines.join("\n")
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut guard_pos = grid.iter().enumerate().find_map(|(i, row)| {
        row.iter().position(|&c| c == '^').map(|j| (i, j))
    }).unwrap();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut heading = Direction::North;
    while (0..grid.len()).contains(&guard_pos.0) && (0..grid[0].len()).contains(&guard_pos.1) {
        visited.insert(guard_pos);
        let (dx, dy) = heading.get_diff();
        let new_pos = (guard_pos.0 as i32 + dx, guard_pos.1 as i32 + dy);
        if new_pos.0 < 0 || new_pos.0 >= grid.len() as i32 || new_pos.1 < 0 || new_pos.1 >= grid[0].len() as i32 {
            break;
        }

        if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            heading = heading.turn_right();
        } else {
            guard_pos = (new_pos.0 as usize, new_pos.1 as usize);
        }
    }

    println!("Part 1: {}", visited.iter().count());

    println!("Part 2: {}", 0);
    println!("Time: {}ms ({}µs)", timer.elapsed().as_millis(), timer.elapsed().as_micros());
}