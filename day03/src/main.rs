mod rsack {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    static LETTERS: [char; 52] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    #[derive(Debug, Clone)]
    pub struct Rucksack {
        items: Vec<char>,
    }

    impl Rucksack {
        pub fn from(items: Vec<char>) -> Rucksack {
            Rucksack { items }
        }

        pub fn left(&self) -> &[char] {
            let middle = self.items.len() / 2;
            &self.items[0..middle]
        }

        pub fn right(&self) -> &[char] {
            let middle = self.items.len() / 2;
            &self.items[middle..]
        }

        fn shared_chars(&self) -> Option<Vec<char>> {
            let mut shared: Vec<char> = Vec::new();

            for character in self.left() {
                if self.right().contains(character) && !shared.contains(character) {
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

        pub fn from_file(path: &str) -> Vec<Rucksack> {
            let file = File::open(path).expect("Input file should exist");
            let reader = BufReader::new(file);

            let mut rucksacks: Vec<Rucksack> = Vec::new();

            for line in reader.lines() {
                let line = line.expect("Lines should be readable");
                rucksacks.push(Rucksack::from(line.chars().collect()));
            }

            rucksacks
        }

        pub fn groups_from_file(path: &str, groups_of: usize) -> Vec<Vec<Rucksack>> {
            let file = File::open(path).expect("Input file should exist");
            let reader = BufReader::new(file);

            let mut rucksack_groups: Vec<Vec<Rucksack>> = Vec::new();
            let mut rucksacks: Vec<Rucksack> = Vec::new();

            for (i, line) in reader.lines().enumerate() {
                let line = line.expect("Lines should be readable");

                rucksacks.push(Rucksack::from(line.chars().collect()));

                if i % groups_of == (groups_of - 1) {
                    rucksack_groups.push(rucksacks.clone());
                    rucksacks.clear();
                }
            }

            rucksack_groups

        }

        fn group_shared_chars(group: &Vec<Rucksack>) -> Vec<char> {
            let mut shared: Vec<char> = Vec::new();
            for chr in &group[0].items {
                let mut is_shared = group[1].items.contains(chr);

                for rucksack in group {
                    is_shared &= rucksack.items.contains(chr)
                }

                if is_shared && !shared.contains(chr) {
                    shared.push(chr.to_owned());
                }
            }
            shared
        }

        pub fn group_priority(group: &Vec<Rucksack>) -> Option<u64> {
            let shared = Rucksack::group_shared_chars(&group);
            let mut priority = 0;

            for chr in &shared {
                // Will propagate None if it appears
                priority += Rucksack::char_priority(chr)?;
            }
            Some(priority)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn char_priorities() {
            assert_eq!(
                Rucksack::char_priority(&'a').expect("'a' char has a priority"),
                1
            );
            assert_eq!(
                Rucksack::char_priority(&'p').expect("'p' char has a priority"),
                16
            );
            assert_eq!(
                Rucksack::char_priority(&'L').expect("'L' char has a priority"),
                38
            );
            assert_eq!(
                Rucksack::char_priority(&'P').expect("'P' char has a priority"),
                42
            );
            assert_eq!(
                Rucksack::char_priority(&'t').expect("'t' char has a priority"),
                20
            );
            assert_eq!(
                Rucksack::char_priority(&'v').expect("'v' char has a priority"),
                22
            );
            assert_eq!(
                Rucksack::char_priority(&'s').expect("'s' char has a priority"),
                19
            );
        }

        #[test]
        #[should_panic]
        fn char_without_priority() {
            Rucksack::char_priority(&'*').unwrap();
        }

        #[test]
        fn group_shared_chars() {
            let group = vec![
                Rucksack::from(vec!['a', 'b', 'c', 'd']),
                Rucksack::from(vec!['b', 'c', 'd']),
                Rucksack::from(vec!['a', 'b']),
                Rucksack::from(vec!['b', 'e', 'f', 'g']),
            ];

            assert_eq!(Rucksack::group_shared_chars(&group), vec!['b']);

        }
    }
}

use rsack::Rucksack;

fn main() {
    let rucksacks = Rucksack::from_file("input.txt");

    let mut priorities = 0;
    for rucksack in &rucksacks {
        priorities += rucksack
            .priority()
            .expect("Every rucksack in the input file should have a priority");
    }

    println!(
        "Part 1: The sum of the rucksacks' priorities is {}",
        priorities
    );


    let rucksack_groups = Rucksack::groups_from_file("input.txt", 3);
    let mut priorities = 0;
    for group in &rucksack_groups {
        priorities += Rucksack::group_priority(group)
            .expect("Every rucksack group in the input file should have a priority");
    }

    println!(
        "Part 2: The sum of the rucksack groups' priorities is {}",
        priorities
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_path() -> &'static str {
        "test-input.txt"
    }

    #[test]
    fn part1() {
        let rucksacks = Rucksack::from_file(test_path());
        let mut priorities = 0;
        for rucksack in &rucksacks {
            priorities += rucksack
                .priority()
                .expect("Every rucksack in the input file should have a priority");
        }

        assert_eq!(priorities, 157);
    }

    #[test]
    fn part2() {
        let rucksack_groups = Rucksack::groups_from_file(test_path(), 3);
        let mut priorities = 0;
        for group in &rucksack_groups {
            println!("{:?}", group);
            priorities += Rucksack::group_priority(group)
                .expect("Every rucksack group in the input file should have a priority");
        }

        assert_eq!(priorities, 70);
    }
}
