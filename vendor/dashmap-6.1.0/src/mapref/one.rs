use crate::lock::{RwLockReadGuard, RwLockWriteGuard};
use crate::HashMap;
use core::hash::Hash;
use core::ops::{Deref, DerefMut};
use std::fmt::{Debug, Formatter};

pub struct Ref<'a, K, V> {
    _guard: RwLockReadGuard<'a, HashMap<K, V>>,
    k: *const K,
    v: *const V,
}

unsafe impl<'a, K: Eq + Hash + Sync, V: Sync> Send for Ref<'a, K, V> {}
unsafe impl<'a, K: Eq + Hash + Sync, V: Sync> Sync for Ref<'a, K, V> {}

impl<'a, K: Eq + Hash, V> Ref<'a, K, V> {
    pub(crate) unsafe fn new(
        guard: RwLockReadGuard<'a, HashMap<K, V>>,
        k: *const K,
        v: *const V,
    ) -> Self {
        Self {
            _guard: guard,
            k,
            v,
        }
    }

    pub fn key(&self) -> &K {
        self.pair().0
    }

    pub fn value(&self) -> &V {
        self.pair().1
    }

    pub fn pair(&self) -> (&K, &V) {
        unsafe { (&*self.k, &*self.v) }
    }

    pub fn map<F, T>(self, f: F) -> MappedRef<'a, K, V, T>
    where
        F: FnOnce(&V) -> &T,
    {
        MappedRef {
            _guard: self._guard,
            k: self.k,
            v: f(unsafe { &*self.v }),
        }
    }

    pub fn try_map<F, T>(self, f: F) -> Result<MappedRef<'a, K, V, T>, Self>
    where
        F: FnOnce(&V) -> Option<&T>,
    {
        if let Some(v) = f(unsafe { &*self.v }) {
            Ok(MappedRef {
                _guard: self._guard,
                k: self.k,
                v,
            })
        } else {
            Err(self)
        }
    }
}

impl<'a, K: Eq + Hash + Debug, V: Debug> Debug for Ref<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ref")
            .field("k", &self.k)
            .field("v", &self.v)
            .finish()
    }
}

impl<'a, K: Eq + Hash, V> Deref for Ref<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &V {
        self.value()
    }
}

pub struct RefMut<'a, K, V> {
    guard: RwLockWriteGuard<'a, HashMap<K, V>>,
    k: *const K,
    v: *mut V,
}

unsafe impl<'a, K: Eq + Hash + Sync, V: Sync> Send for RefMut<'a, K, V> {}
unsafe impl<'a, K: Eq + Hash + Sync, V: Sync> Sync for RefMut<'a, K, V> {}

impl<'a, K: Eq + Hash, V> RefMut<'a, K, V> {
    pub(crate) unsafe fn new(
        guard: RwLockWriteGuard<'a, HashMap<K, V>>,
        k: *const K,
        v: *mut V,
    ) -> Self {
        Self { guard, k, v }
    }

    pub fn key(&self) -> &K {
        self.pair().0
    }

    pub fn value(&self) -> &V {
        self.pair().1
    }

    pub fn value_mut(&mut self) -> &mut V {
        self.pair_mut().1
    }

    pub fn pair(&self) -> (&K, &V) {
        unsafe { (&*self.k, &*self.v) }
    }

    pub fn pair_mut(&mut self) -> (&K, &mut V) {
        unsafe { (&*self.k, &mut *self.v) }
    }

    pub fn downgrade(self) -> Ref<'a, K, V> {
        unsafe { Ref::new(RwLockWriteGuard::downgrade(self.guard), self.k, self.v) }
    }

    pub fn map<F, T>(self, f: F) -> MappedRefMut<'a, K, V, T>
    where
        F: FnOnce(&mut V) -> &mut T,
    {
        MappedRefMut {
            _guard: self.guard,
            k: self.k,
            v: f(unsafe { &mut *self.v }),
        }
    }

    pub fn try_map<F, T>(self, f: F) -> Result<MappedRefMut<'a, K, V, T>, Self>
    where
        F: FnOnce(&mut V) -> Option<&mut T>,
    {
        let v = match f(unsafe { &mut *(self.v as *mut _) }) {
            Some(v) => v,
            None => return Err(self),
        };
        let guard = self.guard;
        let k = self.k;
        Ok(MappedRefMut {
            _guard: guard,
            k,
            v,
        })
    }
}

