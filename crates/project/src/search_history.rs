use collections::HashMap;
use smallvec::SmallVec;
const SEARCH_HISTORY_LIMIT: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SearchHistorySelectionHandle(usize);

#[derive(Default, Debug, Clone)]
pub struct SearchHistory {
    history: SmallVec<[String; SEARCH_HISTORY_LIMIT]>,
    selected: HashMap<SearchHistorySelectionHandle, Option<usize>>,
}

impl SearchHistory {
    pub fn new_handle(&mut self) -> SearchHistorySelectionHandle {
        let handle = SearchHistorySelectionHandle(self.selected.len());
        self.selected.insert(handle, None);
        handle
    }

    pub fn release_handle(&mut self, handle: SearchHistorySelectionHandle) {
        self.selected.remove(&handle);
    }

    pub fn add(&mut self, handle: SearchHistorySelectionHandle, search_string: String) {
        if let Some(selected) = self.selected.get(&handle) {
            if let Some(selected_ix) = selected {
                if search_string == self.history[*selected_ix] {
                    return;
                }
            }
        }

        self.history.push(search_string);
        if self.history.len() > SEARCH_HISTORY_LIMIT {
            self.history.remove(0);
        }
        self.selected.insert(handle, Some(self.history.len() - 1));
    }

    pub fn next(&mut self, handle: SearchHistorySelectionHandle) -> Option<&str> {
        let history_size = self.history.len();
        if history_size == 0 {
            return None;
        }

        let selected = self.selected.get(&handle).cloned().flatten()?;
        if selected == history_size - 1 {
            return None;
        }
        let next_index = selected + 1;
        self.selected.insert(handle, Some(next_index));
        Some(&self.history[next_index])
    }

    pub fn current(&self, handle: SearchHistorySelectionHandle) -> Option<&str> {
        self.selected.get(&handle).and_then(|selected| {
            selected.and_then(|selected_ix| self.history.get(selected_ix).map(|s| s.as_str()))
        })
    }

    pub fn previous(&mut self, handle: SearchHistorySelectionHandle) -> Option<&str> {
        let history_size = self.history.len();
        if history_size == 0 {
            return None;
        }

        let selected = self.selected.get(&handle)?;

        let prev_index = match selected {
            Some(selected_index) => {
                if *selected_index == 0 {
                    return None;
                } else {
                    selected_index - 1
                }
            }
            None => history_size - 1,
        };

        self.selected.insert(handle, Some(prev_index));
        Some(&self.history[prev_index])
    }

    pub fn reset_selection(&mut self, handle: SearchHistorySelectionHandle) {
        self.selected.insert(handle, None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut search_history = SearchHistory::default();
        let handle = search_history.new_handle();

        assert_eq!(
            search_history.current(handle),
            None,
            "No current selection should be set for the default search history"
        );

        search_history.add(handle, "rust".to_string());
        assert_eq!(
            search_history.current(handle),
            Some("rust"),
            "Newly added item should be selected"
        );

        // check if duplicates are not added
        search_history.add(handle, "rust".to_string());
        assert_eq!(
            search_history.history.len(),
            1,
            "Should not add a duplicate"
        );
        assert_eq!(search_history.current(handle), Some("rust"));

        // push enough items to test SEARCH_HISTORY_LIMIT
        for i in 0..SEARCH_HISTORY_LIMIT * 2 {
            search_history.add(handle, format!("item{i}"));
        }
        assert!(search_history.history.len() <= SEARCH_HISTORY_LIMIT);
    }

    #[test]
    fn test_next_and_previous() {
        let mut search_history = SearchHistory::default();
        let handle = search_history.new_handle();

        assert_eq!(
            search_history.next(handle),
            None,
            "Default search history should not have a next item"
        );

        search_history.add(handle, "Rust".to_string());
        assert_eq!(search_history.next(handle), None);
        search_history.add(handle, "JavaScript".to_string());
        assert_eq!(search_history.next(handle), None);
        search_history.add(handle, "TypeScript".to_string());
        assert_eq!(search_history.next(handle), None);

        assert_eq!(search_history.current(handle), Some("TypeScript"));

        assert_eq!(search_history.previous(handle), Some("JavaScript"));
        assert_eq!(search_history.current(handle), Some("JavaScript"));

        assert_eq!(search_history.previous(handle), Some("Rust"));
        assert_eq!(search_history.current(handle), Some("Rust"));

        assert_eq!(search_history.previous(handle), None);
        assert_eq!(search_history.current(handle), Some("Rust"));

        assert_eq!(search_history.next(handle), Some("JavaScript"));
        assert_eq!(search_history.current(handle), Some("JavaScript"));

        assert_eq!(search_history.next(handle), Some("TypeScript"));
        assert_eq!(search_history.current(handle), Some("TypeScript"));

        assert_eq!(search_history.next(handle), None);
        assert_eq!(search_history.current(handle), Some("TypeScript"));
    }

    #[test]
    fn test_reset_selection() {
        let mut search_history = SearchHistory::default();
        let handle = search_history.new_handle();

        search_history.add(handle, "Rust".to_string());
        search_history.add(handle, "JavaScript".to_string());
        search_history.add(handle, "TypeScript".to_string());

        assert_eq!(search_history.current(handle), Some("TypeScript"));
        search_history.reset_selection(handle);
        assert_eq!(search_history.current(handle), None);
        assert_eq!(
            search_history.previous(handle),
            Some("TypeScript"),
            "Should start from the end after reset on previous item query"
        );

        search_history.previous(handle);
        assert_eq!(search_history.current(handle), Some("JavaScript"));
        search_history.previous(handle);
        assert_eq!(search_history.current(handle), Some("Rust"));

        search_history.reset_selection(handle);
        assert_eq!(search_history.current(handle), None);
    }

    #[test]
    fn test_multiple_handles() {
        let mut search_history = SearchHistory::default();
        let handle1 = search_history.new_handle();
        let handle2 = search_history.new_handle();

        search_history.add(handle1, "Rust".to_string());
        search_history.add(handle1, "JavaScript".to_string());
        search_history.add(handle1, "TypeScript".to_string());

        search_history.add(handle2, "Python".to_string());
        search_history.add(handle2, "Java".to_string());
        search_history.add(handle2, "C++".to_string());

        assert_eq!(search_history.current(handle1), Some("TypeScript"));
        assert_eq!(search_history.current(handle2), Some("C++"));

        assert_eq!(search_history.previous(handle1), Some("JavaScript"));
        assert_eq!(search_history.previous(handle2), Some("Java"));

        assert_eq!(search_history.next(handle1), Some("TypeScript"));
        assert_eq!(search_history.next(handle1), Some("Python"));

        search_history.reset_selection(handle1);
        search_history.reset_selection(handle2);

        assert_eq!(search_history.current(handle1), None);
        assert_eq!(search_history.current(handle2), None);
    }
}
