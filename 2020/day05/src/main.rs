use std::{collections::HashSet, fs, io};

fn seat_id(line: &str) -> u32 {
    let (section, side) = line.split_at(7);
    let (row, _) = section.chars().fold((0, 127), |(x, y), ch| {
        let nxt = (y - x) / 2;
        match ch {
            'F' => (x, x + nxt),
            'B' => (x + nxt + 1, y),
            _ => (x, y),
        }
    });

    let (side, _) = side.chars().fold((0, 7), |(x, y), ch| {
        let nxt = (y - x) / 2;
        match ch {
            'L' => (x, x + nxt),
            'R' => (x + nxt + 1, y),
            _ => (x, y),
        }
    });

    (row * 8) + side
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut set = input
        .lines()
        .map(|line| seat_id(line))
        .collect::<HashSet<_>>();

    println!("Part 1: {:?}", set.iter().max());

    let lset = set.clone();
    set.retain(|&id| match id {
        0 => false,
        id => !(lset.contains(&(id - 1)) && lset.contains(&(id + 1))),
    });

    println!("Part 2: {:?}", set);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc_1() {
        let input = "FBFBBFFRLR";
        let output = seat_id(&input);
        assert_eq!(357, output, "seat id did not match");
    }
}
