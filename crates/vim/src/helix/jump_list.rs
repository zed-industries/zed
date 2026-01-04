//! Helix-style jump list for position history navigation.
//!
//! Implements Helix's jump list: save positions with `Ctrl-s`, navigate back with
//! `Ctrl-o`, forward with `Ctrl-i`. Stores full selection ranges (not just cursor)
//! to match Helix's `type Jump = (DocumentId, Selection)`.
//!
//! When a buffer closes, entries convert from anchors to file paths with points,
//! allowing the file to be reopened on jump.

use editor::Anchor;
use gpui::EntityId;
use language::Point;
use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

pub const JUMP_LIST_CAPACITY: usize = 30;

/// Full selection range with start and end anchors.
#[derive(Clone, Debug, PartialEq)]
pub struct SelectionAnchors {
    pub start: Anchor,
    pub end: Anchor,
}

impl SelectionAnchors {
    pub fn new(start: Anchor, end: Anchor) -> Self {
        Self { start, end }
    }
}

/// Selection range as static points (for closed buffers).
#[derive(Clone, Debug, PartialEq)]
pub struct SelectionPoints {
    pub start: Point,
    pub end: Point,
}

impl SelectionPoints {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

/// Location of a jump entry - open buffer or file path.
#[derive(Clone, Debug)]
pub enum JumpLocation {
    Buffer(EntityId),
    Path(Arc<Path>),
}

/// Selection data - anchors for open buffers, points for closed files.
#[derive(Clone, Debug)]
pub enum JumpSelections {
    Anchors(Vec<SelectionAnchors>),
    Points(Vec<SelectionPoints>),
}

/// A single entry in the jump list.
#[derive(Clone, Debug)]
pub struct JumpEntry {
    pub location: JumpLocation,
    pub selections: JumpSelections,
}

impl JumpEntry {
    pub fn new(buffer_id: EntityId, selections: Vec<SelectionAnchors>) -> Self {
        Self {
            location: JumpLocation::Buffer(buffer_id),
            selections: JumpSelections::Anchors(selections),
        }
    }

    pub fn new_path(path: Arc<Path>, selections: Vec<SelectionPoints>) -> Self {
        Self {
            location: JumpLocation::Path(path),
            selections: JumpSelections::Points(selections),
        }
    }

    pub fn buffer_id(&self) -> Option<EntityId> {
        match &self.location {
            JumpLocation::Buffer(id) => Some(*id),
            JumpLocation::Path(_) => None,
        }
    }

    pub fn path(&self) -> Option<&Arc<Path>> {
        match &self.location {
            JumpLocation::Buffer(_) => None,
            JumpLocation::Path(path) => Some(path),
        }
    }

    pub fn is_buffer(&self, buffer_id: EntityId) -> bool {
        matches!(&self.location, JumpLocation::Buffer(id) if *id == buffer_id)
    }

    fn is_duplicate(&self, other: &JumpEntry) -> bool {
        match (&self.location, &other.location) {
            (JumpLocation::Buffer(a), JumpLocation::Buffer(b)) if a == b => {
                self.selections_equal(&other.selections)
            }
            (JumpLocation::Path(a), JumpLocation::Path(b)) if a == b => {
                self.selections_equal(&other.selections)
            }
            _ => false,
        }
    }

    fn selections_equal(&self, other: &JumpSelections) -> bool {
        match (&self.selections, other) {
            (JumpSelections::Anchors(a), JumpSelections::Anchors(b)) => a == b,
            (JumpSelections::Points(a), JumpSelections::Points(b)) => a == b,
            _ => false,
        }
    }
}

/// Helix-style jump list for navigating position history.
///
/// The list uses a cursor (`current`) to track position. When `current == jumps.len()`,
/// we're at "present" - the live editor state, not a stored position. You can only
/// return to present by pushing a new entry; forward navigation stops at the last entry.
#[derive(Debug)]
pub struct JumpList {
    jumps: VecDeque<JumpEntry>,
    current: usize,
}

impl Default for JumpList {
    fn default() -> Self {
        Self::new()
    }
}

impl JumpList {
    pub fn new() -> Self {
        Self {
            jumps: VecDeque::with_capacity(JUMP_LIST_CAPACITY),
            current: 0,
        }
    }

