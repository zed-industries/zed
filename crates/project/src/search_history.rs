use smallvec::SmallVec;
const SEARCH_HISTORY_LIMIT: usize = 20;

/// Determines the behavior to use when inserting a new query into the search history.
#[derive(Default, Debug, Clone, PartialEq)]
pub enum QueryInsertionBehavior {
    #[default]
    /// Always insert the query to the search history.
    AlwaysInsert,
    /// Replace the previous query in the search history, if the new query contains the previous query.
    ReplacePreviousIfContains,
}

/// A handle that maintains the current selection in the search history.
/// This can be passed to the search history to update the selection accordingly,
/// e.g. when using the up and down arrow keys to navigate the search history.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SearchHistorySelectionHandle {
    selection: Option<usize>,
}

impl SearchHistorySelectionHandle {
    /// Resets the selection to `None`.
    pub fn reset(&mut self) {
        self.selection = None;
    }
}

#[derive(Default, Debug, Clone)]
pub struct SearchHistory {
    history: SmallVec<[String; SEARCH_HISTORY_LIMIT]>,
    insertion_behavior: QueryInsertionBehavior,
}

impl SearchHistory {
    pub fn new(insertion_behavior: QueryInsertionBehavior) -> Self {
        SearchHistory {
            insertion_behavior,
            ..Default::default()
        }
    }

    pub fn add(&mut self, handle: &mut SearchHistorySelectionHandle, search_string: String) {
        if let Some(selected_ix) = handle.selection {
            if search_string == self.history[selected_ix] {
                return;
            }
        }

        if self.insertion_behavior == QueryInsertionBehavior::ReplacePreviousIfContains {
            if let Some(previously_searched) = self.history.last_mut() {
                if search_string.contains(previously_searched.as_str()) {
                    *previously_searched = search_string;
                    handle.selection = Some(self.history.len() - 1);
                    return;
                }
            }
        }

        self.history.push(search_string);
        if self.history.len() > SEARCH_HISTORY_LIMIT {
            self.history.remove(0);
        }

        handle.selection = Some(self.history.len() - 1);
    }

    pub fn next(&mut self, handle: &mut SearchHistorySelectionHandle) -> Option<&str> {
        let history_size = self.history.len();
        if history_size == 0 {
            return None;
        }

        let selected = handle.selection?;
        if selected == history_size - 1 {
            return None;
        }
        let next_index = selected + 1;
        handle.selection = Some(next_index);
        Some(&self.history[next_index])
    }

    pub fn current(&self, handle: &SearchHistorySelectionHandle) -> Option<&str> {
        handle
            .selection
            .and_then(|selected_ix| self.history.get(selected_ix).map(|s| s.as_str()))
    }

