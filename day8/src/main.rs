use common::*;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let lines = load_data();

    // Load the path
    let path: Vec<char> = lines[0].chars().collect();

    // Load the graph
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    let graph_regex = Regex::new(r"(\w+)\s*=\s*\((\w+),\s*(\w+)\)").unwrap();
    for line in &lines[2..] {
        if let Some(captures) = graph_regex.captures(line) {
            let node = captures.get(1).map(|m| m.as_str().to_string());
            let left = captures.get(2).map(|m| m.as_str().to_string());
            let right = captures.get(3).map(|m| m.as_str().to_string());
            graph.insert(node.unwrap(), (left.unwrap(), right.unwrap()));
        }
    }

    let mut count = 0;

    let start: String = "AAA".to_string();
    let end: String = "ZZZ".to_string();
    let mut node_name = &start;
    while node_name != end.as_str() {
        let node = graph.get(node_name).unwrap();
        let step = path[count % path.len()];
        if step == 'L' {
            node_name = &node.0;
        } else {
            node_name = &node.1;
        }
        count += 1;
    }

    println!("Count: {}", count);
}
