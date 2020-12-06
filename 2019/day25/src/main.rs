use common::intcode::Intcode;
use std::{
    fs::File,
    io::{self, Read},
};

fn part1(input: &str) {
    let mut intcode = Intcode::init_memory(input, 16384);

    while !intcode.is_halted() {
        intcode.run();

        if intcode.has_output() {
            let output: Vec<u8> = intcode
                .output()
                .into_iter()
                .map(|i| (i & 0xff) as u8)
                .collect();
            print!(
                "{}",
                String::from_utf8(output).expect("failed to read output string")
            );
        }

        if intcode.is_blocked() {
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("failed to read stdin");
            let input: Vec<i64> = buffer.as_bytes().into_iter().map(|i| *i as i64).collect();
            intcode.append(&input);
        }
    }
}

/*
fn part2(input: &str) {
    // TODO
}
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    part1(&input);
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
