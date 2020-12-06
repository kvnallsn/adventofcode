use common::intcode::Intcode;
use std::{
    fs::File,
    io::{self, Read},
};

fn run(input: &str, prog: Option<&str>) {
    let mut intcode = Intcode::init(input);

    if let Some(prog) = prog {
        intcode.append(
            &prog
                .as_bytes()
                .into_iter()
                .map(|&i| i as i64)
                .collect::<Vec<i64>>(),
        );
    }
    while !intcode.is_halted() {
        intcode.run();

        if intcode.has_output() {
            let mut output = intcode.output();
            if prog.is_none() {
                let result =
                    String::from_utf8(output.iter().map(|i| (i % 256) as u8).collect::<Vec<u8>>());

                match result {
                    Ok(output) => println!("{}", output),
                    Err(_) => println!("success! {:?}", output),
                }
            } else {
                println!("ans: {}", output.pop().expect("no output!"));
            }
        }

        if intcode.is_blocked() {
            // read from stdin
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("failed to read line");

            intcode.append(
                &buffer
                    .as_bytes()
                    .into_iter()
                    .map(|&i| i as i64)
                    .collect::<Vec<i64>>(),
            );
        }
    }
}

fn part1(input: &str) {
    // by trial and error
    let prog = "NOT C J\nNOT A T\nOR T J\nAND D J\nWALK\n";
    run(input, Some(prog));
}

fn part2(input: &str) {
    // by trial and error
    let prog = "OR A T\nAND B T\nAND C T\nNOT T T\nOR E J\nOR H J\nAND T J\nAND D J\nRUN\n";
    run(input, Some(prog));
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
