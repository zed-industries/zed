use crate::{FxHashSet, PReg, PRegSet, RegClass};
use alloc::vec;
use alloc::vec::Vec;
use core::{
    fmt,
    ops::{Index, IndexMut},
};

/// A least-recently-used cache organized as a linked list based on a vector.
pub struct Lru {
    /// The list of node information.
    ///
    /// Each node corresponds to a physical register.
    /// The index of a node is the `address` from the perspective of the linked list.
    pub data: Vec<LruNode>,
    /// Index of the most recently used register.
    pub head: u8,
    /// Class of registers in the cache.
    pub regclass: RegClass,
}

#[derive(Clone, Copy, Debug)]
pub struct LruNode {
    /// The previous physical register in the list.
    pub prev: u8,
    /// The next physical register in the list.
    pub next: u8,
}

impl Lru {
    pub fn new(regclass: RegClass, regs: &[PReg]) -> Self {
        let mut data = vec![
            LruNode {
                prev: u8::MAX,
                next: u8::MAX
            };
            PReg::MAX + 1
        ];
        let no_of_regs = regs.len();
        for i in 0..no_of_regs {
            let (reg, prev_reg, next_reg) = (
                regs[i],
                regs[i.checked_sub(1).unwrap_or(no_of_regs - 1)],
                regs[if i >= no_of_regs - 1 { 0 } else { i + 1 }],
            );
            data[reg.hw_enc()].prev = prev_reg.hw_enc() as u8;
            data[reg.hw_enc()].next = next_reg.hw_enc() as u8;
        }
        Self {
            head: if regs.is_empty() {
                u8::MAX
            } else {
                regs[0].hw_enc() as u8
            },
            data,
            regclass,
        }
    }

    /// Marks the physical register `preg` as the most recently used
    pub fn poke(&mut self, preg: PReg) {
        trace!(
            "Before poking: {:?} LRU. head: {:?}, Actual data: {:?}",
            self.regclass,
            self.head,
            self.data
        );
        trace!("About to poke {:?} in {:?} LRU", preg, self.regclass);
        let prev_newest = self.head;
        let hw_enc = preg.hw_enc() as u8;
        if hw_enc == prev_newest {
            return;
        }
        if self.data[prev_newest as usize].prev != hw_enc {
            self.remove(hw_enc as usize);
            self.insert_before(hw_enc, self.head);
        }
        self.head = hw_enc;
        trace!("Poked {:?} in {:?} LRU", preg, self.regclass);
        if cfg!(debug_assertions) {
            self.validate_lru();
        }
    }

    /// Gets the least recently used physical register.
    pub fn pop(&mut self) -> PReg {
        trace!(
            "Before popping: {:?} LRU. head: {:?}, Actual data: {:?}",
            self.regclass,
            self.head,
            self.data
        );
        trace!("Popping {:?} LRU", self.regclass);
        if self.is_empty() {
            panic!("LRU is empty");
        }
        let oldest = self.data[self.head as usize].prev;
        trace!("Popped p{oldest} in {:?} LRU", self.regclass);
        if cfg!(debug_assertions) {
            self.validate_lru();
        }
        PReg::new(oldest as usize, self.regclass)
    }

    /// Get the last PReg in the LRU from the set `from`.
    pub fn last(&self, from: PRegSet) -> Option<PReg> {
        trace!("Getting the last preg from the LRU in set {from}");
        self.last_satisfying(|preg| from.contains(preg))
    }

    /// Get the last PReg from the LRU for which `f` returns true.
    pub fn last_satisfying<F: Fn(PReg) -> bool>(&self, f: F) -> Option<PReg> {
        trace!("Getting the last preg from the LRU satisfying...");
        if self.is_empty() {
            panic!("LRU is empty");
        }
        let mut last = self.data[self.head as usize].prev;
        let init_last = last;
        loop {
            let preg = PReg::new(last as usize, self.regclass);
            if f(preg) {
                return Some(preg);
            }
            last = self.data[last as usize].prev;
            if last == init_last {
                return None;
            }
        }
    }

    /// Splices out a node from the list.
    fn remove(&mut self, hw_enc: usize) {
        trace!(
            "Before removing: {:?} LRU. head: {:?}, Actual data: {:?}",
            self.regclass,
            self.head,
            self.data
        );
        trace!("Removing p{hw_enc} from {:?} LRU", self.regclass);
        let (iprev, inext) = (
            self.data[hw_enc].prev as usize,
            self.data[hw_enc].next as usize,
        );
        self.data[iprev].next = self.data[hw_enc].next;
        self.data[inext].prev = self.data[hw_enc].prev;
        self.data[hw_enc].prev = u8::MAX;
        self.data[hw_enc].next = u8::MAX;
        if hw_enc == self.head as usize {
            if hw_enc == inext {
                // There are no regs in the LRU
                self.head = u8::MAX;
            } else {
                self.head = inext as u8;
            }
        }
        trace!("Removed p{hw_enc} from {:?} LRU", self.regclass);
        if cfg!(debug_assertions) {
            self.validate_lru();
        }
    }

