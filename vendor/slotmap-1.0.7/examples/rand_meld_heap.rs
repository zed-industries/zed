// Randomized meldable heap.
// https://en.wikipedia.org/wiki/Randomized_meldable_heap

use slotmap::{new_key_type, Key, SlotMap};

new_key_type! {
    struct HeapKey;
}

#[derive(Copy, Clone)]
struct NodeHandle(HeapKey);

#[derive(Copy, Clone)]
struct Node<T> {
    value: T,
    children: [HeapKey; 2],
    parent: HeapKey,
}

struct RandMeldHeap<T: Ord> {
    sm: SlotMap<HeapKey, Node<T>>,
    rng: std::num::Wrapping<u32>,
    root: HeapKey,
}

impl<T: Ord + std::fmt::Debug> RandMeldHeap<T> {
    pub fn new() -> Self {
        Self {
            sm: SlotMap::with_key(),
            rng: std::num::Wrapping(0xdead_beef),
            root: HeapKey::null(),
        }
    }

    pub fn coinflip(&mut self) -> bool {
        // Simple LCG for top speed - random quality barely matters.
        self.rng += (self.rng << 8) + std::num::Wrapping(1);
        self.rng >> 31 > std::num::Wrapping(0)
    }

    pub fn insert(&mut self, value: T) -> NodeHandle {
        let k = self.sm.insert(Node {
            value,
            children: [HeapKey::null(), HeapKey::null()],
            parent: HeapKey::null(),
        });

        let root = self.root;
        self.root = self.meld(k, root);

        NodeHandle(k)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.sm.remove(self.root).map(|root| {
            self.root = self.meld(root.children[0], root.children[1]);
            if let Some(new_root) = self.sm.get_mut(self.root) {
                new_root.parent = HeapKey::null();
            }

            root.value
        })
    }

    pub fn remove_key(&mut self, node: NodeHandle) -> T {
        let node = node.0;
        self.unlink_node(node);
        self.sm.remove(node).unwrap().value
    }

    pub fn update_key(&mut self, node: NodeHandle, value: T) {
        let node = node.0;

        // Unlink and re-insert.
        self.unlink_node(node);
        self.sm[node] = Node {
            value,
            children: [HeapKey::null(), HeapKey::null()],
            parent: HeapKey::null(),
        };
        let root = self.root;
        self.root = self.meld(node, root);
    }

    fn unlink_node(&mut self, node: HeapKey) {
        // Remove node from heap by merging children and placing them where
        // node used to be.
        let children = self.sm[node].children;
        let parent_key = self.sm[node].parent;

        let melded_children = self.meld(children[0], children[1]);
        if let Some(mc) = self.sm.get_mut(melded_children) {
            mc.parent = parent_key;
        }

        if let Some(parent) = self.sm.get_mut(parent_key) {
            if parent.children[0] == node {
                parent.children[0] = melded_children;
            } else {
                parent.children[1] = melded_children;
            }
        } else {
            self.root = melded_children;
        }
    }

    fn meld(&mut self, mut a: HeapKey, mut b: HeapKey) -> HeapKey {
        if a.is_null() {
            return b;
        }
        if b.is_null() {
            return a;
        }

        if self.sm[a].value > self.sm[b].value {
            std::mem::swap(&mut a, &mut b);
        }

        let ret = a;

        // From this point parent and trickle are assumed to be valid keys.
        let mut parent = a;
        let mut trickle = b;

        loop {
            // If a child spot is free, put our trickle there.
            let children = self.sm[parent].children;
            if children[0].is_null() {
                self.sm[parent].children[0] = trickle;
                self.sm[trickle].parent = parent;
                break;
            } else if children[1].is_null() {
                self.sm[parent].children[1] = trickle;
                self.sm[trickle].parent = parent;
                break;
            }

            // No spot free, choose a random child.
            let c = self.coinflip() as usize;
            let child = children[c];
            if self.sm[child].value > self.sm[trickle].value {
                self.sm[parent].children[c] = trickle;
                self.sm[trickle].parent = parent;
                parent = trickle;
                trickle = child;
            } else {
                parent = child;
            }
        }

        ret
    }

    pub fn len(&self) -> usize {
        self.sm.len()
    }
}

fn main() {
    let mut rhm = RandMeldHeap::new();
    let the_answer = rhm.insert(-2);
    let big = rhm.insert(999);

    for k in (0..10).rev() {
        rhm.insert(k * k);
    }

    rhm.update_key(the_answer, 42);
    rhm.remove_key(big);

    while rhm.len() > 0 {
        println!("{}", rhm.pop().unwrap());
    }
}
