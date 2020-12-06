use common::intcode::Intcode;
use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    sync::{mpsc::channel, Arc, RwLock},
    thread,
};

fn run(input: &str, n: usize) {
    // create shared memory for all input queues
    let inputs = Arc::new(RwLock::new(vec![Vec::<i64>::new(); n]));
    let nat = Arc::new(RwLock::new(None));
    let idle = Arc::new(RwLock::new(HashSet::new()));
    let (tx, rx) = channel();

    // spawn the NAT
    {
        let (inputs, nat, idle, _tx) = (
            Arc::clone(&inputs),
            Arc::clone(&nat),
            Arc::clone(&idle),
            tx.clone(),
        );
        thread::spawn(move || {
            let mut seen = HashSet::new();
            // thread code
            loop {
                if let Ok(ref idlex) = idle.read() {
                    if idlex.len() == 50 {
                        // all threads idle
                        if let Ok(ref nat) = nat.read() {
                            if let Some((x, y)) = **nat {
                                println!("[NAT]: X = {}, Y = {}", x, y);
                                if !seen.insert(y) {
                                    println!("double! {}", y);
                                }
                                if let Ok(ref mut inputs) = inputs.write() {
                                    inputs[0].push(x);
                                    inputs[0].push(y);
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    for i in 0..n {
        let mut intcode = Intcode::init(input);
        let (inputs, nat, idle, tx) = (
            Arc::clone(&inputs),
            Arc::clone(&nat),
            Arc::clone(&idle),
            tx.clone(),
        );
        thread::spawn(move || {
            // thread code
            intcode.push(i as i64);

            while !intcode.is_halted() {
                intcode.run();

                if intcode.is_blocked() {
                    let mut has_data = false;
                    if let Ok(ref inputs) = inputs.read() {
                        has_data = !inputs[i].is_empty();
                    }

                    if has_data {
                        let data = match inputs.write() {
                            Ok(ref mut inputs) => inputs[i].drain(0..).collect(),
                            Err(_) => vec![],
                        };

                        if let Ok(ref mut idle) = idle.write() {
                            intcode.append(&data);
                            idle.remove(&i);
                        }
                    } else {
                        if let Ok(ref mut idle) = idle.write() {
                            intcode.push(-1);
                            idle.insert(i);
                        }
                    }
                }

                if intcode.has_output() {
                    let output = intcode.output();
                    for chunk in output.chunks_exact(3) {
                        let idx: usize = chunk[0] as usize;
                        if idx < 50 {
                            if let Ok(ref mut inputs) = inputs.write() {
                                inputs[idx].push(chunk[1]);
                                inputs[idx].push(chunk[2]);
                            }
                        } else if idx == 255 {
                            if let Ok(ref mut natx) = nat.write() {
                                **natx = Some((chunk[1], chunk[2]));
                            }
                        }
                    }
                }
            }

            tx.send(()).unwrap();
        });
    }

    rx.recv().unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    run(&input, 50);

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
