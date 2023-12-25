use std::env;
use common::*;

fn main() {
    // Get path to data
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // Load data
    let lines = read_lines(path).unwrap();

    let times = parse_line(&lines[0]);
    let distances: Vec<_> = parse_line(&lines[1]);
    
    let races: Vec<_> = times.into_iter().zip(distances.into_iter()).collect();
    
    let mut product = 1;
    for r in races {
        println!("{:?}", r);
        let mut lower = ((r.0 - (r.0*r.0 - 4.0 * r.1).sqrt()) / 2.0).ceil();
        if (r.0 - lower) * lower <= r.1 {
            lower += 1.0;
        }
        let mut upper = ((r.0 + (r.0*r.0 - 4.0 * r.1).sqrt()) / 2.0).ceil();
        if (r.0 - upper) * upper <= r.1 {
            upper -= 1.0;
        }
        println!("{} {} {}", lower, upper, upper - lower + 1.0);
        product *= (upper as i32) - (lower as i32) + 1;
    }

    println!("Product: {}", product);
}

fn parse_line(line: &String) -> Vec<f32> {
    let data: Vec<_> = line
                        .split(":")
                        .nth(1)
                        .unwrap()
                        .split_whitespace()
                        .map(|s| s.parse::<f32>().unwrap())
                        .collect();
    data
}
