use common::intcode::{intcode, intcode_init};
use std::{fs::File, io::Read};

fn part1(input: &str) -> usize {
    let (mut prog, mut input, mut output, mut state) = intcode_init(input, 4096);

    while !state.is_halted() {
        intcode(&mut prog, &mut input, &mut output, &mut state);
    }

    output.chunks(3).filter(|c| c[2] == 2).count()
}

fn part2(input: &str) -> i64 {
    let (mut prog, mut input, mut output, mut state) = intcode_init(input, 4096);

    // run once to setup the game board
    prog[0] = 2;
    intcode(&mut prog, &mut input, &mut output, &mut state);

    // find paddle start
    let mut paddle = output
        .chunks(3)
        .filter(|c| c[2] == 3)
        .nth(0)
        .expect("failed to find paddle start location!")
        .to_vec();

    while !state.is_halted() {
        // find the ball
        let ball = output
            .chunks(3)
            .filter(|c| c[2] == 4)
            .nth(0)
            .expect("failed to find ball");

        // ball moves at same speed as paddle, so keep the x values sync'd
        if ball[0] > paddle[0] {
            input.push(1);
            paddle[0] += 1;
        } else if ball[0] < paddle[0] {
            input.push(-1);
            paddle[0] -= 1;
        } else {
            input.push(0);
        }

        output.clear();
        intcode(&mut prog, &mut input, &mut output, &mut state);
    }

    output
        .chunks(3)
        .filter(|c| c[0] == -1 && c[1] == 0)
        .map(|c| c[2])
        .nth(0)
        .expect("failed to find score!")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("Block tiles: {}", part1(&input));
    println!("High Score:  {}", part2(&input));

    Ok(())
}
