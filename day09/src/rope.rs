use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_closer_to(&mut self, other: &Point) {
        self.x += (other.x - self.x).signum();
        self.y += (other.y - self.y).signum();
    }

    fn is_touching_with(&self, other: &Point) -> bool {
        (self.x.abs_diff(other.x) <= 1) && (self.y.abs_diff(other.y) <= 1)
    }
}

#[derive(Debug)]
struct RopeMovementError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RopeMovement {
    Up,
    Down,
    Right,
    Left,
}

impl FromStr for RopeMovement {
    type Err = RopeMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(RopeMovement::Up),
            "D" => Ok(RopeMovement::Down),
            "L" => Ok(RopeMovement::Left),
            "R" => Ok(RopeMovement::Right),
            _ => Err(RopeMovementError),
        }
    }
}

impl RopeMovement {
    fn from_list(s: &str) -> Vec<RopeMovement> {
        let re = Regex::new(r"(\w) (\d+)").unwrap();
        let mut movements = Vec::new();

        for caps in re.captures_iter(s) {
            for _ in 0..caps.get(2).unwrap().as_str().parse::<usize>().unwrap() {
                movements.push(
                    caps.get(1)
                        .unwrap()
                        .as_str()
                        .parse::<RopeMovement>()
                        .unwrap(),
                )
            }
        }

        movements
    }
}

#[derive(Debug, Clone)]
struct Rope {
    points: Vec<Point>,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            points: vec![Point { x: 0, y: 0 }, Point { x: 0, y: 0 }],
        }
    }

    fn new_with(point_count: u8) -> Result<Rope, ()> {
        if point_count < 2 {
            Err(())
        } else {
            let mut points = Vec::new();
            for _ in 0..point_count {
                points.push(Point { x: 0, y: 0 })
            }
            Ok(Rope { points })
        }
    }

    fn head(&self) -> &Point {
        self.points.get(0).unwrap()
    }

    fn head_mut(&mut self) -> &mut Point {
        self.points.get_mut(0).unwrap()
    }

    fn tail(&self) -> &Point {
        self.points.last().unwrap()
    }

    fn move_rope(&mut self, movement: &RopeMovement) {
        self.move_head(movement);
        self.move_nonhead();
    }

    fn move_head(&mut self, movement: &RopeMovement) {
        match movement {
            RopeMovement::Up => self.head_mut().y += 1,
            RopeMovement::Down => self.head_mut().y -= 1,
            RopeMovement::Right => self.head_mut().x += 1,
            RopeMovement::Left => self.head_mut().x -= 1,
        }
    }

    fn move_nonhead(&mut self) {
        for i in 1..self.points.len() {
            let range = &mut self.points[i - 1..i + 1];
            let mut knot = range[1];
            let previous = range[0];

            if !knot.is_touching_with(&previous) {
                knot.move_closer_to(&previous);
                range[1] = knot;
            }
        }
    }

    // part 1
    fn is_touching(&self) -> bool {
        (self.head().x.abs_diff(self.tail().x) <= 1) && (self.head().y.abs_diff(self.tail().y) <= 1)
    }
}

#[derive(Debug)]
pub struct RopeHistory {
    history: Vec<Rope>,
}

impl RopeHistory {
    pub fn new() -> RopeHistory {
        RopeHistory {
            history: Vec::new(),
        }
    }

    // part 1
    pub fn from_input(s: &str) -> RopeHistory {
        RopeHistory::from_input_with(s, 2).unwrap()
    }

    pub fn from_input_with(s: &str, knot_count: u8) -> Result<RopeHistory, ()> {
        let mut rh = RopeHistory::new();
        let mut rope = Rope::new_with(knot_count)?;
        rh.history.push(rope.clone());

        let movements = RopeMovement::from_list(s);

        for movement in &movements {
            rope.move_rope(movement);
            rh.history.push(rope.clone());
        }

        Ok(rh)
    }

