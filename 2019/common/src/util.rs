use std::collections::HashMap;

pub fn print_grid(grid: &HashMap<(i64, i64), i64>, chars: &HashMap<i64, char>, default: i64) {
    let min_x = *grid.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *grid.keys().map(|(x, _)| x).max().unwrap();
    let min_y = *grid.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *grid.keys().map(|(_, y)| y).max().unwrap();

    for yi in (min_y..max_y).rev() {
        for xi in min_x..max_x {
            let ch = chars
                .get(&grid.get(&(xi, yi)).map(|v| *v).unwrap_or(default))
                .unwrap();
            print!("{}", ch);
        }

        println!("");
    }
}

pub fn distance(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    let (p1_x, p1_y) = p1;
    let (p2_x, p2_y) = p2;

    (p1_x - p2_x).abs() + (p1_y - p2_y).abs()
}
