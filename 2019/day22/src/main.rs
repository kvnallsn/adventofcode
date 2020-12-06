use num::{cast::ToPrimitive, BigInt};

use regex::{Regex, RegexSet};
use std::{fs::File, io::Read, slice::Iter};

const GOAL_RE: &str = r"Result:\s(?P<numbers>(\d\s?)+)";
const INCR_RE: &str = r"deal\swith\sincrement\s(?P<number>\d+)";
const DEAL_RE: &str = r"deal\sinto\snew\sstack";
const CUT_RE: &str = r"cut\s(?P<number>-?\d+)";

#[derive(Clone, Debug)]
enum Command {
    Deal,
    Cut(i64),
    Increment(usize),
    Goal(Vec<u64>),
}

#[derive(Debug, PartialEq)]
struct Deck(Vec<u64>);

impl Deck {
    pub fn new(size: u64) -> Deck {
        Deck((0..size).collect::<Vec<u64>>())
    }

    pub fn cut(&mut self, n: i64) {
        let rev = n < 0;
        let n = n.abs() as usize;

        if rev {
            let len = self.0.len();
            let mut v: Vec<u64> = self.0.iter().take(len - n).map(|u| *u).collect();
            let mut w: Vec<u64> = self.0.iter().skip(len - n).map(|u| *u).collect();
            w.append(&mut v);
            self.0 = w;
        } else {
            let mut v: Vec<u64> = self.0.iter().take(n).map(|u| *u).collect();
            let mut w: Vec<u64> = self.0.iter().skip(n).map(|u| *u).collect();
            w.append(&mut v);
            self.0 = w;
        }
    }

    pub fn deal(&mut self) {
        let v = self.0.iter().map(|u| *u).rev().collect();
        self.0 = v;
    }

    pub fn increment(&mut self, n: usize) {
        let l = self.0.len();
        let mut i = 0;
        let mut v = vec![0; l];
        for d in &self.0 {
            v[i % l] = *d;
            i += n;
        }
        self.0 = v;
    }

    pub fn find(&self, n: u64) -> usize {
        self.0.binary_search(&n).expect("failed to find idx")
    }

    pub fn iter(&self) -> Iter<u64> {
        self.0.iter()
    }
}

fn read_input(input: &str) -> Vec<Command> {
    // setup up regexs
    let set =
        RegexSet::new(&[GOAL_RE, INCR_RE, DEAL_RE, CUT_RE]).expect("failed to build regex set");

    let goal_re = Regex::new(GOAL_RE).expect("failed to compile goal regex");
    let incr_re = Regex::new(INCR_RE).expect("failed to compile increment regex");
    let cut_re = Regex::new(CUT_RE).expect("failed to compile cut regex");

    let instrs: Vec<Command> = input
        .lines()
        .into_iter()
        .map(|line| {
            let matches = set.matches(line).into_iter().collect::<Vec<usize>>();
            if matches.len() != 1 {
                panic!(
                    "failed to match regex (too many or no match): {}",
                    matches.len()
                );
            }

            match matches[0] {
                0 => {
                    let nums: Vec<u64> = goal_re
                        .captures(line)
                        .expect("failed to get deal captures")
                        .name("numbers")
                        .expect("failed to get deal numbers capture")
                        .as_str()
                        .split(" ")
                        .map(|i| i.parse().expect("failed to parse number"))
                        .collect();
                    Command::Goal(nums)
                }
                1 => {
                    let num = incr_re
                        .captures(line)
                        .expect("failed to get increment captures")
                        .name("number")
                        .expect("failed to get increment number capture")
                        .as_str()
                        .parse()
                        .expect("failed to parse increment number");
                    Command::Increment(num)
                }
                2 => Command::Deal,
                3 => {
                    let num = cut_re
                        .captures(line)
                        .expect("failed to get cut captures")
                        .name("number")
                        .expect("failed to get cut number capture")
                        .as_str()
                        .parse()
                        .expect("failed to parse cut number");
                    Command::Cut(num)
                }
                _ => panic!("Unknown Command"),
            }
        })
        .collect();

    instrs
}

fn shuffle(input: &str, n: u64) -> Deck {
    let instrs = read_input(input);
    let mut deck = Deck::new(n);

    for instr in instrs {
        match instr {
            Command::Cut(n) => deck.cut(n),
            Command::Increment(n) => deck.increment(n),
            Command::Deal => deck.deal(),
            Command::Goal(g) => assert_eq!(Deck(g), deck),
        }
    }

    deck
}

fn part2(input: &str, cards: i64, iterations: u64) {
    let instrs = read_input(input);

    let mut incr_mul: BigInt = 1.into();
    let mut offset_diff: BigInt = 0.into();
    let cards: BigInt = cards.into();
    let exp = &cards - 2;

    for instr in instrs {
        match instr {
            Command::Cut(n) => {
                offset_diff += n * &incr_mul;
                offset_diff %= &cards;
            }
            Command::Increment(n) => {
                let bn: BigInt = n.into();
                incr_mul *= bn.modpow(&exp, &cards);
                incr_mul %= &cards;
            }
            Command::Deal => {
                incr_mul *= -1;
                incr_mul %= &cards;
                offset_diff += &incr_mul;
                offset_diff %= &cards;
            }
            Command::Goal(_) => (),
        }
    }

    let mut increment = incr_mul.modpow(&iterations.into(), &cards);
    let base: BigInt = (1 - &incr_mul) % &cards;
    let offset = &offset_diff * (1 - &increment) * base.modpow(&exp, &cards);
    let offset = offset % &cards;

    let ans: BigInt = (&offset + 2020 * &increment) % &cards;
    println!("Part 2: {}", ans);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    // Part 1
    let deck = shuffle(&input, 10007);
    for (i, card) in deck.iter().enumerate() {
        if card == &2019 {
            println!("Part 1: {}", i);
            break;
        }
    }

    // Part 2
    part2(&input, 119315717514047, 101741582076661);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_generate() {
        let deck = Deck::new(10);
        assert_eq!(deck, Deck(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }

    #[test]
    fn deck_deal() {
        let mut deck = Deck::new(10);
        deck.deal();
        assert_eq!(deck, Deck(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]));
    }

    #[test]
    fn deck_cut_01() {
        let mut deck = Deck::new(10);
        deck.cut(3);
        assert_eq!(deck, Deck(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]));
    }

    #[test]
    fn deck_cut_02() {
        let mut deck = Deck::new(10);
        deck.cut(-4);
        assert_eq!(deck, Deck(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]));
    }

    #[test]
    fn deck_increment() {
        let mut deck = Deck::new(10);
        deck.increment(3);
        assert_eq!(deck, Deck(vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]));
    }

    #[test]
    fn part1_01() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("test_input_1.txt")?;
        let mut input = String::new();
        f.read_to_string(&mut input)?;

        shuffle(&input, 10);

        Ok(())
    }

    #[test]
    fn part1_02() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("test_input_2.txt")?;
        let mut input = String::new();
        f.read_to_string(&mut input)?;

        shuffle(&input, 10);

        Ok(())
    }

    #[test]
    fn part1_03() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("test_input_3.txt")?;
        let mut input = String::new();
        f.read_to_string(&mut input)?;

        shuffle(&input, 10);

        Ok(())
    }

    #[test]
    fn part1_04() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("test_input_4.txt")?;
        let mut input = String::new();
        f.read_to_string(&mut input)?;

        shuffle(&input, 10);

        Ok(())
    }
}
