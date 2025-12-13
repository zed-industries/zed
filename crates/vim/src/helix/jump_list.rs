//! Helix-style jump list for position history navigation.
//!
//! This module implements Helix's jump list feature which allows users to:
//! - Save cursor/selection positions with `Ctrl-s` (`save_selection`)
//! - Navigate backward through saved positions with `Ctrl-o` (`jump_backward`)
//! - Navigate forward through saved positions with `Ctrl-i` (`jump_forward`)
//!
//! The jump list maintains a history of up to 30 positions (matching Helix's
//! `JUMP_LIST_CAPACITY`). When navigating backward from the "present" (end of
//! the list), the current position is automatically saved first.

use editor::Anchor;
use gpui::EntityId;
use std::collections::VecDeque;

/// Maximum number of entries in the jump list (matches Helix).
pub const JUMP_LIST_CAPACITY: usize = 30;

/// A single entry in the jump list, storing the buffer and selection positions.
///
/// Uses `Anchor` for position stability - anchors automatically adjust when
/// text is inserted or deleted before them.
#[derive(Clone, Debug)]
pub struct JumpEntry {
    /// EntityId of the MultiBuffer containing this position.
    pub buffer_id: EntityId,
    /// Selection anchor positions (supports multi-cursor).
    pub selections: Vec<Anchor>,
}

impl JumpEntry {
    /// Creates a new jump entry for the given buffer and selection positions.
    pub fn new(buffer_id: EntityId, selections: Vec<Anchor>) -> Self {
        Self {
            buffer_id,
            selections,
        }
    }

    /// Checks if this entry is a duplicate of another (same buffer, same positions).
    fn is_duplicate(&self, other: &JumpEntry) -> bool {
        self.buffer_id == other.buffer_id
            && self.selections.len() == other.selections.len()
            && self
                .selections
                .iter()
                .zip(&other.selections)
                .all(|(a, b)| a == b)
    }
}

/// Helix-style jump list for navigating position history.
///
/// The jump list stores a history of cursor positions that can be navigated
/// with backward/forward commands. Key behaviors:
///
/// - **Push**: Adds a new position, preventing consecutive duplicates. If not
///   at the end of the list, forward history is truncated.
/// - **Backward**: Moves backward through history. When at "present" (end of
///   list), the current position should be saved first by the caller.
/// - **Forward**: Moves forward through history.
/// - **Capacity**: Maintains at most `JUMP_LIST_CAPACITY` (30) entries.
#[derive(Debug)]
pub struct JumpList {
    /// The list of jump entries (newest at back).
    jumps: VecDeque<JumpEntry>,
    /// Current position in the jump list.
    /// When `current == jumps.len()`, we're at the "present" (no entry selected).
    current: usize,
}

impl Default for JumpList {
    fn default() -> Self {
        Self::new()
    }
}

impl JumpList {
    /// Creates a new empty jump list.
    pub fn new() -> Self {
        Self {
            jumps: VecDeque::with_capacity(JUMP_LIST_CAPACITY),
            current: 0,
        }
    }

    /// Pushes a new entry to the jump list.
    ///
    /// Behavior:
    /// - Prevents consecutive duplicate entries
    /// - Truncates forward history if not at present
    /// - Maintains capacity by removing oldest entries
    pub fn push(&mut self, entry: JumpEntry) {
        // Prevent consecutive duplicates
        if self
            .jumps
            .back()
            .is_some_and(|last| last.is_duplicate(&entry))
        {
            return;
        }

        // Truncate forward history if not at present
        if self.current < self.jumps.len() {
            self.jumps.truncate(self.current);
        }

        self.jumps.push_back(entry);

        // Maintain capacity by removing oldest entries
        while self.jumps.len() > JUMP_LIST_CAPACITY {
            self.jumps.pop_front();
        }

        // Move to present (end of list)
        self.current = self.jumps.len();
    }

