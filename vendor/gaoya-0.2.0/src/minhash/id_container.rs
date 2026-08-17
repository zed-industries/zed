use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};
use std::ops::Index;
use std::slice::Iter;
use smallvec::{Array, SmallVec};
use crate::minhash::MinHashType;

/// Container trait to hold point ids in MinHashIndex.
///
/// MinHashIndex stores id of every point in every band, where a band is roughly
/// a HashMap<Hash, IdContainer<Id>>. MinHashing requires many bands (20 - 50) for optimal
/// recall.
///
/// To support efficient removals use `HashSetContainer` which is backed up by `HashSet`
/// If removals are not required or infrequent use `VecContainer`, which is faster and uses less memory.
/// If the number of similar points is expected to be small use `SmallVecContainer`, which
/// is backed up by `SmallVec`. `SmallVec` stores small number of elements inline in an array, and
/// falls back to heap when inline array is full.
///
pub trait IdContainer<T>: Sync + Send {

    fn new() -> Self;

    fn push(&mut self, item: T);

    fn len(&self) -> usize;

    fn copy_to<S: BuildHasher>(&self, set: &mut HashSet<T, S>);

    fn copy_refs_to<'a, S: BuildHasher>(&'a self, set: &mut HashSet<&'a T, S>);

    fn remove(&mut self, item: &T);

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}


pub struct HashSetContainer<T> {
    set: HashSet<T>
}

impl<T: Hash + Eq + Send + Sync + Clone> IdContainer<T> for HashSetContainer<T> {

    fn new() -> Self {
        HashSetContainer {
            set: HashSet::new()
        }
    }
    fn push(&mut self, item: T) {
        self.set.insert(item);
    }

    fn len(&self) -> usize {
        self.set.len()
    }

    fn copy_to<S: BuildHasher>(&self, set: &mut HashSet<T, S>) {
        set.extend(self.set.iter().cloned());
    }

    fn copy_refs_to<'a, S: BuildHasher>(&'a self, set: &mut HashSet<&'a T, S>) {
        set.extend(self.set.iter())
    }


    fn remove(&mut self, item: &T) {
        self.set.remove(item);
    }
}

pub struct VecContainer<T> {
    vec: Vec<T>
}

impl<T: Hash + Eq + Send + Sync + Clone> IdContainer<T> for  VecContainer<T> {
    fn new() -> Self {
        Self {
            vec: Vec::new()
        }
    }

    fn push(&mut self, item: T) {
        self.vec.push(item);
    }

    fn len(&self) -> usize {
        self.vec.len()
    }

    fn copy_to<S: BuildHasher>(&self, set: &mut HashSet<T, S>) {
        set.extend(self.vec.iter().cloned())
    }

    fn copy_refs_to<'a, S: BuildHasher>(&'a self, set: &mut HashSet<&'a T, S>) {
        set.extend(self.vec.iter())
    }



    fn remove(&mut self, item: &T) {
        if let Some(index) =  self.vec.iter().position(|x| x == item) {
            self.vec.swap_remove(index);
        }
    }
}


/// SmallVecContainer uses SmallVec backed up by an array
pub struct SmallVecContainer<T, const N: usize> {
    vec: SmallVec<[T; N]>
}

impl<T: Hash + Eq + Send + Sync + Clone, const N: usize> IdContainer<T> for SmallVecContainer<T, N> {
    fn new() -> Self {
        SmallVecContainer {
            vec: SmallVec::<[T; N]>::new()
        }
    }

    fn push(&mut self, item: T) {
        self.vec.push(item);
    }

    fn len(&self) -> usize {
        self.vec.len()
    }

    fn copy_to<S: BuildHasher>(&self, set: &mut HashSet<T, S>) {
        set.extend(self.vec.iter().cloned())
    }

    fn copy_refs_to<'a, S: BuildHasher>(&'a self, container: &mut HashSet<&'a T, S>) {
        container.extend(self.vec.iter())
    }



    fn remove(&mut self, item: &T) {
        if let Some(index) = self.vec.iter().position(|x| x == item) {
            self.vec.swap_remove(index);
        };
    }
}

