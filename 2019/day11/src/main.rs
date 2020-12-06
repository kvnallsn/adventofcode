use common::intcode::{intcode, intcode_init};
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn turn(&self, dir: i64) -> Direction {
        match self {
            Direction::Left if dir == 0 => Direction::Down,
            Direction::Left if dir == 1 => Direction::Up,
            Direction::Right if dir == 0 => Direction::Up,
            Direction::Right if dir == 1 => Direction::Down,
            Direction::Up if dir == 0 => Direction::Left,
            Direction::Up if dir == 1 => Direction::Right,
            Direction::Down if dir == 0 => Direction::Right,
            Direction::Down if dir == 1 => Direction::Left,
            _ => panic!("Unknown direction!"),
        }
    }
}

fn part1(input: &str) {
    // mapping of coordinates visited to current panel color
    let mut visited: HashMap<(i64, i64), i64> = HashMap::new();

    let (mut prog, mut inputs, mut outputs, mut state) = intcode_init(input, 4096);

    let mut dir = Direction::Up;
    let mut coord = (0, 0);
    while !state.is_halted() {
        // if not present, default color is black
        inputs.push(*visited.get(&coord).unwrap_or(&0));
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        visited.insert(coord, outputs.remove(0));
        dir = dir.turn(outputs.remove(0));
        let (x, y) = coord;
        coord = match dir {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y + 1),
            Direction::Down => (x, y - 1),
        };
    }

    println!("Painted Panels: {}", visited.keys().len());
}

fn part2(input: &str) {
    // mapping of coordinates visited to current panel color
    let mut grid = [[0; 70]; 70];

    let (mut prog, mut inputs, mut outputs, mut state) = intcode_init(input, 4096);

    let mut coord = (25, 25);
    let mut dir = Direction::Up;

    // start on a white square
    grid[25][25] = 1;
    while !state.is_halted() {
        let (x, y) = coord;
        inputs.push(grid[y][x]);
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        grid[y][x] = outputs.remove(0);
        dir = dir.turn(outputs.remove(0));
        coord = match dir {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y + 1),
            Direction::Down => (x, y - 1),
        };
    }

    grid.iter().rev().for_each(|col| {
        col.iter()
            .for_each(|c| print!("{}", if *c == 1 { "#" } else { " " }));
        println!("");
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    part1(&input);
    part2(&input);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        //
    }
}