    pub fn tail_unique_positions(&self) -> usize {
        let mut tail_positions: HashSet<Point> = HashSet::new();

        for rope in &self.history {
            tail_positions.insert(*rope.tail());
        }

        tail_positions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_movements() -> RopeHistory {
        RopeHistory::from_input(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        )
    }

    #[test]
    fn tail_unique_positions() {
        let history = test_movements();
        assert_eq!(history.tail_unique_positions(), 13);
    }

    #[test]
    fn part2() {
        let knot_count = 10;
        let history = RopeHistory::from_input_with(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
",
            knot_count,
        )
        .unwrap();
        assert_eq!(history.tail_unique_positions(), 1);

        let history = RopeHistory::from_input_with(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
",
            knot_count,
        )
        .unwrap();
        assert_eq!(history.tail_unique_positions(), 36);
    }

    #[test]
    fn move_head() {
        let mut rope = Rope::new();

        rope.move_head(&RopeMovement::Up);
        rope.move_head(&RopeMovement::Up);
        rope.move_head(&RopeMovement::Up);
        assert_eq!(rope.head(), &Point { x: 0, y: 3 });

        rope.move_head(&RopeMovement::Right);
        rope.move_head(&RopeMovement::Right);
        rope.move_head(&RopeMovement::Right);
        rope.move_head(&RopeMovement::Right);
        rope.move_head(&RopeMovement::Right);
        assert_eq!(rope.head(), &Point { x: 5, y: 3 });

        rope.move_head(&RopeMovement::Left);
        rope.move_head(&RopeMovement::Left);
        assert_eq!(rope.head(), &Point { x: 3, y: 3 });

        rope.move_head(&RopeMovement::Down);
        rope.move_head(&RopeMovement::Down);
        rope.move_head(&RopeMovement::Down);
        rope.move_head(&RopeMovement::Down);
        rope.move_head(&RopeMovement::Down);
        rope.move_head(&RopeMovement::Down);
        rope.move_head(&RopeMovement::Down);
        rope.move_head(&RopeMovement::Down);
        assert_eq!(rope.head(), &Point { x: 3, y: -5 });
    }

    #[test]
    fn is_touching() {
        let mut rope = Rope::new();
        assert!(rope.is_touching());

        rope.move_head(&RopeMovement::Up);
        assert!(rope.is_touching());

        rope.move_head(&RopeMovement::Right);
        assert!(rope.is_touching());

        rope.move_head(&RopeMovement::Left);
        assert!(rope.is_touching());

        rope.move_head(&RopeMovement::Up);
        assert!(!rope.is_touching());

        rope.move_head(&RopeMovement::Down);
        assert!(rope.is_touching());

        rope.move_head(&RopeMovement::Down);
        assert!(rope.is_touching());
    }

    #[test]
    fn move_rope() {
        let mut rope = Rope::new();
        assert!(rope.is_touching());

        rope.move_rope(&RopeMovement::Up);
        rope.move_rope(&RopeMovement::Up);
        assert_eq!(rope.head(), &Point { x: 0, y: 2 });
        assert_eq!(rope.tail(), &Point { x: 0, y: 1 });

        rope.move_rope(&RopeMovement::Up);
        rope.move_rope(&RopeMovement::Up);
        assert_eq!(rope.head(), &Point { x: 0, y: 4 });
        assert_eq!(rope.tail(), &Point { x: 0, y: 3 });

        rope.move_rope(&RopeMovement::Left);
        assert_eq!(rope.head(), &Point { x: -1, y: 4 });
        assert_eq!(rope.tail(), &Point { x: 0, y: 3 });

        rope.move_rope(&RopeMovement::Left);
        assert_eq!(rope.head(), &Point { x: -2, y: 4 });
        assert_eq!(rope.tail(), &Point { x: -1, y: 4 });

        rope.move_rope(&RopeMovement::Left);
        assert_eq!(rope.head(), &Point { x: -3, y: 4 });
        assert_eq!(rope.tail(), &Point { x: -2, y: 4 });
    }

    #[test]
    fn rope_movement_from_str() {
        assert_eq!("U".parse::<RopeMovement>().unwrap(), RopeMovement::Up);
        assert_eq!("D".parse::<RopeMovement>().unwrap(), RopeMovement::Down);
        assert_eq!("L".parse::<RopeMovement>().unwrap(), RopeMovement::Left);
        assert_eq!("R".parse::<RopeMovement>().unwrap(), RopeMovement::Right);
        assert!("X".parse::<RopeMovement>().is_err());
        assert!("R 4".parse::<RopeMovement>().is_err());
        assert!("D 5".parse::<RopeMovement>().is_err());
    }
}