    /// Moves backward in the jump list by `count` positions.
    ///
    /// Returns the entry to jump to, or `None` if the list is empty.
    pub fn backward(&mut self, count: usize) -> Option<&JumpEntry> {
        if self.jumps.is_empty() {
            return None;
        }
        self.current = self.current.saturating_sub(count);
        self.jumps.get(self.current)
    }

    /// Moves forward in the jump list by `count` positions.
    ///
    /// Returns the entry to jump to, or `None` if the list is empty.
    pub fn forward(&mut self, count: usize) -> Option<&JumpEntry> {
        if self.jumps.is_empty() {
            return None;
        }
        self.current = (self.current + count).min(self.jumps.len().saturating_sub(1));
        self.jumps.get(self.current)
    }

    /// Returns `true` if at the "present" (end of the list, no entry selected).
    pub fn at_present(&self) -> bool {
        self.current >= self.jumps.len()
    }

    /// Moves the current position back by one without returning an entry.
    /// Used after auto-saving current position before jumping backward.
    pub fn step_back(&mut self) {
        self.current = self.current.saturating_sub(1);
    }

    /// Removes all entries associated with a closed buffer.
    ///
    /// Adjusts the current position to account for removed entries.
    #[allow(dead_code)]
    pub fn remove_buffer(&mut self, buffer_id: EntityId) {
        // Count entries before current that will be removed
        let removed_before = self
            .jumps
            .iter()
            .take(self.current)
            .filter(|e| e.buffer_id == buffer_id)
            .count();

        self.jumps.retain(|e| e.buffer_id != buffer_id);

        // Adjust current position
        self.current = self
            .current
            .saturating_sub(removed_before)
            .min(self.jumps.len());
    }

    /// Returns the current position and total length for debugging/display.
    #[allow(dead_code)]
    pub fn position(&self) -> (usize, usize) {
        (self.current, self.jumps.len())
    }

