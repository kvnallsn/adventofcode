use common::{
    intcode::{intcode, intcode_init},
    util::print_grid,
};
use pathfinding::prelude::{absdiff, astar};
use std::{collections::HashMap, fs::File, io::Read};

/// Build Grid/Map with a depth first search
fn build_grid(input: &str) -> (HashMap<(i64, i64), i64>, Option<(i64, i64)>) {
    let (prog, mut input, mut output, mut state) = intcode_init(input, 4096);

    let mut oxygen = None;
    let mut grid = HashMap::new();
    let mut to_visit = vec![];
    to_visit.push((1, (0, 1), prog.clone()));
    to_visit.push((2, (0, -1), prog.clone()));
    to_visit.push((3, (1, 0), prog.clone()));
    to_visit.push((4, (-1, 0), prog.clone()));
    grid.insert((0, 0), 3);
    while !to_visit.is_empty() {
        let (dir, (x, y), mut p) = to_visit.pop().unwrap();

        input.push(dir);
        intcode(&mut p, &mut input, &mut output, &mut state);
        let out = output.remove(0);
        grid.insert((x, y), out);

        if out == 1 {
            if !grid.contains_key(&(x, y + 1)) {
                to_visit.push((1, (x, y + 1), p.clone()));
            }

            if !grid.contains_key(&(x, y - 1)) {
                to_visit.push((2, (x, y - 1), p.clone()));
            }

            if !grid.contains_key(&(x + 1, y)) {
                to_visit.push((3, (x + 1, y), p.clone()));
            }

            if !grid.contains_key(&(x - 1, y)) {
                to_visit.push((4, (x - 1, y), p.clone()));
            }
        } else if out == 2 {
            oxygen = Some((x, y));
        }
    }

    (grid, oxygen)
}

/// Use A* Search to find path
fn search(
    grid: &HashMap<(i64, i64), i64>,
    start: (i64, i64),
    goal: (i64, i64),
) -> Option<(Vec<(i64, i64)>, i64)> {
    let result = astar(
        &start,
        |&(x, y)| {
            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .into_iter()
                .filter(|p| grid.get(p).unwrap() != &0)
                .map(|p| (p, 1))
        },
        |&(x, y)| absdiff(x, goal.0) + absdiff(y, goal.1),
        |&p| p == goal,
    );

    result
}

fn part1(grid: &HashMap<(i64, i64), i64>, oxygen: (i64, i64)) {
    let goal = (0, 0);
    if let Some((_, steps)) = search(&grid, oxygen, goal) {
        println!("Steps: {}", steps);
    }
}

/// Travse map with breadth first search from oxygen node
fn part2(grid: &mut HashMap<(i64, i64), i64>, oxygen: (i64, i64)) {
    let mut queue = vec![];
    let routes = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut level = 0;
    queue.push((oxygen, 0));

    while !queue.is_empty() {
        let (node, lvl) = queue.remove(0);
        if lvl > level {
            level = lvl;
        }

        let neighbors: Vec<(i64, i64)> = routes
            .iter()
            .map(|(x, y)| (node.0 + x, node.1 + y)) // generate all neighbor nodes
            .filter(|p| grid.get(p).unwrap() != &0) // filter out all walls
            .filter(|p| grid.get(p).unwrap() != &2) // filter out already oxygenated nodes
            .collect();

        for neighbor in neighbors {
            grid.insert(neighbor, 2);
            queue.push((neighbor, lvl + 1));
        }
    }

    println!("Oxygen restoration: {} minutes", level);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let (mut grid, oxygen) = build_grid(&input);
    let oxygen = oxygen.expect("failed to find oxygen source!");

    print_grid(&grid);

    part1(&grid, oxygen);
    part2(&mut grid, oxygen);

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
