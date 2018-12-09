use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut acc = 0;
    // We could use a HashSet but it would incur couple of calls one for
    // insert and another for check
    let mut freq_visited: HashMap<i32, bool> = HashMap::new();
    freq_visited.insert(0, true);
    let mut num_loops = 0;
    loop {
        num_loops += 1;
        for line in contents.lines() {
            let number = line.parse::<i32>().unwrap();
            acc += number;
            if let Some(_) = freq_visited.insert(acc, true) {
                println!("First frequency reached {}", acc);
                println!("Number of loops {}", num_loops);
                return Ok(());
            }
        }
    }
}
