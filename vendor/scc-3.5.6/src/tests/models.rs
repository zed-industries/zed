use sdd::Guard;
use std::borrow::Borrow;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex};

use fnv::FnvBuildHasher;
use loom::model::Builder;
use loom::thread::{spawn, yield_now};

use crate::{HashIndex, HashMap, TreeIndex};

#[derive(Debug)]
struct A(usize, Arc<AtomicUsize>);
impl A {
    fn new(d: usize, c: Arc<AtomicUsize>) -> Self {
        c.fetch_add(1, Relaxed);
        Self(d, c)
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        self.1.fetch_add(1, Relaxed);
        Self(self.0, self.1.clone())
    }
}
impl Drop for A {
    fn drop(&mut self) {
        self.1.fetch_sub(1, Relaxed);
    }
}

static SERIALIZER: Mutex<()> = Mutex::new(());

// Checks if the key is visible when another thread is inserting a key.
#[test]
fn hashmap_key_visibility() {
    let _guard = SERIALIZER.lock().unwrap();

    for max_key in 42..72 {
        let mut model_builder_key_visibility = Builder::new();
        model_builder_key_visibility.max_threads = 2;
        model_builder_key_visibility.max_branches = 1_048_576;
        model_builder_key_visibility.check(move || {
            let hashmap: Arc<HashMap<usize, usize, FnvBuildHasher>> = Arc::new(
                HashMap::with_capacity_and_hasher(64, FnvBuildHasher::default()),
            );
            assert_eq!(hashmap.capacity(), 64);
            for k in 0..max_key {
                assert!(hashmap.insert_sync(k, k).is_ok());
            }
            let hashmap_clone = hashmap.clone();
            let thread_insert = spawn(move || {
                assert!(hashmap_clone.insert_sync(usize::MAX, usize::MAX).is_ok());
                assert!(hashmap_clone.read_sync(&0, |_, _| ()).is_some());
                drop(hashmap_clone);

                loop {
                    let guard = Guard::new();
                    if !guard.has_garbage() {
                        break;
                    }
                    guard.accelerate();
                    yield_now();
                }
            });
            assert!(hashmap.read_sync(&0, |_, _| ()).is_some());
            assert!(thread_insert.join().is_ok());

            for k in 0..max_key {
                assert!(hashmap.read_sync(&k, |_, _| ()).is_some());
            }
            assert!(hashmap.read_sync(&usize::MAX, |_, _| ()).is_some());
            assert_eq!(hashmap.len(), max_key + 1);

            drop(hashmap);
            loop {
                let guard = Guard::new();
                if !guard.has_garbage() {
                    break;
                }
                guard.accelerate();
                yield_now();
            }
        });
    }
}

// Checks if the same key cannot be inserted twice.
#[test]
fn hashmap_key_uniqueness() {
    let _guard = SERIALIZER.lock().unwrap();

    for max_key in 42..72 {
        let mut model_builder_key_uniqueness = Builder::new();
        model_builder_key_uniqueness.max_threads = 2;
        model_builder_key_uniqueness.max_branches = 1_048_576;
        model_builder_key_uniqueness.check(move || {
            let hashmap: Arc<HashMap<usize, usize, FnvBuildHasher>> = Arc::new(
                HashMap::with_capacity_and_hasher(64, FnvBuildHasher::default()),
            );
            let check = Arc::new(AtomicUsize::new(0));
            assert_eq!(hashmap.capacity(), 64);
            for k in 0..max_key {
                assert!(hashmap.insert_sync(k, k).is_ok());
            }
            let hashmap_clone = hashmap.clone();
            let check_clone = check.clone();
            let thread_insert = spawn(move || {
                if hashmap_clone.insert_sync(usize::MAX, usize::MAX).is_ok() {
                    check_clone.fetch_add(1, Relaxed);
                }
                drop(hashmap_clone);

                loop {
                    let guard = Guard::new();
                    if !guard.has_garbage() {
                        break;
                    }
                    guard.accelerate();
                    yield_now();
                }
            });
            if hashmap.insert_sync(usize::MAX, usize::MAX).is_ok() {
                check.fetch_add(1, Relaxed);
            }
            assert!(thread_insert.join().is_ok());

            for k in 0..max_key {
                assert!(hashmap.read_sync(&k, |_, _| ()).is_some(), "{k} {max_key}");
            }
            assert!(hashmap.read_sync(&usize::MAX, |_, _| ()).is_some());
            assert_eq!(hashmap.len(), max_key + 1);
            assert_eq!(check.load(Relaxed), 1);

            drop(hashmap);
            loop {
                let guard = Guard::new();
                if !guard.has_garbage() {
                    break;
                }
                guard.accelerate();
                yield_now();
            }
        });
    }
}

