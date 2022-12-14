use regex::Regex;
use std::fs;
// use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
struct Crate(char);

#[derive(Debug)]
struct Stack {
    crates: Vec<Crate>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { crates: Vec::new() }
    }

    pub fn on_top(&self) -> Option<&Crate> {
        self.crates.last()
    }

    pub fn push(&mut self, crt: Crate) {
        self.crates.push(crt);
    }

    pub fn pop(&mut self) -> Option<Crate> {
        self.crates.pop()
    }

    pub fn insert(&mut self, index: usize, crt: Crate) {
        self.crates.insert(index, crt)
    }
}

#[derive(Debug)]
struct NoMoreCrates;

#[derive(Debug)]
struct Ship {
    stacks: Vec<Stack>,
    move_many_at_once: bool,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            stacks: Vec::new(),
            move_many_at_once: false,
        }
    }

    fn from_str(string: &str) -> Ship {
        let re = Regex::new(r"\[(\w)\]").expect("Hardcoded Regex should compile");

        let mut ship = Ship::new();

        for line in string.lines() {
            for caps in re.captures_iter(line) {
                for cap in caps.iter().skip(1) {
                    let idx = (cap.unwrap().end() - 2) / 4;
                    let stack = match ship.stacks.get_mut(idx) {
                        Some(s) => s,
                        None => {
                            // Create stacks from their start up to requested index
                            for _ in ship.stacks.len()..(idx + 1) {
                                ship.stacks.push(Stack::new());
                            }
                            ship.stacks.get_mut(idx).unwrap()
                        }
                    };
                    stack.insert(0, Crate(cap.unwrap().as_str().parse().unwrap()));
                }
            }
        }

        ship
    }

    fn crate_mover_9001_from_str(string: &str) -> Ship {
        let mut ship = Ship::from_str(string);
        ship.move_many_at_once = true;
        ship
    }

    #[cfg(test)]
    fn on_top(&self, idx: usize) -> Option<&Crate> {
        let stack = self.stacks.get(idx)?;
        stack.on_top()
    }

    fn crates_on_top_as_string(&self) -> String {
        let mut s = String::new();
        for stack in &self.stacks {
            if let Some(crt) = stack.on_top() {
                s.push(crt.0);
            }
        }
        s
    }

    fn move_crate(&mut self, from: usize, to: usize) -> Result<(), NoMoreCrates> {
        let crt = self.stacks[from].pop().ok_or(NoMoreCrates)?;
        self.stacks[to].push(crt);
        Ok(())
    }

    fn move_crates(&mut self, repeat: u8, from: usize, to: usize) -> Result<(), NoMoreCrates> {
        if !self.move_many_at_once {
            for _ in 0..repeat {
                self.move_crate(from, to)?;
            }
        } else {
            // Create temporary stack
            self.stacks.push(Stack::new());
            let last_index = self.stacks.len() - 1;

            for _ in 0..repeat {
                self.move_crate(from, last_index)?;
            }
            for _ in 0..repeat {
                self.move_crate(last_index, to)?;
            }

            self.stacks.pop();
        }
        Ok(())
    }

    fn move_crates_from_commands(&mut self, commands: &str) -> Result<(), NoMoreCrates> {
        let re =
            Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Hardcoded Regex should compile");

        let parse_expect = "Parsing works because regex captured digit";
        for caps in re.captures_iter(commands) {
            let repeat = caps.get(1).unwrap().as_str().parse().expect(parse_expect);
            let from = caps
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect(parse_expect)
                - 1;
            let to = caps
                .get(3)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect(parse_expect)
                - 1;
            self.move_crates(repeat, from, to)?;
        }

        Ok(())
    }
}

