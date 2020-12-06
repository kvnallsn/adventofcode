use std::{collections::HashSet, fs::File, io::Read};

type Maze = Vec<Vec<char>>;
type MazeRef = [Vec<char>];

const ADJACENT: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn part1(maze: &MazeRef) {
    // Find shortest path between the keys, and BFS to each key
    // Treat doors as walls until unlocked

    // width/height of the maze
    let rows = maze.len();
    let cols = maze[0].len();

    // where our starting position is
    let mut start = None;

    // find all the keys and the starting position
    let mut key_locations: Vec<(usize, usize)> = vec![];
    let mut doors = vec![];
    for (y, row) in maze.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch.is_ascii_lowercase() {
                key_locations.push((y, x));
            }

            if ch.is_ascii_uppercase() {
                doors.push(ch);
            }

            if ch == '@' {
                start = Some((y, x));
            }
        }
    }

    let start = start.expect("failed to find starting location");
    let mut seen = HashSet::<(usize, usize)>::new();
    let mut queue = vec![(start, HashSet::<char>::new())];
    seen.insert(start);

    println!("Doors: {:?}", doors);
    while !queue.is_empty() {
        let ((x, y), mut keys) = queue.remove(0);
        if maze[x][y].is_ascii_lowercase() {
            keys.insert(maze[x][y]);
            println!("found {}", maze[x][y]);
            // compute shortest path to all other lowercase nodes
            // prefer going to doors with keys
        }

        // to compute next position, convert to indexs to i64
        let (x, y) = (x as i64, y as i64);
        let nxt = ADJACENT
            .iter()
            .map(|(nx, ny)| (x + nx, y + ny))
            .filter(|&(x, y)| x >= 0 && y >= 0) // make sure we don't go outside the left/top of the maze
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|&(x, y)| x < rows && y < cols) // make sure we don't go outside the right/bottom of the maze
            .filter(|&p| !seen.contains(&p)) // make sure we haven't seen this yet
            .filter(|&(x, y)| {
                keys.contains(&maze[x][y].to_ascii_uppercase())
                    || maze[x][y] == '.'
                    || maze[x][y].is_ascii_lowercase()
            }) // make sure we have a key to a door
            .collect::<Vec<(usize, usize)>>();

        // mark each node as discovered
        nxt.iter().for_each(|p| {
            seen.insert(*p);
            queue.push((*p, keys.clone()));
        });
    }
}

/*
fn part2(input: &str) {
    // TODO
}
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    //let mut f = File::open("input.txt")?;
    let mut f = File::open("test_input_1.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let maze: Maze = input.lines().map(|line| line.chars().collect()).collect();

    part1(&maze);
    //part2(&input);

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
