use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines_from_file<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader.lines().collect()
}

pub fn read_lines_from_string(s: &str) -> Vec<String> {
    s.lines().map(|s| s.to_string()).filter(|x| !x.is_empty()).collect()
}