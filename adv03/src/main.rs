use std::env;
use std::fs::File;
use std::io::prelude::*;

use adv03::part1::{get_squares, overlapped_square_feet};
use adv03::part2::non_overlapped_claim;

fn main() -> std::io::Result<()> {
    let input = env::args().collect::<Vec<String>>();
    let mut file = File::open(&input[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let sq = &get_squares(&contents);
    println!("{}", overlapped_square_feet(&sq));
    println!("{}", non_overlapped_claim(&sq));
    Ok(())
}
