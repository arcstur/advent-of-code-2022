use std::fs::File;
use std::io::{BufRead, BufReader};

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
            Some(self.cmp(other))
        }
    }

    impl Ord for Play {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Ordering::Less,
                (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Ordering::Equal,
                (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Ordering::Greater,
            }
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

        pub fn from_outcome(outcome: Ordering, opponent: &Play) -> Play {
            match (outcome, opponent) {
                (Ordering::Less, Paper)
                | (Ordering::Equal, Rock)
                | (Ordering::Greater, Scissors) => Rock,
                (Ordering::Less, Scissors)
                | (Ordering::Equal, Paper)
                | (Ordering::Greater, Rock) => Paper,
                (Ordering::Less, Rock)
                | (Ordering::Equal, Scissors)
                | (Ordering::Greater, Paper) => Scissors,
            }
        }

        pub fn from_outcome_char(letter: char, opponent: &Play) -> Option<Play> {
            match letter {
                'X' => Some(Play::from_outcome(Ordering::Less, opponent)),
                'Y' => Some(Play::from_outcome(Ordering::Equal, opponent)),
                'Z' => Some(Play::from_outcome(Ordering::Greater, opponent)),
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

        pub fn total_points(&self, opponent: &Self) -> u64 {
            let round_points = match self.cmp(opponent) {
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

    const EXPECT_CHAR: &str = "Columns should have one character";
    const EXPECT_VALID_CHAR: &str = "Columns should have valid characters";

    for line in reader.lines() {
        let line = line.expect("Input file's lines should be readable.");

        if !line.is_empty() {
            let chars: Vec<&str> = line.split_whitespace().collect();

            let opponent =
                Play::from_opponent(chars[0].parse().expect(EXPECT_CHAR)).expect(EXPECT_VALID_CHAR);
            let player = Play::from_outcome_char(chars[1].parse().expect(EXPECT_CHAR), &opponent)
                .expect(EXPECT_VALID_CHAR);

            points += player.total_points(&opponent);
        }
    }

    println!("Your total number of points is {}", points);
}
