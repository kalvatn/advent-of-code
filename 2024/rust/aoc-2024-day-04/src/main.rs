use lib::read_lines_from_file;
use lib::read_lines_from_string;
use std::env;
use std::time::Instant;

const TEST_INPUT: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
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

    let grid: Vec<Vec<char>> = lines.join("\n")
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            count += find_word(&grid, row, col, Direction::all(), "XMAS").len();
        }
    }

    println!("Part 1: {}", count / 2);

    let mut candidates: Vec<(usize, usize)> = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            find_word(&grid, row, col, vec![Direction::SouthEast], "MAS").iter().for_each(|(row, col)| {
                if col + 2 < grid[*row].len() {
                    candidates.push((row.clone(), col.clone() + 2));
                }
            })
        };
    }
    count = 0;
    for (row, col) in &candidates {
        for direction in vec![Direction::SouthWest] {
            count += find_word(&grid, *row, *col, vec![direction], "MAS").len()
        };
    }


    println!("Part 2: {}", count);
    println!("Time: {}ms ({}µs)", timer.elapsed().as_millis(), timer.elapsed().as_micros());
}

fn find_word(grid: &Vec<Vec<char>>, row: usize, col: usize, directions: Vec<Direction>, word: &str) -> Vec<(usize, usize)> {
    let word_backwards: String = word.chars().rev().collect();
    let mut result = Vec::new();

    if grid[row][col] != word.chars().next().unwrap() && grid[row][col] != word_backwards.chars().next().unwrap() {
        return result;
    }

    for direction in directions {
        let (dy, dx) = direction.get_diff();
        let new_row = row as i32;
        let new_col = col as i32;
        let s: String = (0..word.len()).map(|i| {
            let new_row = new_row + dy * i as i32;
            let new_col = new_col + dx * i as i32;
            (new_row, new_col)
        }).filter(|(new_row, new_col)| {
            *new_row >= 0 && *new_col >= 0 && *new_row < grid.len() as i32 && *new_col < grid[row].len() as i32
        }).map(|(new_row, new_col)| {
            grid[new_row as usize][new_col as usize]
        }).collect();
        if s == word || s == word_backwards {
            result.push((row, col));
        }
    };
    result
}