use std::{fs::File, io::Read};

fn intcode(mut arr: Vec<usize>, memory: Option<(usize, usize)>) -> usize {
    if let Some((noun, verb)) = memory {
        arr[1] = noun;
        arr[2] = verb;
    }

    let mut idx: usize = 0;
    while arr[idx] != 99 {
        // Read opcode
        let x = arr[idx + 1];
        let y = arr[idx + 2];
        let z = arr[idx + 3];
        arr[z] = match arr[idx] {
            1 => arr[x] + arr[y],
            2 => arr[x] * arr[y],
            n => panic!("Unrecognized Op Code: {}", n),
        };
        idx += 4;
    }

    arr[0]
}

fn part1(input: &str) {
    let arr: Vec<usize> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let ans = intcode(arr, Some((12, 2)));
    println!("arr[0] = {}", ans);
}

fn part2(input: &str) {
    let arr: Vec<usize> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    for n in 0..99 {
        for v in 0..99 {
            if intcode(arr.clone(), Some((n, v))) == 19690720 {
                println!("noun = {}, verb = {}, ans = {}", n, v, (100 * n + v));
                break;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    part1(&input);
    part2(&input);

    Ok(())
}
