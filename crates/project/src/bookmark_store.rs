use std::{collections::BTreeMap, path::Path, sync::Arc};

use gpui::{Model, ModelContext};
use itertools::Itertools;
use language::Buffer;
use settings::WorktreeId;
use text::{Anchor, BufferId};

use crate::{
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    Item,
};

#[derive(Debug, Clone)]
pub struct Bookmark {
    pub id: usize,
    pub buffer: Model<Buffer>,
    pub anchor: Anchor,
    pub annotation: Option<String>,
}

#[derive(Debug)]
pub struct BookmarkStore {
    bookmark_ordered: Vec<usize>,
    current_id: usize,
    bookmark_map: BTreeMap<usize, Bookmark>,
    next_bookmark_id: usize,
}

impl BookmarkStore {
    pub fn new(worktree_store: Model<WorktreeStore>, cx: &mut ModelContext<Self>) -> Self {
        cx.subscribe(&worktree_store, Self::on_worktree_store_event)
            .detach();
        Self {
            bookmark_ordered: vec![],
            current_id: 0,
            bookmark_map: BTreeMap::default(),
            next_bookmark_id: 0,
        }
    }

    pub fn update_current_id(&mut self, current_id: usize, _cx: &mut ModelContext<Self>) {
        if self.bookmark_ordered.contains(&current_id) {
            self.current_id = current_id;
        }
    }

