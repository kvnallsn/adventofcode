use std::{fs, io};

fn part01(input: &str, preamble: usize) -> usize {
    let input = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    for i in preamble..input.len() {
        //println!("[{}]: {}", i, input[i]);

        let mut corrupted = true;
        'inner: for j in (i - preamble)..i {
            if input[j] > input[i] {
                continue 'inner;
            }

            let a = input[i] - input[j];
            //println!("\t[{}]: {} - {} = {}", j, input[i], input[j], a);

            for k in (i - (preamble - 1))..i {
                //println!("\t\t[{}]: {}", k, input[k]);
                if a == input[k] {
                    corrupted = false;
                    break 'inner;
                }
            }
        }

        if corrupted {
            return input[i];
        }
    }

    0
}

fn part02(input: &str, target: usize) -> usize {
    let input = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut container = Vec::new();
    for i in 0..input.len() {
        container.clear();
        container.push(input[i]);
        let mut sum = input[i];
        for j in (i + 1)..input.len() {
            container.push(input[j]);
            sum += input[j];
            if sum == target {
                if let (Some(min), Some(max)) = (container.iter().min(), container.iter().max()) {
                    return min + max;
                } else {
                    return 0;
                }
            }
        }
    }

    0
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    // Part 1: 14360655
    let p1 = part01(&input, 25);
    println!("Part 1: {}", p1);

    // Part 2: 1962331
    let p2 = part02(&input, 14360655);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
    }

    #[test]
    fn aoc_1() {
        let answer = part01(input(), 5);
        let expected = 127;
        assert_eq!(answer, expected, "Part 1 mismatch");
    }

    #[test]
    fn aoc_2() {
        let answer = part02(input(), 127);
        let expected = 62;
        assert_eq!(answer, expected, "Part 1 mismatch");
    }
}
