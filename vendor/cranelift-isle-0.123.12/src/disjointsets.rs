//! Implementation of [`DisjointSets`], to store disjoint sets and provide efficient operations to
//! merge sets

use std::collections::HashMap;
use std::hash::Hash;

/// Stores disjoint sets and provides efficient operations to merge two sets, and to find a
/// representative member of a set given any member of that set. In this implementation, sets always
/// have at least two members, and can only be formed by the `merge` operation.
#[derive(Clone, Debug, Default)]
pub struct DisjointSets<T> {
    parent: HashMap<T, (T, u8)>,
}

impl<T: Copy + std::fmt::Debug + Eq + Hash> DisjointSets<T> {
    /// Find a representative member of the set containing `x`. If `x` has not been merged with any
    /// other items using `merge`, returns `None`. This method updates the data structure to make
    /// future queries faster, and takes amortized constant time.
    ///
    /// ```
    /// let mut sets = cranelift_isle::disjointsets::DisjointSets::default();
    /// sets.merge(1, 2);
    /// sets.merge(1, 3);
    /// sets.merge(2, 4);
    /// assert_eq!(sets.find_mut(3).unwrap(), sets.find_mut(4).unwrap());
    /// assert_eq!(sets.find_mut(10), None);
    /// ```
    pub fn find_mut(&mut self, mut x: T) -> Option<T> {
        while let Some(node) = self.parent.get(&x) {
            if node.0 == x {
                return Some(x);
            }
            let grandparent = self.parent[&node.0].0;
            // Re-do the lookup but take a mutable borrow this time
            self.parent.get_mut(&x).unwrap().0 = grandparent;
            x = grandparent;
        }
        None
    }

    /// Find a representative member of the set containing `x`. If `x` has not been merged with any
    /// other items using `merge`, returns `None`. This method does not update the data structure to
    /// make future queries faster, so `find_mut` should be preferred.
    ///
    /// ```
    /// let mut sets = cranelift_isle::disjointsets::DisjointSets::default();
    /// sets.merge(1, 2);
    /// sets.merge(1, 3);
    /// sets.merge(2, 4);
    /// assert_eq!(sets.find(3).unwrap(), sets.find(4).unwrap());
    /// assert_eq!(sets.find(10), None);
    /// ```
    pub fn find(&self, mut x: T) -> Option<T> {
        while let Some(node) = self.parent.get(&x) {
            if node.0 == x {
                return Some(x);
            }
            x = node.0;
        }
        None
    }

    /// Merge the set containing `x` with the set containing `y`. This method takes amortized
    /// constant time.
    pub fn merge(&mut self, x: T, y: T) {
        assert_ne!(x, y);
        let mut x = if let Some(x) = self.find_mut(x) {
            self.parent[&x]
        } else {
            self.parent.insert(x, (x, 0));
            (x, 0)
        };
        let mut y = if let Some(y) = self.find_mut(y) {
            self.parent[&y]
        } else {
            self.parent.insert(y, (y, 0));
            (y, 0)
        };

        if x == y {
            return;
        }

        if x.1 < y.1 {
            std::mem::swap(&mut x, &mut y);
        }

        self.parent.get_mut(&y.0).unwrap().0 = x.0;
        if x.1 == y.1 {
            let x_rank = &mut self.parent.get_mut(&x.0).unwrap().1;
            *x_rank = x_rank.saturating_add(1);
        }
    }

    /// Returns whether the given items have both been merged into the same set. If either is not
    /// part of any set, returns `false`.
    ///
    /// ```
    /// let mut sets = cranelift_isle::disjointsets::DisjointSets::default();
    /// sets.merge(1, 2);
    /// sets.merge(1, 3);
    /// sets.merge(2, 4);
    /// sets.merge(5, 6);
    /// assert!(sets.in_same_set(2, 3));
    /// assert!(sets.in_same_set(1, 4));
    /// assert!(sets.in_same_set(3, 4));
    /// assert!(!sets.in_same_set(4, 5));
    /// ```
    pub fn in_same_set(&self, x: T, y: T) -> bool {
        let x = self.find(x);
        let y = self.find(y);
        x.zip(y).filter(|(x, y)| x == y).is_some()
    }

    /// Remove the set containing the given item, and return all members of that set. The set is
    /// returned in sorted order. This method takes time linear in the total size of all sets.
    ///
    /// ```
    /// let mut sets = cranelift_isle::disjointsets::DisjointSets::default();
    /// sets.merge(1, 2);
    /// sets.merge(1, 3);
    /// sets.merge(2, 4);
    /// assert_eq!(sets.remove_set_of(4), &[1, 2, 3, 4]);
    /// assert_eq!(sets.remove_set_of(1), &[]);
    /// assert!(sets.is_empty());
    /// ```
    pub fn remove_set_of(&mut self, x: T) -> Vec<T>
    where
        T: Ord,
    {
        let mut set = Vec::new();
        if let Some(x) = self.find_mut(x) {
            set.extend(self.parent.keys().copied());
            // It's important to use `find_mut` here to avoid quadratic worst-case time.
            set.retain(|&y| self.find_mut(y).unwrap() == x);
            for y in set.iter() {
                self.parent.remove(y);
            }
            set.sort_unstable();
        }
        set
    }

    /// Returns true if there are no sets. This method takes constant time.
    ///
    /// ```
    /// let mut sets = cranelift_isle::disjointsets::DisjointSets::default();
    /// assert!(sets.is_empty());
    /// sets.merge(1, 2);
    /// assert!(!sets.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }

    /// Returns the total number of elements in all sets. This method takes constant time.
    ///
    /// ```
    /// let mut sets = cranelift_isle::disjointsets::DisjointSets::default();
    /// sets.merge(1, 2);
    /// assert_eq!(sets.len(), 2);
    /// sets.merge(3, 4);
    /// sets.merge(3, 5);
    /// assert_eq!(sets.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.parent.len()
    }
}