    fn on_worktree_store_event(
        &mut self,
        _: Model<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            WorktreeStoreEvent::WorktreeAdded(worktree) => cx
                .subscribe(worktree, |this, worktree, event, cx| match event {
                    worktree::Event::DeletedEntry(id) => {
                        // TODO, need better method
                        let worktree_id = worktree.read(cx).id();
                        if let Some(entry) = worktree.read(cx).entry_for_id(id.clone()) {
                            let path = entry.path.clone();
                            this.clear_by_project_entry_id(worktree_id, path, cx);
                        }
                    }
                    _ => {}
                })
                .detach(),
            WorktreeStoreEvent::WorktreeRemoved(_, id) => self.clear_by_worktree_id(*id, cx),
            _ => {}
        }
    }

    pub fn toggle_bookmark(
        &mut self,
        buffer: Model<Buffer>,
        anchor: Anchor,
        annotation: Option<String>,
    ) {
        let id = self
            .bookmark_map
            .iter()
            .find(|(_, bm)| bm.buffer == buffer && bm.anchor == anchor)
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
                buffer,
                anchor,
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

    pub fn prev_bookmark(&mut self) -> Option<Bookmark> {
        if let Some(id) = self.find_previous_id(self.current_id) {
            self.current_id = id;
            return self.bookmark_map.get(&id).cloned();
        }
        None
    }

    pub fn next_bookmark(&mut self) -> Option<Bookmark> {
        if let Some(id) = self.find_next_id(self.current_id) {
            self.current_id = id;
            return self.bookmark_map.get(&id).cloned();
        }
        None
    }

    fn clear_by_project_entry_id(
        &mut self,
        worktree_id: WorktreeId,
        path: Arc<Path>,
        cx: &mut ModelContext<Self>,
    ) {
        let ids_will_remove = self
            .bookmark_map
            .iter()
            .filter_map(|(id, bm)| {
                let file = bm.buffer.read(cx).file()?;
                if file.worktree_id(cx) == worktree_id && file.path() == &path {
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

    pub fn clear_by_buffer_id(&mut self, buffer_id: BufferId, cx: &mut ModelContext<Self>) {
        let ids_will_remove = self
            .bookmark_map
            .iter()
            .filter_map(|(id, bm)| {
                if bm.buffer.read(cx).remote_id() == buffer_id {
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

    pub fn clear_by_worktree_id(&mut self, worktree_id: WorktreeId, cx: &mut ModelContext<Self>) {
        let ids_will_remove = self
            .bookmark_map
            .iter()
            .filter_map(|(id, bm)| {
                if let Some(project_path) = bm.buffer.read(cx).project_path(cx) {
                    if project_path.worktree_id == worktree_id {
                        Some(*id)
                    } else {
                        None
                    }
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

    pub fn get_bookmark_by_buffer_id(
        &self,
        buffer_id: BufferId,
        cx: &mut ModelContext<Self>,
    ) -> Vec<Bookmark> {
        self.bookmark_map
            .values()
            .filter_map(|bm| {
                if bm.buffer.read(cx).remote_id() == buffer_id {
                    Some(bm.clone())
                } else {
                    None
                }
            })
            .collect_vec()
    }

    pub fn get_bookmark_by_worktree_id(
        &self,
        worktree_id: WorktreeId,
        cx: &mut ModelContext<Self>,
    ) -> Vec<Bookmark> {
        self.bookmark_map
            .values()
            .filter_map(|bm| {
                if bm.buffer.read(cx).project_path(cx).unwrap().worktree_id == worktree_id {
                    Some(bm.clone())
                } else {
                    None
                }
            })
            .collect_vec()
    }

    pub fn get_bookmark_all(&self) -> Vec<Bookmark> {
        self.bookmark_map
            .values()
            .map(|bm| bm.clone())
            .collect_vec()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::path::Path;

//     #[test]
//     fn test_new() {
//         let bookmark_store = BookmarkStore::new();
//         assert_eq!(bookmark_store.current_id, 0);
//         assert_eq!(bookmark_store.bookmark_ordered.len(), 0);
//         assert_eq!(bookmark_store.bookmark_map, BTreeMap::default());
//         assert_eq!(bookmark_store.next_bookmark_id, 0);
//     }

//     #[test]
//     fn test_toggle() {
//         let mut bookmark_store = BookmarkStore::new();
//         let project_path = ProjectPath {
//             worktree_id: WorktreeId::from_usize(1),
//             path: Path::new("test.rs").into(),
//         };

//         // First toggle
//         bookmark_store.toggle(project_path.clone(), 10, None);
//         assert_eq!(bookmark_store.current_id, 1);
//         assert_eq!(bookmark_store.bookmark_ordered, vec![1]);
//         assert_eq!(
//             bookmark_store.bookmark_map,
//             BTreeMap::from([(
//                 1,
//                 Bookmark {
//                     id: 1,
//                     project_path: project_path.clone(),
//                     line_no: 10,
//                     annotation: None
//                 }
//             )])
//         );

//         // Toggle the same bookmark
//         bookmark_store.toggle(project_path.clone(), 10, None);
//         assert_eq!(bookmark_store.current_id, 0);
//         assert_eq!(bookmark_store.bookmark_map, BTreeMap::default());

//         // Toggle another bookmark
//         bookmark_store.toggle(project_path.clone(), 20, Some("test".into()));
//         assert_eq!(bookmark_store.current_id, 2);
//         assert_eq!(bookmark_store.bookmark_ordered, vec![2]);
//         assert_eq!(
//             bookmark_store.bookmark_map,
//             BTreeMap::from([(
//                 2,
//                 Bookmark {
//                     id: 2,
//                     project_path: project_path.clone(),
//                     line_no: 20,
//                     annotation: Some("test".into())
//                 }
//             )])
//         );
//     }

//     #[test]
//     fn test_find_previous_id() {
//         let mut bookmark_store = BookmarkStore::new();
//         bookmark_store.bookmark_ordered = vec![1, 2, 3];

//         assert_eq!(bookmark_store.find_previous_id(3), Some(2));
//         assert_eq!(bookmark_store.find_previous_id(2), Some(1));
//         assert_eq!(bookmark_store.find_previous_id(1), None);
//     }

//     #[test]
//     fn test_find_next_id() {
//         let mut bookmark_store = BookmarkStore::new();
//         bookmark_store.bookmark_ordered = vec![1, 2, 3];

//         assert_eq!(bookmark_store.find_next_id(1), Some(2));
//         assert_eq!(bookmark_store.find_next_id(2), Some(3));
//         assert_eq!(bookmark_store.find_next_id(3), None);
//     }

//     #[test]
//     fn test_prev() {
//         let mut bookmark_store = BookmarkStore::new();
//         let project_path = ProjectPath {
//             worktree_id: WorktreeId::from_usize(1),
//             path: Path::new("test.rs").into(),
//         };

//         // Add bookmarks
//         bookmark_store.toggle(project_path.clone(), 10, None);
//         bookmark_store.toggle(project_path.clone(), 20, None);
//         bookmark_store.toggle(project_path.clone(), 30, None);

//         // Current ID is 3
//         assert_eq!(bookmark_store.current_id, 3);

//         // Prev to ID 2
//         assert_eq!(
//             bookmark_store.prev(),
//             Some(Bookmark {
//                 id: 2,
//                 project_path: project_path.clone(),
//                 line_no: 20,
//                 annotation: None
//             })
//         );
//         assert_eq!(bookmark_store.current_id, 2);

//         // Prev to ID 1
//         assert_eq!(
//             bookmark_store.prev(),
//             Some(Bookmark {
//                 id: 1,
//                 project_path: project_path.clone(),
//                 line_no: 10,
//                 annotation: None
//             })
//         );
//         assert_eq!(bookmark_store.current_id, 1);

//         // Prev from ID 1
//         assert_eq!(bookmark_store.prev(), None);
//     }

//     #[test]
//     fn test_next() {
//         let mut bookmark_store = BookmarkStore::new();
//         let project_path = ProjectPath {
//             worktree_id: WorktreeId::from_usize(1),
//             path: Path::new("test.rs").into(),
//         };

//         // Add bookmarks
//         bookmark_store.toggle(project_path.clone(), 10, None);
//         bookmark_store.toggle(project_path.clone(), 20, None);
//         bookmark_store.toggle(project_path.clone(), 30, None);

//         // Current ID is 1
//         assert_eq!(bookmark_store.current_id, 3);
//         bookmark_store.prev();
//         bookmark_store.prev();
//         bookmark_store.prev();
//         assert_eq!(bookmark_store.current_id, 1);

//         // Next to ID 2
//         assert_eq!(
//             bookmark_store.next(),
//             Some(Bookmark {
//                 id: 2,
//                 project_path: project_path.clone(),
//                 line_no: 20,
//                 annotation: None
//             })
//         );
//         assert_eq!(bookmark_store.current_id, 2);

//         // Next to ID 3
//         assert_eq!(
//             bookmark_store.next(),
//             Some(Bookmark {
//                 id: 3,
//                 project_path: project_path.clone(),
//                 line_no: 30,
//                 annotation: None
//             })
//         );
//         assert_eq!(bookmark_store.current_id, 3);

//         // Next from ID 3
//         assert_eq!(bookmark_store.next(), None);
//     }

//     #[test]
//     fn test_clear_current_editor() {
//         let mut bookmark_store = BookmarkStore::new();
//         let project_path1 = ProjectPath {
//             worktree_id: WorktreeId::from_usize(1),
//             path: Path::new("test1.rs").into(),
//         };
//         let project_path2 = ProjectPath {
//             worktree_id: WorktreeId::from_usize(1),
//             path: Path::new("test2.rs").into(),
//         };

//         // Add bookmarks
//         bookmark_store.toggle(project_path1.clone(), 10, None);
//         bookmark_store.toggle(project_path2.clone(), 20, None);
//         bookmark_store.toggle(project_path1.clone(), 30, None);

//         // Current ID is 3
//         assert_eq!(bookmark_store.current_id, 3);

//         // Clear bookmarks in project_path1
//         bookmark_store.clear_current_editor(project_path1.clone());
//         assert_eq!(bookmark_store.current_id, 2);
//         assert_eq!(
//             bookmark_store.bookmark_map,
//             BTreeMap::from([(
//                 2,
//                 Bookmark {
//                     id: 2,
//                     project_path: project_path2.clone(),
//                     line_no: 20,
//                     annotation: None
//                 }
//             )])
//         );
//     }

//     #[test]
//     fn test_clear_current_worktree() {
//         let mut bookmark_store = BookmarkStore::new();
//         let worktree_id1 = WorktreeId::from_usize(1);
//         let worktree_id2 = WorktreeId::from_usize(2);
//         let project_path1 = ProjectPath {
//             worktree_id: worktree_id1,
//             path: Path::new("test1.rs").into(),
//         };
//         let project_path2 = ProjectPath {
//             worktree_id: worktree_id2,
//             path: Path::new("test2.rs").into(),
//         };

//         // Add bookmarks
//         bookmark_store.toggle(project_path1.clone(), 10, None);
//         bookmark_store.toggle(project_path2.clone(), 20, None);
//         bookmark_store.toggle(project_path1.clone(), 30, None);

//         // Current ID is 3
//         assert_eq!(bookmark_store.current_id, 3);

//         // Clear bookmarks in worktree_id1
//         bookmark_store.clear_current_worktree(worktree_id1);
//         assert_eq!(bookmark_store.current_id, 2);
//         assert_eq!(
//             bookmark_store.bookmark_map,
//             BTreeMap::from([(
//                 2,
//                 Bookmark {
//                     id: 2,
//                     project_path: project_path2.clone(),
//                     line_no: 20,
//                     annotation: None
//                 }
//             )])
//         );
//     }

//     #[test]
//     fn test_clear_all() {
//         let mut bookmark_store = BookmarkStore::new();
//         let project_path = ProjectPath {
//             worktree_id: WorktreeId::from_usize(1),
//             path: Path::new("test.rs").into(),
//         };

//         // Add bookmarks
//         bookmark_store.toggle(project_path.clone(), 10, None);
//         bookmark_store.toggle(project_path.clone(), 20, None);
//         bookmark_store.toggle(project_path.clone(), 30, None);

//         // Clear all bookmarks
//         bookmark_store.clear_all();
//         assert_eq!(bookmark_store.current_id, 0);
//         assert_eq!(bookmark_store.bookmark_ordered.len(), 0);
//         assert_eq!(bookmark_store.bookmark_map, BTreeMap::default());
//     }

//     #[test]
//     fn test_get_current_editor() {
//         let mut bookmark_store = BookmarkStore::new();
//         let project_path1 = ProjectPath {
//             worktree_id: WorktreeId::from_usize(1),
//             path: Path::new("test1.rs").into(),
//         };
//         let project_path2 = ProjectPath {
//             worktree_id: WorktreeId::from_usize(1),
//             path: Path::new("test2.rs").into(),
//         };

//         // Add bookmarks
//         bookmark_store.toggle(project_path1.clone(), 10, None);
//         bookmark_store.toggle(project_path2.clone(), 20, None);
//         bookmark_store.toggle(project_path1.clone(), 30, None);

//         // Get bookmarks in project_path1
//         let bookmarks = bookmark_store.get_current_editor(project_path1.clone());
//         assert_eq!(bookmarks.len(), 2);
//         assert_eq!(
//             bookmarks,
//             vec![
//                 Bookmark {
//                     id: 1,
//                     project_path: project_path1.clone(),
//                     line_no: 10,
//                     annotation: None
//                 },
//                 Bookmark {
//                     id: 3,
//                     project_path: project_path1.clone(),
//                     line_no: 30,
//                     annotation: None
//                 }
//             ]
//         );
//     }

//     #[test]
//     fn test_get_current_worktree() {
//         let mut bookmark_store = BookmarkStore::new();
//         let worktree_id1 = WorktreeId::from_usize(1);
//         let worktree_id2 = WorktreeId::from_usize(2);
//         let project_path1 = ProjectPath {
//             worktree_id: worktree_id1,
//             path: Path::new("test1.rs").into(),
//         };
//         let project_path2 = ProjectPath {
//             worktree_id: worktree_id2,
//             path: Path::new("test2.rs").into(),
//         };

//         // Add bookmarks
//         bookmark_store.toggle(project_path1.clone(), 10, None);
//         bookmark_store.toggle(project_path2.clone(), 20, None);
//         bookmark_store.toggle(project_path1.clone(), 30, None);

//         // Get bookmarks in worktree_id1
//         let bookmarks = bookmark_store.get_current_worktree(worktree_id1);
//         assert_eq!(bookmarks.len(), 2);
//         assert_eq!(
//             bookmarks,
//             vec![
//                 Bookmark {
//                     id: 1,
//                     project_path: project_path1.clone(),
//                     line_no: 10,
//                     annotation: None
//                 },
//                 Bookmark {
//                     id: 3,
//                     project_path: project_path1.clone(),
//                     line_no: 30,
//                     annotation: None
//                 }
//             ]
//         );
//     }

//     #[test]
//     fn test_get_all() {
//         let mut bookmark_store = BookmarkStore::new();
//         let project_path1 = ProjectPath {
//             worktree_id: WorktreeId::from_usize(1),
//             path: Path::new("test1.rs").into(),
//         };
//         let project_path2 = ProjectPath {
//             worktree_id: WorktreeId::from_usize(1),
//             path: Path::new("test2.rs").into(),
//         };

//         // Add bookmarks
//         bookmark_store.toggle(project_path1.clone(), 10, None);
//         bookmark_store.toggle(project_path2.clone(), 20, None);
//         bookmark_store.toggle(project_path1.clone(), 30, None);

//         // Get all bookmarks
//         let bookmarks = bookmark_store.get_all();
//         assert_eq!(bookmarks.len(), 3);
//         assert_eq!(
//             bookmarks,
//             vec![
//                 Bookmark {
//                     id: 1,
//                     project_path: project_path1.clone(),
//                     line_no: 10,
//                     annotation: None
//                 },
//                 Bookmark {
//                     id: 2,
//                     project_path: project_path2.clone(),
//                     line_no: 20,
//                     annotation: None
//                 },
//                 Bookmark {
//                     id: 3,
//                     project_path: project_path1.clone(),
//                     line_no: 30,
//                     annotation: None
//                 }
//             ]
//         );
//     }
// }
