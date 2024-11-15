use std::{
    env,
    fs::File,
    fs::read_to_string,
    io::{self, BufRead, BufReader},
    path::Path,
};

/// Loads lines of data from the file specified in the command into a vector of strings
pub fn load_lines() -> Vec<String> {
    // Get path
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // Load data
    read_lines(path).expect(&format!("Could not read the file \"{}\"", path))
}

/// Reads a file line by line into a vector of strings
fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

/// Reads a file into a vector of strings separated by ','.
pub fn load_comma_separated_values() -> Vec<String> {
    // Get path
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // Load data
    read_comma_separated_values(path).expect(&format!("Could not read the file \"{}\"", path))
}

/// Reads an entire file into a string and splits it by ',' into a vector of strings
fn read_comma_separated_values(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let input = read_to_string(filename)?;
    Ok(input.split(',').map(|s| s.trim().to_string()).collect())
}