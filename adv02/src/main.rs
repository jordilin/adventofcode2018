use std::env;
use std::fs::File;
use std::io::prelude::*;

use adv02::part_one::count_frequency;
use adv02::part_two::correct_box_id;

fn main() -> std::io::Result<()> {
    let input = env::args().collect::<Vec<String>>();
    let mut file = File::open(&input[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", count_frequency(&contents));
    println!("{}", correct_box_id(&contents));
    Ok(())
}
