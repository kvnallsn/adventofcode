use std::{fs::File, io::Read, str::FromStr};

fn part1(input: &str) {
    let fuel = input
        .lines()
        .map(|l| i32::from_str(l).unwrap())
        .fold(0, |acc, mass| acc + ((mass / 3) - 2));

    println!("fuel: {}", fuel);
}

fn part2(input: &str) {
    let fuel = input
        .lines()
        .map(|l| i32::from_str(l).unwrap())
        .fold(0, |acc, mass| {
            let mut total = 0;
            let mut mass = mass;
            loop {
                mass = (mass / 3) - 2;
                if mass <= 0 {
                    break;
                }
                total += mass;
            }
            acc + total
        });

    println!("fuel: {}", fuel);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    part1(&input);
    part2(&input);

    Ok(())
}
