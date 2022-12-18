#![allow(unused)]
use std::fs;
use std::io::Error;

mod rope;

use rope::RopeHistory;

fn main() -> Result<(), Error> {
    let string = fs::read_to_string("data/input.txt")?;
    let history = RopeHistory::from_input(&string);
    println!(
        "The tail visited, at least once, {} positions",
        history.tail_unique_positions()
    );

    Ok(())
}
