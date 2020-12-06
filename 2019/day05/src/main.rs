use std::{fs::File, io::Read};

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

fn intcode(arr: &mut [i32], system: i32) -> Vec<i32> {
    let mut output = Vec::new();

    let mut idx: usize = 0;
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
                arr[arr[idx + 1] as usize] = system;
                idx += 2;
            }
            Instruction::Output(a) => {
                output.push(a.load(arr, idx + 1));
                idx += 2;
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
            Instruction::Halt => break,
        }
    }

    output
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let mut arr: Vec<i32> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    //let output = intcode(&mut arr, 1); // Part 1
    let output = intcode(&mut arr, 5); // Part 2
    println!("{:?}", output);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction_01() {
        let instr = Instruction::parse(1002);
        assert!(instr == Instruction::Multiply(Mode::Position, Mode::Immediate, Mode::Position));
    }

    #[test]
    fn parse_instruction_02() {
        let instr = Instruction::parse(10002);
        assert!(instr == Instruction::Multiply(Mode::Position, Mode::Position, Mode::Immediate));
    }

    #[test]
    fn parse_instruction_03() {
        let instr = Instruction::parse(10102);
        assert!(instr == Instruction::Multiply(Mode::Immediate, Mode::Position, Mode::Immediate));
    }

    #[test]
    fn parse_instruction_04() {
        let instr = Instruction::parse(102);
        assert!(instr == Instruction::Multiply(Mode::Immediate, Mode::Position, Mode::Position));
    }

    #[test]
    fn parse_instruction_05() {
        let instr = Instruction::parse(02);
        assert!(instr == Instruction::Multiply(Mode::Position, Mode::Position, Mode::Position));
    }

    #[test]
    fn parse_instruction_06() {
        let instr = Instruction::parse(2);
        assert!(instr == Instruction::Multiply(Mode::Position, Mode::Position, Mode::Position));
    }

    #[test]
    fn part2_01() {
        let mut arr: Vec<i32> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let output = intcode(&mut arr, 5);
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn part2_02() {
        let mut arr: Vec<i32> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let output = intcode(&mut arr, 8);
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn part2_03() {
        let mut arr: Vec<i32> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let output = intcode(&mut arr, 5);
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn part2_04() {
        let mut arr: Vec<i32> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let output = intcode(&mut arr, 9);
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn part2_05() {
        let mut arr: Vec<i32> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let output = intcode(&mut arr, 8);
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn part2_06() {
        let mut arr: Vec<i32> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let output = intcode(&mut arr, 9);
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn part2_07() {
        let mut arr: Vec<i32> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let output = intcode(&mut arr, 7);
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn part2_08() {
        let mut arr: Vec<i32> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let output = intcode(&mut arr, 8);
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn part2_09() {
        let mut arr: Vec<i32> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let output = intcode(&mut arr, 0);
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn part2_10() {
        let mut arr: Vec<i32> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let output = intcode(&mut arr, 1);
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn part2_11() {
        let mut arr: Vec<i32> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let output = intcode(&mut arr, 0);
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn part2_12() {
        let mut arr: Vec<i32> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let output = intcode(&mut arr, 1);
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn part2_13() {
        let mut arr: Vec<i32> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let output = intcode(&mut arr, 1);
        assert_eq!(output, vec![999]);
    }

    #[test]
    fn part2_14() {
        let mut arr: Vec<i32> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let output = intcode(&mut arr, 8);
        assert_eq!(output, vec![1000]);
    }

    #[test]
    fn part2_15() {
        let mut arr: Vec<i32> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let output = intcode(&mut arr, 10);
        assert_eq!(output, vec![1001]);
    }
}
