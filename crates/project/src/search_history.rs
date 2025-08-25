use std::collections::VecDeque;

/// Determines the behavior to use when inserting a new query into the search history.
#[derive(Default, Debug, Clone, PartialEq)]
pub enum QueryInsertionBehavior {
    #[default]
    /// Always insert the query to the search history.
    AlwaysInsert,
    /// Replace the previous query in the search history, if the new query contains the previous query.
    ReplacePreviousIfContains,
}

/// A cursor that stores an index to the currently selected query in the search history.
/// This can be passed to the search history to update the selection accordingly,
/// e.g. when using the up and down arrow keys to navigate the search history.
///
/// Note: The cursor can point to the wrong query, if the maximum length of the history is exceeded
/// and the old query is overwritten.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SearchHistoryCursor {
    selection: Option<usize>,
}

impl SearchHistoryCursor {
    /// Resets the selection to `None`.
    pub fn reset(&mut self) {
        self.selection = None;
    }
}

#[derive(Debug, Clone)]
pub struct SearchHistory {
    history: VecDeque<String>,
    max_history_len: Option<usize>,
    insertion_behavior: QueryInsertionBehavior,
}

impl SearchHistory {
    pub fn new(max_history_len: Option<usize>, insertion_behavior: QueryInsertionBehavior) -> Self {
        SearchHistory {
            max_history_len,
            insertion_behavior,
            history: VecDeque::new(),
        }
    }

    pub fn add(&mut self, cursor: &mut SearchHistoryCursor, search_string: String) {
        if self.insertion_behavior == QueryInsertionBehavior::ReplacePreviousIfContains
            && let Some(previously_searched) = self.history.back_mut()
            && search_string.contains(previously_searched.as_str())
        {
            *previously_searched = search_string;
            cursor.selection = Some(self.history.len() - 1);
            return;
        }

        if let Some(max_history_len) = self.max_history_len
            && self.history.len() >= max_history_len
        {
            self.history.pop_front();
        }
        self.history.push_back(search_string);

        cursor.selection = Some(self.history.len() - 1);
    }

    pub fn next(&mut self, cursor: &mut SearchHistoryCursor) -> Option<&str> {
        let selected = cursor.selection?;
        let next_index = selected + 1;

        let next = self.history.get(next_index)?;
        cursor.selection = Some(next_index);
        Some(next)
    }

    pub fn current(&self, cursor: &SearchHistoryCursor) -> Option<&str> {
        cursor
            .selection
            .and_then(|selected_ix| self.history.get(selected_ix).map(|s| s.as_str()))
    }

    /// Get the previous history entry using the given `SearchHistoryCursor`.
    /// Uses the last element in the history when there is no cursor.
    pub fn previous(&mut self, cursor: &mut SearchHistoryCursor) -> Option<&str> {
        let prev_index = match cursor.selection {
            Some(index) => index.checked_sub(1)?,
            None => self.history.len().checked_sub(1)?,
        };

        let previous = self.history.get(prev_index)?;
        cursor.selection = Some(prev_index);
        Some(previous)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        const MAX_HISTORY_LEN: usize = 20;
        let mut search_history = SearchHistory::new(
            Some(MAX_HISTORY_LEN),
            QueryInsertionBehavior::ReplacePreviousIfContains,
        );
        let mut cursor = SearchHistoryCursor::default();

        assert_eq!(
            search_history.current(&cursor),
            None,
            "No current selection should be set for the default search history"
        );

        search_history.add(&mut cursor, "rust".to_string());
        assert_eq!(
            search_history.current(&cursor),
            Some("rust"),
            "Newly added item should be selected"
        );

        // check if duplicates are not added
        search_history.add(&mut cursor, "rust".to_string());
        assert_eq!(
            search_history.history.len(),
            1,
            "Should not add a duplicate"
        );
        assert_eq!(search_history.current(&cursor), Some("rust"));

        // check if new string containing the previous string replaces it
        search_history.add(&mut cursor, "rustlang".to_string());
        assert_eq!(
            search_history.history.len(),
            1,
            "Should replace previous item if it's a substring"
        );
        assert_eq!(search_history.current(&cursor), Some("rustlang"));

        // add item when it equals to current item if it's not the last one
        search_history.add(&mut cursor, "php".to_string());
        search_history.previous(&mut cursor);
        assert_eq!(search_history.current(&cursor), Some("rustlang"));
        search_history.add(&mut cursor, "rustlang".to_string());
        assert_eq!(search_history.history.len(), 3, "Should add item");
        assert_eq!(search_history.current(&cursor), Some("rustlang"));

        // push enough items to test SEARCH_HISTORY_LIMIT
        for i in 0..MAX_HISTORY_LEN * 2 {
            search_history.add(&mut cursor, format!("item{i}"));
        }
        assert!(search_history.history.len() <= MAX_HISTORY_LEN);
    }

