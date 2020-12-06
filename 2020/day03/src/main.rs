use anyhow::{bail, Result};
use std::fs;

#[derive(Debug, PartialEq)]
enum Space {
    Open,
    Tree,
}

fn check_slope(grid: &[Vec<Space>], dx: usize, dy: usize) -> usize {
    let mut pos = (0, 0);
    let mut trees = 0;
    while pos.1 < grid.len() {
        let (x, y) = pos;
        if grid[y][x % 31] == Space::Tree {
            trees += 1;
        }
        pos = (x + dx, y + dy);
    }
    trees
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Open,
                    _ => Space::Tree,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if grid.len() == 0 {
        bail!("failed to read input")
    }

    let r3d1 = check_slope(&grid, 3, 1);
    println!("Part 1: {}", r3d1);

    let r1d1 = check_slope(&grid, 1, 1);
    let r5d1 = check_slope(&grid, 5, 1);
    let r7d1 = check_slope(&grid, 7, 1);
    let r1d2 = check_slope(&grid, 1, 2);
    let part2 = r1d1 * r3d1 * r5d1 * r7d1 * r1d2;
    println!("Part 2: {}", part2);

    Ok(())
}
