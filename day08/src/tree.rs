use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TreeGrid {
    trees: HashMap<Point, Tree>,
}

impl TreeGrid {
    fn new() -> TreeGrid {
        TreeGrid {
            trees: HashMap::new(),
        }
    }

    pub fn from_string(string: &str) -> TreeGrid {
        let mut grid = TreeGrid::new();

        for (y, line) in string.lines().enumerate() {
            for (x, num) in line.chars().enumerate() {
                grid.trees.insert(
                    Point { x, y },
                    Tree {
                        height: num.to_string().parse().unwrap(),
                    },
                );
            }
        }

        grid
    }

    pub fn visible_trees(&self) -> Option<u64> {
        let mut visible = 0;

        for point in self.trees.keys() {
            if self
                .is_visible(point)
                .expect("Tree from Grid has a visibility.")
            {
                visible += 1;
            }
        }

        Some(visible)
    }

    fn is_visible(&self, point: &Point) -> Option<bool> {
        let tree = self.trees.get(point)?;

        let mut is_visible = false;

        for point_list in self.maybe_blocking(point) {
            let mut is_visible_in_this_line = true;
            for other_point in point_list {
                let other_tree = self
                    .trees
                    .get(&other_point)
                    .expect("Point obtained from custom method");
                if other_tree.height >= tree.height {
                    is_visible_in_this_line = false;
                }
            }
            is_visible = is_visible || is_visible_in_this_line;
        }

        Some(is_visible)
    }

    fn maybe_blocking(&self, point: &Point) -> [Vec<Point>; 4] {
        let mut tree_lists: [Vec<Point>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        for other_point in self.trees.keys() {
            let index = match (other_point.x.cmp(&point.x), other_point.y.cmp(&point.y)) {
                (Ordering::Equal, Ordering::Less) => Some(0),
                (Ordering::Equal, Ordering::Greater) => Some(1),
                (Ordering::Less, Ordering::Equal) => Some(2),
                (Ordering::Greater, Ordering::Equal) => Some(3),
                _ => None,
            };

            if let Some(index) = index {
                tree_lists[index].push(*other_point);
            }
        }

        tree_lists
    }

    pub fn max_scenic_score(&self) -> Option<u64> {
        if self.trees.is_empty() {
            return None;
        }
        let mut max = 0;

        for point in self.trees.keys() {
            let score = self
                .scenic_score(point)
                .expect("This point should have a tree");

            if max < score {
                max = score;
            }
        }

        Some(max)
    }

    fn scenic_score(&self, point: &Point) -> Option<u64> {
        let tree = self.trees.get(point)?;
        let mut score = 1;

        for mut point_list in self.maybe_blocking(point) {
            let mut line_score = 0;

            point_list.sort_by_key(|p| p.distance(point));
            for other_point in point_list {
                let other_tree = self
                    .trees
                    .get(&other_point)
                    .expect("Point obtained from custom method");

                line_score += 1;
                if other_tree.height >= tree.height {
                    break;
                }
            }
            score *= line_score;
        }

        Some(score)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance(&self, other: &Point) -> u64 {
        (((other.x.abs_diff(self.x)) as f64).hypot((other.y.abs_diff(self.y)) as f64) * 1000.0)
            .round() as u64
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Tree {
    height: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_grid() -> TreeGrid {
        TreeGrid::from_string("30373\n25512\n65332\n33549\n35390")
    }

    fn test_grid_border_points() -> Vec<Point> {
        let mut points = Vec::new();

        for x in [0, 4] {
            for y in 0..5 {
                points.push(Point { x, y });
            }
        }
        for y in [0, 4] {
            for x in 0..5 {
                points.push(Point { x, y });
            }
        }

        points
    }

    #[test]
    fn from_string() {
        let grid = test_grid();
        assert_eq!(grid.trees.len(), 25);
    }

    #[test]
    fn is_visible() {
        let grid = test_grid();

        // border trees
        for point in test_grid_border_points() {
            assert!(grid.is_visible(&point).unwrap());
        }

        // visible internal trees
        assert!(grid.is_visible(&Point { x: 1, y: 1 }).unwrap()); // 5
        assert!(grid.is_visible(&Point { x: 2, y: 1 }).unwrap()); // 5
        assert!(grid.is_visible(&Point { x: 1, y: 2 }).unwrap()); // 5
        assert!(grid.is_visible(&Point { x: 3, y: 2 }).unwrap()); // 3
        assert!(grid.is_visible(&Point { x: 2, y: 3 }).unwrap()); // 5

        // not visible internal trees
        assert!(!grid.is_visible(&Point { x: 3, y: 1 }).unwrap()); // 1
        assert!(!grid.is_visible(&Point { x: 2, y: 2 }).unwrap()); // 3
        assert!(!grid.is_visible(&Point { x: 1, y: 3 }).unwrap()); // 3
        assert!(!grid.is_visible(&Point { x: 3, y: 3 }).unwrap()); // 4
    }

    #[test]
    fn visible_trees() {
        let grid = test_grid();
        assert_eq!(grid.visible_trees().unwrap(), 21);
    }

    #[test]
    fn scenic_score() {
        let grid = test_grid();

        for point in test_grid_border_points() {
            assert_eq!(grid.scenic_score(&point).unwrap(), 0);
        }

        assert_eq!(grid.scenic_score(&Point { x: 2, y: 1 }).unwrap(), 4);
        assert_eq!(grid.scenic_score(&Point { x: 2, y: 3 }).unwrap(), 8);
    }

    #[test]
    fn max_scenic_score() {
        let grid = test_grid();
        assert_eq!(grid.max_scenic_score().unwrap(), 8);
    }
}
