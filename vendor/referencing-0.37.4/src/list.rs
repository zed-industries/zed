use std::sync::Arc;

/// An immutable singly-linked list.
pub struct List<T> {
    head: Option<Arc<Node<T>>>,
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        List {
            head: self.head.clone(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T: Eq> Eq for List<T> {}

impl<T: std::hash::Hash> std::hash::Hash for List<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for item in self {
            item.hash(state);
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(node) = current {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                current = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<T> List<T> {
    pub(crate) fn new() -> Self {
        Self { head: None }
    }
    /// Returns true if the list contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    /// Creates a new list with the given value at the front, sharing the rest of the nodes.
    #[must_use]
    pub fn push_front(&self, value: Arc<T>) -> Self {
        List {
            head: Some(Arc::new(Node {
                value,
                next: self.head.clone(),
            })),
        }
    }
    /// Returns an iterator over references to the list elements.
    #[must_use]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.as_ref(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Node<T> {
    value: Arc<T>,
    next: Option<Arc<Node<T>>>,
}

/// Iterator over references to elements in a `List`.
#[derive(Debug)]
pub struct Iter<'a, T> {
    current: Option<&'a Arc<Node<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|current| {
            let value = &current.value;
            self.current = current.next.as_ref();
            &**value
        })
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
