// A simple doubly linked list example using slotmap.

use slotmap::{new_key_type, Key, SlotMap};

new_key_type! {
    pub struct ListKey;
}

#[derive(Copy, Clone)]
struct Node<T> {
    value: T,
    prev: ListKey,
    next: ListKey,
}

pub struct List<T> {
    sm: SlotMap<ListKey, Node<T>>,
    head: ListKey,
    tail: ListKey,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            sm: SlotMap::with_key(),
            head: ListKey::null(),
            tail: ListKey::null(),
        }
    }

    pub fn len(&self) -> usize {
        self.sm.len()
    }

    pub fn push_head(&mut self, value: T) -> ListKey {
        let k = self.sm.insert(Node {
            value,
            prev: ListKey::null(),
            next: self.head,
        });

        if let Some(old_head) = self.sm.get_mut(self.head) {
            old_head.prev = k;
        } else {
            self.tail = k;
        }
        self.head = k;
        k
    }

    pub fn push_tail(&mut self, value: T) -> ListKey {
        let k = self.sm.insert(Node {
            value,
            prev: self.tail,
            next: ListKey::null(),
        });

        if let Some(old_tail) = self.sm.get_mut(self.tail) {
            old_tail.next = k;
        } else {
            self.head = k;
        }
        self.tail = k;
        k
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.sm.remove(self.head).map(|old_head| {
            self.head = old_head.next;
            old_head.value
        })
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        self.sm.remove(self.tail).map(|old_tail| {
            self.tail = old_tail.prev;
            old_tail.value
        })
    }

    pub fn remove(&mut self, key: ListKey) -> Option<T> {
        self.sm.remove(key).map(|node| {
            if let Some(prev_node) = self.sm.get_mut(node.prev) {
                prev_node.next = node.next;
            } else {
                self.head = node.next;
            }

            if let Some(next_node) = self.sm.get_mut(node.next) {
                next_node.prev = node.prev;
            } else {
                self.tail = node.prev;
            }

            node.value
        })
    }

    pub fn head(&self) -> ListKey {
        self.head
    }

    pub fn tail(&self) -> ListKey {
        self.tail
    }

    pub fn get(&self, key: ListKey) -> Option<&T> {
        self.sm.get(key).map(|node| &node.value)
    }

    pub fn get_mut(&mut self, key: ListKey) -> Option<&mut T> {
        self.sm.get_mut(key).map(|node| &mut node.value)
    }
}

fn main() {
    let mut dll = List::new();
    dll.push_head(5);
    dll.push_tail(6);
    let k = dll.push_head(3);
    dll.push_tail(7);
    dll.push_head(4);

    assert_eq!(dll.len(), 4);
    assert_eq!(dll.pop_head(), Some(4));
    assert_eq!(dll.pop_head(), Some(5));
    assert_eq!(dll.head(), k);
    dll.push_head(10);
    assert_eq!(dll.remove(k), Some(3));
    assert_eq!(dll.pop_tail(), Some(7));
    assert_eq!(dll.pop_tail(), Some(6));
    assert_eq!(dll.pop_head(), Some(10));
    assert_eq!(dll.pop_head(), None);
    assert_eq!(dll.pop_tail(), None);
}
