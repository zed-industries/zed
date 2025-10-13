use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use itertools::Itertools;
use util::paths::SanitizedPath;

/// A list of absolute paths, in a specific order.
///
/// The paths are stored in lexicographic order, so that they can be compared to
/// other path lists without regard to the order of the paths.
///
/// The paths can be retrieved in the original order using `ordered_paths()`.
#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct PathList {
    /// The paths, in lexicographic order.
    paths: Arc<[PathBuf]>,
    /// The order in which the paths were provided.
    ///
    /// See `ordered_paths()` for a way to get the paths in the original order.
    order: Arc<[usize]>,
}

#[derive(Debug)]
pub struct SerializedPathList {
    pub paths: String,
    pub order: String,
}

impl PathList {
    pub fn new<P: AsRef<Path>>(paths: &[P]) -> Self {
        let mut indexed_paths: Vec<(usize, PathBuf)> = paths
            .iter()
            .enumerate()
            .map(|(ix, path)| (ix, SanitizedPath::new(path).into()))
            .collect();
        indexed_paths.sort_by(|(_, a), (_, b)| a.cmp(b));
        let order = indexed_paths.iter().map(|e| e.0).collect::<Vec<_>>().into();
        let paths = indexed_paths
            .into_iter()
            .map(|e| e.1)
            .collect::<Vec<_>>()
            .into();
        Self { order, paths }
    }

    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }

    /// Get the paths in lexicographic order.
    pub fn paths(&self) -> &[PathBuf] {
        self.paths.as_ref()
    }

    /// Get the order in which the paths were provided.
    pub fn order(&self) -> &[usize] {
        self.order.as_ref()
    }

    /// Get the paths in the original order.
    pub fn ordered_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.order
            .iter()
            .zip(self.paths.iter())
            .sorted_by_key(|(i, _)| **i)
            .map(|(_, path)| path)
    }

    pub fn is_lexicographically_ordered(&self) -> bool {
        self.order.iter().enumerate().all(|(i, &j)| i == j)
    }

    pub fn deserialize(serialized: &SerializedPathList) -> Self {
        let mut paths: Vec<PathBuf> = if serialized.paths.is_empty() {
            Vec::new()
        } else {
            serialized.paths.split('\n').map(PathBuf::from).collect()
        };

        let mut order: Vec<usize> = serialized
            .order
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();

        if !paths.is_sorted() || order.len() != paths.len() {
            order = (0..paths.len()).collect();
            paths.sort();
        }

        Self {
            paths: paths.into(),
            order: order.into(),
        }
    }

    pub fn serialize(&self) -> SerializedPathList {
        use std::fmt::Write as _;

        let mut paths = String::new();
        for path in self.paths.iter() {
            if !paths.is_empty() {
                paths.push('\n');
            }
            paths.push_str(&path.to_string_lossy());
        }

        let mut order = String::new();
        for ix in self.order.iter() {
            if !order.is_empty() {
                order.push(',');
            }
            write!(&mut order, "{}", *ix).unwrap();
        }
        SerializedPathList { paths, order }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_list() {
        let list1 = PathList::new(&["a/d", "a/c"]);
        let list2 = PathList::new(&["a/c", "a/d"]);

        assert_eq!(list1.paths(), list2.paths(), "paths differ");
        assert_eq!(list1.order(), &[1, 0], "list1 order incorrect");
        assert_eq!(list2.order(), &[0, 1], "list2 order incorrect");

        let list1_deserialized = PathList::deserialize(&list1.serialize());
        assert_eq!(list1_deserialized, list1, "list1 deserialization failed");

        let list2_deserialized = PathList::deserialize(&list2.serialize());
        assert_eq!(list2_deserialized, list2, "list2 deserialization failed");

        assert_eq!(
            list1.ordered_paths().collect_array().unwrap(),
            [&PathBuf::from("a/d"), &PathBuf::from("a/c")],
            "list1 ordered paths incorrect"
        );
        assert_eq!(
            list2.ordered_paths().collect_array().unwrap(),
            [&PathBuf::from("a/c"), &PathBuf::from("a/d")],
            "list2 ordered paths incorrect"
        );
    }

    #[test]
    fn test_path_list_ordering() {
        let list = PathList::new(&["b", "a", "c"]);
        assert_eq!(
            list.paths(),
            &[PathBuf::from("a"), PathBuf::from("b"), PathBuf::from("c")]
        );
        assert_eq!(list.order(), &[1, 0, 2]);
        assert!(!list.is_lexicographically_ordered());

        let serialized = list.serialize();
        let deserialized = PathList::deserialize(&serialized);
        assert_eq!(deserialized, list);

        assert_eq!(
            deserialized.ordered_paths().collect_array().unwrap(),
            [
                &PathBuf::from("b"),
                &PathBuf::from("a"),
                &PathBuf::from("c")
            ]
        );

        let list = PathList::new(&["b", "c", "a"]);
        assert_eq!(
            list.paths(),
            &[PathBuf::from("a"), PathBuf::from("b"), PathBuf::from("c")]
        );
        assert_eq!(list.order(), &[2, 0, 1]);
        assert!(!list.is_lexicographically_ordered());

        let serialized = list.serialize();
        let deserialized = PathList::deserialize(&serialized);
        assert_eq!(deserialized, list);

        assert_eq!(
            deserialized.ordered_paths().collect_array().unwrap(),
            [
                &PathBuf::from("b"),
                &PathBuf::from("c"),
                &PathBuf::from("a"),
            ]
        );
    }
}