fn main() {
    let string = fs::read_to_string("data/input.txt").unwrap();

    let mut ship = Ship::from_str(&string);
    ship.move_crates_from_commands(&string).unwrap();

    println!("The crates on top are {}", ship.crates_on_top_as_string());

    let mut ship = Ship::crate_mover_9001_from_str(&string);
    ship.move_crates_from_commands(&string).unwrap();

    println!(
        "(Crate Mover 9001) The crates on top are {}",
        ship.crates_on_top_as_string()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_basic_ship() -> Ship {
        let mut ship = Ship::new();
        let stack1 = Stack {
            crates: vec![Crate('a'), Crate('b'), Crate('c')],
        };
        let stack2 = Stack {
            crates: vec![Crate('d'), Crate('e'), Crate('f')],
        };
        let stack3 = Stack {
            crates: vec![Crate('g'), Crate('h')],
        };
        ship.stacks = vec![stack1, stack2, stack3];
        ship
    }

    #[test]
    fn on_top() {
        let mut stack = Stack {
            crates: vec![Crate('a'), Crate('b'), Crate('c')],
        };
        assert_eq!(stack.on_top(), Some(&Crate('c')));
        stack.push(Crate('d'));
        assert_eq!(stack.on_top(), Some(&Crate('d')));
        stack.pop();
        stack.pop();
        stack.pop();
        assert_eq!(stack.on_top(), Some(&Crate('a')));
        stack.pop();
        assert_eq!(stack.on_top(), None);
    }

    #[test]
    fn move_crates() {
        let mut ship = get_basic_ship();

        assert_eq!(ship.on_top(1), Some(&Crate('f')));

        ship.move_crate(0, 1).unwrap();
        assert_eq!(ship.on_top(1), Some(&Crate('c')));

        ship.move_crate(0, 1).unwrap();
        assert_eq!(ship.on_top(1), Some(&Crate('b')));

        ship.move_crate(0, 1).unwrap();
        assert_eq!(ship.on_top(1), Some(&Crate('a')));

        ship.move_crate(1, 0).unwrap();

        ship.move_crate(2, 0).unwrap();
        assert_eq!(ship.on_top(0), Some(&Crate('h')));

        ship.move_crate(2, 0).unwrap();
        assert_eq!(ship.on_top(0), Some(&Crate('g')));

        assert!(ship.move_crate(2, 0).is_err());

        ship.move_crates(3, 0, 1).unwrap();
        assert_eq!(ship.on_top(1), Some(&Crate('a')));

        assert!(ship.move_crates(3, 0, 1).is_err());
    }

    #[test]
    fn command() {
        let mut ship = get_basic_ship();

        ship.move_crates_from_commands("move 3 from 1 to 3")
            .unwrap();
        assert_eq!(ship.on_top(0), None);
        assert_eq!(ship.on_top(2), Some(&Crate('a')));

        ship.move_crates_from_commands("move 5 from 3 to 2")
            .unwrap();
        assert_eq!(ship.on_top(0), None);
        assert_eq!(ship.on_top(1), Some(&Crate('g')));
        assert_eq!(ship.on_top(2), None);

        ship.move_crates_from_commands("move 8 from 2 to 1")
            .unwrap();
        assert_eq!(ship.on_top(0), Some(&Crate('d')));
        assert_eq!(ship.on_top(1), None);
        assert_eq!(ship.on_top(2), None);
    }

    #[test]
    fn ship_from_file() {
        let ship = Ship::from_str(
            "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3
        ",
        );
        assert_eq!(ship.on_top(0), Some(&Crate('N')));
        assert_eq!(ship.on_top(1), Some(&Crate('D')));
        assert_eq!(ship.on_top(2), Some(&Crate('P')));
        assert_eq!(ship.on_top(3), None);
    }

    #[test]
    fn part1() {
        let string = fs::read_to_string("data/test-input.txt").unwrap();

        let mut ship = Ship::from_str(&string);
        ship.move_crates_from_commands(&string).unwrap();

        assert_eq!(ship.crates_on_top_as_string(), "CMZ");
    }

    #[test]
    fn part2() {
        let string = fs::read_to_string("data/test-input.txt").unwrap();

        let mut ship = Ship::crate_mover_9001_from_str(&string);
        ship.move_crates_from_commands(&string).unwrap();

        assert_eq!(ship.crates_on_top_as_string(), "MCD");
    }
}
