use std::fmt::{self, Debug};
use std::slice;

pub(crate) use self::ordered::OrderedSet;
pub(crate) use self::unordered::UnorderedSet;

mod ordered {
    use super::{Iter, UnorderedSet};
    use std::hash::Hash;

    pub(crate) struct OrderedSet<T> {
        set: UnorderedSet<T>,
        vec: Vec<T>,
    }

    impl<'a, T> OrderedSet<&'a T>
    where
        T: Hash + Eq,
    {
        pub(crate) fn new() -> Self {
            OrderedSet {
                set: UnorderedSet::new(),
                vec: Vec::new(),
            }
        }

        pub(crate) fn insert(&mut self, value: &'a T) -> bool {
            let new = self.set.insert(value);
            if new {
                self.vec.push(value);
            }
            new
        }
    }

    impl<'a, T> OrderedSet<&'a T> {
        pub(crate) fn is_empty(&self) -> bool {
            self.vec.is_empty()
        }

        pub(crate) fn iter(&self) -> Iter<'_, 'a, T> {
            Iter(self.vec.iter())
        }
    }

    impl<'s, 'a, T> IntoIterator for &'s OrderedSet<&'a T> {
        type Item = &'a T;
        type IntoIter = Iter<'s, 'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<'a, T> IntoIterator for OrderedSet<&'a T> {
        type Item = &'a T;
        type IntoIter = <Vec<&'a T> as IntoIterator>::IntoIter;
        fn into_iter(self) -> Self::IntoIter {
            self.vec.into_iter()
        }
    }
}

mod unordered {
    use std::borrow::Borrow;
    use std::collections::HashSet;
    use std::hash::Hash;

    // Wrapper prohibits accidentally introducing iteration over the set, which
    // could lead to nondeterministic generated code.
    pub(crate) struct UnorderedSet<T>(HashSet<T>);

    impl<T> UnorderedSet<T>
    where
        T: Hash + Eq,
    {
        pub(crate) fn new() -> Self {
            UnorderedSet(HashSet::new())
        }

        pub(crate) fn insert(&mut self, value: T) -> bool {
            self.0.insert(value)
        }

        pub(crate) fn contains<Q>(&self, value: &Q) -> bool
        where
            T: Borrow<Q>,
            Q: ?Sized + Hash + Eq,
        {
            self.0.contains(value)
        }

        #[allow(dead_code)] // only used by cxx-build, not cxxbridge-cmd
        pub(crate) fn get<Q>(&self, value: &Q) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: ?Sized + Hash + Eq,
        {
            self.0.get(value)
        }

        pub(crate) fn retain(&mut self, f: impl FnMut(&T) -> bool) {
            self.0.retain(f);
        }

        #[cfg_attr(not(proc_macro), expect(dead_code))]
        pub(crate) fn extend(&mut self, iter: impl IntoIterator<Item = T>) {
            for value in iter {
                self.insert(value);
            }
        }
    }
}

pub(crate) struct Iter<'s, 'a, T>(slice::Iter<'s, &'a T>);

impl<'s, 'a, T> Iterator for Iter<'s, 'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T> Debug for OrderedSet<&T>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.debug_set().entries(self).finish()
    }
}
