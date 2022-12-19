#![allow(unused)]
use std::fs;

mod cpu;

use cpu::Program;

fn main() {
    let s = fs::read_to_string("data/input.txt").unwrap();
    let program = Program::from_input(&s);

    let sum = program.puzzle_sum().unwrap();
    println!(
        "The sum of the signal strengths at
the 20th, 60th, ..., 220th cycles is: {}",
        sum
    );

    let render = program.render();
    println!("And the rendered image is");
    println!("{}", render);
}
