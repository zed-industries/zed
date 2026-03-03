use serde::{Deserialize, Serialize};

const DEFAULT_MAX_QUERY_HISTORY: usize = 100;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryHistory {
    entries: Vec<String>,
    #[serde(skip)]
    index: Option<usize>,
    #[serde(skip, default = "default_max")]
    max_entries: usize,
}

fn default_max() -> usize {
    DEFAULT_MAX_QUERY_HISTORY
}

impl QueryHistory {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            index: None,
            max_entries,
        }
    }

    pub fn with_entries(entries: Vec<String>, max_entries: usize) -> Self {
        Self {
            entries,
            index: None,
            max_entries,
        }
    }

    pub fn push(&mut self, sql: &str) {
        if self.entries.last().map(|last| last != sql).unwrap_or(true) {
            self.entries.push(sql.to_string());
            if self.entries.len() > self.max_entries {
                self.entries.remove(0);
            }
        }
        self.index = None;
    }

    pub fn navigate_previous(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }
        let new_index = match self.index {
            Some(index) => {
                if index + 1 < self.entries.len() {
                    index + 1
                } else {
                    return None;
                }
            }
            None => 0,
        };
        self.index = Some(new_index);
        Some(&self.entries[self.entries.len() - 1 - new_index])
    }

    pub fn navigate_next(&mut self) -> NavigateResult {
        let Some(current_index) = self.index else {
            return NavigateResult::AtEnd;
        };
        if current_index == 0 {
            self.index = None;
            return NavigateResult::Cleared;
        }
        let new_index = current_index - 1;
        self.index = Some(new_index);
        NavigateResult::Entry(self.entries[self.entries.len() - 1 - new_index].clone())
    }

    #[allow(dead_code)]
    pub fn reset_index(&mut self) {
        self.index = None;
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[allow(dead_code)]
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    pub fn can_go_back(&self) -> bool {
        !self.entries.is_empty()
            && self
                .index
                .map(|i| i + 1 < self.entries.len())
                .unwrap_or(true)
    }

    pub fn can_go_forward(&self) -> bool {
        self.index.is_some()
    }

    #[allow(dead_code)]
    pub fn set_max_entries(&mut self, max: usize) {
        self.max_entries = max;
        while self.entries.len() > max {
            self.entries.remove(0);
        }
    }
}

pub enum NavigateResult {
    Entry(String),
    Cleared,
    AtEnd,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_deduplicates_consecutive() {
        let mut history = QueryHistory::new(100);
        history.push("SELECT 1");
        history.push("SELECT 1");
        assert_eq!(history.entries.len(), 1);
    }

    #[test]
    fn test_push_respects_max() {
        let mut history = QueryHistory::new(3);
        history.push("a");
        history.push("b");
        history.push("c");
        history.push("d");
        assert_eq!(history.entries, vec!["b", "c", "d"]);
    }

    #[test]
    fn test_navigate_previous_and_next() {
        let mut history = QueryHistory::new(100);
        history.push("first");
        history.push("second");
        history.push("third");

        assert_eq!(history.navigate_previous(), Some("third"));
        assert_eq!(history.navigate_previous(), Some("second"));
        assert_eq!(history.navigate_previous(), Some("first"));
        assert!(history.navigate_previous().is_none());

        assert!(matches!(
            history.navigate_next(),
            NavigateResult::Entry(ref s) if s == "second"
        ));
        assert!(matches!(
            history.navigate_next(),
            NavigateResult::Entry(ref s) if s == "third"
        ));
        assert!(matches!(history.navigate_next(), NavigateResult::Cleared));
        assert!(matches!(history.navigate_next(), NavigateResult::AtEnd));
    }

    #[test]
    fn test_push_resets_index() {
        let mut history = QueryHistory::new(100);
        history.push("first");
        history.push("second");
        history.navigate_previous();
        assert!(history.index.is_some());
        history.push("third");
        assert!(history.index.is_none());
    }
}
