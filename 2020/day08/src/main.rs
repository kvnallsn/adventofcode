use std::{collections::HashSet, fs, io, str::FromStr};

#[derive(Debug)]
enum Instr {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let instr = split.next().ok_or(())?;
        let val = split.next().ok_or(())?.parse::<isize>().map_err(|_| ())?;

        match instr {
            "acc" => Ok(Instr::Acc(val)),
            "jmp" => Ok(Instr::Jmp(val)),
            "nop" => Ok(Instr::Nop(val)),
            _ => Err(()),
        }
    }
}

fn part01(input: &str) -> isize {
    let instrs = input
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Instr>>();

    let (mut acc, mut rip) = (0, 0);
    let mut seen = HashSet::new();
    loop {
        //println!("{}: {:?} -> {}", rip, instrs[rip as usize], acc);
        if !seen.insert(rip) || (rip as usize) >= instrs.len() {
            return acc;
        }

        let incr = match instrs[rip as usize] {
            Instr::Acc(v) => {
                acc += v;
                1
            }
            Instr::Jmp(v) => v,
            Instr::Nop(_) => 1,
        };

        rip += incr;
    }
}

fn part02(input: &str) -> isize {
    let instrs = input
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Instr>>();

    let mut mutated = HashSet::new();

    loop {
        let (mut acc, mut rip) = (0, 0);
        let mut seen = HashSet::new();
        let mut has_mutated = false;

        'inner: loop {
            if !seen.insert(rip) {
                break 'inner;
            }

            if (rip as usize) >= instrs.len() {
                return acc;
            }

            //println!("{}: {:?} -> {}", rip, instrs[rip as usize], acc);
            let incr = match instrs[rip as usize] {
                Instr::Acc(v) => {
                    acc += v;
                    1
                }
                Instr::Jmp(v) => match (has_mutated, mutated.contains(&rip)) {
                    (true, _) => v,
                    (false, true) => v,
                    (false, false) => {
                        mutated.insert(rip);
                        has_mutated = true;
                        1
                    }
                },

                Instr::Nop(v) => match (has_mutated, mutated.contains(&rip)) {
                    (true, _) => 1,
                    (false, true) => 1,
                    (false, false) => {
                        mutated.insert(rip);
                        has_mutated = true;
                        v
                    }
                },
            };

            rip += incr;
        }
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    // Part 1: 1941
    println!("Part 1: {}", part01(&input));

    // Part 2:
    println!("Part 2: {}", part02(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
    }

    #[test]
    fn aoc_1() {
        let output = part01(input());
        let expected = 5;
        assert_eq!(
            output, expected,
            "acc ({}) did not match expected value ({})",
            output, expected
        );
    }

    #[test]
    fn aoc_2() {
        let output = part02(input());
        let expected = 8;
        assert_eq!(
            output, expected,
            "acc ({}) did not match expected value ({})",
            output, expected
        );
    }
}
