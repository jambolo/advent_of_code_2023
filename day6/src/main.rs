use std::env;
use common::*;

fn main() {
    // Get path to data
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // Load data
    let lines = read_lines(path).unwrap();

    let times = parse_line(&lines[0]);
    let distances = parse_line(&lines[1]);
    let r = (times, distances);
    let mut lower = ((r.0 - (r.0 * r.0 - 4.0 * r.1).sqrt()) / 2.0).ceil();
    if (r.0 - lower) * lower <= r.1 {
        lower += 1.0;
    }
    let mut upper = ((r.0 + (r.0 * r.0 - 4.0 * r.1).sqrt()) / 2.0).ceil();
    if (r.0 - upper) * upper <= r.1 {
        upper -= 1.0;
    }
    println!("{} {} {}", lower, upper, upper - lower + 1.0);
}

fn parse_line(line: &String) -> f64 {
    let data = line
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<f64>().unwrap();
data
}