// Checks if the key is visible when another thread is inserting a key.
#[test]
fn hashindex_key_visibility() {
    let _guard = SERIALIZER.lock().unwrap();

    for max_key in 42..72 {
        let mut model_builder_key_visibility = Builder::new();
        model_builder_key_visibility.max_threads = 2;
        model_builder_key_visibility.max_branches = 1_048_576;
        model_builder_key_visibility.check(move || {
            let hashindex: Arc<HashIndex<usize, usize, FnvBuildHasher>> = Arc::new(
                HashIndex::with_capacity_and_hasher(64, FnvBuildHasher::default()),
            );
            assert_eq!(hashindex.capacity(), 64);
            for k in 0..max_key {
                assert!(hashindex.insert_sync(k, k).is_ok());
            }
            let hashindex_clone = hashindex.clone();
            let thread_insert = spawn(move || {
                assert!(hashindex_clone.insert_sync(usize::MAX, usize::MAX).is_ok());
                assert!(hashindex_clone.peek_with(&0, |_, _| ()).is_some());
                drop(hashindex_clone);

                loop {
                    let guard = Guard::new();
                    if !guard.has_garbage() {
                        break;
                    }
                    guard.accelerate();
                    yield_now();
                }
            });
            assert!(hashindex.peek_with(&0, |_, _| ()).is_some());
            assert!(thread_insert.join().is_ok());

            for k in 0..max_key {
                assert!(hashindex.peek_with(&k, |_, _| ()).is_some());
            }
            assert!(hashindex.peek_with(&usize::MAX, |_, _| ()).is_some());
            assert_eq!(hashindex.len(), max_key + 1);

            drop(hashindex);
            loop {
                let guard = Guard::new();
                if !guard.has_garbage() {
                    break;
                }
                guard.accelerate();
                yield_now();
            }
        });
    }
}

// Checks if keys are visible while the leaf node is being split.
#[test]
fn tree_index_split_leaf_node() {
    let _guard = SERIALIZER.lock().unwrap();

    let keys = 14;
    let key_to_remove = 0;
    let mut model_builder_leaf_node = Builder::new();
    model_builder_leaf_node.max_branches = 1_048_576;
    model_builder_leaf_node.check(move || {
        let cnt = Arc::new(AtomicUsize::new(0));
        let tree_index = Arc::new(TreeIndex::<usize, A>::default());

        for k in 0..keys {
            assert!(tree_index.insert_sync(k, A::new(k, cnt.clone())).is_ok());
        }

        let cnt_clone = cnt.clone();
        let tree_index_clone = tree_index.clone();
        let thread_insert = spawn(move || {
            assert!(
                tree_index_clone
                    .insert_sync(keys, A::new(keys, cnt_clone))
                    .is_ok()
            );
        });

        let thread_remove = spawn(move || {
            let key: usize = key_to_remove;
            assert_eq!(
                tree_index
                    .peek_with(key.borrow(), |_key, value| value.0)
                    .unwrap(),
                key
            );
        });

        assert!(thread_insert.join().is_ok());
        assert!(thread_remove.join().is_ok());

        while cnt.load(Relaxed) != 0 {
            Guard::new().accelerate();
            yield_now();
        }
    });
}

