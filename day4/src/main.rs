use std::env;
use common::read_lines;
use std::vec::Vec;

fn main() {
    // Get path to data
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // Load data
    let lines = read_lines(path).unwrap();

    // Parse the cards
    let mut cards: Vec<(i32, Vec<i32>, Vec<i32>)> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let id: i32 = parts[0].trim().split_whitespace().last().unwrap().parse().unwrap();

        let sets: Vec<&str> = parts[1].split("|").collect();
        let mut winning: Vec<i32> = sets[0].trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let mut yours: Vec<i32> = sets[1].trim().split_whitespace().map(|s| s.parse().unwrap()).collect();

        winning.sort();
        yours.sort();
        cards.push((id, winning, yours));
    }

    // Find the winning cards and sum the points
    let mut points = 0;
    for c in cards {
        let winners = intersection(c.1, c.2);
        if winners.len() > 0 {
            points += score(winners.len());
        }
    }

    println!("Points: {}", points);
}

// Returns the score for a given number of cards
fn score(n: usize) -> i32 {
    if n > 0 {
        return 1 << (n - 1);
    }
    else {
        return 0;
    }
}
// Returns the intersection of two sorted vectors
fn intersection(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            i += 1;
        } else if a[i] > b[j] {
            j += 1;
        } else {
            result.push(a[i]);
            i += 1;
            j += 1;
        }
    }

    result
}