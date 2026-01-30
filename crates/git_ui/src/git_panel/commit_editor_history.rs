use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

const MAX_HISTORY: usize = 50;

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct SerializedCommitEditorHistory {
    entries: Vec<String>,
}

#[derive(Debug, Default)]
pub(crate) struct CommitEditorHistory {
    entries: VecDeque<String>,
    cursor: Option<usize>, // index into entries VecDeque (0..len-1), the greater the index, the older is an item; None means pointing before the first entry
    pending_edit: String,
}

impl CommitEditorHistory {
    pub fn add_new_entry(&mut self, message: String) {
        if message.is_empty() {
            panic!("Empty commit message is not allowed in commit history"); // relying on "commit changes" checks logic
        }

        // remove any exact duplicate
        if let Some(pos) = self.entries.iter().position(|m| m == &message) {
            self.entries.remove(pos);
        }

        self.entries.push_front(message);

        while self.entries.len() > MAX_HISTORY {
            self.entries.truncate(MAX_HISTORY);
        }

        self.cursor = None;
    }

    pub fn prev(&mut self) -> Option<&str> {
        match self.cursor {
            Some(cursor) => {
                if cursor < self.entries.len() - 1 {
                    self.cursor = Some(cursor + 1);
                } else {
                    return None;
                }
            }
            None => {
                if self.entries.is_empty() {
                    return None;
                } else {
                    self.cursor = Some(0usize);
                }
            }
        }

        return self
            .entries
            .get(
                self.cursor
                    .expect("History must contain at least one entry"),
            )
            .map(|s| s.as_str());
    }

    pub fn next(&mut self) -> Option<&str> {
        match self.cursor {
            Some(cursor) => {
                if cursor > 0 {
                    self.cursor = Some(cursor - 1);
                    self.entries
                        .get(
                            self.cursor
                                .expect("History must contain at least one entry"),
                        )
                        .map(|s| s.as_str())
                } else {
                    self.cursor = None;
                    None
                }
            }
            None => None,
        }
    }

    pub fn get_pending_edit(&self) -> &str {
        self.pending_edit.as_str()
    }

    pub fn set_pending_edit(&mut self, message: String) {
        self.pending_edit = message;
        self.cursor = None;
    }

    pub fn to_serialized(&self) -> SerializedCommitEditorHistory {
        let mut entries = Vec::with_capacity(MAX_HISTORY);
        let (front, back) = self.entries.as_slices();
        entries.extend_from_slice(front);
        entries.extend_from_slice(back);

        SerializedCommitEditorHistory { entries }
    }

    pub fn from_serialized(serialized: SerializedCommitEditorHistory) -> Self {
        Self {
            entries: VecDeque::from_iter(serialized.entries),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_history_is_empty() {
        let history = CommitEditorHistory::default();
        assert_eq!(history.entries.len(), 0);
        assert!(history.cursor.is_none());
        assert_eq!(history.pending_edit, "");
    }

    #[test]
    fn test_max_history_limit() {
        let mut history = CommitEditorHistory::default();

        // Add more than MAX_HISTORY entries
        for i in 0..60 {
            history.add_new_entry(format!("Commit {}", i));
        }

        assert_eq!(history.entries.len(), MAX_HISTORY);
        assert_eq!(history.entries[0], "Commit 59");
        assert_eq!(history.entries[MAX_HISTORY - 1], "Commit 10");
    }

    #[test]
    fn test_prev_on_empty_history() {
        let mut history = CommitEditorHistory::default();
        assert!(history.prev().is_none());
        assert!(history.cursor.is_none());
    }

    #[test]
    fn test_prev_navigates_through_history() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("First".to_string());
        history.add_new_entry("Second".to_string());
        history.add_new_entry("Third".to_string());

        assert_eq!(history.prev(), Some("Third"));
        assert_eq!(history.cursor, Some(0));

        assert_eq!(history.prev(), Some("Second"));
        assert_eq!(history.cursor, Some(1));

        assert_eq!(history.prev(), Some("First"));
        assert_eq!(history.cursor, Some(2));
    }

    #[test]
    fn test_prev_at_end_of_history() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("First".to_string());
        history.add_new_entry("Second".to_string());

        history.prev();
        history.prev();

        // Try to go past the end
        assert!(history.prev().is_none());
        assert_eq!(history.cursor, Some(1)); // Cursor should not change
    }

    #[test]
    fn test_next_on_empty_history() {
        let mut history = CommitEditorHistory::default();
        assert!(history.next().is_none());
        assert!(history.cursor.is_none());
    }

    #[test]
    fn test_next_beyond_beginning() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("First".to_string());
        history.add_new_entry("Second".to_string());

