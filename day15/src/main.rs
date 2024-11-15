use common::*;

fn main() {
    println!("Day 15, part 1");
    let steps = load_comma_separated_values();
    let sum = steps.iter().map(|s| hash(&s)).sum::<i64>();
    println!("Sum: {}", sum);
}

fn hash(s: &str) -> i64 {
    s.chars().fold(0, |sum, c| ((sum + c as i64) * 17) & 0xFF)
}
