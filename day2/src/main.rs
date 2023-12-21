use std::env;
use regex::Regex;
use common::read_lines;

// which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
const MAX: (i32, i32, i32) = ( 12, 13, 14 );

fn main() {
    println!("Bag max is {} red, {} green, {} blue", MAX.0, MAX.1, MAX.2);

    // Get path to games
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // Load games
    let games = read_lines(path).unwrap();

    let mut id_sum = 0;

    // Check games
    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut failed = false;

        // Parse a game
        let game_regex = Regex::new(r"Game (\d+):").unwrap();
        let game_id: i32 = game_regex.captures(&game).unwrap()[1].parse().unwrap();
        let tuple_regex = Regex::new(r"(\d+) (\w+)([,;]?)").unwrap();
        for cap in tuple_regex.captures_iter(&game) {
            let number: i32 = cap[1].parse().unwrap();
            let color: &str = &cap[2];
            let separator: &str = &cap[3];
            if color == "red" {
                red += number;
                if red > MAX.0 {
                    failed = true;
                    break;
                }
            } else if color == "green" {
                green += number;
                if green > MAX.1 {
                    failed = true;
                    break;
                }
            } else if color == "blue" {
                blue += number;
                if blue > MAX.2 {
                    failed = true;
                    break;
                }
            } else {
                println!("Unknown color {}", color);
            }
            if separator != "," {
                red = 0;
                green = 0;
                blue = 0;
            }
        }
        if !failed {
            id_sum += game_id;
        }
    }

    println!("Sum of game ids is {}", id_sum);
}
