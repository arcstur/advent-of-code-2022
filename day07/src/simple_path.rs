use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SimplePath {
    path: String,
}

impl SimplePath {
    pub fn new(path: &str) -> SimplePath {
        SimplePath {
            path: String::from(path),
        }
    }

    pub fn join(&self, path: &str) -> SimplePath {
        let p = Path::new(&self.path);
        SimplePath::new(p.join(path).as_path().to_str().unwrap())
    }

    pub fn parent(&self) -> Option<SimplePath> {
        let p = Path::new(&self.path);
        match p.parent() {
            Some(path) => Some(SimplePath::new(path.to_str().unwrap())),
            None => None,
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        other.path.starts_with(&self.path)
    }
}

impl Clone for SimplePath {
    fn clone(&self) -> SimplePath {
        SimplePath::new(&self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join() {
        let initial = SimplePath::new("/initial/path");
        assert_eq!(
            initial.join("new_folder"),
            SimplePath::new("/initial/path/new_folder")
        );
        assert_eq!(
            initial.join("other_folder"),
            SimplePath::new("/initial/path/other_folder")
        );
        assert_eq!(
            initial.join("new_folder").join("final"),
            SimplePath::new("/initial/path/new_folder/final")
        );
    }

    #[test]
    fn parent() {
        let initial = SimplePath::new("/initial/path");

        assert_eq!(initial.parent(), Some(SimplePath::new("/initial")));
        assert_eq!(
            initial.parent().unwrap().parent(),
            Some(SimplePath::new("/"))
        );
        assert_eq!(initial.parent().unwrap().parent().unwrap().parent(), None);
    }

    #[test]
    fn contains() {
        let parent = SimplePath::new("/a/b/c");
        let child = SimplePath::new("/a/b/c/d/e");
        let not_child = SimplePath::new("/z/a/b/c/f");

        assert!(parent.contains(&child));
        assert!(!parent.contains(&not_child));
    }
}
