pub struct MessageHistory<T> {
    items: Vec<T>,
    current: Option<usize>,
}

impl<T> Default for MessageHistory<T> {
    fn default() -> Self {
        MessageHistory {
            items: Vec::new(),
            current: None,
        }
    }
}

impl<T> MessageHistory<T> {
    pub fn push(&mut self, message: T) {
        self.current.take();
        self.items.push(message);
    }

    pub fn reset_position(&mut self) {
        self.current.take();
    }

    pub fn prev(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }

        let new_ix = self
            .current
            .get_or_insert(self.items.len())
            .saturating_sub(1);

        self.current = Some(new_ix);
        self.items.get(new_ix)
    }

    pub fn next(&mut self) -> Option<&T> {
        let current = self.current.as_mut()?;
        *current += 1;

        self.items.get(*current).or_else(|| {
            self.current.take();
            None
        })
    }

    #[cfg(test)]
    pub fn items(&self) -> &[T] {
        &self.items
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prev_next() {
        let mut history = MessageHistory::default();

        // Test empty history
        assert_eq!(history.prev(), None);
        assert_eq!(history.next(), None);

        // Add some messages
        history.push("first");
        history.push("second");
        history.push("third");

        // Test prev navigation
        assert_eq!(history.prev(), Some(&"third"));
        assert_eq!(history.prev(), Some(&"second"));
        assert_eq!(history.prev(), Some(&"first"));
        assert_eq!(history.prev(), Some(&"first"));

        assert_eq!(history.next(), Some(&"second"));

        // Test mixed navigation
        history.push("fourth");
        assert_eq!(history.prev(), Some(&"fourth"));
        assert_eq!(history.prev(), Some(&"third"));
        assert_eq!(history.next(), Some(&"fourth"));
        assert_eq!(history.next(), None);

        // Test that push resets navigation
        history.prev();
        history.prev();
        history.push("fifth");
        assert_eq!(history.prev(), Some(&"fifth"));
    }
}
