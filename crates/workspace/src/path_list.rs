use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use util::paths::SanitizedPath;

use crate::persistence::model::{LocalPaths, SerializedSshProject};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PathList {
    paths: Arc<[PathBuf]>,
    path_order: Arc<[usize]>,
}

impl PathList {
    pub fn new<P: AsRef<Path>>(paths: &[P]) -> Self {
        let mut indexed_paths: Vec<(usize, PathBuf)> = paths
            .iter()
            .enumerate()
            .map(|(ix, path)| (ix, SanitizedPath::from(path).into()))
            .collect();
        indexed_paths.sort_by(|(_, a), (_, b)| a.cmp(b));
        let path_order = indexed_paths.iter().map(|e| e.0).collect::<Vec<_>>().into();
        let paths = indexed_paths
            .into_iter()
            .map(|e| e.1)
            .collect::<Vec<_>>()
            .into();
        Self { path_order, paths }
    }

    pub fn from_strings((paths_string, order_string): &(String, String)) -> Self {
        let mut paths: Vec<PathBuf> = if paths_string.is_empty() {
            Vec::new()
        } else {
            paths_string
                .split(',')
                .map(|s| SanitizedPath::from(s).into())
                .collect()
        };

        let mut path_order: Vec<usize> = order_string
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();

        if !paths.is_sorted() || path_order.len() != paths.len() {
            path_order = (0..paths.len()).collect();
            paths.sort();
        }

        Self {
            paths: paths.into(),
            path_order: path_order.into(),
        }
    }

    pub fn paths(&self) -> &[PathBuf] {
        self.paths.as_ref()
    }

    pub fn order(&self) -> &[usize] {
        self.path_order.as_ref()
    }

    pub fn to_strings(&self) -> (String, String) {
        use std::fmt::Write as _;

        let mut paths = String::new();
        let mut path_order = String::new();
        for path in self.paths.iter() {
            if !paths.is_empty() {
                paths.push(',');
            }
            write!(&mut paths, "{}", path.display()).unwrap();
        }
        for ix in self.path_order.iter() {
            if !path_order.is_empty() {
                path_order.push(',');
            }
            write!(&mut path_order, "{}", *ix).unwrap();
        }
        (paths, path_order)
    }

    pub fn from_local(local_paths: LocalPaths) -> Self {
        let paths = local_paths.paths();
        Self::new(paths.as_slice())
    }

    pub fn from_ssh(ssh_project: SerializedSshProject) -> Self {
        let paths: Vec<PathBuf> = ssh_project.paths.iter().map(|p| PathBuf::from(p)).collect();
        Self::new(&paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_list() {
        let list1 = PathList::new(&["a/d", "a/c"]);
        let list2 = PathList::new(&["a/c", "a/d"]);

        assert_eq!(list1.paths(), list2.paths());
        assert_ne!(list1, list2);
        assert_eq!(list1.order(), &[1, 0]);
        assert_eq!(list2.order(), &[0, 1]);

        let list1_deserialized = PathList::from_strings(&list1.to_strings());
        assert_eq!(list1_deserialized, list1);

        let list2_deserialized = PathList::from_strings(&list2.to_strings());
        assert_eq!(list2_deserialized, list2);
    }
}
