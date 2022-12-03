#![allow(unused)]
use std::fs::File;
use std::io::{BufRead, BufReader};

mod rucksack {

    static LETTERS: [char; 52] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    #[derive(Debug)]
    pub struct Rucksack {
        pub left: Vec<char>,
        pub right: Vec<char>,
    }

    impl Rucksack {
        pub fn from(left: Vec<char>, right: Vec<char>) -> Rucksack {
            Rucksack { left, right }
        }

        fn shared_chars(&self) -> Option<Vec<char>> {
            let mut shared: Vec<char> = Vec::new();

            for character in &self.left {
                if self.right.contains(character) && !shared.contains(character) {
                    shared.push(character.to_owned());
                }
            }
            if !shared.is_empty() {
                Some(shared)
            } else {
                None
            }
        }

        fn char_priority(letter: &char) -> Option<u64> {
            if let Some(index) = LETTERS.iter().position(|&x| &x == letter) {
                return Some(index as u64 + 1);
            }
            None
        }

        pub fn priority(&self) -> Option<u64> {
            let mut priority = 0;
            // The ? operator will early return None if shared_chars is None
            for character in self.shared_chars()? {
                if let Some(char_priority) = Rucksack::char_priority(&character) {
                    priority += char_priority;
                }
            }
            Some(priority)
        }
    }
}

use rucksack::Rucksack;

fn main() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("Input file should exist at input.txt");
    let reader = BufReader::new(file);

    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Lines should be readable");

        let mut left: Vec<char> = line.chars().collect();
        if left.len() % 2 != 0 {
            panic!("Every line should contain an even number of characters");
        }
        let right = left.split_off(left.len() / 2);

        rucksacks.push(Rucksack::from(left, right));
    }

    let mut priorities = 0;
    for rucksack in rucksacks {
        priorities += rucksack
            .priority()
            .expect("Every rucksack in the input file should have a priority");
    }

    println!("The sum of the rucksacks' priorities is {}", priorities);
}
