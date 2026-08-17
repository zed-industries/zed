// Copyright 2016 oauth-client-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Performs topological sorting.

#![warn(bad_style)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(clippy::if_not_else)]
#![warn(clippy::invalid_upcast_comparisons)]
#![warn(clippy::items_after_statements)]
#![warn(clippy::mut_mut)]
#![warn(clippy::never_loop)]
#![warn(clippy::nonminimal_bool)]
#![warn(clippy::used_underscore_binding)]

use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Clone)]
struct Dependency<T> {
    num_prec: usize,
    succ: HashSet<T>,
}

impl<T: Hash + Eq> Dependency<T> {
    fn new() -> Dependency<T> {
        Dependency {
            num_prec: 0,
            succ: HashSet::new(),
        }
    }
}

/// Performs topological sorting.
#[derive(Clone)]
pub struct TopologicalSort<T> {
    top: HashMap<T, Dependency<T>>,
}

impl<T> Default for TopologicalSort<T> {
    fn default() -> TopologicalSort<T> {
        TopologicalSort {
            top: HashMap::new(),
        }
    }
}

impl<T: Hash + Eq + Clone> TopologicalSort<T> {
    /// Creates new empty `TopologicalSort`.
    ///
    /// ```rust
    /// # extern crate topological_sort;
    /// # fn main() {
    /// use topological_sort::TopologicalSort;
    /// let mut ts = TopologicalSort::<&str>::new();
    /// ts.add_dependency("hello_world.o", "hello_world");
    /// ts.add_dependency("hello_world.c", "hello_world");
    /// ts.add_dependency("stdio.h", "hello_world.o");
    /// ts.add_dependency("glibc.so", "hello_world");
    /// assert_eq!(vec!["glibc.so", "hello_world.c", "stdio.h"],
    ///            { let mut v = ts.pop_all(); v.sort(); v });
    /// assert_eq!(vec!["hello_world.o"],
    ///            { let mut v = ts.pop_all(); v.sort(); v });
    /// assert_eq!(vec!["hello_world"],
    ///            { let mut v = ts.pop_all(); v.sort(); v });
    /// assert!(ts.pop_all().is_empty());
    /// # }
    /// ```
    #[inline]
    pub fn new() -> TopologicalSort<T> {
        Default::default()
    }

    /// Returns the number of elements in the `TopologicalSort`.
    #[inline]
    pub fn len(&self) -> usize {
        self.top.len()
    }

    /// Returns true if the `TopologicalSort` contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.top.is_empty()
    }

    /// Registers the two elements' dependency.
    ///
    /// # Arguments
    ///
    /// * `prec` - The element appears before `succ`. `prec` is depended on by `succ`.
    /// * `succ` - The element appears after `prec`. `succ` depends on `prec`.
    pub fn add_dependency<P, S>(&mut self, prec: P, succ: S)
    where
        P: Into<T>,
        S: Into<T>,
    {
        self.add_dependency_impl(prec.into(), succ.into())
    }

    fn add_dependency_impl(&mut self, prec: T, succ: T) {
        match self.top.entry(prec) {
            Entry::Vacant(e) => {
                let mut dep = Dependency::new();
                let _ = dep.succ.insert(succ.clone());
                let _ = e.insert(dep);
            }
            Entry::Occupied(e) => {
                if !e.into_mut().succ.insert(succ.clone()) {
                    // Already registered
                    return;
                }
            }
        }

        match self.top.entry(succ) {
            Entry::Vacant(e) => {
                let mut dep = Dependency::new();
                dep.num_prec += 1;
                let _ = e.insert(dep);
            }
            Entry::Occupied(e) => {
                e.into_mut().num_prec += 1;
            }
        }
    }

    /// Registers a dependency link.
    pub fn add_link(&mut self, link: DependencyLink<T>) {
        self.add_dependency(link.prec, link.succ)
    }

    /// Inserts an element, without adding any dependencies from or to it.
    ///
    /// If the `TopologicalSort` did not have this element present, `true` is returned.
    ///
    /// If the `TopologicalSort` already had this element present, `false` is returned.
    pub fn insert<U>(&mut self, elt: U) -> bool
    where
        U: Into<T>,
    {
        match self.top.entry(elt.into()) {
            Entry::Vacant(e) => {
                let dep = Dependency::new();
                let _ = e.insert(dep);
                true
            }
            Entry::Occupied(_) => false,
        }
    }

    /// Removes the item that is not depended on by any other items and returns it, or `None` if
    /// there is no such item.
    ///
    /// If `pop` returns `None` and `len` is not 0, there is cyclic dependencies.
    pub fn pop(&mut self) -> Option<T> {
        self.peek().map(T::clone).map(|key| {
            let _ = self.remove(&key);
            key
        })
    }

    /// Removes all items that are not depended on by any other items and returns it, or empty
    /// vector if there are no such items.
    ///
    /// If `pop_all` returns an empty vector and `len` is not 0, there is cyclic dependencies.
    pub fn pop_all(&mut self) -> Vec<T> {
        let keys = self
            .top
            .iter()
            .filter(|&(_, v)| v.num_prec == 0)
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>();
        for k in &keys {
            let _ = self.remove(k);
        }
        keys
    }

    /// Return a reference to the first item that does not depend on any other items, or `None` if
    /// there is no such item.
    pub fn peek(&self) -> Option<&T> {
        self.top
            .iter()
            .filter(|&(_, v)| v.num_prec == 0)
            .map(|(k, _)| k)
            .next()
    }

    /// Return a vector of references to all items that do not depend on any other items, or an
    /// empty vector if there are no such items.
    pub fn peek_all(&self) -> Vec<&T> {
        self.top
            .iter()
            .filter(|&(_, v)| v.num_prec == 0)
            .map(|(k, _)| k)
            .collect::<Vec<_>>()
    }

    fn remove(&mut self, prec: &T) -> Option<Dependency<T>> {
        let result = self.top.remove(prec);
        if let Some(ref p) = result {
            for s in &p.succ {
                if let Some(y) = self.top.get_mut(s) {
                    y.num_prec -= 1;
                }
            }
        }
        result
    }
}

