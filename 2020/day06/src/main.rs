use std::{collections::HashSet, fs, io};

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| c.is_ascii_lowercase()) // to ignore newlines/whitespace
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let all = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]
    .into_iter()
    .collect::<HashSet<_>>();

    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold(all.clone(), |acc, group| {
                    acc.intersection(&group).map(|c| *c).collect::<HashSet<_>>()
                })
                .len()
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    // Part 1: 6596
    println!("Part 1: {}", part1(&input));

    // Part 2: 3219
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_1() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

        let output = part1(&input);
        assert_eq!(11, output, "bad count");
    }

    #[test]
    fn aoc_2() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

        let output = part2(&input);
        assert_eq!(6, output, "bad count");
    }
}
