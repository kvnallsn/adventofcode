use std::{fs::File, io::Read};

fn boost(mut prog: Vec<i64>, input: i64, msg: &'static str) {
    let mut inputs = vec![input];
    let mut outputs = vec![];
    let mut state = common::intcode::State::new();

    prog.resize(4096, 0);
    common::intcode::intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
    if outputs.len() != 1 {
        println!("Malfunctioning opcodes: {:?}", outputs);
    } else {
        println!("{}: {}", msg, outputs[0]);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let prog: Vec<i64> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    boost(prog.clone(), 1, "BOOST keycode");
    boost(prog.clone(), 2, "Distress Singal Coordinates");

    Ok(())
}