impl<T: PartialOrd + Eq + Hash + Clone> FromIterator<T> for TopologicalSort<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> TopologicalSort<T> {
        let mut top = TopologicalSort::new();
        let mut seen = Vec::<T>::default();
        for item in iter {
            let _ = top.insert(item.clone());
            for seen_item in seen.iter().cloned() {
                match seen_item.partial_cmp(&item) {
                    Some(Ordering::Less) => {
                        top.add_dependency(seen_item, item.clone());
                    }
                    Some(Ordering::Greater) => {
                        top.add_dependency(item.clone(), seen_item);
                    }
                    _ => (),
                }
            }
            seen.push(item);
        }
        top
    }
}

/// A link between two items in a sort.
#[derive(Copy, Clone, Debug)]
pub struct DependencyLink<T> {
    /// The element which is depened upon by `succ`.
    pub prec: T,
    /// The element which depends on `prec`.
    pub succ: T,
}

impl<T> From<(T, T)> for DependencyLink<T> {
    fn from(tuple: (T, T)) -> Self {
        DependencyLink {
            succ: tuple.0,
            prec: tuple.1,
        }
    }
}

impl<T: Eq + Hash + Clone> FromIterator<DependencyLink<T>> for TopologicalSort<T> {
    fn from_iter<I: IntoIterator<Item = DependencyLink<T>>>(iter: I) -> TopologicalSort<T> {
        let mut top = TopologicalSort::new();
        for link in iter {
            top.add_link(link);
        }
        top
    }
}

impl<T: Hash + Eq + Clone> Iterator for TopologicalSort<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

impl<T: fmt::Debug + Hash + Eq> fmt::Debug for Dependency<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "prec={}, succ={:?}", self.num_prec, self.succ)
    }
}

impl<T: fmt::Debug + Hash + Eq + Clone> fmt::Debug for TopologicalSort<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.top)
    }
}

#[cfg(test)]
mod test {
    use super::TopologicalSort;
    use quickcheck_macros::quickcheck;
    use std::iter::FromIterator;

    #[test]
    fn from_iter() {
        let t = vec![4, 3, 3, 5, 7, 6, 8];
        let mut ts = TopologicalSort::<i32>::from_iter(t);
        assert_eq!(Some(3), ts.next());
        assert_eq!(Some(4), ts.next());
        assert_eq!(Some(5), ts.next());
        assert_eq!(Some(6), ts.next());
        assert_eq!(Some(7), ts.next());
        assert_eq!(Some(8), ts.next());
        assert_eq!(None, ts.next());
    }

