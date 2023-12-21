use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

// Reads a file line by line into a vector of strings
pub fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
