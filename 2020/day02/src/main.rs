use regex::Regex;
use std::{fs, io};

fn main() -> io::Result<()> {
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<password>\w+)").unwrap();
    let data = fs::read_to_string("input.txt")?;

    let mut correct_1 = 0;
    let mut correct_2 = 0;
    for line in data.lines() {
        if let Some(groups) = re.captures(line) {
            let min: usize = groups.name("min").unwrap().as_str().parse().unwrap();
            let max: usize = groups.name("max").unwrap().as_str().parse().unwrap();
            let letter: char = groups.name("letter").unwrap().as_str().parse().unwrap();
            let password = groups.name("password").unwrap().as_str();

            // part 1
            let count = password.chars().filter(|c| *c == letter).count();
            if (count >= min) && (count <= max) {
                correct_1 += 1;
            }

            // part 2
            let password: Vec<char> = password.chars().collect();
            let min = password.get(min - 1);
            let max = password.get(max - 1);
            if let (Some(min), Some(max)) = (min, max) {
                if (min != max) && (*min == letter || *max == letter) {
                    correct_2 += 1;
                }
            }
        }
    }

    println!("Part 1: number of correct passwords: {}", correct_1);
    println!("Part 1: number of correct passwords: {}", correct_2);

    Ok(())
}
