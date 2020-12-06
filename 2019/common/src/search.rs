use std::{collections::HashSet, hash::Hash};

pub type Coord = (usize, usize);
pub type Maze = Vec<Vec<char>>;

/// Generate neighbor positions, ensuring the returned vector only contains nodes
/// inside the bounds of the maze
///
/// ### Arguments
/// * `positions` - modifiers to current coord to get neighbors (e.g., (1, 0), (0, -1))
/// * `coord` - current coordinate
/// * `x_max` - maximum x coordinate size
/// * `y_max` - maximum y coordiante size
pub fn get_neighbors(
    positions: &[(i64, i64)],
    coord: Coord,
    x_max: usize,
    y_max: usize,
) -> Vec<Coord> {
    let (x, y) = coord;
    let x = x as i64;
    let y = y as i64;
    positions
        .iter()
        .map(|(xi, yi)| (xi + x, yi + y))
        .filter(|&(xi, yi)| xi >= 0 && yi >= 0)
        .map(|(xi, yi)| (xi as usize, yi as usize))
        .filter(|&(xi, yi)| xi < x_max && yi < y_max)
        .collect()
}

/// Breadth-First Search Algorithm
///
/// ### Arguments
/// * `maze` - The maze or grid to wander through
/// * `start` - The starting coordinates
/// * `goal` - The target end coordinate
/// * `map` - Modifies neighbor nodes
/// * `filter` - Additional filter criteria/rules
pub fn bfs<C, N, M, F>(start: C, goal: C, gen_neighbors: N, map: M, filter: F) -> Option<usize>
where
    C: Copy + Eq + PartialEq + Hash,
    N: Fn(C) -> Vec<C>,
    M: Fn(C) -> C,
    F: Fn(&C) -> bool,
{
    let mut depth = None;
    let mut queue = vec![];
    let mut seen = HashSet::new();

    queue.push((start, 0));
    while !queue.is_empty() {
        let (c, lvl) = queue.remove(0);
        if c == goal {
            depth = Some(lvl);
            break;
        }

        let neighbors: Vec<C> = gen_neighbors(c)
            .into_iter()
            .map(|coord| map(coord))
            .filter(|&coord| filter(&coord))
            .filter(|&coord| !seen.contains(&coord)) // filter out nodes we've been to
            .collect();

        for neighbor in neighbors {
            seen.insert(c);
            queue.push((neighbor, lvl + 1));
        }
    }

    depth
}
