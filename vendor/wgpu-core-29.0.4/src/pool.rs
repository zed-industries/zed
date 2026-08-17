use alloc::sync::{Arc, Weak};
use core::hash::Hash;

use hashbrown::{hash_map::Entry, HashMap};
use once_cell::sync::OnceCell;

use crate::lock::{rank, Mutex};
use crate::FastHashMap;

type SlotInner<V> = Weak<V>;
type ResourcePoolSlot<V> = Arc<OnceCell<SlotInner<V>>>;

pub struct ResourcePool<K, V> {
    inner: Mutex<FastHashMap<K, ResourcePoolSlot<V>>>,
}

impl<K: Clone + Eq + Hash, V> ResourcePool<K, V> {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(rank::RESOURCE_POOL_INNER, HashMap::default()),
        }
    }

    /// Get a resource from the pool with the given entry map, or create a new
    /// one if it doesn't exist using the given constructor.
    ///
    /// Behaves such that only one resource will be created for each unique
    /// entry map at any one time.
    pub fn get_or_init<F, E>(&self, key: K, constructor: F) -> Result<Arc<V>, E>
    where
        F: FnOnce(K) -> Result<Arc<V>, E>,
    {
        // We can't prove at compile time that these will only ever be consumed once,
        // so we need to do the check at runtime.
        let mut key = Some(key);
        let mut constructor = Some(constructor);

        'race: loop {
            let mut map_guard = self.inner.lock();

            let entry = match map_guard.entry(key.clone().unwrap()) {
                // An entry exists for this resource.
                //
                // We know that either:
                // - The resource is still alive, and Weak::upgrade will succeed.
                // - The resource is in the process of being dropped, and Weak::upgrade will fail.
                //
                // The entry will never be empty while the BGL is still alive.
                Entry::Occupied(entry) => Arc::clone(entry.get()),
                // No entry exists for this resource.
                //
                // We know that the resource is not alive, so we can create a new entry.
                Entry::Vacant(entry) => Arc::clone(entry.insert(Arc::new(OnceCell::new()))),
            };

            drop(map_guard);

            // Some other thread may beat us to initializing the entry, but OnceCell guarantees that only one thread
            // will actually initialize the entry.
            //
            // We pass the strong reference outside of the closure to keep it alive while we're the only one keeping a reference to it.
            let mut strong = None;
            let weak = entry.get_or_try_init(|| {
                let strong_inner = constructor.take().unwrap()(key.take().unwrap())?;
                let weak = Arc::downgrade(&strong_inner);
                strong = Some(strong_inner);
                Ok(weak)
            })?;

            // If strong is Some, that means we just initialized the entry, so we can just return it.
            if let Some(strong) = strong {
                return Ok(strong);
            }

            // The entry was already initialized by someone else, so we need to try to upgrade it.
            if let Some(strong) = weak.upgrade() {
                // We succeed, the resource is still alive, just return that.
                return Ok(strong);
            }

            // The resource is in the process of being dropped, because upgrade failed.
            // The entry still exists in the map, but it points to nothing.
            //
            // We're in a race with the drop implementation of the resource,
            //  so lets just go around again. When we go around again:
            // - If the entry exists, we might need to go around a few more times.
            // - If the entry doesn't exist, we'll create a new one.
            continue 'race;
        }
    }

    /// Remove the given entry map from the pool.
    ///
    /// Must *only* be called in the Drop impl of [`BindGroupLayout`].
    ///
    /// [`BindGroupLayout`]: crate::binding_model::BindGroupLayout
    pub fn remove(&self, key: &K) {
        let mut map_guard = self.inner.lock();

        // Weak::upgrade will be failing long before this code is called. All threads trying to access the resource will be spinning,
        // waiting for the entry to be removed. It is safe to remove the entry from the map.
        map_guard.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use core::{
        sync::atomic::{AtomicU32, Ordering},
        time::Duration,
    };
    use std::{eprintln, sync::Barrier, thread};

    use super::*;

    #[test]
    fn deduplication() {
        let pool = ResourcePool::<u32, u32>::new();

        let mut counter = 0_u32;

        let arc1 = pool
            .get_or_init::<_, ()>(0, |key| {
                counter += 1;
                Ok(Arc::new(key))
            })
            .unwrap();

        assert_eq!(*arc1, 0);
        assert_eq!(counter, 1);

        let arc2 = pool
            .get_or_init::<_, ()>(0, |key| {
                counter += 1;
                Ok(Arc::new(key))
            })
            .unwrap();

        assert!(Arc::ptr_eq(&arc1, &arc2));
        assert_eq!(*arc2, 0);
        assert_eq!(counter, 1);

        drop(arc1);
        drop(arc2);
        pool.remove(&0);

        let arc3 = pool
            .get_or_init::<_, ()>(0, |key| {
                counter += 1;
                Ok(Arc::new(key))
            })
            .unwrap();

        assert_eq!(*arc3, 0);
        assert_eq!(counter, 2);
    }

    // Test name has "2_threads" in the name so nextest reserves two threads for it.
    #[test]
    fn concurrent_creation_2_threads() {
        struct Resources {
            pool: ResourcePool<u32, u32>,
            counter: AtomicU32,
            barrier: Barrier,
        }

        let resources = Arc::new(Resources {
            pool: ResourcePool::<u32, u32>::new(),
            counter: AtomicU32::new(0),
            barrier: Barrier::new(2),
        });

        // Like all races, this is not inherently guaranteed to work, but in practice it should work fine.
        //
        // To validate the expected order of events, we've put print statements in the code, indicating when each thread is at a certain point.
        // The output will look something like this if the test is working as expected:
        //
        // ```
        // 0: prewait
        // 1: prewait
        // 1: postwait
        // 0: postwait
        // 1: init
        // 1: postget
        // 0: postget
        // ```
        fn thread_inner(idx: u8, resources: &Resources) -> Arc<u32> {
            eprintln!("{idx}: prewait");

            // Once this returns, both threads should hit get_or_init at about the same time,
            // allowing us to actually test concurrent creation.
            //
            // Like all races, this is not inherently guaranteed to work, but in practice it should work fine.
            resources.barrier.wait();

            eprintln!("{idx}: postwait");

            let ret = resources
                .pool
                .get_or_init::<_, ()>(0, |key| {
                    eprintln!("{idx}: init");

                    // Simulate long running constructor, ensuring that both threads will be in get_or_init.
                    thread::sleep(Duration::from_millis(250));

                    resources.counter.fetch_add(1, Ordering::SeqCst);

                    Ok(Arc::new(key))
                })
                .unwrap();

            eprintln!("{idx}: postget");

            ret
        }

        let thread1 = thread::spawn({
            let resource_clone = Arc::clone(&resources);
            move || thread_inner(1, &resource_clone)
        });

        let arc0 = thread_inner(0, &resources);

        assert_eq!(resources.counter.load(Ordering::Acquire), 1);

        let arc1 = thread1.join().unwrap();

        assert!(Arc::ptr_eq(&arc0, &arc1));
    }

    // Test name has "2_threads" in the name so nextest reserves two threads for it.
    #[test]
    fn create_while_drop_2_threads() {
        struct Resources {
            pool: ResourcePool<u32, u32>,
            barrier: Barrier,
        }

        let resources = Arc::new(Resources {
            pool: ResourcePool::<u32, u32>::new(),
            barrier: Barrier::new(2),
        });

        // Like all races, this is not inherently guaranteed to work, but in practice it should work fine.
        //
        // To validate the expected order of events, we've put print statements in the code, indicating when each thread is at a certain point.
        // The output will look something like this if the test is working as expected:
        //
        // ```
        // 0: prewait
        // 1: prewait
        // 1: postwait
        // 0: postwait
        // 1: postsleep
        // 1: removal
        // 0: postget
        // ```
        //
        // The last two _may_ be flipped.

        let existing_entry = resources
            .pool
            .get_or_init::<_, ()>(0, |key| Ok(Arc::new(key)))
            .unwrap();

        // Drop the entry, but do _not_ remove it from the pool.
        // This simulates the situation where the resource arc has been dropped, but the Drop implementation
        // has not yet run, which calls remove.
        drop(existing_entry);

        fn thread0_inner(resources: &Resources) {
            eprintln!("0: prewait");
            resources.barrier.wait();

            eprintln!("0: postwait");
            // We try to create a new entry, but the entry already exists.
            //
            // As Arc::upgrade is failing, we will just keep spinning until remove is called.
            resources
                .pool
                .get_or_init::<_, ()>(0, |key| Ok(Arc::new(key)))
                .unwrap();
            eprintln!("0: postget");
        }

        fn thread1_inner(resources: &Resources) {
            eprintln!("1: prewait");
            resources.barrier.wait();

            eprintln!("1: postwait");
            // We wait a little bit, making sure that thread0_inner has started spinning.
            thread::sleep(Duration::from_millis(250));
            eprintln!("1: postsleep");

            // We remove the entry from the pool, allowing thread0_inner to re-create.
            resources.pool.remove(&0);
            eprintln!("1: removal");
        }

        let thread1 = thread::spawn({
            let resource_clone = Arc::clone(&resources);
            move || thread1_inner(&resource_clone)
        });

        thread0_inner(&resources);

        thread1.join().unwrap();
    }
}
