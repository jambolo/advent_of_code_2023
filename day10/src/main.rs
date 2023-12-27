use common::*;

fn main() {
    let lines = load_data();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in &lines {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    let starting_point = find_start(&grid);

    let mut points: Vec<((usize, usize), (isize, isize))> = Vec::new();
    if starting_point.1 > 0 {
        let d: (isize, isize) = (0, -1);
        let n: (usize, usize) = next(starting_point, d);
        let g = grid[n.1][n.0];
        if g == '|' || g == 'F' || g == '7' {
            points.push((n, d));
        }
    }
    if starting_point.0 < grid[starting_point.1].len() - 1 {
        let d: (isize, isize) = (1, 0);
        let n: (usize, usize) = next(starting_point, d);
        let g = grid[n.1][n.0];
        if g == '-' || g == 'J' || g == '7' {
            points.push((n, d));
        }
    }
    if starting_point.1 < grid.len() - 1 {
        let d: (isize, isize) = (0, 1);
        let n: (usize, usize) = next(starting_point, d);
        let g = grid[n.1][n.0];
        if g == '|' || g == 'J' || g == 'L' {
            points.push((n, d));
        }
    }
    if starting_point.0 > 0 {
        let d: (isize, isize) = (-1, 0);
        let n: (usize, usize) = next(starting_point, d);
        let g = grid[n.1][n.0];
        if g == '-' || g == 'L' || g == 'F' {
            points.push((n, d));
        }
    }

    // Progress until the two points meet
    let mut number_of_steps = 1;
    while points[0].0.0 != points[1].0.0 || points[0].0.1 != points[1].0.1 {
        let mut new_points: Vec<((usize, usize), (isize, isize))> = Vec::new();
        
        // Compute the next step for each of the two points
        for p in &points {
            let d = direction(p.1, grid[p.0.1][p.0.0]);
            let n = next(p.0, d);
            new_points.push((n, d));
        }

        // Check if the two progress points have reached each other
        if points[0].0.0 == new_points[1].0.0 && points[0].0.1 == new_points[1].0.1 {
            break;
        }
        points = new_points;
        number_of_steps += 1;
    }
    println!("number of steps: {}", number_of_steps);
}

fn next(p: (usize, usize), d: (isize, isize)) -> (usize, usize) {
    let n:(usize, usize) = ((p.0 as isize + d.0) as usize, (p.1 as isize + d.1) as usize);
    n
}

// Returns the direction specified by the character.
fn direction(d: (isize, isize), c: char) -> (isize, isize) {
    match c {
        '-' => if d.0 == 1 { (1, 0) } else { (-1, 0) },
        '|' => if d.1 == 1 { (0, 1) } else { (0, -1) },
        'F' => if d.1 == -1 { (1, 0) } else { (0, 1) },
        '7' => if d.0 == 1 { (0, 1) } else { (-1, 0) },
        'J' => if d.0 == 1 { (0, -1) } else { (-1, 0) },
        'L' => if d.1 == 1 { (1, 0) } else { (0, -1) },
        _ => (0, 0)
    }
}

// Returns the starting point of the grid
fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    panic!("No starting point found");
}
