use common::{intcode::Intcode, util::print_grid};
use std::{collections::HashMap, fs::File, io::Read};

fn part1(input: &str, width: i64, height: i64) {
    let coords = (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .collect::<Vec<(i64, i64)>>();

    let mut intcode = Intcode::init(input);
    let mut grid: HashMap<(i64, i64), i64> = HashMap::new();
    coords.iter().for_each(|&(x, y)| {
        intcode.reset();
        intcode.push(x);
        intcode.push(y);
        intcode.run();
        grid.insert((x, y), intcode.pop(0));
    });

    let mut chars = HashMap::new();
    chars.insert(0, '.');
    chars.insert(1, '#');
    chars.insert(2, '?');

    print_grid(&grid, &chars, 2);
    println!("Affected: {}", grid.values().filter(|&&v| v == 1).count());
}

fn part2(input: &str, n: i64) {
    let mut intcode = Intcode::init(input);
    let mut sz = 0;
    let n = n - 1;
    'rows: for y in n..2000 {
        'cols: for x in sz..2000 {
            intcode.reset();
            intcode.push(x);
            intcode.push(y);
            intcode.run();
            let z = intcode.pop(0);

            if z == 0 {
                continue 'cols;
            }

            sz = x;
            intcode.reset();
            intcode.push(x + n);
            intcode.push(y - n);
            intcode.run();
            let z = intcode.pop(0);

            if z == 0 {
                break 'cols;
            }

            println!("({}, {}): {}", x, y - n, (x * 10000) + (y - n));
            break 'rows;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    part1(&input, 50, 50);
    part2(&input, 100);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        //
    }
}
