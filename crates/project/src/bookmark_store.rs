use std::collections::BTreeMap;

use itertools::Itertools;
use settings::WorktreeId;

use crate::ProjectPath;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Bookmark {
    pub id: usize,
    pub project_path: ProjectPath,
    pub line_no: usize,
    pub annotation: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct BookmarkStore {
    bookmark_ordered: Vec<usize>,
    current_id: usize,
    bookmark_map: BTreeMap<usize, Bookmark>,
    next_bookmark_id: usize,
}

impl BookmarkStore {
    pub fn new() -> Self {
        Self {
            bookmark_ordered: vec![],
            current_id: 0,
            bookmark_map: BTreeMap::default(),
            next_bookmark_id: 0,
        }
    }

    pub fn update_current_id(&mut self, current_id: usize) {
        if self.bookmark_ordered.contains(&current_id) {
            self.current_id = current_id;
        }
    }

    pub fn toggle(
        &mut self,
        project_path: ProjectPath,
        line_no: usize,
        annotation: Option<String>,
    ) {
        let id = self
            .bookmark_map
            .iter()
            .find(|(_, bm)| bm.project_path == project_path && bm.line_no == line_no)
            .map(|(id, _)| *id);
        if let Some(id) = id {
            self.bookmark_map.remove(&id);
            self.bookmark_ordered.retain(|i| i != &id);
            if self.current_id == id {
                self.current_id = self.find_nearest_id(self.current_id);
            }
        } else {
            self.next_bookmark_id += 1;
            let bookmark_id = self.next_bookmark_id;
            let bookmark = Bookmark {
                id: bookmark_id,
                project_path,
                line_no,
                annotation,
            };

            self.bookmark_map.insert(bookmark_id, bookmark);
            self.bookmark_ordered.push(bookmark_id);
            self.current_id = bookmark_id;
        }
    }

    fn find_nearest_id(&mut self, target_id: usize) -> usize {
        // first find previous id
        if let Some(id) = self.find_previous_id(target_id) {
            return id;
        }
        // find next id
        if let Some(id) = self.find_next_id(target_id) {
            return id;
        }
        // empty
        0
    }

    fn find_previous_id(&mut self, target_id: usize) -> Option<usize> {
        let mut previous = None;
        for id in self.bookmark_ordered.iter() {
            if id >= &target_id {
                break;
            }
            previous = Some(*id);
        }
        previous
    }

    fn find_next_id(&mut self, target_id: usize) -> Option<usize> {
        let mut next = None;
        for id in self.bookmark_ordered.iter() {
            if id > &target_id {
                next = Some(*id);
                break;
            }
        }
        next
    }

    pub fn prev(&mut self) -> Option<Bookmark> {
        if let Some(id) = self.find_previous_id(self.current_id) {
            self.current_id = id;
            return self.bookmark_map.get(&id).cloned();
        }
        None
    }

    pub fn next(&mut self) -> Option<Bookmark> {
        if let Some(id) = self.find_next_id(self.current_id) {
            self.current_id = id;
            return self.bookmark_map.get(&id).cloned();
        }
        None
    }

    pub fn clear_current_editor(&mut self, project_path: ProjectPath) {
        let ids_will_remove = self
            .bookmark_map
            .iter()
            .filter_map(|(id, bm)| {
                if bm.project_path == project_path {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect_vec();

        self.bookmark_ordered
            .retain(|id| !ids_will_remove.contains(id));
        self.bookmark_map
            .retain(|id, _| !ids_will_remove.contains(id));
        if ids_will_remove.contains(&self.current_id) {
            self.current_id = self.find_nearest_id(self.current_id);
        }
    }

    pub fn clear_current_worktree(&mut self, worktree_id: WorktreeId) {
        let ids_will_remove = self
            .bookmark_map
            .iter()
            .filter_map(|(id, bm)| {
                if bm.project_path.worktree_id == worktree_id {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect_vec();

        self.bookmark_ordered
            .retain(|id| !ids_will_remove.contains(id));
        self.bookmark_map
            .retain(|id, _| !ids_will_remove.contains(id));
        if ids_will_remove.contains(&self.current_id) {
            self.current_id = self.find_nearest_id(self.current_id);
        }
    }

    pub fn clear_all(&mut self) {
        self.current_id = 0;
        self.next_bookmark_id = 0;
        self.bookmark_ordered.clear();
        self.bookmark_map.clear();
    }

    pub fn get_current_editor(&self, project_path: ProjectPath) -> Vec<Bookmark> {
        self.bookmark_map
            .values()
            .filter_map(|bm| {
                if bm.project_path == project_path {
                    Some(bm.clone())
                } else {
                    None
                }
            })
            .collect_vec()
    }

    pub fn get_current_worktree(&self, worktree_id: WorktreeId) -> Vec<Bookmark> {
        self.bookmark_map
            .values()
            .filter_map(|bm| {
                if bm.project_path.worktree_id == worktree_id {
                    Some(bm.clone())
                } else {
                    None
                }
            })
            .collect_vec()
    }

    pub fn get_all(&self) -> Vec<Bookmark> {
        self.bookmark_map
            .values()
            .map(|bm| bm.clone())
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_new() {
        let bookmark_store = BookmarkStore::new();
        assert_eq!(bookmark_store.current_id, 0);
        assert_eq!(bookmark_store.bookmark_ordered.len(), 0);
        assert_eq!(bookmark_store.bookmark_map, BTreeMap::default());
        assert_eq!(bookmark_store.next_bookmark_id, 0);
    }

    #[test]
    fn test_toggle() {
        let mut bookmark_store = BookmarkStore::new();
        let project_path = ProjectPath {
            worktree_id: WorktreeId::from_usize(1),
            path: Path::new("test.rs").into(),
        };

        // First toggle
        bookmark_store.toggle(project_path.clone(), 10, None);
        assert_eq!(bookmark_store.current_id, 1);
        assert_eq!(bookmark_store.bookmark_ordered, vec![1]);
        assert_eq!(
            bookmark_store.bookmark_map,
            BTreeMap::from([(
                1,
                Bookmark {
                    id: 1,
                    project_path: project_path.clone(),
                    line_no: 10,
                    annotation: None
                }
            )])
        );

        // Toggle the same bookmark
        bookmark_store.toggle(project_path.clone(), 10, None);
        assert_eq!(bookmark_store.current_id, 0);
        assert_eq!(bookmark_store.bookmark_map, BTreeMap::default());

        // Toggle another bookmark
        bookmark_store.toggle(project_path.clone(), 20, Some("test".into()));
        assert_eq!(bookmark_store.current_id, 2);
        assert_eq!(bookmark_store.bookmark_ordered, vec![2]);
        assert_eq!(
            bookmark_store.bookmark_map,
            BTreeMap::from([(
                2,
                Bookmark {
                    id: 2,
                    project_path: project_path.clone(),
                    line_no: 20,
                    annotation: Some("test".into())
                }
            )])
        );
    }

    #[test]
    fn test_find_previous_id() {
        let mut bookmark_store = BookmarkStore::new();
        bookmark_store.bookmark_ordered = vec![1, 2, 3];

        assert_eq!(bookmark_store.find_previous_id(3), Some(2));
        assert_eq!(bookmark_store.find_previous_id(2), Some(1));
        assert_eq!(bookmark_store.find_previous_id(1), None);
    }

    #[test]
    fn test_find_next_id() {
        let mut bookmark_store = BookmarkStore::new();
        bookmark_store.bookmark_ordered = vec![1, 2, 3];

        assert_eq!(bookmark_store.find_next_id(1), Some(2));
        assert_eq!(bookmark_store.find_next_id(2), Some(3));
        assert_eq!(bookmark_store.find_next_id(3), None);
    }

    #[test]
    fn test_prev() {
        let mut bookmark_store = BookmarkStore::new();
        let project_path = ProjectPath {
            worktree_id: WorktreeId::from_usize(1),
            path: Path::new("test.rs").into(),
        };

        // Add bookmarks
        bookmark_store.toggle(project_path.clone(), 10, None);
        bookmark_store.toggle(project_path.clone(), 20, None);
        bookmark_store.toggle(project_path.clone(), 30, None);

        // Current ID is 3
        assert_eq!(bookmark_store.current_id, 3);

        // Prev to ID 2
        assert_eq!(
            bookmark_store.prev(),
            Some(Bookmark {
                id: 2,
                project_path: project_path.clone(),
                line_no: 20,
                annotation: None
            })
        );
        assert_eq!(bookmark_store.current_id, 2);

        // Prev to ID 1
        assert_eq!(
            bookmark_store.prev(),
            Some(Bookmark {
                id: 1,
                project_path: project_path.clone(),
                line_no: 10,
                annotation: None
            })
        );
        assert_eq!(bookmark_store.current_id, 1);

        // Prev from ID 1
        assert_eq!(bookmark_store.prev(), None);
    }

    #[test]
    fn test_next() {
        let mut bookmark_store = BookmarkStore::new();
        let project_path = ProjectPath {
            worktree_id: WorktreeId::from_usize(1),
            path: Path::new("test.rs").into(),
        };

        // Add bookmarks
        bookmark_store.toggle(project_path.clone(), 10, None);
        bookmark_store.toggle(project_path.clone(), 20, None);
        bookmark_store.toggle(project_path.clone(), 30, None);

        // Current ID is 1
        assert_eq!(bookmark_store.current_id, 3);
        bookmark_store.prev();
        bookmark_store.prev();
        bookmark_store.prev();
        assert_eq!(bookmark_store.current_id, 1);

        // Next to ID 2
        assert_eq!(
            bookmark_store.next(),
            Some(Bookmark {
                id: 2,
                project_path: project_path.clone(),
                line_no: 20,
                annotation: None
            })
        );
        assert_eq!(bookmark_store.current_id, 2);

        // Next to ID 3
        assert_eq!(
            bookmark_store.next(),
            Some(Bookmark {
                id: 3,
                project_path: project_path.clone(),
                line_no: 30,
                annotation: None
            })
        );
        assert_eq!(bookmark_store.current_id, 3);

        // Next from ID 3
        assert_eq!(bookmark_store.next(), None);
    }

    #[test]
    fn test_clear_current_editor() {
        let mut bookmark_store = BookmarkStore::new();
        let project_path1 = ProjectPath {
            worktree_id: WorktreeId::from_usize(1),
            path: Path::new("test1.rs").into(),
        };
        let project_path2 = ProjectPath {
            worktree_id: WorktreeId::from_usize(1),
            path: Path::new("test2.rs").into(),
        };

        // Add bookmarks
        bookmark_store.toggle(project_path1.clone(), 10, None);
        bookmark_store.toggle(project_path2.clone(), 20, None);
        bookmark_store.toggle(project_path1.clone(), 30, None);

        // Current ID is 3
        assert_eq!(bookmark_store.current_id, 3);

        // Clear bookmarks in project_path1
        bookmark_store.clear_current_editor(project_path1.clone());
        assert_eq!(bookmark_store.current_id, 2);
        assert_eq!(
            bookmark_store.bookmark_map,
            BTreeMap::from([(
                2,
                Bookmark {
                    id: 2,
                    project_path: project_path2.clone(),
                    line_no: 20,
                    annotation: None
                }
            )])
        );
    }

    #[test]
    fn test_clear_current_worktree() {
        let mut bookmark_store = BookmarkStore::new();
        let worktree_id1 = WorktreeId::from_usize(1);
        let worktree_id2 = WorktreeId::from_usize(2);
        let project_path1 = ProjectPath {
            worktree_id: worktree_id1,
            path: Path::new("test1.rs").into(),
        };
        let project_path2 = ProjectPath {
            worktree_id: worktree_id2,
            path: Path::new("test2.rs").into(),
        };

        // Add bookmarks
        bookmark_store.toggle(project_path1.clone(), 10, None);
        bookmark_store.toggle(project_path2.clone(), 20, None);
        bookmark_store.toggle(project_path1.clone(), 30, None);

        // Current ID is 3
        assert_eq!(bookmark_store.current_id, 3);

        // Clear bookmarks in worktree_id1
        bookmark_store.clear_current_worktree(worktree_id1);
        assert_eq!(bookmark_store.current_id, 2);
        assert_eq!(
            bookmark_store.bookmark_map,
            BTreeMap::from([(
                2,
                Bookmark {
                    id: 2,
                    project_path: project_path2.clone(),
                    line_no: 20,
                    annotation: None
                }
            )])
        );
    }

    #[test]
    fn test_clear_all() {
        let mut bookmark_store = BookmarkStore::new();
        let project_path = ProjectPath {
            worktree_id: WorktreeId::from_usize(1),
            path: Path::new("test.rs").into(),
        };

        // Add bookmarks
        bookmark_store.toggle(project_path.clone(), 10, None);
        bookmark_store.toggle(project_path.clone(), 20, None);
        bookmark_store.toggle(project_path.clone(), 30, None);

        // Clear all bookmarks
        bookmark_store.clear_all();
        assert_eq!(bookmark_store.current_id, 0);
        assert_eq!(bookmark_store.bookmark_ordered.len(), 0);
        assert_eq!(bookmark_store.bookmark_map, BTreeMap::default());
    }

    #[test]
    fn test_get_current_editor() {
        let mut bookmark_store = BookmarkStore::new();
        let project_path1 = ProjectPath {
            worktree_id: WorktreeId::from_usize(1),
            path: Path::new("test1.rs").into(),
        };
        let project_path2 = ProjectPath {
            worktree_id: WorktreeId::from_usize(1),
            path: Path::new("test2.rs").into(),
        };

        // Add bookmarks
        bookmark_store.toggle(project_path1.clone(), 10, None);
        bookmark_store.toggle(project_path2.clone(), 20, None);
        bookmark_store.toggle(project_path1.clone(), 30, None);

        // Get bookmarks in project_path1
        let bookmarks = bookmark_store.get_current_editor(project_path1.clone());
        assert_eq!(bookmarks.len(), 2);
        assert_eq!(
            bookmarks,
            vec![
                Bookmark {
                    id: 1,
                    project_path: project_path1.clone(),
                    line_no: 10,
                    annotation: None
                },
                Bookmark {
                    id: 3,
                    project_path: project_path1.clone(),
                    line_no: 30,
                    annotation: None
                }
            ]
        );
    }

    #[test]
    fn test_get_current_worktree() {
        let mut bookmark_store = BookmarkStore::new();
        let worktree_id1 = WorktreeId::from_usize(1);
        let worktree_id2 = WorktreeId::from_usize(2);
        let project_path1 = ProjectPath {
            worktree_id: worktree_id1,
            path: Path::new("test1.rs").into(),
        };
        let project_path2 = ProjectPath {
            worktree_id: worktree_id2,
            path: Path::new("test2.rs").into(),
        };

        // Add bookmarks
        bookmark_store.toggle(project_path1.clone(), 10, None);
        bookmark_store.toggle(project_path2.clone(), 20, None);
        bookmark_store.toggle(project_path1.clone(), 30, None);

        // Get bookmarks in worktree_id1
        let bookmarks = bookmark_store.get_current_worktree(worktree_id1);
        assert_eq!(bookmarks.len(), 2);
        assert_eq!(
            bookmarks,
            vec![
                Bookmark {
                    id: 1,
                    project_path: project_path1.clone(),
                    line_no: 10,
                    annotation: None
                },
                Bookmark {
                    id: 3,
                    project_path: project_path1.clone(),
                    line_no: 30,
                    annotation: None
                }
            ]
        );
    }

    #[test]
    fn test_get_all() {
        let mut bookmark_store = BookmarkStore::new();
        let project_path1 = ProjectPath {
            worktree_id: WorktreeId::from_usize(1),
            path: Path::new("test1.rs").into(),
        };
        let project_path2 = ProjectPath {
            worktree_id: WorktreeId::from_usize(1),
            path: Path::new("test2.rs").into(),
        };

        // Add bookmarks
        bookmark_store.toggle(project_path1.clone(), 10, None);
        bookmark_store.toggle(project_path2.clone(), 20, None);
        bookmark_store.toggle(project_path1.clone(), 30, None);

        // Get all bookmarks
        let bookmarks = bookmark_store.get_all();
        assert_eq!(bookmarks.len(), 3);
        assert_eq!(
            bookmarks,
            vec![
                Bookmark {
                    id: 1,
                    project_path: project_path1.clone(),
                    line_no: 10,
                    annotation: None
                },
                Bookmark {
                    id: 2,
                    project_path: project_path2.clone(),
                    line_no: 20,
                    annotation: None
                },
                Bookmark {
                    id: 3,
                    project_path: project_path1.clone(),
                    line_no: 30,
                    annotation: None
                }
            ]
        );
    }
}
