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

        for (point, _) in &self.trees {
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

        for tree_list in self.maybe_blocking(point) {
            let mut is_visible_in_this_line = true;
            for other_tree in tree_list {
                if other_tree.height >= tree.height {
                    is_visible_in_this_line = false;
                }
            }
            is_visible = is_visible || is_visible_in_this_line;
        }

        Some(is_visible)
    }

    fn maybe_blocking(&self, point: &Point) -> [Vec<Tree>; 4] {
        let mut tree_lists: [Vec<Tree>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        for (other_point, other_tree) in &self.trees {
            let other_tree = other_tree.clone();

            let index = match (other_point.x.cmp(&point.x), other_point.y.cmp(&point.y)) {
                (Ordering::Equal, Ordering::Less) => Some(0),
                (Ordering::Equal, Ordering::Greater) => Some(1),
                (Ordering::Less, Ordering::Equal) => Some(2),
                (Ordering::Greater, Ordering::Equal) => Some(3),
                _ => None,
            };

            if let Some(index) = index {
                tree_lists[index].push(other_tree);
            }
        }

        tree_lists
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
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

    #[test]
    fn from_string() {
        let grid = test_grid();
        assert_eq!(grid.trees.len(), 25);
    }

    #[test]
    fn is_visible() {
        let grid = test_grid();

        // border trees
        for x in [0, 4] {
            for y in 0..5 {
                assert!(grid.is_visible(&Point { x, y }).unwrap());
            }
        }
        for y in [0, 4] {
            for x in 0..5 {
                assert!(grid.is_visible(&Point { x, y }).unwrap());
            }
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
}
