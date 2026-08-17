use core::fmt;

use crate::ion::data_structures::VRegIndex;
use crate::VReg;
use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone)]
struct VRegNode {
    next: VRegIndex,
    prev: VRegIndex,
    vreg: VReg,
}

// Using a doubly linked list here for fast insertion,
// removal and iteration.
pub struct VRegSet {
    items: Vec<VRegNode>,
    head: VRegIndex,
}

impl VRegSet {
    pub fn with_capacity(num_vregs: usize) -> Self {
        Self {
            items: vec![
                VRegNode {
                    prev: VRegIndex::new(num_vregs),
                    next: VRegIndex::new(num_vregs),
                    vreg: VReg::invalid()
                };
                num_vregs + 1
            ],
            head: VRegIndex::new(num_vregs),
        }
    }

    pub fn insert(&mut self, vreg: VReg) {
        debug_assert_eq!(self.items[vreg.vreg()].vreg, VReg::invalid());
        let old_head_next = self.items[self.head.index()].next;
        self.items[vreg.vreg()] = VRegNode {
            next: old_head_next,
            prev: self.head,
            vreg,
        };
        self.items[self.head.index()].next = VRegIndex::new(vreg.vreg());
        self.items[old_head_next.index()].prev = VRegIndex::new(vreg.vreg());
    }

    pub fn remove(&mut self, vreg_num: usize) {
        let prev = self.items[vreg_num].prev;
        let next = self.items[vreg_num].next;
        self.items[prev.index()].next = next;
        self.items[next.index()].prev = prev;
        self.items[vreg_num].vreg = VReg::invalid();
    }

    pub fn is_empty(&self) -> bool {
        self.items[self.head.index()].next == self.head
    }

    pub fn iter(&self) -> VRegSetIter {
        VRegSetIter {
            curr_item: self.items[self.head.index()].next,
            head: self.head,
            items: &self.items,
        }
    }
}

pub struct VRegSetIter<'a> {
    curr_item: VRegIndex,
    head: VRegIndex,
    items: &'a [VRegNode],
}

impl<'a> Iterator for VRegSetIter<'a> {
    type Item = VReg;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_item != self.head {
            let item = self.items[self.curr_item.index()].clone();
            self.curr_item = item.next;
            Some(item.vreg)
        } else {
            None
        }
    }
}

impl fmt::Debug for VRegSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for vreg in self.iter() {
            write!(f, "{vreg} ")?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RegClass;
    use RegClass::*;
    const VREG: fn(usize, RegClass) -> VReg = VReg::new;

    #[test]
    fn operations() {
        let mut set = VRegSet::with_capacity(3090);
        assert!(set.is_empty());
        set.insert(VREG(10, Int));
        set.insert(VREG(2000, Int));
        set.insert(VREG(11, Vector));
        set.insert(VREG(199, Float));
        set.insert(VREG(23, Int));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(VREG(23, Int)));
        assert_eq!(iter.next(), Some(VREG(199, Float)));
        assert_eq!(iter.next(), Some(VREG(11, Vector)));
        assert_eq!(iter.next(), Some(VREG(2000, Int)));
        assert_eq!(iter.next(), Some(VREG(10, Int)));

        set.remove(23);
        set.remove(11);
        set.insert(VREG(73, Vector));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(VREG(73, Vector)));
        assert_eq!(iter.next(), Some(VREG(199, Float)));
        assert_eq!(iter.next(), Some(VREG(2000, Int)));
        assert_eq!(iter.next(), Some(VREG(10, Int)));
        assert!(!set.is_empty());
    }

    #[test]
    fn empty() {
        let mut set = VRegSet::with_capacity(2000);
        assert!(set.is_empty());
        set.insert(VREG(100, Int));
        assert!(!set.is_empty());
        set.remove(100);
        assert!(set.is_empty());
    }
}
