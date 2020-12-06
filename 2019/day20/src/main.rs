use common::search::{bfs, get_neighbors, Coord, Maze};
use std::{collections::HashMap, fs::File, io::Read};

const POSITIONS: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const PORTAL_POSITIONS: [(i64, i64); 2] = [(1, 0), (0, 1)];

type PortalMap = HashMap<Coord, (Coord, Direction)>;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Down,
    Up,
}

pub fn gen_2d_neighbors(
    x_max: usize,
    y_max: usize,
) -> impl Fn((usize, usize)) -> Vec<(usize, usize)> {
    move |(x, y)| {
        [(1_i64, 0_i64), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(xi, yi)| (xi + (x as i64), yi + (y as i64)))
            .filter(|&(xi, yi)| xi >= 0 && yi >= 0)
            .map(|(xi, yi)| (xi as usize, yi as usize))
            .filter(|&(xi, yi)| xi < x_max && yi < y_max)
            .collect()
    }
}

pub fn gen_3d_neighbors(
    x_max: usize,
    y_max: usize,
    z_max: usize,
) -> impl Fn((usize, usize, usize)) -> Vec<(usize, usize, usize)> {
    move |(x, y, z)| {
        [(1_i64, 0_i64, 0_i64), (-1, 0, 0), (0, 1, 0), (0, -1, 0)]
            .into_iter()
            .map(|(xi, yi, zi)| (xi + (x as i64), yi + (y as i64), zi + (z as i64)))
            .filter(|&(xi, yi, zi)| xi >= 0 && yi >= 0 && zi >= 0)
            .map(|(xi, yi, zi)| (xi as usize, yi as usize, zi as usize))
            .filter(|&(xi, yi, zi)| xi < x_max && yi < y_max && zi < z_max)
            .collect()
    }
}

/// Parses the input for the puzzle and builds a map of all
/// the portals, start location and end goal
fn build_maze(input: &str) -> (Maze, PortalMap, Coord, Coord) {
    let maze: Maze = input.lines().map(|line| line.chars().collect()).collect();

    let height = maze.len();
    let width = maze[0].len();

    let mut start = None;
    let mut end = None;

    // look for portals
    let mut portals: HashMap<String, Vec<(Coord, Coord, Coord)>> = HashMap::new();
    for (y, row) in maze.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if ch.is_ascii_uppercase() {
                // look left, down, up, right for next character
                if let Some((xl, yl)) = get_neighbors(&PORTAL_POSITIONS, (x, y), width, height)
                    .into_iter()
                    .filter(|&(xi, yi)| maze[yi][xi].is_ascii_uppercase())
                    .nth(0)
                {
                    let code = format!("{}{}", ch, maze[yl][xl]);

                    // ok, now found the entry position for this portal
                    let entry_a = get_neighbors(&POSITIONS, (x, y), width, height)
                        .into_iter()
                        .filter(|&(xc, yc)| maze[yc][xc] == '.')
                        .nth(0);

                    let entry_b = get_neighbors(&POSITIONS, (xl, yl), width, height)
                        .into_iter()
                        .filter(|&(xc, yc)| maze[yc][xc] == '.')
                        .nth(0);

                    let entry = entry_a.unwrap_or_else(|| {
                        entry_b.expect("failed to find portal entry coordinate")
                    });

                    if &code == "AA" {
                        start = Some(entry);
                    } else if &code == "ZZ" {
                        end = Some(entry);
                    } else {
                        let node = ((x, y), (xl, yl), entry);
                        if let Some(v) = portals.get_mut(&code) {
                            v.push(node);
                        } else {
                            portals.insert(code, vec![node]);
                        }
                    }
                }
            }
        }
    }

    let start = start.expect("failed to find start position");
    let end = end.expect("failed to find end position");

    let mut portal_map = HashMap::new();
    portals.values().for_each(|coords| {
        let (aa, ab, entry_a) = coords[0];
        let (bb, ba, entry_b) = coords[1];

        // figure out which is inside and which is outside
        let (eax, eay) = entry_a;

        let mut entry_a_dir = Direction::Down;
        let mut entry_b_dir = Direction::Down;
        if (eax <= 2) || (eax >= maze[2].len() - 3) || (eay <= 2) || (eay >= maze.len() - 3) {
            // entry a is the outer node, we are going down a level
            entry_b_dir = Direction::Up;
        } else {
            // entry b is the outer node, we are going up a level
            entry_a_dir = Direction::Up;
        }

        portal_map.insert(bb, (entry_a, entry_a_dir));
        portal_map.insert(ba, (entry_a, entry_a_dir));
        portal_map.insert(aa, (entry_b, entry_b_dir));
        portal_map.insert(ab, (entry_b, entry_b_dir));
    });

    (maze, portal_map, start, end)
}

fn part1(maze: &Maze, portals: &PortalMap, start: Coord, end: Coord) {
    let path = bfs(
        start,
        end,
        gen_2d_neighbors(maze[0].len(), maze.len()),
        |coord| portals.get(&coord).map(|(c, _)| *c).unwrap_or(coord),
        |&(xi, yi)| maze[yi][xi] == '.' || maze[yi][xi].is_ascii_uppercase(),
    );

    match path {
        Some(p) => println!("Part 1: {}", p),
        None => println!("Part 1 not solved"),
    }
}

fn part2(maze: &Maze, portals: &PortalMap, start: Coord, end: Coord) {
    // treat as a 3d-puzzle, each "recursion" adds depth. start / goal is on level 0
    let (sx, sy) = start;
    let start = (sx, sy, 0);

    let (gx, gy) = end;
    let goal = (gx, gy, 0);

    let path = bfs(
        start,
        goal,
        gen_3d_neighbors(maze[0].len(), maze.len(), 35),
        |(x, y, z)| {
            portals
                .get(&(x, y))
                .map(|((xi, yi), d)| {
                    if z == 0 && d == &Direction::Up {
                        (x, y, z)
                    } else {
                        (
                            *xi,
                            *yi,
                            match d {
                                Direction::Down => z + 1,
                                Direction::Up => z - 1,
                            },
                        )
                    }
                })
                .unwrap_or((x, y, z))
        },
        |&(x, y, _)| maze[y][x] == '.' || maze[y][x].is_ascii_uppercase(),
    );

    match path {
        Some(p) => println!("Part 2: {}", p),
        None => println!("Part 2 not solved"),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let (maze, portals, start, end) = build_maze(&input);
    part1(&maze, &portals, start.clone(), end.clone());
    part2(&maze, &portals, start, end);

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
