use std::slice::Iter;

macro_rules! mode {
    ($x:expr,$p:expr) => {
        match ($x / 10_i64.pow($p + 1)) % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            x => panic!("Unrecognized mode: `{}`", x),
        }
    };
}

macro_rules! load {
    ($mode:expr, $prog:expr, $state:expr, $offset:expr) => {
        match $mode {
            Mode::Position => ($prog[$state.idx + $offset] as usize),
            Mode::Immediate => $state.idx + $offset,
            Mode::Relative => (($state.base + $prog[$state.idx + $offset]) as usize),
        }
    };
}

#[derive(Clone, Debug, PartialEq)]
enum Mode {
    /// Load the data at the absolute position in the program
    Position,

    /// Treat the value as an immediate
    Immediate,

    /// Load the data at the relative position according to the base pointer in the program
    Relative,
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
    AdjustBase(Mode),
    Halt,
}

impl Instruction {
    /// Parses an opcode returning the instruction to execute
    ///
    /// # Arguments
    /// * `opcode` - Opcode to parse
    pub fn parse(opcode: i64) -> Instruction {
        match opcode % 100 {
            1 => Instruction::Add(mode!(opcode, 1), mode!(opcode, 2), mode!(opcode, 3)),
            2 => Instruction::Multiply(mode!(opcode, 1), mode!(opcode, 2), mode!(opcode, 3)),
            3 => Instruction::Input(mode!(opcode, 1)),
            4 => Instruction::Output(mode!(opcode, 1)),
            5 => Instruction::JumpNotZero(mode!(opcode, 1), mode!(opcode, 2)),
            6 => Instruction::JumpZero(mode!(opcode, 1), mode!(opcode, 2)),
            7 => Instruction::LessThan(mode!(opcode, 1), mode!(opcode, 2), mode!(opcode, 3)),
            8 => Instruction::Equal(mode!(opcode, 1), mode!(opcode, 2), mode!(opcode, 3)),
            9 => Instruction::AdjustBase(mode!(opcode, 1)),
            99 => Instruction::Halt,
            x => panic!("Unrecognized opcode `{}`", x),
        }
    }
}

#[derive(Debug)]
/// Contains internal information regarding the operation of the intcode computer
pub struct State {
    /// Current instruction pointer
    idx: usize,

    /// Current offset to use for relative addressing
    base: i64,

    /// True if the program has halted, false otherwise
    halted: bool,

    /// True if the program is blocking on input, false otherwise
    blocked: bool,
}

impl State {
    /// Creates a new state instance for an intcode compute
    pub fn new() -> State {
        State {
            idx: 0,
            base: 0,
            halted: false,
            blocked: false,
        }
    }

    /// Returns true if the program has halted (i.e., read opcode 99)
    pub fn is_halted(&self) -> bool {
        self.halted
    }

    /// Returns true if the program is blocking on input (i.e., program
    /// needs to read input but there is no input to read)
    pub fn is_blocked(&self) -> bool {
        self.blocked
    }
}

/// Sets up all required input parameters for an intcode program
///
/// # Arguments
/// * `input` - Comma-delimitated string containing instructors
/// * `size` - Minimum size for the returned program (for scratch space)
pub fn intcode_init(input: &str, size: usize) -> (Vec<i64>, Vec<i64>, Vec<i64>, State) {
    (parse_program(input, size), vec![], vec![], State::new())
}

/// Parses a program from a string input and turns it into a vector readable
/// by the intcode computer
///
/// # Arguments
/// * `input` - Comma-delimitated string containing instructors
/// * `size` - Minimum size for the returned program (for scratch space)
pub fn parse_program(input: &str, size: usize) -> Vec<i64> {
    let mut prog: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    prog.resize(size, 0);
    prog
}

/// The intcode computer algorithm. Takes 4 parameters, the program to run, inputs, outputs and
/// the internal state.
///
/// # Arguments
/// * `prog` - Program to run
/// * `inputs` - Values to use as inputs when an input opcode is read.  Values will be removed from
///              the front of the vector (i.e., `inputs.remove(0)`
/// * `outputs` - Stores the outputs of the program.  Outputs will be pushed onto the back of the
///               vector as they are encountered
/// * `state` - Internal state of the intcode computer
pub fn intcode(prog: &mut [i64], inputs: &mut Vec<i64>, outputs: &mut Vec<i64>, state: &mut State) {
    if state.halted {
        return;
    }

    loop {
        match Instruction::parse(prog[state.idx]) {
            Instruction::Add(a, b, c) => {
                prog[load!(c, prog, state, 3)] =
                    prog[load!(a, prog, state, 1)] + prog[load!(b, prog, state, 2)];
                state.idx += 4;
            }
            Instruction::Multiply(a, b, c) => {
                prog[load!(c, prog, state, 3)] =
                    prog[load!(a, prog, state, 1)] * prog[load!(b, prog, state, 2)];
                state.idx += 4;
            }
            Instruction::Input(a) => {
                if inputs.is_empty() {
                    // yield waiting for input
                    state.blocked = true;
                    break;
                } else {
                    state.blocked = false;
                    prog[load!(a, prog, state, 1)] = inputs.remove(0);
                    state.idx += 2;
                }
            }
            Instruction::Output(a) => {
                outputs.push(prog[load!(a, prog, state, 1)]);
                state.idx += 2;
            }
            Instruction::JumpNotZero(a, b) => {
                state.idx = match prog[load!(a, prog, state, 1)] {
                    0 => state.idx + 3,
                    _ => prog[load!(b, prog, state, 2)] as usize,
                };
            }
            Instruction::JumpZero(a, b) => {
                state.idx = match prog[load!(a, prog, state, 1)] {
                    0 => prog[load!(b, prog, state, 2)] as usize,
                    _ => state.idx + 3,
                };
            }
            Instruction::LessThan(a, b, c) => {
                prog[load!(c, prog, state, 3)] =
                    (prog[load!(a, prog, state, 1)] < prog[load!(b, prog, state, 2)]).into();
                state.idx += 4;
            }
            Instruction::Equal(a, b, c) => {
                prog[load!(c, prog, state, 3)] =
                    (prog[load!(a, prog, state, 1)] == prog[load!(b, prog, state, 2)]).into();
                state.idx += 4;
            }
            Instruction::AdjustBase(a) => {
                state.base += prog[load!(a, prog, state, 1)];
                state.idx += 2;
            }
            Instruction::Halt => {
                state.halted = true;
                break;
            }
        }
    }
}

