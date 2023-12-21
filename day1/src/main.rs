use std::env;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

// Reads a file line by line into a vector of strings
fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

// Returns the first digit in a string
fn find_first_digit(s: &String) -> Option<u32> {
    s.chars().find(|c| c.is_ascii_digit()).and_then(|c| c.to_digit(10))
}

// Returns the last digit in a string
fn find_last_digit(s: &String) -> Option<u32> {
    s.chars().rfind(|c| c.is_ascii_digit()).and_then(|c| c.to_digit(10))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    println!("From file {}", path);
    let lines = read_lines(path).expect("Could not read file.");

    let mut sum = 0;
    for line in lines {
        let first_digit = find_first_digit(&line);
        let last_digit = find_last_digit(&line);
        sum += first_digit.unwrap() * 10 + last_digit.unwrap();
    }

    println!("Sum: {}", sum);
}
