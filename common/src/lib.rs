use std::{
    env,
    fs::read_to_string,
    io,
    path::Path,
};

/// Loads lines of data from the file specified in the command into a vector of strings
pub fn load_lines() -> Vec<String> {
    let path = get_path();

    // Load data
    read_lines(&path).expect(&format!("Could not read the file \"{}\"", path))
}

/// Reads a file line by line into a vector of strings
fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let input = read_to_string(filename)?;
    Ok(input.lines().map(|line| line.to_string()).collect())
}

/// Loads a file into a vector of strings separated by ','.
pub fn load_comma_separated_values() -> Vec<String> {
    let path = get_path();

    // Load data
    read_comma_separated_values(&path).expect(&format!("Could not read the file \"{}\"", path))
}

/// Reads an entire file into a string and splits it by ',' into a vector of strings
fn read_comma_separated_values(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let input = read_to_string(filename)?;
    Ok(input.split(',').map(|s| s.trim().to_string()).collect())
}

/// Loads a file into a 2D array of characters
pub fn load_map() -> Vec<Vec<char>> {
    let path = get_path();

    // Load data
    read_map(&path).expect(&format!("Could not read the file \"{}\"", path))
}

/// Reads an entire file into a 2D array of characters
fn read_map(filename: impl AsRef<Path>) -> io::Result<Vec<Vec<char>>> {
    let input = read_to_string(filename)?;
    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

/// Gets the path from the command line arguments
fn get_path() -> String {
    let args: Vec<String> = env::args().collect();
    args[1].clone()
}
