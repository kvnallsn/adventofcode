use std::{collections::HashMap, fs::File, io::Read};

fn part1(input: &str) -> u32 {
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    let nodes: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(')').collect())
        .collect();

    // Build Tree
    for node in nodes {
        if let Some(kids) = tree.get_mut(&node[0]) {
            kids.push(node[1]);
        } else {
            tree.insert(node[0], vec![node[1]]);
        }
    }

    let mut node_map: HashMap<&str, u32> = HashMap::new();
    node_map.insert("COM", 0);

    let mut to_visit = vec!["COM"];
    while to_visit.len() != 0 {
        let node = to_visit.remove(0);

        // check if node is in map
        let val = node_map.get(node).map(|x| *x).unwrap_or(0);

        if let Some(kids) = tree.get(node) {
            for kid in kids {
                node_map.insert(kid, 1 + val);
            }
            to_visit.extend_from_slice(kids);
        }
    }

    node_map.values().fold(0, |acc, x| acc + *x)
}

fn part2(input: &str) -> u32 {
    let nodes: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(')').collect())
        .collect();

    let mut tree: HashMap<&str, &str> = HashMap::new();
    for node in nodes {
        tree.insert(node[1], node[0]);
    }

    // Build a map from SAN -> COM
    let mut san_map = vec!["SAN"];
    let mut cur = "SAN";
    while cur != "COM" {
        cur = tree.get(cur).unwrap();
        san_map.push(cur);
    }

    // Build a map from YOU -> COM
    let mut you_map = vec!["YOU"];
    let mut cur = "YOU";
    while cur != "COM" {
        cur = tree.get(cur).unwrap();
        you_map.push(cur);
    }

    let mut y = 0;
    let mut s = 0;
    let mut stop = false;
    for you in &you_map {
        let mut sy = 0;
        for map in &san_map {
            if you == map {
                s = sy;
                stop = true;
                break;
            }
            sy += 1;
        }
        if stop {
            break;
        }
        y += 1;
    }

    // Find first overlapping node
    s + y - 2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(part1(input), 42);
    }

    #[test]
    fn test_02() {
        let input = "B)F\nCOM)A\nA)B";
        assert_eq!(part1(input), 6);
    }
}