    /// Insert node `i` before node `j` in the list.
    fn insert_before(&mut self, i: u8, j: u8) {
        trace!(
            "Before inserting: {:?} LRU. head: {:?}, Actual data: {:?}",
            self.regclass,
            self.head,
            self.data
        );
        trace!("Inserting p{i} before {j} in {:?} LRU", self.regclass);
        let prev = self.data[j as usize].prev;
        self.data[prev as usize].next = i;
        self.data[j as usize].prev = i;
        self.data[i as usize] = LruNode { next: j, prev };
        trace!("Done inserting p{i} before {j} in {:?} LRU", self.regclass);
        if cfg!(debug_assertions) {
            self.validate_lru();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head == u8::MAX
    }

    // Using this to debug.
    fn validate_lru(&self) {
        trace!(
            "{:?} LRU. head: {:?}, Actual data: {:?}",
            self.regclass,
            self.head,
            self.data
        );
        if self.head != u8::MAX {
            let mut node = self.data[self.head as usize].next;
            let mut seen = FxHashSet::default();
            while node != self.head {
                if seen.contains(&node) {
                    panic!(
                        "Cycle detected in {:?} LRU.\n
                        head: {:?}, actual data: {:?}",
                        self.regclass, self.head, self.data
                    );
                }
                seen.insert(node);
                node = self.data[node as usize].next;
            }
            for i in 0..self.data.len() {
                if self.data[i].prev == u8::MAX && self.data[i].next == u8::MAX {
                    // Removed
                    continue;
                }
                if self.data[i].prev == u8::MAX || self.data[i].next == u8::MAX {
                    panic!(
                        "Invalid LRU. p{} next or previous is an invalid value, but not both",
                        i
                    );
                }
                if self.data[self.data[i].prev as usize].next != i as u8 {
                    panic!(
                        "Invalid LRU. p{i} prev is p{:?}, but p{:?} next is {:?}",
                        self.data[i].prev,
                        self.data[i].prev,
                        self.data[self.data[i].prev as usize].next
                    );
                }
                if self.data[self.data[i].next as usize].prev != i as u8 {
                    panic!(
                        "Invalid LRU. p{i} next is p{:?}, but p{:?} prev is p{:?}",
                        self.data[i].next,
                        self.data[i].next,
                        self.data[self.data[i].next as usize].prev
                    );
                }
            }
        }
    }
}

impl fmt::Debug for Lru {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use alloc::format;
        let data_str = if self.head == u8::MAX {
            format!("<empty>")
        } else {
            let mut data_str = format!("p{}", self.head);
            let mut node = self.data[self.head as usize].next;
            let mut seen = FxHashSet::default();
            while node != self.head {
                if seen.contains(&node) {
                    panic!(
                        "The {:?} LRU is messed up: 
                       head: {:?}, {:?} -> p{node}, actual data: {:?}",
                        self.regclass, self.head, data_str, self.data
                    );
                }
                seen.insert(node);
                data_str += &format!(" -> p{}", node);
                node = self.data[node as usize].next;
            }
            data_str
        };
        f.debug_struct("Lru")
            .field("head", if self.is_empty() { &"none" } else { &self.head })
            .field("class", &self.regclass)
            .field("data", &data_str)
            .finish()
    }
}

#[derive(Clone)]
pub struct PartedByRegClass<T> {
    pub items: [T; 3],
}

impl<T> Index<RegClass> for PartedByRegClass<T> {
    type Output = T;

    fn index(&self, index: RegClass) -> &Self::Output {
        &self.items[index as usize]
    }
}

impl<T> IndexMut<RegClass> for PartedByRegClass<T> {
    fn index_mut(&mut self, index: RegClass) -> &mut Self::Output {
        &mut self.items[index as usize]
    }
}

/// Least-recently-used caches for register classes Int, Float, and Vector, respectively.
pub type Lrus = PartedByRegClass<Lru>;

impl Lrus {
    pub fn new(int_regs: &[PReg], float_regs: &[PReg], vec_regs: &[PReg]) -> Self {
        Self {
            items: [
                Lru::new(RegClass::Int, int_regs),
                Lru::new(RegClass::Float, float_regs),
                Lru::new(RegClass::Vector, vec_regs),
            ],
        }
    }
}

use core::fmt::{Debug, Display};

impl<T: Display> Display for PartedByRegClass<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ int: {}, float: {}, vector: {} }}",
            self.items[0], self.items[1], self.items[2]
        )
    }
}

impl<T: Debug> Debug for PartedByRegClass<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ int: {:?}, float: {:?}, vector: {:?} }}",
            self.items[0], self.items[1], self.items[2]
        )
    }
}
