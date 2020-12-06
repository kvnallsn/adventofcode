use common::intcode::{intcode, intcode_init};
use std::{fs::File, io::Read};

fn part1(input: &str) {
    let (mut prog, mut input, mut output, mut state) = intcode_init(input, 4096);

    while !state.is_halted() {
        intcode(&mut prog, &mut input, &mut output, &mut state);
    }

    let grid: Vec<Vec<char>> = output
        .iter()
        .map(|i| match i {
            35 => '#',
            46 => '.',
            10 => '\n',
            60 => '<',
            62 => '>',
            94 => '^',
            x => panic!("unknown output: {}", x),
        })
        .collect::<String>()
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let positions: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut sum = 0;
    for (y, col) in grid.iter().enumerate() {
        for (x, ch) in col.iter().enumerate() {
            if *ch != '#' {
                continue;
            }
            let intersections = positions
                .iter()
                .map(|(px, py)| (x as i64 + px, y as i64 + py))
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .filter(|(x, y)| *x < col.len() && *y < grid.len())
                .filter(|(x, y)| grid[*y][*x] == '#')
                .count();

            if intersections == 4 {
                sum += x * y;
            }
        }
    }

    println!("Part 1: {}", sum);
}

fn part2(input: &str) {
    let (mut prog, mut input, mut output, mut state) = intcode_init(input, 4096);
    prog[0] = 2;

    // solved by hand, but essentially allows follow the largest path
    // then find the largest common substrings
    let main = "A,B,A,B,A,C,A,C,B,C\n";
    let a = "R,6,L,10,R,10,R,10\n";
    let b = "L,10,L,12,R,10\n";
    let c = "R,6,L,12,L,10\n";
    let feed = "n\n";

    input.append(&mut main.as_bytes().iter().map(|b| *b as i64).collect());
    input.append(&mut a.as_bytes().iter().map(|b| *b as i64).collect());
    input.append(&mut b.as_bytes().iter().map(|b| *b as i64).collect());
    input.append(&mut c.as_bytes().iter().map(|b| *b as i64).collect());
    input.append(&mut feed.as_bytes().iter().map(|b| *b as i64).collect());

    while !state.is_halted() {
        intcode(&mut prog, &mut input, &mut output, &mut state);
    }

    if output.is_empty() {
        panic!("no output!");
    } else {
        println!("Part 2: {}", output.pop().unwrap());
    }
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
