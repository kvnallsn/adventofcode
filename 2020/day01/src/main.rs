use std::{collections::HashSet, fs, io};

fn main() -> io::Result<()> {
    // part 1
    // use a hash set to avoid iterator multiple times over the set
    let input = fs::read_to_string("input.txt")?;
    let set: HashSet<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    for a in set.iter() {
        let b = 2020 - a;
        if set.contains(&b) {
            println!("Part 1: {a} * {b} = {c}", a = a, b = b, c = a * b);
            break;
        }
    }

    // part 2
    // 3 nested loops, looking forward only
    // (probably a better way to do this, but the list is only 200 lines, so we can crunch through
    // it fast)
    let input = fs::read_to_string("input.txt")?;
    let set: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    'outer: for i in 0..(set.len() - 2) {
        for j in (i + 1)..(set.len() - 1) {
            for k in (j + 1)..set.len() {
                if (set[i] + set[j] + set[k]) == 2020 {
                    println!(
                        "Part 2: {i} * {j} * {k} = {a}",
                        i = set[i],
                        j = set[j],
                        k = set[k],
                        a = set[i] * set[j] * set[k]
                    );
                    break 'outer;
                }
            }
        }
    }

    Ok(())
}
