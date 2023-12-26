use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/// Loads data from the file specified in the command into a vector of strings
pub fn load_data() -> Vec<String> {
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