        assert!(history.next().is_none());
        assert!(history.cursor.is_none());
    }

    #[test]
    fn test_next_navigates_forward() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("First".to_string());
        history.add_new_entry("Second".to_string());
        history.add_new_entry("Third".to_string());

        history.prev();
        history.prev();
        history.prev();

        assert_eq!(history.next(), Some("Second"));
        assert_eq!(history.cursor, Some(1));

        assert_eq!(history.next(), Some("Third"));
        assert_eq!(history.cursor, Some(0));
    }

    #[test]
    fn test_next_returns_to_beginning() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("First".to_string());
        history.add_new_entry("Second".to_string());

        history.prev();
        history.prev();

        history.next();
        assert_eq!(history.next(), None);
        assert!(history.cursor.is_none());
    }

    #[test]
    fn test_prev_next_combination() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("A".to_string());
        history.add_new_entry("B".to_string());
        history.add_new_entry("C".to_string());

        assert_eq!(history.prev(), Some("C"));
        assert_eq!(history.prev(), Some("B"));
        assert_eq!(history.next(), Some("C"));
        assert_eq!(history.prev(), Some("B"));
        assert_eq!(history.prev(), Some("A"));
        assert_eq!(history.next(), Some("B"));
        assert_eq!(history.next(), Some("C"));
        assert_eq!(history.next(), None);
    }

    #[test]
    fn test_single_entry_navigation() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("Only entry".to_string());

        assert_eq!(history.prev(), Some("Only entry"));
        assert!(history.prev().is_none()); // Can't go further
        assert_eq!(history.next(), None); // Back to start
        assert!(history.next().is_none()); // Already at start
    }
    #[test]
    fn test_pending_edit_default() {
        let history = CommitEditorHistory::default();
        assert_eq!(history.get_pending_edit(), "");
    }

    #[test]
    fn test_set_and_get_pending_edit() {
        let mut history = CommitEditorHistory::default();
        history.set_pending_edit("Work in progress".to_string());

        assert_eq!(history.get_pending_edit(), "Work in progress");
    }

    #[test]
    fn test_set_pending_edit_resets_cursor() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("First".to_string());
        history.add_new_entry("Second".to_string());

        history.prev();
        assert_eq!(history.cursor, Some(0));

        history.set_pending_edit("New edit".to_string());
        assert!(history.cursor.is_none());
    }

    #[test]
    #[should_panic(expected = "Empty commit message is not allowed in commit history")]
    fn test_add_empty_message_panics() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("".to_string());
    }

    #[test]
    fn test_add_multiple_entries() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("First commit".to_string());
        history.add_new_entry("Second commit".to_string());
        history.add_new_entry("Third commit".to_string());

        assert_eq!(history.entries.len(), 3);
        assert_eq!(history.entries[0], "Third commit");
        assert_eq!(history.entries[1], "Second commit");
        assert_eq!(history.entries[2], "First commit");
    }

    #[test]
    fn test_add_duplicate_removes_old_entry() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("First commit".to_string());
        history.add_new_entry("Second commit".to_string());
        history.add_new_entry("Third commit".to_string());
        history.add_new_entry("Second commit".to_string());

        assert_eq!(history.entries.len(), 3);
        assert_eq!(history.entries[0], "Second commit");
        assert_eq!(history.entries[1], "Third commit");
        assert_eq!(history.entries[2], "First commit");
    }
    #[test]
    fn test_add_new_entry_resets_cursor() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("First".to_string());
        history.add_new_entry("Second".to_string());

        history.prev();
        assert_eq!(history.cursor, Some(0));

        history.add_new_entry("Third".to_string());
        assert!(history.cursor.is_none());
    }

    #[test]
    fn test_navigation_after_add() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("First".to_string());

        history.prev();
        assert_eq!(history.cursor, Some(0));

        // Adding new entry should reset cursor
        history.add_new_entry("Second".to_string());
        assert!(history.cursor.is_none());

        // Should be able to navigate again
        assert_eq!(history.prev(), Some("Second"));
    }
    #[test]
    fn test_serialization_with_entries() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("First".to_string());
        history.add_new_entry("Second".to_string());
        history.add_new_entry("Third".to_string());

        let serialized = history.to_serialized();

        assert_eq!(serialized.entries.len(), 3);
        assert_eq!(serialized.entries[0], "Third");
        assert_eq!(serialized.entries[1], "Second");
        assert_eq!(serialized.entries[2], "First");
    }

    #[test]
    fn test_deserialization_empty() {
        let serialized = SerializedCommitEditorHistory { entries: vec![] };

        let history = CommitEditorHistory::from_serialized(serialized);

        assert_eq!(history.entries.len(), 0);
        assert!(history.cursor.is_none());
        assert_eq!(history.pending_edit, "");
    }

    #[test]
    fn test_deserialization_with_entries() {
        let serialized = SerializedCommitEditorHistory {
            entries: vec![
                "Third".to_string(),
                "Second".to_string(),
                "First".to_string(),
            ],
        };

        let history = CommitEditorHistory::from_serialized(serialized);

        assert_eq!(history.entries.len(), 3);
        assert_eq!(history.entries[0], "Third");
        assert_eq!(history.entries[1], "Second");
        assert_eq!(history.entries[2], "First");
        assert!(history.cursor.is_none());
        assert_eq!(history.pending_edit, "");
    }

    #[test]
    fn test_round_trip_serialization() {
        let mut history = CommitEditorHistory::default();
        history.add_new_entry("Message 1".to_string());
        history.add_new_entry("Message 2".to_string());
        history.add_new_entry("Message 3".to_string());

        let serialized = history.to_serialized();
        let deserialized = CommitEditorHistory::from_serialized(serialized);

        assert_eq!(history.entries.len(), deserialized.entries.len());
        for (i, entry) in history.entries.iter().enumerate() {
            assert_eq!(entry, &deserialized.entries[i]);
        }
    }
}
