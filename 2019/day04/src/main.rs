fn check_num(x: u32) -> (bool, bool) {
    let digits: Vec<u32> = x
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    let mut adjc = 1;
    let mut p1_adj = false;
    let mut p2_adj = false;
    let mut d = digits[0];
    for n in 1..digits.len() {
        if digits[n] < d {
            return (false, false);
        }

        if digits[n] == d {
            adjc += 1;
            p1_adj = true;
        } else if adjc == 2 {
            p2_adj = true;
        } else {
            adjc = 1;
        }

        d = digits[n];
    }

    if adjc == 2 {
        p2_adj = true;
    }

    (p1_adj, p2_adj)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    println!("{:?}", check_num(688898));
    println!("{:?}", check_num(688899));
    */
    let min = 165432;
    let max = 707912;

    let mut part1 = vec![];
    let mut part2 = vec![];

    for x in min..max {
        let (p1, p2) = check_num(x);
        if p1 {
            part1.push(x);
        }

        if p2 {
            part2.push(x);
        }
    }

    println!("Part 1: {}", part1.len());
    println!("Part 2: {}", part2.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let (p1, p2) = check_num(677787);
        assert!(!p1);
        assert!(!p2);
    }

    #[test]
    fn test_02() {
        let (p1, p2) = check_num(677788);
        assert!(p1);
        assert!(p2);
    }

    #[test]
    fn test_03() {
        let (p1, p2) = check_num(677789);
        assert!(p1);
        assert!(!p2);
    }
}
