#![allow(unused)]
use std::fs::File;
use std::io::{BufRead, BufReader};

const WIN_SCORE: u32 = 6;
const DRAW_SCORE: u32 = 3;
const LOOSE_SCORE: u32 = 0;

const ROCK_SCORE: u8 = 1;
const PAPER_SCORE: u8 = 2;
const SCISSORS_SCORE: u8 = 3;

mod rock_paper_scissors {
    use std::cmp::Ordering;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Play {
        Rock,
        Paper,
        Scissors,
    }

    use Play::{Paper, Rock, Scissors};

    impl PartialOrd for Play {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self {
                Rock => match other {
                    Rock => Some(Ordering::Equal),
                    Paper => Some(Ordering::Less),
                    Scissors => Some(Ordering::Greater),
                },
                Paper => match other {
                    Rock => Some(Ordering::Greater),
                    Paper => Some(Ordering::Equal),
                    Scissors => Some(Ordering::Less),
                },
                Scissors => match other {
                    Rock => Some(Ordering::Less),
                    Paper => Some(Ordering::Greater),
                    Scissors => Some(Ordering::Equal),
                },
            }
        }
    }

    impl Ord for Play {
        fn cmp(&self, other: &Self) -> Ordering {
            // Dont't know how to make this better
            // To just use Ord, it is necessary to also derive PartialOrd...
            self.partial_cmp(other)
                .expect("All variants are comparable")
        }
    }

    impl Play {
        pub fn from_opponent(letter: char) -> Option<Play> {
            match letter {
                'A' => Some(Play::Rock),
                'B' => Some(Play::Paper),
                'C' => Some(Play::Scissors),
                _ => None,
            }
        }

        pub fn from_player(letter: char) -> Option<Play> {
            match letter {
                'X' => Some(Play::Rock),
                'Y' => Some(Play::Paper),
                'Z' => Some(Play::Scissors),
                _ => None,
            }
        }

        fn points_from_variant(&self) -> u64 {
            match self {
                Rock => 1,
                Paper => 2,
                Scissors => 3,
            }
        }

        pub fn total_points_from_play(&self, opponent_play: &Self) -> u64 {
            let round_points = match self.cmp(opponent_play) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
            };

            round_points + self.points_from_variant()
        }
    }
}

use rock_paper_scissors::Play;

fn main() {
    println!("Hello, world!");

    let input_path = "input.txt";
    let file = File::open(input_path).expect("Input file should exist at input.txt");
    let reader = BufReader::new(file);

    let mut points: u64 = 0;

    for line in reader.lines() {
        let line = line.expect("Input file's lines should be readable.");

        if !line.is_empty() {
            let mut chars = line.chars();

            let opponent_play =
                Play::from_opponent(chars.next().expect("Line should have a first character"))
                    .expect("Line should have a valid character");

            let _ = chars.next();

            let play = Play::from_player(chars.next().expect("Line should have a third character"))
                .expect("Line should have a valid character");

            points += play.total_points_from_play(&opponent_play);
        }
    }

    println!("Your total number of points is {}", points);
}