    pub fn previous(&mut self, handle: &mut SearchHistorySelectionHandle) -> Option<&str> {
        let history_size = self.history.len();
        if history_size == 0 {
            return None;
        }

        let prev_index = match handle.selection {
            Some(selected_index) => {
                if selected_index == 0 {
                    return None;
                } else {
                    selected_index - 1
                }
            }
            None => history_size - 1,
        };

        handle.selection = Some(prev_index);
        Some(&self.history[prev_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut search_history =
            SearchHistory::new(QueryInsertionBehavior::ReplacePreviousIfContains);
        let mut handle = SearchHistorySelectionHandle::default();

        assert_eq!(
            search_history.current(&handle),
            None,
            "No current selection should be set for the default search history"
        );

        search_history.add(&mut handle, "rust".to_string());
        assert_eq!(
            search_history.current(&handle),
            Some("rust"),
            "Newly added item should be selected"
        );

        // check if duplicates are not added
        search_history.add(&mut handle, "rust".to_string());
        assert_eq!(
            search_history.history.len(),
            1,
            "Should not add a duplicate"
        );
        assert_eq!(search_history.current(&handle), Some("rust"));

        // check if new string containing the previous string replaces it
        search_history.add(&mut handle, "rustlang".to_string());
        assert_eq!(
            search_history.history.len(),
            1,
            "Should replace previous item if it's a substring"
        );
        assert_eq!(search_history.current(&handle), Some("rustlang"));

        // push enough items to test SEARCH_HISTORY_LIMIT
        for i in 0..SEARCH_HISTORY_LIMIT * 2 {
            search_history.add(&mut handle, format!("item{i}"));
        }
        assert!(search_history.history.len() <= SEARCH_HISTORY_LIMIT);
    }

    #[test]
    fn test_next_and_previous() {
        let mut search_history = SearchHistory::default();
        let mut handle = SearchHistorySelectionHandle::default();

        assert_eq!(
            search_history.next(&mut handle),
            None,
            "Default search history should not have a next item"
        );

        search_history.add(&mut handle, "Rust".to_string());
        assert_eq!(search_history.next(&mut handle), None);
        search_history.add(&mut handle, "JavaScript".to_string());
        assert_eq!(search_history.next(&mut handle), None);
        search_history.add(&mut handle, "TypeScript".to_string());
        assert_eq!(search_history.next(&mut handle), None);

        assert_eq!(search_history.current(&handle), Some("TypeScript"));

        assert_eq!(search_history.previous(&mut handle), Some("JavaScript"));
        assert_eq!(search_history.current(&handle), Some("JavaScript"));

        assert_eq!(search_history.previous(&mut handle), Some("Rust"));
        assert_eq!(search_history.current(&handle), Some("Rust"));

        assert_eq!(search_history.previous(&mut handle), None);
        assert_eq!(search_history.current(&handle), Some("Rust"));

        assert_eq!(search_history.next(&mut handle), Some("JavaScript"));
        assert_eq!(search_history.current(&handle), Some("JavaScript"));

        assert_eq!(search_history.next(&mut handle), Some("TypeScript"));
        assert_eq!(search_history.current(&handle), Some("TypeScript"));

        assert_eq!(search_history.next(&mut handle), None);
        assert_eq!(search_history.current(&handle), Some("TypeScript"));
    }

    #[test]
    fn test_reset_selection() {
        let mut search_history = SearchHistory::default();
        let mut handle = SearchHistorySelectionHandle::default();

        search_history.add(&mut handle, "Rust".to_string());
        search_history.add(&mut handle, "JavaScript".to_string());
        search_history.add(&mut handle, "TypeScript".to_string());

        assert_eq!(search_history.current(&handle), Some("TypeScript"));
        handle.reset();
        assert_eq!(search_history.current(&mut handle), None);
        assert_eq!(
            search_history.previous(&mut handle),
            Some("TypeScript"),
            "Should start from the end after reset on previous item query"
        );

        search_history.previous(&mut handle);
        assert_eq!(search_history.current(&handle), Some("JavaScript"));
        search_history.previous(&mut handle);
        assert_eq!(search_history.current(&handle), Some("Rust"));

        handle.reset();
        assert_eq!(search_history.current(&handle), None);
    }

    #[test]
    fn test_multiple_handles() {
        let mut search_history = SearchHistory::default();
        let mut handle1 = SearchHistorySelectionHandle::default();
        let mut handle2 = SearchHistorySelectionHandle::default();

        search_history.add(&mut handle1, "Rust".to_string());
        search_history.add(&mut handle1, "JavaScript".to_string());
        search_history.add(&mut handle1, "TypeScript".to_string());

        search_history.add(&mut handle2, "Python".to_string());
        search_history.add(&mut handle2, "Java".to_string());
        search_history.add(&mut handle2, "C++".to_string());

        assert_eq!(search_history.current(&handle1), Some("TypeScript"));
        assert_eq!(search_history.current(&handle2), Some("C++"));

        assert_eq!(search_history.previous(&mut handle1), Some("JavaScript"));
        assert_eq!(search_history.previous(&mut handle2), Some("Java"));

        assert_eq!(search_history.next(&mut handle1), Some("TypeScript"));
        assert_eq!(search_history.next(&mut handle1), Some("Python"));

        handle1.reset();
        handle2.reset();

        assert_eq!(search_history.current(&handle1), None);
        assert_eq!(search_history.current(&handle2), None);
    }
}
