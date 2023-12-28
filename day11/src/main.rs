use common::*;

fn main() {
    let lines = load_data();

//    println!("Original galaxy:");
//    draw_galaxy(&lines);

    let galaxy = expand(&lines);

//    println!("Expanded galaxy:");
//    draw_galaxy(&galaxy);

    let stars = find_stars(&galaxy);
//    println!("Stars: {:?}", stars);

    let distances = find_distances(&stars);
//    println!("Distances: {:?}", distances);

    println!("Sum of distances: {}", distances.iter().sum::<i32>());
}

fn find_distances(stars: &Vec<(usize, usize)>) -> Vec<i32> {
    let mut distances: Vec<i32> = Vec::new();
    for i in 0..stars.len() - 1 {
        for j in i + 1..stars.len() {
            distances.push(distance(&stars[i], &stars[j]));
        }
    }
    distances
}

fn distance(star1: &(usize, usize), star2: &(usize, usize)) -> i32 {
    let dr = (star1.0 as isize - star2.0 as isize).abs();
    let dc = (star1.1 as isize - star2.1 as isize).abs();
    (dr + dc) as i32
}

fn find_stars(galaxy: &Vec<String>) -> Vec<(usize, usize)> {
    let mut stars: Vec<(usize, usize)> = Vec::new();
    for i in 0..galaxy.len() {
        for j in 0..galaxy[i].len() {
            if galaxy[i].chars().nth(j).unwrap() == '#' {
                stars.push((i, j));
            }
        }
    }
    stars
}

fn _draw_galaxy(galaxy: &Vec<String>) {
    for line in galaxy {
        println!("{}", line);
    }
    println!("")
}

// Expands the galaxy
fn expand(galaxy: &Vec<String>) -> Vec<String> {
    expand_vertically(&expand_horizontally(galaxy))
}

// Expands the galaxy horizontally
fn expand_horizontally(galaxy: &Vec<String>) -> Vec<String> {
    let mut new_galaxy: Vec<String> = galaxy.clone();
    for i in (0..galaxy[0].len()).rev() {
        if column_is_empty(galaxy, i) {
            for line in &mut new_galaxy {
                line.insert(i, '.');
            }
        }
    }
    new_galaxy
}

// Returns true if the column is empty
fn column_is_empty(galaxy: &Vec<String>, column: usize) -> bool {
    for line in galaxy {
        if line.chars().nth(column).unwrap() != '.' {
            return false;
        }
    }
    true
}

fn expand_vertically(galaxy: &Vec<String>) -> Vec<String> {
    let mut new_galaxy: Vec<String> = galaxy.clone();
    for i in (0..galaxy.len()).rev() {
        if row_is_empty(&galaxy[i]) {
            new_galaxy.insert(i, galaxy[i].clone());
        }
    }
    new_galaxy
}

// Returns true if the row is empty
fn row_is_empty(row: &String) -> bool {
    for c in row.chars() {
        if c != '.' {
            return false;
        }
    }
    true
}