    #[test]
    fn test_next_and_previous() {
        let mut search_history = SearchHistory::new(None, QueryInsertionBehavior::AlwaysInsert);
        let mut cursor = SearchHistoryCursor::default();

        assert_eq!(
            search_history.next(&mut cursor),
            None,
            "Default search history should not have a next item"
        );

        search_history.add(&mut cursor, "Rust".to_string());
        assert_eq!(search_history.next(&mut cursor), None);
        search_history.add(&mut cursor, "JavaScript".to_string());
        assert_eq!(search_history.next(&mut cursor), None);
        search_history.add(&mut cursor, "TypeScript".to_string());
        assert_eq!(search_history.next(&mut cursor), None);

        assert_eq!(search_history.current(&cursor), Some("TypeScript"));

        assert_eq!(search_history.previous(&mut cursor), Some("JavaScript"));
        assert_eq!(search_history.current(&cursor), Some("JavaScript"));

        assert_eq!(search_history.previous(&mut cursor), Some("Rust"));
        assert_eq!(search_history.current(&cursor), Some("Rust"));

        assert_eq!(search_history.previous(&mut cursor), None);
        assert_eq!(search_history.current(&cursor), Some("Rust"));

        assert_eq!(search_history.next(&mut cursor), Some("JavaScript"));
        assert_eq!(search_history.current(&cursor), Some("JavaScript"));

        assert_eq!(search_history.next(&mut cursor), Some("TypeScript"));
        assert_eq!(search_history.current(&cursor), Some("TypeScript"));

        assert_eq!(search_history.next(&mut cursor), None);
        assert_eq!(search_history.current(&cursor), Some("TypeScript"));
    }

    #[test]
    fn test_reset_selection() {
        let mut search_history = SearchHistory::new(None, QueryInsertionBehavior::AlwaysInsert);
        let mut cursor = SearchHistoryCursor::default();

        search_history.add(&mut cursor, "Rust".to_string());
        search_history.add(&mut cursor, "JavaScript".to_string());
        search_history.add(&mut cursor, "TypeScript".to_string());

        assert_eq!(search_history.current(&cursor), Some("TypeScript"));
        cursor.reset();
        assert_eq!(search_history.current(&cursor), None);
        assert_eq!(
            search_history.previous(&mut cursor),
            Some("TypeScript"),
            "Should start from the end after reset on previous item query"
        );

        search_history.previous(&mut cursor);
        assert_eq!(search_history.current(&cursor), Some("JavaScript"));
        search_history.previous(&mut cursor);
        assert_eq!(search_history.current(&cursor), Some("Rust"));

        cursor.reset();
        assert_eq!(search_history.current(&cursor), None);
    }

    #[test]
    fn test_multiple_cursors() {
        let mut search_history = SearchHistory::new(None, QueryInsertionBehavior::AlwaysInsert);
        let mut cursor1 = SearchHistoryCursor::default();
        let mut cursor2 = SearchHistoryCursor::default();

        search_history.add(&mut cursor1, "Rust".to_string());
        search_history.add(&mut cursor1, "JavaScript".to_string());
        search_history.add(&mut cursor1, "TypeScript".to_string());

        search_history.add(&mut cursor2, "Python".to_string());
        search_history.add(&mut cursor2, "Java".to_string());
        search_history.add(&mut cursor2, "C++".to_string());

        assert_eq!(search_history.current(&cursor1), Some("TypeScript"));
        assert_eq!(search_history.current(&cursor2), Some("C++"));

        assert_eq!(search_history.previous(&mut cursor1), Some("JavaScript"));
        assert_eq!(search_history.previous(&mut cursor2), Some("Java"));

        assert_eq!(search_history.next(&mut cursor1), Some("TypeScript"));
        assert_eq!(search_history.next(&mut cursor1), Some("Python"));

        cursor1.reset();
        cursor2.reset();

        assert_eq!(search_history.current(&cursor1), None);
        assert_eq!(search_history.current(&cursor2), None);
    }
}
