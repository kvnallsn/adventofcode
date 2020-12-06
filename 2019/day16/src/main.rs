use std::{fs::File, io::Read};

const DIGITS: [i32; 8] = [10000000, 1000000, 100000, 10000, 1000, 100, 10, 1];

fn build_patterns(len: usize) -> Vec<Vec<i32>> {
    let pattern = [0, 1, 0, -1];

    (0..len)
        .map(|round| {
            let count = round + 1;
            let mut idx = 0;
            let mut offset = 0;
            let mut rpattern = vec![];

            while rpattern.len() != len {
                if idx != 0 {
                    rpattern.push(pattern[offset % 4]);
                }
                if (idx + 1) % count == 0 {
                    offset += 1
                };
                idx += 1;
            }
            rpattern
        })
        .collect()
}

fn fft(input: &[i32], patterns: &[Vec<i32>]) -> Vec<i32> {
    let output: Vec<i32> = patterns
        .iter()
        .map(|p| {
            p.iter()
                .zip(input.iter())
                .map(|(&a, &b)| a * b)
                .sum::<i32>()
                .abs()
                % 10
        })
        .collect();

    output
}

fn part1(mut input: Vec<i32>) -> i32 {
    let patterns = build_patterns(input.len());
    for _ in 0..100 {
        input = fft(&input, &patterns);
    }

    input
        .into_iter()
        .take(8)
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (DIGITS[i] * x))
}

/// we can assume the first half will be zero'd out,
/// no need to compute everythin
fn part2(input: Vec<i32>) -> i32 {
    let mut signal = input.clone();
    for _ in 1..10000 {
        signal.extend_from_slice(&input);
    }

    let offset = input
        .into_iter()
        .take(7)
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (DIGITS[i + 1] * x)) as usize;

    let mut signal: Vec<i32> = signal.into_iter().skip(offset).collect();

    for _ in 0..100 {
        for i in (0..(signal.len() - 1)).rev() {
            signal[i] = (signal[i] + signal[i + 1]).abs() % 10;
        }
    }

    signal
        .into_iter()
        .take(8)
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (DIGITS[i] * x))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let input: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    println!("Part 1: {:?}", part1(input.clone()));
    println!("Part 2: {:?}", part2(input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_01() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8];
        let patterns = build_patterns(input.len());
        let out_1 = fft(&input, &patterns);
        assert_eq!(out_1, vec![4, 8, 2, 2, 6, 1, 5, 8], "failed fft round 1");
        let out_2 = fft(&out_1, &patterns);
        assert_eq!(out_2, vec![3, 4, 0, 4, 0, 4, 3, 8], "failed fft round 2");
        let out_3 = fft(&out_2, &patterns);
        assert_eq!(out_3, vec![0, 3, 4, 1, 5, 5, 1, 8], "failed fft round 3");
        let out_4 = fft(&out_3, &patterns);
        assert_eq!(out_4, vec![0, 1, 0, 2, 9, 4, 9, 8], "failed fft round 4");
    }

    #[test]
    fn part1_02() {
        let mut input = vec![
            8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4, 5,
            5, 9, 5,
        ];
        let patterns = build_patterns(input.len());

        for _ in 0..100 {
            input = fft(&input, &patterns);
        }
        let out: Vec<i32> = input.into_iter().take(8).collect();
        assert_eq!(out, vec![2, 4, 1, 7, 6, 1, 7, 6]);
    }

    #[test]
    fn part1_03() {
        let mut input = vec![
            1, 9, 6, 1, 7, 8, 0, 4, 2, 0, 7, 2, 0, 2, 2, 0, 9, 1, 4, 4, 9, 1, 6, 0, 4, 4, 1, 8, 9,
            9, 1, 7,
        ];
        let patterns = build_patterns(input.len());

        for _ in 0..100 {
            input = fft(&input, &patterns);
        }
        let out: Vec<i32> = input.into_iter().take(8).collect();
        assert_eq!(out, vec![7, 3, 7, 4, 5, 4, 1, 8]);
    }

    #[test]
    fn part1_04() {
        let mut input = vec![
            6, 9, 3, 1, 7, 1, 6, 3, 4, 9, 2, 9, 4, 8, 6, 0, 6, 3, 3, 5, 9, 9, 5, 9, 2, 4, 3, 1, 9,
            8, 7, 3,
        ];
        let patterns = build_patterns(input.len());

        for _ in 0..100 {
            input = fft(&input, &patterns);
        }
        let out: Vec<i32> = input.into_iter().take(8).collect();
        assert_eq!(out, vec![5, 2, 4, 3, 2, 1, 3, 3]);
    }

    #[test]
    fn part2_01() {
        let input = vec![
            0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9, 1, 5, 6, 5, 4, 7, 4,
            6, 6, 4,
        ];

        let answer = part2(input);
        assert_eq!(answer, 84462026);
    }

    #[test]
    fn part2_02() {
        let input = vec![
            0, 2, 9, 3, 5, 1, 0, 9, 6, 9, 9, 9, 4, 0, 8, 0, 7, 4, 0, 7, 5, 8, 5, 4, 4, 7, 0, 3, 4,
            3, 2, 3,
        ];

        let answer = part2(input);
        assert_eq!(answer, 78725270);
    }

    #[test]
    fn part2_03() {
        let input = vec![
            0, 3, 0, 8, 1, 7, 7, 0, 8, 8, 4, 9, 2, 1, 9, 5, 9, 7, 3, 1, 1, 6, 5, 4, 4, 6, 8, 5, 0,
            5, 1, 7,
        ];

        let answer = part2(input);
        assert_eq!(answer, 53553731);
    }
}
