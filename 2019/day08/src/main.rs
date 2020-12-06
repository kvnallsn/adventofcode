use std::{fs::File, io::Read};

fn part1(image: &[u32], width: usize, height: usize) -> usize {
    let layers: Vec<&[u32]> = image.chunks(width * height).collect();

    let mut zero_count = usize::max_value();
    let mut part1 = 0;

    for layer in layers {
        let zeros = layer.iter().filter(|i| **i == 0).count();
        if zeros < zero_count {
            zero_count = zeros;
            let ones = layer.iter().filter(|i| **i == 1).count();
            let twos = layer.iter().filter(|i| **i == 2).count();
            part1 = ones * twos;
        }
    }

    part1
}

fn part2(image: &[u32], width: usize, height: usize) -> Vec<u32> {
    let layers: Vec<&[u32]> = image.chunks(width * height).collect();

    // Initialize output to all transparent
    let mut output: Vec<u32> = vec![2; width * height];
    for layer in layers {
        for (i, pixel) in layer.iter().enumerate() {
            if output[i] == 2 {
                output[i] = *pixel;
            }
        }
    }

    output
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let width = 25;
    let height = 6;

    // Convert to image format
    let image: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    println!("Part 1: {}", part1(&image, width, height));

    part2(&image, width, height).chunks(width).for_each(|line| {
        line.iter().for_each(|pixel| {
            if *pixel == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        });
        println!("");
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_01() {
        let image = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        assert_eq!(part1(&image, 3, 2), 1);
    }

    #[test]
    fn part_02() {
        let image = vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0];
        assert_eq!(part2(&image, 2, 2), vec![0, 1, 1, 0]);
    }
}
