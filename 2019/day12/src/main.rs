use num::Integer;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Moon {
    px: i32,
    py: i32,
    pz: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            px: x,
            py: y,
            pz: z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    #[allow(dead_code)]
    pub fn with_velocity(px: i32, py: i32, pz: i32, vx: i32, vy: i32, vz: i32) -> Moon {
        Moon {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        }
    }
}

fn time_step(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in i..moons.len() {
            if moons[i].px < moons[j].px {
                moons[i].vx += 1;
                moons[j].vx -= 1;
            } else if moons[i].px > moons[j].px {
                moons[i].vx -= 1;
                moons[j].vx += 1;
            }

            if moons[i].py < moons[j].py {
                moons[i].vy += 1;
                moons[j].vy -= 1;
            } else if moons[i].py > moons[j].py {
                moons[i].vy -= 1;
                moons[j].vy += 1;
            }

            if moons[i].pz < moons[j].pz {
                moons[i].vz += 1;
                moons[j].vz -= 1;
            } else if moons[i].pz > moons[j].pz {
                moons[i].vz -= 1;
                moons[j].vz += 1;
            }
        }
    }

    for mut moon in moons {
        moon.px += moon.vx;
        moon.py += moon.vy;
        moon.pz += moon.vz;
    }
}

fn part1(mut moons: Vec<Moon>, steps: usize) -> i32 {
    for _ in 0..steps {
        time_step(&mut moons);
    }

    moons
        .iter()
        .map(|moon| {
            (moon.px.abs() + moon.py.abs() + moon.pz.abs())
                * (moon.vx.abs() + moon.vy.abs() + moon.vz.abs())
        })
        .sum()
}

fn part2(mut moons: Vec<Moon>) -> usize {
    let mut i = 1;
    let mut cx = 0;
    let mut cy = 0;
    let mut cz = 0;

    let ix: Vec<(i32, i32)> = moons.iter().map(|m| (m.px, m.vx)).collect();
    let iy: Vec<(i32, i32)> = moons.iter().map(|m| (m.py, m.vy)).collect();
    let iz: Vec<(i32, i32)> = moons.iter().map(|m| (m.pz, m.vz)).collect();
    loop {
        time_step(&mut moons);
        let nx: Vec<(i32, i32)> = moons.iter().map(|m| (m.px, m.vx)).collect();
        let ny: Vec<(i32, i32)> = moons.iter().map(|m| (m.py, m.vy)).collect();
        let nz: Vec<(i32, i32)> = moons.iter().map(|m| (m.pz, m.vz)).collect();

        if ix == nx {
            cx = i;
        }

        if iy == ny {
            cy = i;
        }

        if iz == nz {
            cz = i;
        }

        if (cx != 0) && (cy != 0) && (cz != 0) {
            break;
        }

        i += 1;
    }

    cx.lcm(&cy).lcm(&cz)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file

    let moons = vec![
        Moon::new(-8, -18, 6),
        Moon::new(-11, -14, 4),
        Moon::new(8, -3, -10),
        Moon::new(-2, -16, 1),
    ];

    println!("Total Engery: {}", part1(moons.clone(), 1000));
    println!("Steps: {}", part2(moons));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_01() {
        let mut moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];

        time_step(&mut moons);

        let expected = vec![
            Moon::with_velocity(2, -1, 1, 3, -1, -1),
            Moon::with_velocity(3, -7, -4, 1, 3, 3),
            Moon::with_velocity(1, -7, 5, -3, 1, -3),
            Moon::with_velocity(2, 2, 0, -1, -3, 1),
        ];

        assert_eq!(moons, expected)
    }

    #[test]
    fn part1_02() {
        let mut moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];

        time_step(&mut moons);
        time_step(&mut moons);

        let expected = vec![
            Moon::with_velocity(5, -3, -1, 3, -2, -2),
            Moon::with_velocity(1, -2, 2, -2, 5, 6),
            Moon::with_velocity(1, -4, -1, 0, 3, -6),
            Moon::with_velocity(1, -4, 2, -1, -6, 2),
        ];

        assert_eq!(moons, expected)
    }

    #[test]
    fn part1_03() {
        let mut moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];

        for _ in 0..10 {
            time_step(&mut moons);
        }

        let expected = vec![
            Moon::with_velocity(2, 1, -3, -3, -2, 1),
            Moon::with_velocity(1, -8, 0, -1, 1, 3),
            Moon::with_velocity(3, -6, 1, 3, 2, -3),
            Moon::with_velocity(2, 0, 4, 1, -1, -1),
        ];

        assert_eq!(moons, expected)
    }

    #[test]
    fn part1_04() {
        let moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];

        assert_eq!(part1(moons, 10), 179)
    }
}