pub struct Intcode {
    src: String,
    prog: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
    state: State,
}

impl Intcode {
    pub fn init(src: &str) -> Intcode {
        Self::init_memory(src, 4096)
    }

    pub fn init_memory(src: &str, amt: usize) -> Intcode {
        let (prog, input, output, state) = intcode_init(src, amt);
        Intcode {
            src: src.to_owned(),
            prog,
            input,
            output,
            state,
        }
    }

    pub fn reset(&mut self) {
        let (prog, input, output, state) = intcode_init(&self.src, 4096);
        self.prog = prog;
        self.input = input;
        self.output = output;
        self.state = state;
    }

    pub fn run(&mut self) {
        intcode(
            &mut self.prog,
            &mut self.input,
            &mut self.output,
            &mut self.state,
        );
    }

    pub fn push(&mut self, val: i64) {
        self.input.push(val);
    }

    pub fn append(&mut self, vals: &[i64]) {
        self.input.extend_from_slice(vals);
    }

    /// returns true if output exists
    pub fn has_output(&self) -> bool {
        self.output.len() > 0
    }

    /// removes an element at a specific index
    pub fn pop(&mut self, i: usize) -> i64 {
        self.output.remove(i)
    }

    /// returns all output in the buffer then clears it
    pub fn output(&mut self) -> Vec<i64> {
        let output = self.output.clone();
        self.output.clear();
        output
    }

    // iterates over the output, leaving it in the output buffer
    pub fn iter(&self) -> Iter<i64> {
        self.output.iter()
    }

    pub fn is_halted(&self) -> bool {
        self.state.is_halted()
    }

    pub fn is_blocked(&self) -> bool {
        self.state.is_blocked()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_01() {
        let mut prog = vec![1, 0, 0, 0, 99];
        let mut inputs = vec![];
        let mut outputs = vec![];
        let mut state = State::new();
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        assert_eq!(prog, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn base_02() {
        let mut prog = vec![2, 3, 0, 3, 99];
        let mut inputs = vec![];
        let mut outputs = vec![];
        let mut state = State::new();
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        assert_eq!(prog, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn base_03() {
        let mut prog = vec![2, 4, 4, 5, 99, 0];
        let mut inputs = vec![];
        let mut outputs = vec![];
        let mut state = State::new();
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        assert_eq!(prog, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn base_04() {
        let mut prog = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut inputs = vec![];
        let mut outputs = vec![];
        let mut state = State::new();
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        assert_eq!(prog, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn position_01() {
        let mut prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut inputs = vec![7];
        let mut outputs = vec![];
        let mut state = State::new();
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        assert_eq!(outputs, vec![0]);
    }

    #[test]
    fn position_02() {
        let mut prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut inputs = vec![8];
        let mut outputs = vec![];
        let mut state = State::new();
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        assert_eq!(outputs, vec![1]);
    }

    #[test]
    fn immediate_01() {
        let mut prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut inputs = vec![7];
        let mut outputs = vec![];
        let mut state = State::new();
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        assert_eq!(outputs, vec![0]);
    }

    #[test]
    fn immediate_02() {
        let mut prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut inputs = vec![8];
        let mut outputs = vec![];
        let mut state = State::new();
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        assert_eq!(outputs, vec![1]);
    }

    #[test]
    fn relative_01() {
        let mut prog = vec![104, 1125899906842624, 99];
        let mut inputs = vec![];
        let mut outputs = vec![];
        let mut state = State::new();
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0], 1125899906842624);
    }

    #[test]
    fn relative_02() {
        let mut prog = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        prog.resize(1010, 0);
        let mut inputs = vec![];
        let mut outputs = vec![];
        let mut state = State::new();
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        assert_eq!(
            outputs,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn relative_03() {
        let mut prog = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut inputs = vec![];
        let mut outputs = vec![];
        let mut state = State::new();
        intcode(&mut prog, &mut inputs, &mut outputs, &mut state);
        assert_eq!(outputs.len(), 1);
        assert_eq!(format!("{}", outputs[0]).len(), 16);
    }

}
