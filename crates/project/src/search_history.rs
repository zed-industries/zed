use smallvec::SmallVec;
const SEARCH_HISTORY_LIMIT: usize = 20;

#[derive(Default, Debug, Clone)]
pub struct SearchHistory {
    history: SmallVec<[String; SEARCH_HISTORY_LIMIT]>,
    selected: Option<usize>,
}

impl SearchHistory {
    pub fn add(&mut self, search_string: String) {
        if let Some(i) = self.selected {
            if search_string == self.history[i] {
                return;
            }
        }

        if let Some(previously_searched) = self.history.last_mut() {
            if search_string.contains(previously_searched.as_str()) {
                *previously_searched = search_string;
                self.selected = Some(self.history.len() - 1);
                return;
            }
        }

        self.history.push(search_string);
        if self.history.len() > SEARCH_HISTORY_LIMIT {
            self.history.remove(0);
        }
        self.selected = Some(self.history.len() - 1);
    }

    pub fn next(&mut self) -> Option<&str> {
        let history_size = self.history.len();
        if history_size == 0 {
            return None;
        }

        let selected = self.selected?;
        if selected == history_size - 1 {
            return None;
        }
        let next_index = selected + 1;
        self.selected = Some(next_index);
        Some(&self.history[next_index])
    }

    pub fn current(&self) -> Option<&str> {
        Some(&self.history[self.selected?])
    }

    pub fn previous(&mut self) -> Option<&str> {
        let history_size = self.history.len();
        if history_size == 0 {
            return None;
        }

        let prev_index = match self.selected {
            Some(selected_index) => {
                if selected_index == 0 {
                    return None;
                } else {
                    selected_index - 1
                }
            }
            None => history_size - 1,
        };

        self.selected = Some(prev_index);
        Some(&self.history[prev_index])
    }

    pub fn reset_selection(&mut self) {
        self.selected = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut search_history = SearchHistory::default();
        assert_eq!(
            search_history.current(),
            None,
            "No current selection should be set for the default search history"
        );

        search_history.add("rust".to_string());
        assert_eq!(
            search_history.current(),
            Some("rust"),
            "Newly added item should be selected"
        );

        // check if duplicates are not added
        search_history.add("rust".to_string());
        assert_eq!(
            search_history.history.len(),
            1,
            "Should not add a duplicate"
        );
        assert_eq!(search_history.current(), Some("rust"));

        // check if new string containing the previous string replaces it
        search_history.add("rustlang".to_string());
        assert_eq!(
            search_history.history.len(),
            1,
            "Should replace previous item if it's a substring"
        );
        assert_eq!(search_history.current(), Some("rustlang"));

        // push enough items to test SEARCH_HISTORY_LIMIT
        for i in 0..SEARCH_HISTORY_LIMIT * 2 {
            search_history.add(format!("item{i}"));
        }
        assert!(search_history.history.len() <= SEARCH_HISTORY_LIMIT);
    }

    #[test]
    fn test_next_and_previous() {
        let mut search_history = SearchHistory::default();
        assert_eq!(
            search_history.next(),
            None,
            "Default search history should not have a next item"
        );

        search_history.add("Rust".to_string());
        assert_eq!(search_history.next(), None);
        search_history.add("JavaScript".to_string());
        assert_eq!(search_history.next(), None);
        search_history.add("TypeScript".to_string());
        assert_eq!(search_history.next(), None);

        assert_eq!(search_history.current(), Some("TypeScript"));

        assert_eq!(search_history.previous(), Some("JavaScript"));
        assert_eq!(search_history.current(), Some("JavaScript"));

        assert_eq!(search_history.previous(), Some("Rust"));
        assert_eq!(search_history.current(), Some("Rust"));

        assert_eq!(search_history.previous(), None);
        assert_eq!(search_history.current(), Some("Rust"));

        assert_eq!(search_history.next(), Some("JavaScript"));
        assert_eq!(search_history.current(), Some("JavaScript"));

        assert_eq!(search_history.next(), Some("TypeScript"));
        assert_eq!(search_history.current(), Some("TypeScript"));

        assert_eq!(search_history.next(), None);
        assert_eq!(search_history.current(), Some("TypeScript"));
    }

    #[test]
    fn test_reset_selection() {
        let mut search_history = SearchHistory::default();
        search_history.add("Rust".to_string());
        search_history.add("JavaScript".to_string());
        search_history.add("TypeScript".to_string());

        assert_eq!(search_history.current(), Some("TypeScript"));
        search_history.reset_selection();
        assert_eq!(search_history.current(), None);
        assert_eq!(
            search_history.previous(),
            Some("TypeScript"),
            "Should start from the end after reset on previous item query"
        );

        search_history.previous();
        assert_eq!(search_history.current(), Some("JavaScript"));
        search_history.previous();
        assert_eq!(search_history.current(), Some("Rust"));

        search_history.reset_selection();
        assert_eq!(search_history.current(), None);
    }
}
