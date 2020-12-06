use permutohedron::LexicalPermutation;
use std::{cmp, fs::File, io::Read};

macro_rules! mode {
    ($x:expr,$p:expr) => {
        match ($x / 10_i32.pow($p + 1)) % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            x => panic!("Unrecognized mode: `{}`", x),
        }
    };
}

#[derive(Clone, Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

impl Mode {
    pub fn load(&self, arr: &[i32], idx: usize) -> i32 {
        match self {
            Mode::Position => arr[arr[idx] as usize],
            Mode::Immediate => arr[idx],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    Add(Mode, Mode, Mode),
    Multiply(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    JumpNotZero(Mode, Mode),
    JumpZero(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equal(Mode, Mode, Mode),
    Halt,
}

impl Instruction {
    pub fn parse(opcode: i32) -> Instruction {
        match opcode % 100 {
            1 => Instruction::Add(mode!(opcode, 1), mode!(opcode, 2), mode!(opcode, 3)),
            2 => Instruction::Multiply(mode!(opcode, 1), mode!(opcode, 2), mode!(opcode, 3)),
            3 => Instruction::Input(mode!(opcode, 1)),
            4 => Instruction::Output(mode!(opcode, 1)),
            5 => Instruction::JumpNotZero(mode!(opcode, 1), mode!(opcode, 2)),
            6 => Instruction::JumpZero(mode!(opcode, 1), mode!(opcode, 2)),
            7 => Instruction::LessThan(mode!(opcode, 1), mode!(opcode, 2), mode!(opcode, 3)),
            8 => Instruction::Equal(mode!(opcode, 1), mode!(opcode, 2), mode!(opcode, 3)),
            99 => Instruction::Halt,
            x => panic!("Unrecognized opcode `{}`", x),
        }
    }
}

fn intcode(
    arr: &mut [i32],
    inputs: &mut Vec<i32>,
    outputs: &mut Vec<i32>,
    start: &mut Option<usize>,
) {
    let mut idx = match start {
        Some(x) => *x,
        None => panic!("intcode machine halted!"),
    };

    loop {
        match Instruction::parse(arr[idx]) {
            Instruction::Add(a, b, _) => {
                arr[arr[idx + 3] as usize] = a.load(arr, idx + 1) + b.load(arr, idx + 2);
                idx += 4;
            }
            Instruction::Multiply(a, b, _) => {
                arr[arr[idx + 3] as usize] = a.load(arr, idx + 1) * b.load(arr, idx + 2);
                idx += 4;
            }
            Instruction::Input(_) => {
                arr[arr[idx + 1] as usize] = inputs.remove(0);
                idx += 2;
            }
            Instruction::Output(a) => {
                outputs.push(a.load(arr, idx + 1));
                *start = Some(idx + 2);
                break;
            }
            Instruction::JumpNotZero(a, b) => {
                if a.load(arr, idx + 1) != 0 {
                    idx = b.load(arr, idx + 2) as usize;
                } else {
                    idx += 3;
                }
            }
            Instruction::JumpZero(a, b) => {
                if a.load(arr, idx + 1) == 0 {
                    idx = b.load(arr, idx + 2) as usize;
                } else {
                    idx += 3;
                }
            }
            Instruction::LessThan(a, b, _) => {
                if a.load(arr, idx + 1) < b.load(arr, idx + 2) {
                    arr[arr[idx + 3] as usize] = 1;
                } else {
                    arr[arr[idx + 3] as usize] = 0;
                }
                idx += 4;
            }
            Instruction::Equal(a, b, _) => {
                if a.load(arr, idx + 1) == b.load(arr, idx + 2) {
                    arr[arr[idx + 3] as usize] = 1;
                } else {
                    arr[arr[idx + 3] as usize] = 0;
                }
                idx += 4;
            }
            Instruction::Halt => {
                *start = None;
                break;
            }
        }
    }
}

fn run_intcode(prog: Vec<i32>, phases: &[i32]) -> i32 {
    let mut prog_a = prog.clone();
    let mut prog_b = prog.clone();
    let mut prog_c = prog.clone();
    let mut prog_d = prog.clone();
    let mut prog_e = prog;

    let mut ctr_a: Option<usize> = Some(0);
    let mut ctr_b: Option<usize> = Some(0);
    let mut ctr_c: Option<usize> = Some(0);
    let mut ctr_d: Option<usize> = Some(0);
    let mut ctr_e: Option<usize> = Some(0);

    let mut data_a = vec![phases[0], 0];
    let mut data_b = vec![phases[1]];
    let mut data_c = vec![phases[2]];
    let mut data_d = vec![phases[3]];
    let mut data_e = vec![phases[4]];
    loop {
        intcode(&mut prog_a, &mut data_a, &mut data_b, &mut ctr_a);
        intcode(&mut prog_b, &mut data_b, &mut data_c, &mut ctr_b);
        intcode(&mut prog_c, &mut data_c, &mut data_d, &mut ctr_c);
        intcode(&mut prog_d, &mut data_d, &mut data_e, &mut ctr_d);
        intcode(&mut prog_e, &mut data_e, &mut data_a, &mut ctr_e);

        if ctr_e.is_none() {
            return data_a.remove(0);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let arr: Vec<i32> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let mut signal = 0;
    let mut phases = [0, 1, 2, 3, 4];
    loop {
        if !phases.next_permutation() {
            break;
        }
        signal = cmp::max(signal, run_intcode(arr.clone(), &phases));
    }

    println!("part1: {:?}", signal);

    let mut signal = 0;
    let mut phases = [5, 6, 7, 8, 9];
    loop {
        if !phases.next_permutation() {
            break;
        }
        signal = cmp::max(signal, run_intcode(arr.clone(), &phases));
    }

    println!("part2: {:?}", signal);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_01() {
        let prog = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let output = run_intcode(prog, &[4, 3, 2, 1, 0]);
        assert_eq!(output, 43210);
    }

    #[test]
    fn part1_02() {
        let prog = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let output = run_intcode(prog, &[0, 1, 2, 3, 4]);
        assert_eq!(output, 54321);
    }

    #[test]
    fn part1_03() {
        let prog = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let output = run_intcode(prog, &[1, 0, 4, 3, 2]);
        assert_eq!(output, 65210);
    }

    #[test]
    fn part2_01() {
        let prog = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let output = run_intcode(prog, &[9, 8, 7, 6, 5]);
        assert_eq!(output, 139629729);
    }

    #[test]
    fn part2_02() {
        let prog = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let output = run_intcode(prog, &[9, 7, 8, 5, 6]);
        assert_eq!(output, 18216);
    }
}