// Checks if keys are visible while the internal node is being split.
#[test]
fn tree_index_split_internal_node() {
    let _guard = SERIALIZER.lock().unwrap();

    let keys = 365;
    let key_to_remove = 0;
    let mut model_builder_new_internal_node = Builder::new();
    model_builder_new_internal_node.max_branches = 1_048_576 * 16;
    model_builder_new_internal_node.check(move || {
        let cnt = Arc::new(AtomicUsize::new(0));
        let tree_index = Arc::new(TreeIndex::<usize, A>::default());

        for k in 0..keys {
            assert!(tree_index.insert_sync(k, A::new(k, cnt.clone())).is_ok());
        }

        let cnt_clone = cnt.clone();
        let tree_index_clone = tree_index.clone();
        let thread_insert = spawn(move || {
            assert!(
                tree_index_clone
                    .insert_sync(keys, A::new(keys, cnt_clone))
                    .is_ok()
            );
        });

        let thread_remove = spawn(move || {
            let key: usize = key_to_remove;
            assert_eq!(
                tree_index
                    .peek_with(key.borrow(), |_key, value| value.0)
                    .unwrap(),
                key
            );
            assert!(tree_index.remove_sync(key.borrow()));
        });

        assert!(thread_insert.join().is_ok());
        assert!(thread_remove.join().is_ok());

        while cnt.load(Relaxed) != 0 {
            Guard::new().accelerate();
            yield_now();
        }
    });
}

// Checks if keys are visible while a leaf node is being removed.
#[test]
fn tree_index_remove_leaf_node() {
    let _guard = SERIALIZER.lock().unwrap();

    let keys = 15;
    let key_to_remove = 14;
    let mut model_builder_remove_leaf = Builder::new();
    model_builder_remove_leaf.max_branches = 1_048_576 * 16;
    model_builder_remove_leaf.check(move || {
        let cnt = Arc::new(AtomicUsize::new(0));
        let tree_index = Arc::new(TreeIndex::<usize, A>::default());

        for k in 0..keys {
            assert!(tree_index.insert_sync(k, A::new(k, cnt.clone())).is_ok());
        }

        for k in 0..keys - 3 {
            assert!(tree_index.remove_sync(k.borrow()));
        }

        let tree_index_clone = tree_index.clone();
        let thread_remove = spawn(move || {
            let key_to_remove = keys - 2;
            assert!(tree_index_clone.remove_sync(key_to_remove.borrow()));
        });

        let thread_read = spawn(move || {
            let key = key_to_remove;
            assert_eq!(
                tree_index.peek_with(&key, |_key, value| value.0).unwrap(),
                key
            );
        });

        assert!(thread_remove.join().is_ok());
        assert!(thread_read.join().is_ok());

        while cnt.load(Relaxed) != 0 {
            Guard::new().accelerate();
            yield_now();
        }
    });
}

// Check if keys are visible while a node is being deallocated.
#[test]
fn tree_index_remove_internal_node() {
    let _guard = SERIALIZER.lock().unwrap();

    let keys = 366;
    let key_to_remove = 338;
    let mut model_builder_remove_node = Builder::new();
    model_builder_remove_node.max_branches = 1_048_576 * 16;
    model_builder_remove_node.check(move || {
        let cnt = Arc::new(AtomicUsize::new(0));
        let tree_index = Arc::new(TreeIndex::<usize, A>::default());

        for k in 0..keys {
            assert!(tree_index.insert_sync(k, A::new(k, cnt.clone())).is_ok());
        }

        for k in key_to_remove + 1..keys {
            assert!(tree_index.remove_sync(&k));
        }

        let tree_index_clone = tree_index.clone();
        let thread_read = spawn(move || {
            assert_eq!(
                tree_index_clone
                    .peek_with(&0, |_key, value| value.0)
                    .unwrap(),
                0
            );
        });

        let thread_remove = spawn(move || {
            assert!(tree_index.remove_sync(key_to_remove.borrow()));
        });

        assert!(thread_read.join().is_ok());
        assert!(thread_remove.join().is_ok());

        while cnt.load(Relaxed) != 0 {
            Guard::new().accelerate();
            yield_now();
        }
    });
}
