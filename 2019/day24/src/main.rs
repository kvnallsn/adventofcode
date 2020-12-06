use std::{collections::HashSet, fs::File, io::Read};

type Grid = Vec<char>;

macro_rules! neighbors {
    ($x:expr, $g:expr, $w:expr) => {{
        let mut v = vec![];
        // look up
        if $x >= $w {
            v.push(
                $g[$x
                    .checked_sub($w)
                    .expect(&format!("up underflow: {} - {}", $x, $w))],
            );
        }

        // look down
        if ($x + $w) < $g.len() {
            v.push($g[$x + $w]);
        }

        // look left
        if $x % $w != 0 {
            v.push(
                $g[$x
                    .checked_sub(1)
                    .expect(&format!("left underflow: {} - 1", $x))],
            );
        }

        // look right
        if ($x + 1) % $w != 0 {
            v.push($g[$x + 1]);
        }

        v
    }};
}

macro_rules! neighbors_3d {
    ($x:expr, $g:expr, $w:expr, $lvl:expr) => {{
        let mut v = vec![];
        // look up
        if $x >= $w {
            v.push(
                $g[$lvl][$x
                    .checked_sub($w)
                    .expect(&format!("up underflow: {} - {}", $x, $w))],
            );
        }

        // look down
        if ($x + $w) < $g.len() {
            v.push($g[$lvl][$x + $w]);
        }

        // look left
        if $x % $w != 0 {
            // check if touching dead center node
            let n = $x.checked_sub(1).expect("left underflow: {} - 1", $x);
            if n == 13 {
                // center, go down a level on right
            } else {
                v.push($g[n]);
            }
        } else {
            // left edge, mark up a level on left
        }

        // look right
        if ($x + 1) % $w != 0 {
            let n = $x + 1;
            if n == 13 {
                // center, go down a level on left
            } else {
                v.push($g[n]);
            }
        } else {
            // right edge, mark up a level on right
        }

        v
    }};
}

fn read_grid(filename: &str) -> Result<Grid, Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open(filename)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    Ok(input.chars().filter(|&ch| ch != '\n').collect())
}

#[allow(dead_code)]
fn print_grid(grid: &Grid, width: usize, time: usize) {
    println!("Time: {}", time);
    for (x, ch) in grid.iter().enumerate() {
        print!(
            "{}{}",
            ch,
            match (x + 1) % width {
                0 => "\n",
                _ => "",
            }
        );
    }
    print!("\n");
}

fn part1(filename: &str) -> usize {
    // Load the file
    let mut grid = read_grid(filename).expect(&format!("failed to read file: {}", filename));

    //print_grid(&grid, 5, 0);

    let mut seen = HashSet::new();
    seen.insert(grid.clone());
    for _time in 1..500 {
        let mut nxt = vec![];
        for (i, ch) in grid.iter().enumerate() {
            let cnt = neighbors!(i, grid, 5)
                .into_iter()
                .filter(|&c| c == '#')
                .count();
            nxt.push(match ch {
                '.' if cnt == 1 || cnt == 2 => '#',
                '#' if cnt == 1 => '#',
                _ => '.',
            });
        }

        //print_grid(&nxt, 5, time);
        grid = nxt;
        if !seen.insert(grid.clone()) {
            break;
        }
    }

    //println!("Repeat");
    //print_grid(&grid, 5, 0);

    // score the repeat
    let score = grid
        .into_iter()
        .enumerate()
        .filter(|&(_, ch)| ch == '#')
        .fold(0, |acc, (i, _)| acc + 2_usize.pow(i as u32));

    score
}

fn part2(filename: &str, width: usize) {
    let mut grids: Vec<Grid> = vec![vec!['.'; width * width]; 201];
    grids[100] = read_grid(filename).expect(&format!("failed to read file: {}", filename));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Part 1 Score: {}", part1("input.txt"));
    part2("input.txt", 5);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_neighbors_01() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_grid("test_input_1.txt")?;
        assert_eq!(vec!['.', '.', '.'], neighbors!(23_usize, grid, 5));
        Ok(())
    }

    #[test]
    fn macro_neighbors_02() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_grid("test_input_1.txt")?;
        assert_eq!(vec!['.', '.', '.', '.'], neighbors!(17_usize, grid, 5));
        Ok(())
    }

    #[test]
    fn macro_neighbors_03() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_grid("test_input_1.txt")?;
        assert_eq!(vec!['.', '#', '.', '#'], neighbors!(12_usize, grid, 5));
        Ok(())
    }

    #[test]
    fn macro_neighbors_04() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_grid("test_input_1.txt")?;
        assert_eq!(vec!['#', '.'], neighbors!(0_usize, grid, 5));
        Ok(())
    }

    #[test]
    fn macro_neighbors_05() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_grid("test_input_1.txt")?;
        assert_eq!(vec!['.', '.'], neighbors!(24_usize, grid, 5));
        Ok(())
    }

    #[test]
    fn part1_01() {
        assert_eq!(part1("test_input_1.txt"), 2129920);
    }
}
