#![allow(unused)]
use std::fs;

mod tree;

use std::io::Error;
use tree::TreeGrid;

fn main() -> Result<(), Error> {
    let string = fs::read_to_string("data/input.txt")?;
    let grid = TreeGrid::from_string(&string);
    println!(
        "The numbe of visible trees is: {}",
        grid.visible_trees().unwrap()
    );

    Ok(())
}
