use std::fs;
use std::str::FromStr;

enum Command {
    Addx(i32),
    Noop,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Command::Noop)
        } else if s.contains("addx ") {
            Ok(Command::Addx(s.split_whitespace().last().unwrap().parse::<i32>().unwrap()))
        } else {
            Err(())
        }
    }

}

#[derive(Debug)]
pub struct Program {
    x_values: Vec<i32>,
    screen_size: (u16, u16),
    sprite_length: u16,
}

impl Program {
    pub fn from_input(s: &str) -> Program {
        let mut commands = Vec::new();

        for line in s.lines() {
            commands.push(line.parse().unwrap());
        }

        Program::from_commands(&commands)
    }

    fn from_commands(commands: &Vec<Command>) -> Program {
        let mut x = 1;
        let mut program = Program::new();
        for command in commands {
            match command {
                Command::Noop => program.x_values.push(x),
                Command::Addx(value) => {
                    program.x_values.push(x);
                    program.x_values.push(x);
                    x += value;
                }
            }
        }
        program
    }

    fn new() -> Program {
        Program {
            x_values: Vec::new(),
            screen_size: (40, 6),
            sprite_length: 3,
        }
    }

    pub fn puzzle_sum(&self) -> Option<i32> {
        let mut sum = 0;
        for i in [20, 60, 100, 140, 180, 220] {
            sum += self.signal_strength(i)?;
        }
        Some(sum)
    }

    fn signal_strength(&self, cycle_num: usize) -> Option<i32> {
        Some(self.x_values.get(cycle_num - 1)? * (cycle_num as i32))
    }

    pub fn render(&self) -> String {
        let mut screen = String::new();
        for cycle_num in 1..(self.x_values.len() + 1) {
            self.draw(cycle_num, &mut screen).expect("Using indices from vec length");
        }
        screen
    }

    fn draw(&self, cycle_num: usize, screen: &mut String) -> Result<(), ()> {
        if self.should_draw(cycle_num).ok_or(())? {
            screen.push('#');
        } else {
            screen.push('.');
        }

        if cycle_num % (self.screen_size.0 as usize) == 0 {
            screen.push('\n');
        }

        Ok(())
    }

    fn should_draw(&self, cycle_num: usize) -> Option<bool> {
        let pixel_hor_index = (cycle_num as i32) - 1;
        Some(self.sprite_hor_position(cycle_num)?.abs_diff(pixel_hor_index % self.screen_size.0 as i32) <= 1)
    }

    fn sprite_hor_position(&self, cycle_num: usize) -> Option<&i32> {
        self.x_values.get(cycle_num - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_test_program() -> Program {
        let commands = vec![
            Command::Noop,
            Command::Addx(3),
            Command::Addx(-5),
        ];
        Program::from_commands(&commands)
    }

    fn big_test_program() -> Program {
        let s = fs::read_to_string("data/test-input.txt").unwrap();
        Program::from_input(&s)
    }

    #[test]
    fn signal_strength() {
        let program = simple_test_program();

        assert_eq!(program.signal_strength(1).unwrap(), 1);
        assert_eq!(program.signal_strength(2).unwrap(), 2);
        assert_eq!(program.signal_strength(3).unwrap(), 3);
        assert_eq!(program.signal_strength(4).unwrap(), 16);
        assert_eq!(program.signal_strength(5).unwrap(), 20);
        assert_eq!(program.signal_strength(6), None);
    }

    #[test]
    fn part1() {
        let program = big_test_program();
        assert_eq!(program.puzzle_sum().unwrap(), 13140);
    }

    #[test]
    fn should_draw() {
        let program = big_test_program();
        assert!(program.should_draw(1).unwrap());
        assert!(program.should_draw(2).unwrap());
        assert!(!program.should_draw(3).unwrap());
        assert!(!program.should_draw(4).unwrap());
        assert!(program.should_draw(5).unwrap());
        assert!(program.should_draw(6).unwrap());
        assert!(!program.should_draw(7).unwrap());
        assert!(!program.should_draw(8).unwrap());
        assert!(program.should_draw(9).unwrap());
        assert!(program.should_draw(10).unwrap());
        assert!(!program.should_draw(11).unwrap());
        assert!(!program.should_draw(12).unwrap());
        assert!(program.should_draw(13).unwrap());
        assert!(program.should_draw(14).unwrap());
        assert!(!program.should_draw(15).unwrap());
        assert!(!program.should_draw(16).unwrap());
        assert!(program.should_draw(17).unwrap());
        assert!(program.should_draw(18).unwrap());
        assert!(!program.should_draw(19).unwrap());
        assert!(!program.should_draw(20).unwrap());
    }

    #[test]
    fn part2() {
        let program = big_test_program();
        assert_eq!(program.render(), "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");
    }
}