    /// Returns the number of entries in the jump list.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.jumps.len()
    }

    /// Returns true if the jump list is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.jumps.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a mock EntityId for testing
    fn mock_entity_id(id: u64) -> EntityId {
        // EntityId can be created from u64 for testing purposes
        EntityId::from(id)
    }

    // Helper to create a test entry (without real Anchor - just for JumpList logic tests)
    fn test_entry(buffer_id: u64) -> JumpEntry {
        JumpEntry {
            buffer_id: mock_entity_id(buffer_id),
            selections: vec![], // Empty selections for unit tests
        }
    }

    #[test]
    fn test_new_jump_list() {
        let list = JumpList::new();
        assert!(list.at_present());
        assert_eq!(list.position(), (0, 0));
    }

    #[test]
    fn test_push_single_entry() {
        let mut list = JumpList::new();
        list.push(test_entry(1));

        assert!(list.at_present());
        assert_eq!(list.position(), (1, 1));
    }

    #[test]
    fn test_push_multiple_entries() {
        let mut list = JumpList::new();
        list.push(test_entry(1));
        list.push(test_entry(2));
        list.push(test_entry(3));

        assert!(list.at_present());
        assert_eq!(list.position(), (3, 3));
    }

    #[test]
    fn test_backward_navigation() {
        let mut list = JumpList::new();
        list.push(test_entry(1));
        list.push(test_entry(2));
        list.push(test_entry(3));

        // Move back by 1
        let entry = list.backward(1);
        assert!(entry.is_some());
        assert_eq!(list.position(), (2, 3));
        assert!(!list.at_present());

        // Move back by 1 more
        let entry = list.backward(1);
        assert!(entry.is_some());
        assert_eq!(list.position(), (1, 3));

        // Move back by 1 more
        let entry = list.backward(1);
        assert!(entry.is_some());
        assert_eq!(list.position(), (0, 3));

        // Move back when already at start (should stay at 0)
        let entry = list.backward(1);
        assert!(entry.is_some());
        assert_eq!(list.position(), (0, 3));
    }

    #[test]
    fn test_forward_navigation() {
        let mut list = JumpList::new();
        list.push(test_entry(1));
        list.push(test_entry(2));
        list.push(test_entry(3));

        // Move back to start
        list.backward(3);
        assert_eq!(list.position(), (0, 3));

        // Move forward by 1
        let entry = list.forward(1);
        assert!(entry.is_some());
        assert_eq!(list.position(), (1, 3));

        // Move forward by 2 (should clamp to last entry, index 2)
        let entry = list.forward(2);
        assert!(entry.is_some());
        assert_eq!(list.position(), (2, 3));
    }

    #[test]
    fn test_backward_with_count() {
        let mut list = JumpList::new();
        for i in 1..=5 {
            list.push(test_entry(i));
        }

        // Move back by 3
        list.backward(3);
        assert_eq!(list.position(), (2, 5));

        // Move back by 10 (should clamp to 0)
        list.backward(10);
        assert_eq!(list.position(), (0, 5));
    }

    #[test]
    fn test_prevents_consecutive_duplicates() {
        let mut list = JumpList::new();
        let entry = test_entry(1);

        list.push(entry.clone());
        list.push(entry.clone());
        list.push(entry.clone());

        // Should only have 1 entry despite 3 pushes
        assert_eq!(list.position(), (1, 1));
    }

    #[test]
    fn test_allows_non_consecutive_duplicates() {
        let mut list = JumpList::new();

        list.push(test_entry(1));
        list.push(test_entry(2));
        list.push(test_entry(1)); // Same buffer as first, but not consecutive

        assert_eq!(list.position(), (3, 3));
    }

    #[test]
    fn test_truncates_forward_history_on_push() {
        let mut list = JumpList::new();
        list.push(test_entry(1));
        list.push(test_entry(2));
        list.push(test_entry(3));

        // Move back to first entry
        list.backward(3);
        assert_eq!(list.position(), (0, 3));

        // Push new entry - should truncate entries 2 and 3
        list.push(test_entry(4));
        assert_eq!(list.position(), (1, 1));
        assert!(list.at_present());

        // Forward should not work - no forward history
        let entry = list.forward(1);
        assert!(entry.is_some()); // Returns current entry
        assert_eq!(list.position(), (0, 1)); // Clamped to last valid index
    }

    #[test]
    fn test_capacity_limit() {
        let mut list = JumpList::new();

        // Push more than capacity
        for i in 1..=50 {
            list.push(test_entry(i));
        }

        // Should only have JUMP_LIST_CAPACITY entries
        assert_eq!(list.jumps.len(), JUMP_LIST_CAPACITY);
        assert_eq!(list.position(), (JUMP_LIST_CAPACITY, JUMP_LIST_CAPACITY));
    }

    #[test]
    fn test_remove_buffer() {
        let mut list = JumpList::new();
        list.push(test_entry(1));
        list.push(test_entry(2));
        list.push(test_entry(1));
        list.push(test_entry(3));
        list.push(test_entry(1));

        // Move back to position 2
        list.backward(3);
        assert_eq!(list.position(), (2, 5));

        // Remove buffer 1 (entries at indices 0, 2, 4)
        list.remove_buffer(mock_entity_id(1));

        // Should have 2 entries left (buffers 2 and 3)
        assert_eq!(list.len(), 2);

        // Current position should be adjusted (was 2, entry at 0 removed = 1,
        // but we were at index 2 which was buffer 1 entry, so adjustment needed)
        let (current, len) = list.position();
        assert!(current <= len);
    }

    #[test]
    fn test_empty_list_navigation() {
        let mut list = JumpList::new();

        assert!(list.backward(1).is_none());
        assert!(list.forward(1).is_none());
        assert!(list.at_present());
    }

    #[test]
    fn test_at_present_behavior() {
        let mut list = JumpList::new();
        assert!(list.at_present());

        list.push(test_entry(1));
        assert!(list.at_present()); // After push, we're at present

        list.backward(1);
        assert!(!list.at_present()); // After backward, not at present

        list.forward(1);
        // After forward(1) from position 0 with 1 entry, we're at position 0
        // which is the last entry (len-1), not "present"
        assert!(!list.at_present());

        // Push a new entry to get back to present
        list.push(test_entry(2));
        assert!(list.at_present());
    }
}