    #[test]
    fn iter() {
        let mut ts = TopologicalSort::<i32>::new();
        ts.add_dependency(1, 2);
        ts.add_dependency(2, 3);
        ts.add_dependency(3, 4);
        assert_eq!(Some(1), ts.next());
        assert_eq!(Some(2), ts.next());
        assert_eq!(Some(3), ts.next());
        assert_eq!(Some(4), ts.next());
        assert_eq!(None, ts.next());
    }

    #[test]
    fn pop_all() {
        fn check(result: &[i32], ts: &mut TopologicalSort<i32>) {
            let l = ts.len();
            let mut v = ts.pop_all();
            v.sort();
            assert_eq!(result, &v[..]);
            assert_eq!(l - result.len(), ts.len());
        }

        let mut ts = TopologicalSort::new();
        ts.add_dependency(7, 11);
        assert_eq!(2, ts.len());
        ts.add_dependency(7, 8);
        assert_eq!(3, ts.len());
        ts.add_dependency(5, 11);
        assert_eq!(4, ts.len());
        ts.add_dependency(3, 8);
        assert_eq!(5, ts.len());
        ts.add_dependency(3, 10);
        assert_eq!(6, ts.len());
        ts.add_dependency(11, 2);
        assert_eq!(7, ts.len());
        ts.add_dependency(11, 9);
        assert_eq!(8, ts.len());
        ts.add_dependency(11, 10);
        assert_eq!(8, ts.len());
        ts.add_dependency(8, 9);
        assert_eq!(8, ts.len());

        check(&[3, 5, 7], &mut ts);
        check(&[8, 11], &mut ts);
        check(&[2, 9, 10], &mut ts);
        check(&[], &mut ts);
    }

    #[test]
    fn cyclic_deadlock() {
        let mut ts = TopologicalSort::new();
        ts.add_dependency("stone", "sharp");

        ts.add_dependency("bucket", "hole");
        ts.add_dependency("hole", "straw");
        ts.add_dependency("straw", "axe");
        ts.add_dependency("axe", "sharp");
        ts.add_dependency("sharp", "water");
        ts.add_dependency("water", "bucket");
        assert_eq!(ts.pop(), Some("stone"));
        assert!(ts.pop().is_none());
        println!("{:?}", ts);
    }

    #[quickcheck]
    fn topo_test_quickcheck(n: usize, edges: Vec<(usize, usize)>) {
        use std::collections::{HashMap, HashSet};

        let n = n.max(1).min(1000);
        let mut marked = vec![false; n];
        let edges = edges
            .into_iter()
            .map(|(x, y)| (x % n, y % n))
            .collect::<Vec<_>>();
        let mut deps = HashMap::new();
        let mut toposort = TopologicalSort::<usize>::new();

        for i in 0..n {
            let _ = deps.insert(i, HashSet::new());
            assert!(toposort.insert(i));
        }

        for (op, inp) in edges.iter().map(|(x, y)| (y, x)) {
            let inps = deps.get_mut(op).unwrap();
            let _ = inps.insert(*inp);
        }

        let deps = deps;
        for (inp, op) in edges {
            toposort.add_dependency(inp, op);
        }
        while let Some(x) = toposort.pop() {
            for dep in deps.get(&x).unwrap().iter() {
                assert!(marked[*dep]);
            }
            marked[x] = true;
        }

        if toposort.is_empty() {
            assert!(marked.into_iter().all(|x| x));
        } else {
            let dep_fixed = {
                let mut ret = (0..n)
                    .map(|i| (i, HashSet::new()))
                    .collect::<HashMap<_, _>>();
                let mut new_to_add = deps;

                while !new_to_add.is_empty() {
                    for (k, v) in new_to_add.drain() {
                        let inps = ret.get_mut(&k).unwrap();
                        inps.extend(v.into_iter());
                    }
                    for (k, vs) in ret.iter() {
                        for k2 in vs.iter() {
                            for v2 in ret.get(k2).unwrap().iter() {
                                if !vs.contains(v2) {
                                    let _ = new_to_add
                                        .entry(*k)
                                        .or_insert_with(HashSet::new)
                                        .insert(*v2);
                                }
                            }
                        }
                    }
                }

                ret
            };

            assert!(dep_fixed.into_iter().any(|(op, deps)| deps.contains(&op)));
        }
    }
}
