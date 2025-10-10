//! Column management for git graph rendering.

/// Manager for column allocation.
#[derive(Debug)]
pub struct ColumnManager {
    c: i32,
}

impl ColumnManager {
    /// Create a new column manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::column::ColumnManager;
    ///
    /// let manager = ColumnManager::new();
    /// ```
    pub fn new() -> Self {
        ColumnManager { c: -1 }
    }

    /// Get the next column index.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::column::ColumnManager;
    ///
    /// let mut manager = ColumnManager::new();
    /// assert_eq!(manager.next(), 0);
    /// assert_eq!(manager.next(), 1);
    /// assert_eq!(manager.next(), 2);
    /// ```
    pub fn next(&mut self) -> i32 {
        self.c += 1;
        self.c
    }

    /// Decrement the column counter.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::column::ColumnManager;
    ///
    /// let mut manager = ColumnManager::new();
    /// manager.next(); // c = 0
    /// manager.next(); // c = 1
    /// manager.decr(); // c = 0
    /// assert_eq!(manager.next(), 1);
    /// ```
    pub fn decr(&mut self) {
        self.c -= 1;
    }
}

impl Default for ColumnManager {
    fn default() -> Self {
        Self::new()
    }
}
