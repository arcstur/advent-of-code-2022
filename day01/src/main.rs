use std::fs::File;
use std::io::{BufRead, BufReader};

mod elfs {
    #[derive(Debug)]
    pub struct Elf {
        calories: u64,
    }

    impl Elf {
        pub fn new(calories: u64) -> Elf {
            Elf { calories }
        }

        pub fn calories(&self) -> u64 {
            self.calories
        }
    }
}

use elfs::Elf;

fn main() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("Input file should exist at input.txt");
    let reader = BufReader::new(file);

    let mut elfs: Vec<Elf> = Vec::new();
    let mut sum_calories: u64 = 0;

    for line in reader.lines() {
        // It is possible to do `if let Ok(line) = line`, but
        // every line in the input file should be readable.
        let line = line.expect("Input file's lines should be readable.");

        if !line.is_empty() {
            let calories: u64 = line
                .parse()
                .expect("Each non-empty line should contain a non-zero integer.");
            sum_calories += calories;
        } else {
            elfs.push(Elf::new(sum_calories));
            sum_calories = 0;
        }
    }

    elfs.sort_by(|elf1, elf2| elf2.calories().cmp(&elf1.calories()));

    let top_qnt = 3;
    let mut sum_calories = 0;

    println!("The top {top_qnt} Elfs carrying the most calories are");

    for elf in &elfs[0..top_qnt] {
        println!("{:?}", elf);
        sum_calories += elf.calories();
    }

    println!("and they're carrying a total of {} calories", sum_calories);
}