impl<'a, K: Eq + Hash + Debug, V: Debug> Debug for RefMut<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefMut")
            .field("k", &self.k)
            .field("v", &self.v)
            .finish()
    }
}

impl<'a, K: Eq + Hash, V> Deref for RefMut<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &V {
        self.value()
    }
}

impl<'a, K: Eq + Hash, V> DerefMut for RefMut<'a, K, V> {
    fn deref_mut(&mut self) -> &mut V {
        self.value_mut()
    }
}

pub struct MappedRef<'a, K, V, T> {
    _guard: RwLockReadGuard<'a, HashMap<K, V>>,
    k: *const K,
    v: *const T,
}

impl<'a, K: Eq + Hash, V, T> MappedRef<'a, K, V, T> {
    pub fn key(&self) -> &K {
        self.pair().0
    }

    pub fn value(&self) -> &T {
        self.pair().1
    }

    pub fn pair(&self) -> (&K, &T) {
        unsafe { (&*self.k, &*self.v) }
    }

    pub fn map<F, T2>(self, f: F) -> MappedRef<'a, K, V, T2>
    where
        F: FnOnce(&T) -> &T2,
    {
        MappedRef {
            _guard: self._guard,
            k: self.k,
            v: f(unsafe { &*self.v }),
        }
    }

    pub fn try_map<F, T2>(self, f: F) -> Result<MappedRef<'a, K, V, T2>, Self>
    where
        F: FnOnce(&T) -> Option<&T2>,
    {
        let v = match f(unsafe { &*self.v }) {
            Some(v) => v,
            None => return Err(self),
        };
        let guard = self._guard;
        Ok(MappedRef {
            _guard: guard,
            k: self.k,
            v,
        })
    }
}

impl<'a, K: Eq + Hash + Debug, V, T: Debug> Debug for MappedRef<'a, K, V, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MappedRef")
            .field("k", &self.k)
            .field("v", &self.v)
            .finish()
    }
}

impl<'a, K: Eq + Hash, V, T> Deref for MappedRef<'a, K, V, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value()
    }
}

impl<'a, K: Eq + Hash, V, T: std::fmt::Display> std::fmt::Display for MappedRef<'a, K, V, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.value(), f)
    }
}

impl<'a, K: Eq + Hash, V, T: AsRef<TDeref>, TDeref: ?Sized> AsRef<TDeref>
    for MappedRef<'a, K, V, T>
{
    fn as_ref(&self) -> &TDeref {
        self.value().as_ref()
    }
}

pub struct MappedRefMut<'a, K, V, T> {
    _guard: RwLockWriteGuard<'a, HashMap<K, V>>,
    k: *const K,
    v: *mut T,
}

impl<'a, K: Eq + Hash, V, T> MappedRefMut<'a, K, V, T> {
    pub fn key(&self) -> &K {
        self.pair().0
    }

    pub fn value(&self) -> &T {
        self.pair().1
    }

    pub fn value_mut(&mut self) -> &mut T {
        self.pair_mut().1
    }

    pub fn pair(&self) -> (&K, &T) {
        unsafe { (&*self.k, &*self.v) }
    }

    pub fn pair_mut(&mut self) -> (&K, &mut T) {
        unsafe { (&*self.k, &mut *self.v) }
    }

    pub fn map<F, T2>(self, f: F) -> MappedRefMut<'a, K, V, T2>
    where
        F: FnOnce(&mut T) -> &mut T2,
    {
        MappedRefMut {
            _guard: self._guard,
            k: self.k,
            v: f(unsafe { &mut *self.v }),
        }
    }

    pub fn try_map<F, T2>(self, f: F) -> Result<MappedRefMut<'a, K, V, T2>, Self>
    where
        F: FnOnce(&mut T) -> Option<&mut T2>,
    {
        let v = match f(unsafe { &mut *(self.v as *mut _) }) {
            Some(v) => v,
            None => return Err(self),
        };
        let guard = self._guard;
        let k = self.k;
        Ok(MappedRefMut {
            _guard: guard,
            k,
            v,
        })
    }
}

impl<'a, K: Eq + Hash + Debug, V, T: Debug> Debug for MappedRefMut<'a, K, V, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MappedRefMut")
            .field("k", &self.k)
            .field("v", &self.v)
            .finish()
    }
}

impl<'a, K: Eq + Hash, V, T> Deref for MappedRefMut<'a, K, V, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value()
    }
}

impl<'a, K: Eq + Hash, V, T> DerefMut for MappedRefMut<'a, K, V, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value_mut()
    }
}
