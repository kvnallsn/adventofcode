use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

fn parse_input(input: &str) -> Vec<Vec<(&str, i32)>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.split_at(1))
                .map(|(d, i)| (d, i.parse().unwrap()))
                .collect()
        })
        .collect()
}

fn walk_list(nodes: &[(&str, i32)]) -> HashMap<(i32, i32), i32> {
    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();
    let mut cursor = (0, 0);
    let mut distance = 0;
    for (d, i) in nodes {
        let (x, y) = cursor;
        let (nx, ny) = match d {
            &"U" => {
                for n in (y + 1)..(y + i + 1) {
                    distance += 1;
                    visited.insert((x, n), distance);
                }
                (x, y + i)
            }
            &"D" => {
                for n in ((y - i)..y).rev() {
                    distance += 1;
                    visited.insert((x, n), distance);
                }
                (x, y - i)
            }
            &"L" => {
                for n in ((x - i)..x).rev() {
                    distance += 1;
                    visited.insert((n, y), distance);
                }
                (x - i, y)
            }
            &"R" => {
                for n in (x + 1)..(x + i + 1) {
                    distance += 1;
                    visited.insert((n, y), distance);
                }
                (x + i, y)
            }
            x => panic!("Unknown Direction `{}`!", x),
        };

        cursor = (nx, ny);
    }
    visited
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let lines = parse_input(&input);
    let a = walk_list(&lines[0]);
    println!("");
    let b = walk_list(&lines[1]);

    let mut a_k: HashSet<(i32, i32)> = HashSet::new();
    let mut b_k: HashSet<(i32, i32)> = HashSet::new();

    for node in a.keys() {
        a_k.insert(*node);
    }

    for (x, y) in b.keys() {
        b_k.insert((*x, *y));
    }

    let closest = a_k
        .intersection(&b_k)
        .map(|(x, y)| x.abs() + y.abs())
        .filter(|d| *d > 0)
        .min();

    println!("Part 1: {:?}", closest);

    let inter = a_k.intersection(&b_k);
    let min = inter
        .map(|node| {
            let a_s = a.get(node).unwrap();
            let b_s = b.get(node).unwrap();
            a_s + b_s
        })
        .min();

    println!("Part 2: {:?}", min);

    Ok(())
}
