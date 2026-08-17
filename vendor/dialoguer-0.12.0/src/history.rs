use std::collections::VecDeque;

/// Trait for history handling.
pub trait History<T> {
    /// This is called with the current position that should
    /// be read from history. The `pos` represents the number
    /// of times the `Up`/`Down` arrow key has been pressed.
    /// This would normally be used as an index to some sort
    /// of vector. If the `pos` does not have an entry, `None`
    /// should be returned.
    fn read(&self, pos: usize) -> Option<String>;

    /// This is called with the next value you should store
    /// in history at the first location. Normally history
    /// is implemented as a FIFO queue.
    fn write(&mut self, val: &T);
}

pub struct BasicHistory {
    max_entries: Option<usize>,
    deque: VecDeque<String>,
    no_duplicates: bool,
}

impl BasicHistory {
    /// Creates a new basic history value which has no limit on the number of
    /// entries and allows for duplicates.
    ///
    /// # Example
    ///
    /// A history with at most 8 entries and no duplicates:
    ///
    /// ```rs
    /// let mut history = BasicHistory::new().max_entries(8).no_duplicates(true);
    /// ```
    pub fn new() -> Self {
        Self {
            max_entries: None,
            deque: VecDeque::new(),
            no_duplicates: false,
        }
    }

    /// Limit the number of entries stored in the history.
    pub fn max_entries(self, max_entries: usize) -> Self {
        Self {
            max_entries: Some(max_entries),
            ..self
        }
    }

    /// Prevent duplicates in the history. This means that any previous entries
    /// that are equal to a new entry are removed before the new entry is added.
    pub fn no_duplicates(self, no_duplicates: bool) -> Self {
        Self {
            no_duplicates,
            ..self
        }
    }
}

impl<T: ToString> History<T> for BasicHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.deque.get(pos).cloned()
    }

    fn write(&mut self, val: &T) {
        let val = val.to_string();

        if self.no_duplicates {
            self.deque.retain(|v| v != &val);
        }

        self.deque.push_front(val);

        if let Some(max_entries) = self.max_entries {
            self.deque.truncate(max_entries);
        }
    }
}

impl Default for BasicHistory {
    fn default() -> Self {
        Self::new()
    }
}
