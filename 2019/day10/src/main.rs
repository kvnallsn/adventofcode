use num::Integer;
use std::{collections::HashSet, fs::File, io::Read};

type Grid = Vec<Vec<char>>;

fn read_grid(file: &str) -> Result<Grid, std::io::Error> {
    let mut f = File::open(file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

fn score_cell(x: i32, y: i32, grid: &Grid) -> (HashSet<(i32, i32)>, (i32, i32)) {
    let mut seen = HashSet::new();

    // Compute start locations
    for (yi, line) in grid.iter().enumerate() {
        for (xi, c) in line.iter().enumerate() {
            let xi = xi as i32;
            let yi = yi as i32;
            if (x == xi && y == yi) || (*c != '#') {
                continue;
            }

            // compute distance between points
            let xd = xi - x;
            let yd = yi - y;

            // find greatest divisor between two inputs
            let gcd = xd.gcd(&yd);

            if gcd == 1 {
                // if the greatest divisor is one, no nodes in between
                seen.insert((xi, yi));
            } else {
                // otherwise check all nodes in between
                let mut visible = true;
                for i in 1..gcd {
                    let xs = x + ((xd / gcd) * i);
                    let ys = y + ((yd / gcd) * i);
                    visible = visible && grid[ys as usize][xs as usize] != '#';
                }

                if visible {
                    seen.insert((xi, yi));
                }
            }
        }
    }

    (seen, (x, y))
}

fn part1(grid: &Grid) -> (HashSet<(i32, i32)>, (i32, i32)) {
    // brute force for life
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(|(x, _)| score_cell(x as i32, y as i32, grid))
                .max_by(|(a, _), (b, _)| a.len().cmp(&b.len()))
                .unwrap()
        })
        .max_by(|(a, _), (b, _)| a.len().cmp(&b.len()))
        .unwrap()
}

fn part2(grid: &mut Grid, station: (i32, i32)) -> (i32, i32, f64) {
    let (sx, sy) = station;

    let (visible, _) = part1(grid);
    let mut angles = visible
        .iter()
        .map(|(nx, ny)| (nx, ny, ((nx - sx) as f64).atan2((sy - ny) as f64)))
        .map(|(nx, ny, na)| {
            if na < 0.0 {
                (*nx, *ny, 6.28 + na)
            } else {
                (*nx, *ny, na)
            }
        })
        .collect::<Vec<(i32, i32, f64)>>();
    angles.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());

    return angles[199];
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut grid = read_grid("input.txt")?;

    let (p1_set, p1_station) = part1(&grid);
    println!("Part 1: {} visible asteroids", p1_set.len());
    let (x, y, _) = part2(&mut grid, p1_station);
    println!("Part 2: {} ({}, {})", (x * 100) + y, x, y);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_grid() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_grid("test_input_1.txt")?;
        assert_eq!(
            grid,
            vec![
                vec!['.', '#', '.', '.', '#'],
                vec!['.', '.', '.', '.', '.'],
                vec!['#', '#', '#', '#', '#'],
                vec!['.', '.', '.', '.', '#'],
                vec!['.', '.', '.', '#', '#']
            ]
        );
        Ok(())
    }

    #[test]
    fn test_part1_01() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_grid("test_input_1.txt")?;
        let (nodes, _) = part1(&grid);
        assert_eq!(nodes.len(), 8);
        Ok(())
    }

    #[test]
    fn test_part1_02() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_grid("test_input_2.txt")?;
        let (nodes, _) = part1(&grid);
        assert_eq!(nodes.len(), 33);
        Ok(())
    }

    #[test]
    fn test_part1_03() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_grid("test_input_3.txt")?;
        let (nodes, _) = part1(&grid);
        assert_eq!(nodes.len(), 35);
        Ok(())
    }

    #[test]
    fn test_part1_04() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_grid("test_input_4.txt")?;
        let (nodes, _) = part1(&grid);
        assert_eq!(nodes.len(), 41);
        Ok(())
    }

    #[test]
    fn test_part1_05() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_grid("test_input_5.txt")?;
        let (nodes, _) = part1(&grid);
        assert_eq!(nodes.len(), 210);
        Ok(())
    }

    #[test]
    fn test_part2_01() -> Result<(), Box<dyn std::error::Error>> {
        let mut grid = read_grid("test_input_5.txt")?;
        let (x, y, _) = part2(&mut grid, (11, 13));
        assert_eq!((x * 100) + y, 802);
        Ok(())
    }
}
