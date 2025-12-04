use lib::read_lines_from_file;
use lib::read_lines_from_string;
use std::env;
use std::time::Instant;

const TEST_INPUT: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;


#[derive(Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn get_diff(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::NorthEast => (-1, 1),
            Direction::East => (0, 1),
            Direction::SouthEast => (1, 1),
            Direction::South => (1, 0),
            Direction::SouthWest => (1, -1),
            Direction::West => (0, -1),
            Direction::NorthWest => (-1, -1),
        }
    }

    fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ]
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

    let mut grid: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut accessable = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '@' {

                let count = Direction::all().iter().map(|dir| {
                    let (dx, dy) = dir.get_diff();
                    (row as i32 - dy, col as i32 - dx)
                }).filter(|(row, col)| *row >= 0 && *col >= 0)
                    .map(|(row, col)| (row as usize, col as usize))
                    .filter(|(row, col)| (0..grid.len()).contains(row) && (0..grid[0].len()).contains(col))
                    .filter(|(row, col)| grid[*row][*col] == '@')
                    .count();
                if count < 4 {
                    accessable += 1
                }
            }
        }
    }
    let part1 = accessable;

    println!("Part 1: {}", part1);

    let part2 = remove(&mut grid, 0);

    println!("Part 2: {}", part2);
    println!("Time: {}ms ({}µs)", timer.elapsed().as_millis(), timer.elapsed().as_micros());
}

fn remove(grid: &mut Vec<Vec<char>>, mut count: usize) -> usize {
    let mut removable = vec![];
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '@' {
                let neighbors: Vec<_> = Direction::all().iter().map(|dir| {
                    let (dx, dy) = dir.get_diff();
                    (row as i32 - dy, col as i32 - dx)
                }).filter(|(row, col)| *row >= 0 && *col >= 0)
                    .map(|(row, col)| (row as usize, col as usize))
                    .filter(|(row, col)| (0..grid.len()).contains(row) && (0..grid[0].len()).contains(col))
                    .filter(|(row, col)| grid[*row][*col] == '@')
                    .map(|(row, col)| (row as usize, col as usize))
                    .collect();
                if neighbors.iter().count() < 4 {
                    removable.push((row, col));
                }
            }
        }
    }
    if removable.is_empty() {
        return count
    }
    for (row, col) in &removable {
        grid[*row][*col] = '.';
    }
    remove(grid, count + removable.len())

}