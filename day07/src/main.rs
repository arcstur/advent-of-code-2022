use regex::Regex;
use std::collections::HashMap;
use std::fs;

mod simple_path;

use simple_path::SimplePath;

#[derive(Debug)]
struct FileSystem {
    // u64 value is the folder's total file size, not counting subfolders
    // for that, use the dir_size method
    dirs: HashMap<SimplePath, u64>,
    cwd: SimplePath,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {
            dirs: HashMap::new(),
            cwd: SimplePath::new("/"),
        }
    }

    pub fn from_commands(commands: &str) -> FileSystem {
        let re = Regex::new(r"\$[^\$]*").unwrap();
        let re_files = Regex::new(r"(\d+) ").unwrap();

        let mut fs = FileSystem::new();

        for caps in re.captures_iter(commands) {
            let command_and_output = caps.get(0).unwrap().as_str();

            if command_and_output.starts_with("$ cd") {
                let dir_name = command_and_output.split_whitespace().last().unwrap();

                if dir_name == ".." {
                    fs.cwd = fs.cwd.parent().unwrap();
                } else {
                    fs.cwd = fs.cwd.join(dir_name);
                }
            } else if command_and_output.starts_with("$ ls") {
                let mut size: u64 = 0;

                for size_caps in re_files.captures_iter(command_and_output) {
                    size += size_caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
                }

                fs.dirs.insert(fs.cwd.clone(), size);
            }
        }

        fs
    }
}

impl FileSystem {
    fn dir_size(&self, path: &SimplePath) -> Option<u64> {
        let mut size = 0;

        for (other_path, other_size) in &self.dirs {
            if path.contains(other_path) {
                size += other_size;
            }
        }

        Some(size)
    }

    fn dir_sizes(&self) -> Vec<u64> {
        let mut dir_sizes: Vec<u64> = Vec::new();
        for (path, _) in &self.dirs {
            if let Some(size) = self.dir_size(path) {
                dir_sizes.push(size);
            }
        }
        dir_sizes
    }

    fn sum_of_sizes_below(&self, max_size: u64) -> u64 {
        let mut sum = 0;

        for size in self.dir_sizes() {
            if size <= max_size {
                sum += size;
            }
        }

        sum
    }

    fn min_dir_size_gt(&self, greater_than: u64) -> Option<u64> {
        let mut min_size = None;
        for size in self.dir_sizes() {
            if size >= greater_than {
                min_size = match min_size {
                    None => Some(size),
                    Some(s) => {
                        if size < s {
                            Some(size)
                        } else {
                            Some(s)
                        }
                    }
                };
            }
        }

        min_size
    }

    fn dir_size_to_delete(&self, total_size: u64, needed_size: u64) -> Option<u64> {
        let used_size = self.dir_size(&SimplePath::new("/"))?;

        let free_size = total_size - used_size;

        if free_size >= needed_size {
            None
        } else {
            self.min_dir_size_gt(needed_size - free_size)
        }
    }
}

fn main() {
    let string = fs::read_to_string("data/input.txt").unwrap();
    let file_system = FileSystem::from_commands(&string);

    println!(
        "The sum of sizes of directories with size of at most 100000 is: {}",
        file_system.sum_of_sizes_below(100_000)
    );

    let total_size = 70000000;
    let needed_size = 30000000;
    println!(
        "The total disk size is {}, and we need {}",
        total_size, needed_size
    );

    println!(
        "Therefore, we need to delete a directory of size {}",
        file_system
            .dir_size_to_delete(total_size, needed_size)
            .unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_size() {
        let string = fs::read_to_string("data/test-input.txt").unwrap();
        let file_system = FileSystem::from_commands(&string);

        assert_eq!(file_system.dir_size(&SimplePath::new("/a/e")), Some(584));
        assert_eq!(file_system.dir_size(&SimplePath::new("/a")), Some(94853));
        assert_eq!(file_system.dir_size(&SimplePath::new("/d")), Some(24933642));
        assert_eq!(file_system.dir_size(&SimplePath::new("/")), Some(48381165));
    }

    #[test]
    fn part1() {
        let string = fs::read_to_string("data/test-input.txt").unwrap();
        let file_system = FileSystem::from_commands(&string);

        assert_eq!(file_system.sum_of_sizes_below(100_000), 95437);
    }

    #[test]
    fn part2() {
        let string = fs::read_to_string("data/test-input.txt").unwrap();
        let file_system = FileSystem::from_commands(&string);

        assert_eq!(
            file_system.dir_size_to_delete(70000000, 30000000),
            Some(24933642)
        );
    }
}
