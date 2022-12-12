use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Assignment {
    start: u64,
    end: u64,
}

struct AssignmentPair(Assignment, Assignment);

impl Assignment {
    fn from_str(string: &str) -> Option<Assignment> {
        let (start, end) = string.split_once('-')?;
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        Some(Assignment { start, end })
    }

    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        !((self.start > other.end) || (other.start > self.end))
    }
}

impl AssignmentPair {
    fn from_file(path: &str) -> Vec<AssignmentPair> {
        let file = File::open(path).expect("Input file should exist");
        let reader = BufReader::new(file);

        let mut pairs: Vec<AssignmentPair> = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Lines should be readable");
            let (a, b) = line.split_once(',').unwrap();
            let pair = AssignmentPair(
                Assignment::from_str(a).unwrap(),
                Assignment::from_str(b).unwrap(),
            );
            pairs.push(pair);

        }
        pairs
    }

    fn has_contained(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn count_contained(pairs: &Vec<AssignmentPair>) -> u64 {
        let mut count = 0;
        for pair in pairs {
            if pair.has_contained() {
                count += 1;
            }
        }
        count
    }

    fn has_overlapped(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }

    fn count_overlapped(pairs: &Vec<AssignmentPair>) -> u64 {
        let mut count = 0;
        for pair in pairs {
            if pair.has_overlapped() {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let pairs = AssignmentPair::from_file("data/input.txt");
    let count_contained = AssignmentPair::count_contained(&pairs);
    println!("Part 1: the number of pairst that have one assignment that contains the other is {}", count_contained);

    let pairs = AssignmentPair::from_file("data/input.txt");
    let count_overlapped = AssignmentPair::count_overlapped(&pairs);
    println!("Part 2: the number of pairst that have one assignment that overlaps the other is {}", count_overlapped);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        assert!(Assignment{start: 0, end: 0}.contains(&Assignment{start: 0, end: 0}));
        assert!(Assignment{start: 3, end: 5}.contains(&Assignment{start: 3, end: 4}));
        assert!(Assignment{start: 5, end: 10}.contains(&Assignment{start: 6, end: 9}));
        assert!(!Assignment{start: 5, end: 6}.contains(&Assignment{start: 4, end: 6}));
        assert!(!Assignment{start: 7, end: 8}.contains(&Assignment{start: 6, end: 8}));
    }

    #[test]
    fn overlaps() {
        assert!(Assignment{start: 0, end: 0}.overlaps(&Assignment{start: 0, end: 0}));
        assert!(Assignment{start: 3, end: 5}.overlaps(&Assignment{start: 3, end: 4}));
        assert!(Assignment{start: 5, end: 10}.overlaps(&Assignment{start: 6, end: 9}));
        assert!(Assignment{start: 5, end: 6}.overlaps(&Assignment{start: 4, end: 6}));
        assert!(Assignment{start: 7, end: 8}.overlaps(&Assignment{start: 6, end: 8}));
    }

    fn test_path() -> &'static str {
        "data/test-input.txt"
    }

    #[test]
    fn part1() {
        let pairs = AssignmentPair::from_file(test_path());
        let count_contained = AssignmentPair::count_contained(&pairs);
        assert_eq!(count_contained, 2);
    }

    #[test]
    fn part2() {
        let pairs = AssignmentPair::from_file(test_path());
        let count_overlapped = AssignmentPair::count_overlapped(&pairs);
        assert_eq!(count_overlapped, 4);
    }
}