    pub fn push(&mut self, entry: JumpEntry) {
        if self
            .jumps
            .back()
            .is_some_and(|last| last.is_duplicate(&entry))
        {
            return;
        }

        if self.current < self.jumps.len() {
            self.jumps.truncate(self.current);
        }

        self.jumps.push_back(entry);

        while self.jumps.len() > JUMP_LIST_CAPACITY {
            self.jumps.pop_front();
        }

        self.current = self.jumps.len();
    }

    pub fn backward(&mut self, count: usize) -> Option<&JumpEntry> {
        if self.jumps.is_empty() {
            return None;
        }
        self.current = self.current.saturating_sub(count);
        self.jumps.get(self.current)
    }

    /// Forward stops at the last entry - cannot navigate past it to "present".
    pub fn forward(&mut self, count: usize) -> Option<&JumpEntry> {
        if self.jumps.is_empty() {
            return None;
        }
        self.current = (self.current + count).min(self.jumps.len().saturating_sub(1));
        self.jumps.get(self.current)
    }

    pub fn at_present(&self) -> bool {
        self.current >= self.jumps.len()
    }

    pub fn step_back(&mut self) {
        self.current = self.current.saturating_sub(1);
    }

    pub fn remove_buffer(&mut self, buffer_id: EntityId) {
        let removed_before = self
            .jumps
            .iter()
            .take(self.current)
            .filter(|e| e.is_buffer(buffer_id))
            .count();

        self.jumps.retain(|e| !e.is_buffer(buffer_id));

        self.current = self
            .current
            .saturating_sub(removed_before)
            .min(self.jumps.len());
    }

    pub fn convert_buffer_to_path<F>(&mut self, buffer_id: EntityId, path: Arc<Path>, convert_fn: F)
    where
        F: Fn(&SelectionAnchors) -> SelectionPoints,
    {
        for entry in &mut self.jumps {
            if entry.is_buffer(buffer_id) {
                if let JumpSelections::Anchors(anchors) = &entry.selections {
                    let points: Vec<SelectionPoints> = anchors.iter().map(&convert_fn).collect();
                    entry.location = JumpLocation::Path(path.clone());
                    entry.selections = JumpSelections::Points(points);
                }
            }
        }
    }

