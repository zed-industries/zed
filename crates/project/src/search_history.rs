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
    draft: Option<String>,
}

impl SearchHistoryCursor {
    /// Resets the selection to `None` and clears the draft.
    pub fn reset(&mut self) {
        self.selection = None;
        self.draft = None;
    }

    /// Takes the stored draft query, if any.
    pub fn take_draft(&mut self) -> Option<String> {
        self.draft.take()
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
        cursor.draft = None;

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
    ///
    /// `current_query` is the current text in the search editor. If it differs
    /// from the history entry at the cursor position (or if the cursor has no
    /// selection), it is saved as a draft so it can be restored later.
    pub fn previous(
        &mut self,
        cursor: &mut SearchHistoryCursor,
        current_query: &str,
    ) -> Option<&str> {
        let matches_history = cursor
            .selection
            .and_then(|i| self.history.get(i))
            .is_some_and(|entry| entry == current_query);
        if !matches_history {
            cursor.draft = Some(current_query.to_string());
        }

        let prev_index = match cursor.selection {
            Some(index) => index.checked_sub(1)?,
            None => self.history.len().checked_sub(1)?,
        };

        let previous = self.history.get(prev_index)?;
        cursor.selection = Some(prev_index);
        Some(previous)
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }
}
