use std::env;
use common::*;

/*
const MAP_NAMES: [&str; 7] = [
    "seed-to-soil map", 
    "soil-to-fertilizer map", 
    "fertilizer-to-water map", 
    "water-to-light map", 
    "light-to-temperature map", 
    "temperature-to-humidity map", 
    "humidity-to-location map"
];
*/

fn main() {
    // Get path to data
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // Load data
    let lines = read_lines(path).unwrap();
    let mut iter = lines.iter();

    // Load the seeds
    let seeds = parse_seeds(&mut iter);

    // println!("Seeds: {:?}", seeds);

    // Load each map
    let mut maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    while let Some(_line) = iter.next() {
        // Ignore name of map
        maps.push(parse_map(&mut iter));
    }

    // for m in &maps {
    //     println!("{:?}", m);
    // }

    let mut min_location: Option<u64> = None;

    for s in &seeds {
        let mut x = *s;
        for m in &maps {
            x = transform(x, m);
        }
        // println!("Seed {} -> {}", s, x);
        if min_location.is_none() || x < min_location.unwrap() {
            min_location = Some(x);
        }   
    }

    println!("Min location: {}", min_location.unwrap());
}

fn parse_seeds<'a, I>(iter: &mut I) -> Vec<u64>
where
    I: Iterator<Item = &'a String>,
{
    let input = iter.next().unwrap();
    let parts: Vec<&str> = input.split(":").collect();
    let seeds: Vec<u64> = parts[1]
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    iter.next(); // Skip blank line

    seeds
}

fn parse_map<'a, I>(iter: &mut I) -> Vec<(u64, u64, u64)>
where
    I: Iterator<Item = &'a String>,
{
    let mut map = Vec::new();

    while let Some(line) = iter.next() {
        if line.trim().is_empty() {
            break;
        }

        let parts: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        map.push((parts[0], parts[1], parts[2]));
    }

    map
}

fn transform(x: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    for e in map {
        if x >= e.1 && x < e.1 + e.2 {
            return e.0 + x - e.1;
        }
    }
    x
}