    pub fn convert_path_to_buffer<F>(&mut self, path: &Path, buffer_id: EntityId, convert_fn: F)
    where
        F: Fn(&SelectionPoints) -> SelectionAnchors,
    {
        for entry in &mut self.jumps {
            if let JumpLocation::Path(entry_path) = &entry.location {
                if entry_path.as_ref() == path {
                    if let JumpSelections::Points(points) = &entry.selections {
                        let anchors: Vec<SelectionAnchors> =
                            points.iter().map(&convert_fn).collect();
                        entry.location = JumpLocation::Buffer(buffer_id);
                        entry.selections = JumpSelections::Anchors(anchors);
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn position(&self) -> (usize, usize) {
        (self.current, self.jumps.len())
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.jumps.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.jumps.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entity(id: u64) -> EntityId {
        EntityId::from(id)
    }

    fn entry(buffer_id: u64) -> JumpEntry {
        JumpEntry::new(entity(buffer_id), vec![])
    }

    fn path_entry(path: &str) -> JumpEntry {
        JumpEntry::new_path(Arc::from(Path::new(path)), vec![])
    }

    #[test]
    fn new_list_is_at_present() {
        let list = JumpList::new();
        assert!(list.at_present());
        assert_eq!(list.position(), (0, 0));
    }

    #[test]
    fn push_keeps_at_present() {
        let mut list = JumpList::new();
        list.push(entry(1));
        assert!(list.at_present());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn backward_navigation() {
        let mut list = JumpList::new();
        list.push(entry(1));
        list.push(entry(2));
        list.push(entry(3));

        list.backward(1);
        assert_eq!(list.position(), (2, 3));
        assert!(!list.at_present());

        list.backward(2);
        assert_eq!(list.position(), (0, 3));

        list.backward(1);
        assert_eq!(list.position(), (0, 3)); // clamped
    }

    #[test]
    fn forward_navigation() {
        let mut list = JumpList::new();
        list.push(entry(1));
        list.push(entry(2));
        list.push(entry(3));

        list.backward(3);
        assert_eq!(list.position(), (0, 3));

        list.forward(1);
        assert_eq!(list.position(), (1, 3));

        list.forward(10);
        assert_eq!(list.position(), (2, 3)); // clamped to last entry
        assert!(!list.at_present()); // forward never reaches present
    }

    #[test]
    fn prevents_consecutive_duplicates() {
        let mut list = JumpList::new();
        list.push(entry(1));
        list.push(entry(1));
        list.push(entry(1));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn allows_non_consecutive_duplicates() {
        let mut list = JumpList::new();
        list.push(entry(1));
        list.push(entry(2));
        list.push(entry(1));
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn truncates_forward_history_on_push() {
        let mut list = JumpList::new();
        list.push(entry(1));
        list.push(entry(2));
        list.push(entry(3));

        list.backward(3);
        list.push(entry(4));

        assert_eq!(list.len(), 1);
        assert!(list.at_present());
    }

    #[test]
    fn respects_capacity() {
        let mut list = JumpList::new();
        for i in 1..=50 {
            list.push(entry(i));
        }
        assert_eq!(list.len(), JUMP_LIST_CAPACITY);
    }

    #[test]
    fn remove_buffer() {
        let mut list = JumpList::new();
        list.push(entry(1));
        list.push(entry(2));
        list.push(entry(1));

        list.remove_buffer(entity(1));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn empty_list_navigation() {
        let mut list = JumpList::new();
        assert!(list.backward(1).is_none());
        assert!(list.forward(1).is_none());
    }

    #[test]
    fn path_entries() {
        let mut list = JumpList::new();
        list.push(path_entry("/file.rs"));
        list.push(entry(1));
        list.push(path_entry("/other.rs"));

        let e = list.backward(1).unwrap();
        assert!(e.path().is_some());

        let e = list.backward(1).unwrap();
        assert!(e.buffer_id().is_some());
    }

    #[test]
    fn convert_buffer_to_path() {
        let mut list = JumpList::new();
        list.push(entry(1));
        list.push(entry(2));

        let path = Arc::from(Path::new("/file.rs"));
        list.convert_buffer_to_path(entity(1), path, |_| {
            SelectionPoints::new(Point::new(0, 0), Point::new(0, 0))
        });

        list.backward(2);
        assert!(list.jumps.get(0).unwrap().path().is_some());
        assert!(list.jumps.get(1).unwrap().buffer_id().is_some());
    }

    #[test]
    fn convert_path_to_buffer() {
        let mut list = JumpList::new();
        list.push(path_entry("/file.rs"));

        list.convert_path_to_buffer(Path::new("/file.rs"), entity(3), |_| SelectionAnchors {
            start: Anchor::min(),
            end: Anchor::min(),
        });

        assert_eq!(list.jumps.get(0).unwrap().buffer_id(), Some(entity(3)));
    }

    #[test]
    fn path_duplicate_detection() {
        let mut list = JumpList::new();
        list.push(path_entry("/file.rs"));
        list.push(path_entry("/file.rs"));
        assert_eq!(list.len(), 1);

        list.push(path_entry("/other.rs"));
        assert_eq!(list.len(), 2);
    }
}
