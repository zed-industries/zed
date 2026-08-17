mod common {
    use std::hash::{Hash, Hasher};
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering::Relaxed;

    use crate::Equivalent;

    pub struct R(pub &'static AtomicUsize);
    impl R {
        pub fn new(cnt: &'static AtomicUsize) -> R {
            cnt.fetch_add(1, Relaxed);
            R(cnt)
        }
    }
    impl Clone for R {
        fn clone(&self) -> Self {
            self.0.fetch_add(1, Relaxed);
            R(self.0)
        }
    }
    impl Drop for R {
        fn drop(&mut self) {
            self.0.fetch_sub(1, Relaxed);
        }
    }

    #[derive(Debug)]
    pub struct MaybeEq(pub u64, pub u64);
    impl Eq for MaybeEq {}
    impl Hash for MaybeEq {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl PartialEq for MaybeEq {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct EqTest(pub String, pub usize);

    impl Equivalent<EqTest> for str {
        fn equivalent(&self, key: &EqTest) -> bool {
            key.0.eq(self)
        }
    }

    impl Hash for EqTest {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
}

mod hashmap {
    use std::collections::BTreeSet;
    use std::hash::{Hash, Hasher};
    use std::panic::{RefUnwindSafe, UnwindSafe};
    use std::rc::Rc;
    use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
    use std::sync::atomic::{AtomicU64, AtomicUsize};
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::Duration;

    use futures::future::join_all;
    use proptest::prelude::*;
    use proptest::strategy::ValueTree;
    use proptest::test_runner::TestRunner;
    use tokio::sync::Barrier as AsyncBarrier;

    use super::common::{EqTest, MaybeEq, R};
    use crate::HashMap;
    use crate::async_helper::AsyncGuard;
    use crate::hash_map::{self, Entry, ReplaceResult, Reserve};
    use crate::hash_table::bucket::{MAP, Writer};

    static_assertions::assert_eq_size!(Option<Writer<usize, usize, (), MAP>>, usize);
    static_assertions::assert_impl_all!(AsyncGuard: Send, Sync);
    static_assertions::assert_eq_size!(AsyncGuard, usize);
    static_assertions::assert_not_impl_any!(HashMap<Rc<String>, Rc<String>>: Send, Sync);
    static_assertions::assert_not_impl_any!(hash_map::Entry<Rc<String>, Rc<String>>: Send, Sync);
    static_assertions::assert_impl_all!(HashMap<String, String>: Send, Sync, RefUnwindSafe, UnwindSafe);
    static_assertions::assert_impl_all!(Reserve<String, String>: Send, Sync, UnwindSafe);
    static_assertions::assert_not_impl_any!(HashMap<String, *const String>: Send, Sync);
    static_assertions::assert_not_impl_any!(Reserve<String, *const String>: Send, Sync);
    static_assertions::assert_impl_all!(hash_map::OccupiedEntry<String, String>: Send, Sync);
    static_assertions::assert_not_impl_any!(hash_map::OccupiedEntry<String, *const String>: Send, Sync, RefUnwindSafe, UnwindSafe);
    static_assertions::assert_impl_all!(hash_map::VacantEntry<String, String>: Send, Sync);
    static_assertions::assert_not_impl_any!(hash_map::VacantEntry<String, *const String>: Send, Sync, RefUnwindSafe, UnwindSafe);

    struct Data {
        data: usize,
        checker: Arc<AtomicUsize>,
    }

    impl Data {
        fn new(data: usize, checker: Arc<AtomicUsize>) -> Data {
            checker.fetch_add(1, Relaxed);
            Data { data, checker }
        }
    }

    impl Clone for Data {
        fn clone(&self) -> Self {
            Data::new(self.data, self.checker.clone())
        }
    }

    impl Drop for Data {
        fn drop(&mut self) {
            self.checker.fetch_sub(1, Relaxed);
        }
    }

    impl Eq for Data {}

    impl Hash for Data {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.data.hash(state);
        }
    }

    impl PartialEq for Data {
        fn eq(&self, other: &Self) -> bool {
            self.data == other.data
        }
    }

    #[test]
    fn equivalent() {
        let hashmap: HashMap<EqTest, usize> = HashMap::default();
        assert!(
            hashmap
                .insert_sync(EqTest("HELLO".to_owned(), 1), 1)
                .is_ok()
        );
        assert!(!hashmap.contains_sync("NO"));
        assert!(hashmap.contains_sync("HELLO"));
    }

    #[test]
    fn future_size() {
        // Small type.
        {
            let limit = 488; // In v2, 104.
            let hashmap: HashMap<(), ()> = HashMap::default();
            let get_size = size_of_val(&hashmap.get_async(&()));
            assert!(get_size <= limit + 24, "{get_size}");
            let contains_size = size_of_val(&hashmap.contains_async(&()));
            assert!(contains_size <= limit + 16, "{contains_size}");
            let insert_size = size_of_val(&hashmap.insert_async((), ()));
            assert!(insert_size <= limit, "{insert_size}");
            let entry_size = size_of_val(&hashmap.entry_async(()));
            assert!(entry_size <= limit + 8, "{entry_size}");
            let read_size = size_of_val(&hashmap.read_async(&(), |(), ()| {}));
            assert!(read_size <= limit + 16, "{read_size}");
            let remove_size = size_of_val(&hashmap.remove_async(&()));
            assert!(remove_size <= limit + 48, "{remove_size}");
            let iter_size = size_of_val(&hashmap.iter_async(|(), ()| true));
            assert!(iter_size <= limit + 40, "{iter_size}");
        }
        // Medium type.
        {
            let limit = 480 + 2 * size_of::<(u64, u64)>(); // In v2, 104 + 2 * size_of::<(u64, u64)>.
            let hashmap: HashMap<u64, u64> = HashMap::default();
            let get_size = size_of_val(&hashmap.get_async(&0));
            assert!(get_size <= limit, "{get_size}");
            let contains_size = size_of_val(&hashmap.contains_async(&0));
            assert!(contains_size <= limit, "{contains_size}");
            let insert_size = size_of_val(&hashmap.insert_async(0, 0));
            assert!(insert_size <= limit + 8, "{insert_size}");
            let entry_size = size_of_val(&hashmap.entry_async(0));
            assert!(entry_size <= limit, "{entry_size}");
            let read_size = size_of_val(&hashmap.read_async(&0, |_, _| {}));
            assert!(read_size <= limit, "{read_size}");
            let remove_size = size_of_val(&hashmap.remove_async(&0));
            assert!(remove_size <= limit + 24, "{remove_size}");
            let iter_size = size_of_val(&hashmap.iter_async(|_, _| true));
            assert!(iter_size <= limit + 8, "{iter_size}");
        }
        {
            type Large = [u64; 32];
            let limit = 480 + 2 * size_of::<(Vec<usize>, Large)>(); // In v2, 104 + 2 * size_of::<(Vec<usize>, Large)>.
            let hashmap: HashMap<Vec<usize>, Large> = HashMap::default();
            let get_size = size_of_val(&hashmap.get_async(&vec![]));
            assert!(get_size <= limit, "{get_size}");
            let contains_size = size_of_val(&hashmap.contains_async(&vec![]));
            assert!(contains_size <= limit + 16, "{contains_size}");
            let insert_size = size_of_val(&hashmap.insert_async(vec![], [0; 32]));
            assert!(insert_size <= limit + 8, "{insert_size}");
            let entry_size = size_of_val(&hashmap.entry_async(vec![]));
            assert!(entry_size <= limit, "{entry_size}");
            let read_size = size_of_val(&hashmap.read_async(&vec![], |_, _| {}));
            assert!(read_size <= limit, "{read_size}");
            let remove_size = size_of_val(&hashmap.remove_async(&vec![]));
            assert!(remove_size <= limit, "{remove_size}");
            let iter_size = size_of_val(&hashmap.iter_async(|_, _| true));
            assert!(iter_size <= limit, "{iter_size}");
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn capacity_async() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);

        let hashmap: HashMap<usize, R> = HashMap::default();
        let workload_size = 1_048_576;
        for k in 0..workload_size {
            assert!(hashmap.insert_async(k, R::new(&INST_CNT)).await.is_ok());
        }
        assert_eq!(INST_CNT.load(Relaxed), workload_size);
        assert_eq!(hashmap.len(), workload_size);
        assert!(hashmap.capacity() >= workload_size);
        drop(hashmap);
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[cfg_attr(miri, ignore)]
    #[test]
    fn capacity_sync() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);

        let hashmap: HashMap<usize, R> = HashMap::default();
        assert_eq!(hashmap.capacity(), 0);
        assert!(
            hashmap
                .reserve((1_usize << (usize::BITS - 1)) + 1)
                .is_none()
        );

        let reserved_capacity = 1_048_576_usize;

        let reserve = hashmap.reserve(reserved_capacity);
        assert!(reserve.is_some());
        for k in 0..256 {
            assert!(hashmap.insert_sync(k, R::new(&INST_CNT)).is_ok());
        }
        assert_eq!(hashmap.capacity(), reserved_capacity);
        assert_eq!(hashmap.len(), 256);
        for k in 0..256 {
            assert!(hashmap.insert_sync(k, R::new(&INST_CNT)).is_err());
        }

        drop(reserve);
        hashmap.clear_sync();

        assert!(hashmap.capacity() < reserved_capacity);
        assert_eq!(hashmap.len(), 0);
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn insert_drop_async() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);

        let hashmap: HashMap<usize, R> = HashMap::default();
        let workload_size = 1024;
        for k in 0..workload_size {
            assert!(hashmap.insert_async(k, R::new(&INST_CNT)).await.is_ok());
        }
        assert_eq!(INST_CNT.load(Relaxed), workload_size);
        assert_eq!(hashmap.len(), workload_size);
        drop(hashmap);
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[test]
    fn insert_drop_sync() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);

        let hashmap: HashMap<usize, R> = HashMap::default();
        let workload_size = 256;
        for k in 0..workload_size {
            assert!(hashmap.insert_sync(k, R::new(&INST_CNT)).is_ok());
        }
        assert_eq!(INST_CNT.load(Relaxed), workload_size);
        assert_eq!(hashmap.len(), workload_size);
        drop(hashmap);
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn replace_async() {
        let hashmap: HashMap<MaybeEq, u64> = HashMap::default();
        let workload_size = 256;
        for k in 0..workload_size {
            let ReplaceResult::NotReplaced(v) = hashmap.replace_async(MaybeEq(k, 0)).await else {
                unreachable!();
            };
            v.insert_entry(k);
        }
        for k in 0..workload_size {
            let ReplaceResult::Replaced(o, p) = hashmap.replace_async(MaybeEq(k, 1)).await else {
                unreachable!();
            };
            assert_eq!(p.1, 0);
            assert_eq!(o.get(), &k);
        }
        assert!(hashmap.any_async(|k, _| k.1 == 0).await.is_none());
    }

    #[test]
    fn replace_sync() {
        let hashmap: HashMap<MaybeEq, u64> = HashMap::default();
        let workload_size = 256;
        for k in 0..workload_size {
            let ReplaceResult::NotReplaced(v) = hashmap.replace_sync(MaybeEq(k, 0)) else {
                unreachable!();
            };
            v.insert_entry(k);
        }
        for k in 0..workload_size {
            let ReplaceResult::Replaced(o, p) = hashmap.replace_sync(MaybeEq(k, 1)) else {
                unreachable!();
            };
            assert_eq!(p.1, 0);
            assert_eq!(o.get(), &k);
        }
        assert!(hashmap.any_sync(|k, _| k.1 == 0).is_none());
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn clear_async() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);

        let hashmap: HashMap<usize, R> = HashMap::default();
        let workload_size = 1_usize << 18;
        for _ in 0..2 {
            for k in 0..workload_size {
                assert!(hashmap.insert_async(k, R::new(&INST_CNT)).await.is_ok());
            }
            assert_eq!(INST_CNT.load(Relaxed), workload_size);
            assert_eq!(hashmap.len(), workload_size);
            hashmap.clear_async().await;
            assert_eq!(INST_CNT.load(Relaxed), 0);
        }
    }

    #[test]
    fn clear_sync() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);

        let hashmap: HashMap<usize, R> = HashMap::default();
        for workload_size in 2..64 {
            for k in 0..workload_size {
                assert!(hashmap.insert_sync(k, R::new(&INST_CNT)).is_ok());
            }
            assert_eq!(INST_CNT.load(Relaxed), workload_size);
            assert_eq!(hashmap.len(), workload_size);
            hashmap.clear_sync();
            assert_eq!(INST_CNT.load(Relaxed), 0);
        }
    }

    #[test]
    fn read_remove_sync() {
        let hashmap = Arc::new(HashMap::<String, Vec<u8>>::new());
        let barrier = Arc::new(Barrier::new(2));

        hashmap.insert_sync("first".into(), vec![123]).unwrap();

        let hashmap_clone = hashmap.clone();
        let barrier_clone = barrier.clone();
        let task = thread::spawn(move || {
            hashmap_clone.read_sync("first", |_key, value| {
                {
                    let first_item = value.first();
                    assert_eq!(first_item.unwrap(), &123_u8);
                }
                barrier_clone.wait();
                thread::sleep(Duration::from_millis(16));
                {
                    let first_item = value.first();
                    assert_eq!(first_item.unwrap(), &123_u8);
                }
            });
        });

        barrier.wait();
        assert!(hashmap.remove_sync("first").is_some());
        assert!(task.join().is_ok());
    }

    #[test]
    fn from_iter() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);

        let workload_size = 256;
        let hashmap = (0..workload_size)
            .map(|k| (k / 2, R::new(&INST_CNT)))
            .collect::<HashMap<usize, R>>();
        assert_eq!(hashmap.len(), workload_size / 2);
        hashmap.clear_sync();
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[test]
    fn clone() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);

        let hashmap: HashMap<usize, R> = HashMap::default();
        let workload_size = 256;
        for k in 0..workload_size {
            assert!(hashmap.insert_sync(k, R::new(&INST_CNT)).is_ok());
        }
        let hashmap_clone = hashmap.clone();
        hashmap.clear_sync();
        for k in 0..workload_size {
            assert!(hashmap_clone.read_sync(&k, |_, _| ()).is_some());
        }
        hashmap_clone.clear_sync();
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[test]
    fn compare() {
        let hashmap1: HashMap<String, usize> = HashMap::new();
        let hashmap2: HashMap<String, usize> = HashMap::new();
        assert_eq!(hashmap1, hashmap2);

        assert!(hashmap1.insert_sync("Hi".to_string(), 1).is_ok());
        assert_ne!(hashmap1, hashmap2);

        assert!(hashmap2.insert_sync("Hello".to_string(), 2).is_ok());
        assert_ne!(hashmap1, hashmap2);

        assert!(hashmap1.insert_sync("Hello".to_string(), 2).is_ok());
        assert_ne!(hashmap1, hashmap2);

        assert!(hashmap2.insert_sync("Hi".to_string(), 1).is_ok());
        assert_eq!(hashmap1, hashmap2);

        assert!(hashmap1.remove_sync("Hi").is_some());
        assert_ne!(hashmap1, hashmap2);
    }

    #[test]
    fn string_key() {
        let num_iter = if cfg!(miri) { 4 } else { 4096 };
        let hashmap1: HashMap<String, u32> = HashMap::default();
        let hashmap2: HashMap<u32, String> = HashMap::default();
        let mut checker1 = BTreeSet::new();
        let mut checker2 = BTreeSet::new();
        let mut runner = TestRunner::default();
        for i in 0..num_iter {
            let prop_str = "[a-z]{1,16}".new_tree(&mut runner).unwrap();
            let str_val = prop_str.current();
            if hashmap1.insert_sync(str_val.clone(), i).is_ok() {
                checker1.insert((str_val.clone(), i));
            }
            let str_borrowed = str_val.as_str();
            assert!(hashmap1.contains_sync(str_borrowed));
            assert!(hashmap1.read_sync(str_borrowed, |_, _| ()).is_some());

            if hashmap2.insert_sync(i, str_val.clone()).is_ok() {
                checker2.insert((i, str_val.clone()));
            }
        }
        assert_eq!(hashmap1.len(), checker1.len());
        assert_eq!(hashmap2.len(), checker2.len());
        for iter in checker1 {
            let v = hashmap1.remove_sync(iter.0.as_str());
            assert_eq!(v.unwrap().1, iter.1);
        }
        for iter in checker2 {
            let e = hashmap2.entry_sync(iter.0);
            match e {
                Entry::Occupied(o) => assert_eq!(o.remove(), iter.1),
                Entry::Vacant(_) => unreachable!(),
            }
        }
        assert_eq!(hashmap1.len(), 0);
        assert_eq!(hashmap2.len(), 0);
    }

    #[test]
    fn local_ref() {
        struct L<'a>(&'a AtomicUsize);
        impl<'a> L<'a> {
            fn new(cnt: &'a AtomicUsize) -> Self {
                cnt.fetch_add(1, Relaxed);
                L(cnt)
            }
        }
        impl Drop for L<'_> {
            fn drop(&mut self) {
                self.0.fetch_sub(1, Relaxed);
            }
        }

        let workload_size = 256;
        let cnt = AtomicUsize::new(0);
        let hashmap: HashMap<usize, L> = HashMap::default();

        for k in 0..workload_size {
            assert!(hashmap.insert_sync(k, L::new(&cnt)).is_ok());
        }
        hashmap.retain_sync(|k, _| {
            assert!(*k < workload_size);
            true
        });
        assert_eq!(cnt.load(Relaxed), workload_size);

        for k in 0..workload_size / 2 {
            assert!(hashmap.remove_sync(&k).is_some());
        }
        hashmap.retain_sync(|k, _| {
            assert!(*k >= workload_size / 2);
            true
        });
        assert_eq!(cnt.load(Relaxed), workload_size / 2);

        drop(hashmap);
        assert_eq!(cnt.load(Relaxed), 0);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn insert_async() {
        for _ in 0..64 {
            let hashmap: Arc<HashMap<usize, usize>> = Arc::new(HashMap::default());
            let num_tasks = 8;
            let workload_size = 256;
            let mut tasks = Vec::with_capacity(num_tasks);
            let barrier = Arc::new(AsyncBarrier::new(num_tasks));
            for task_id in 0..num_tasks {
                let barrier = barrier.clone();
                let hashmap = hashmap.clone();
                tasks.push(tokio::task::spawn(async move {
                    barrier.wait().await;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashmap.insert_async(id, id).await;
                        assert!(result.is_ok());
                    }
                    for id in range.clone() {
                        let result = hashmap.insert_async(id, id).await;
                        assert_eq!(result, Err((id, id)));
                    }
                }));
            }

            for task in join_all(tasks).await {
                assert!(task.is_ok());
            }

            assert_eq!(hashmap.len(), num_tasks * workload_size);
        }
    }

    #[test]
    fn insert_sync() {
        let num_threads = if cfg!(miri) { 2 } else { 8 };
        let num_iters = if cfg!(miri) { 1 } else { 64 };
        for _ in 0..num_iters {
            let hashmap: Arc<HashMap<usize, usize>> = Arc::new(HashMap::default());
            let workload_size = 256;
            let mut threads = Vec::with_capacity(num_threads);
            let barrier = Arc::new(Barrier::new(num_threads));
            for thread_id in 0..num_threads {
                let barrier = barrier.clone();
                let hashmap = hashmap.clone();
                threads.push(thread::spawn(move || {
                    barrier.wait();
                    let range = (thread_id * workload_size)..((thread_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashmap.insert_sync(id, id);
                        assert!(result.is_ok());
                    }
                    for id in range.clone() {
                        let result = hashmap.insert_sync(id, id);
                        assert_eq!(result, Err((id, id)));
                    }
                }));
            }

            for thread in threads {
                assert!(thread.join().is_ok());
            }

            assert_eq!(hashmap.len(), num_threads * workload_size);
        }
    }
    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn insert_duplicate_async() {
        let num_tasks = 8;
        let num_iters = 64;
        for _ in 0..num_iters {
            let hashmap: Arc<HashMap<usize, usize>> = Arc::new(HashMap::default());
            let workload_size = 16 + num_iters;
            let mut tasks = Vec::with_capacity(num_tasks);
            let barrier = Arc::new(AsyncBarrier::new(num_tasks));
            for task_id in 0..num_tasks {
                let barrier = barrier.clone();
                let hashmap = hashmap.clone();
                tasks.push(tokio::task::spawn(async move {
                    barrier.wait().await;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    for id in range.clone() {
                        assert!(hashmap.insert_async(id, id).await.is_ok());
                        tokio::task::yield_now().await;
                        assert!(hashmap.insert_async(id, id).await.is_err());
                        let _result: Result<(), (usize, usize)> =
                            hashmap.insert_async(num_tasks * workload_size, 0).await;
                    }
                }));
            }
            for task in join_all(tasks).await {
                assert!(task.is_ok());
            }
            assert_eq!(hashmap.len(), num_tasks * workload_size + 1);
        }
    }

    #[test]
    fn insert_duplicate_sync() {
        let num_threads = if cfg!(miri) { 2 } else { 8 };
        let num_iters = if cfg!(miri) { 1 } else { 64 };
        for _ in 0..num_iters {
            let hashmap: Arc<HashMap<usize, usize>> = Arc::new(HashMap::default());
            let workload_size = 16 + num_iters;
            let mut threads = Vec::with_capacity(num_threads);
            let barrier = Arc::new(Barrier::new(num_threads));
            for thread_id in 0..num_threads {
                let barrier = barrier.clone();
                let hashmap = hashmap.clone();
                threads.push(thread::spawn(move || {
                    barrier.wait();
                    let range = (thread_id * workload_size)..((thread_id + 1) * workload_size);
                    for id in range.clone() {
                        assert!(hashmap.insert_sync(id, id).is_ok());
                        thread::yield_now();
                        assert!(hashmap.insert_sync(id, id).is_err());
                        let _result: Result<(), (usize, usize)> =
                            hashmap.insert_sync(num_threads * workload_size, 0);
                    }
                }));
            }
            for thread in threads {
                assert!(thread.join().is_ok());
            }
            assert_eq!(hashmap.len(), num_threads * workload_size + 1);
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn insert_update_read_remove_async() {
        let hashmap: Arc<HashMap<usize, usize>> = Arc::new(HashMap::default());
        let num_tasks = 8;
        let workload_size = 256;
        let mut tasks = Vec::with_capacity(num_tasks);
        let barrier = Arc::new(AsyncBarrier::new(num_tasks));
        for task_id in 0..num_tasks {
            let barrier = barrier.clone();
            let hashmap = hashmap.clone();
            tasks.push(tokio::task::spawn(async move {
                barrier.wait().await;
                let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                for id in range.clone() {
                    let result = hashmap.update_async(&id, |_, _| 1).await;
                    assert!(result.is_none());
                }
                for id in range.clone() {
                    if id % 10 == 0 {
                        hashmap.entry_async(id).await.or_insert(id);
                    } else if id % 3 == 0 {
                        let entry = hashmap.entry_async(id).await;
                        let o = match entry {
                            Entry::Occupied(mut o) => {
                                *o.get_mut() = id;
                                o
                            }
                            Entry::Vacant(v) => v.insert_entry(id),
                        };
                        assert_eq!(*o.get(), id);
                    } else {
                        let result = hashmap.insert_async(id, id).await;
                        assert!(result.is_ok());
                    }
                }
                for id in range.clone() {
                    if id % 7 == 4 {
                        let entry = hashmap.entry_async(id).await;
                        match entry {
                            Entry::Occupied(mut o) => {
                                *o.get_mut() += 1;
                            }
                            Entry::Vacant(v) => {
                                v.insert_entry(id);
                            }
                        }
                    } else {
                        let result = hashmap
                            .update_async(&id, |_, v| {
                                *v += 1;
                                *v
                            })
                            .await;
                        assert_eq!(result, Some(id + 1));
                    }
                }
                for id in range.clone() {
                    let result = hashmap.read_async(&id, |_, v| *v).await;
                    assert_eq!(result, Some(id + 1));
                    assert_eq!(*hashmap.get_async(&id).await.unwrap().get(), id + 1);
                }
                for id in range.clone() {
                    let result = hashmap.remove_if_async(&id, |v| *v == id + 1).await;
                    assert_eq!(result, Some((id, id + 1)));
                    assert!(hashmap.read_async(&id, |_, v| *v).await.is_none());
                    assert!(hashmap.get_async(&id).await.is_none());
                }
                for id in range {
                    let result = hashmap.remove_if_async(&id, |v| *v == id + 1).await;
                    assert_eq!(result, None);
                }
            }));
        }

        for task in join_all(tasks).await {
            assert!(task.is_ok());
        }

        assert_eq!(hashmap.len(), 0);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn entry_read_next_async() {
        let hashmap: Arc<HashMap<usize, usize>> = Arc::new(HashMap::default());
        for _ in 0..64 {
            let num_tasks = 8;
            let workload_size = 512;
            let mut tasks = Vec::with_capacity(num_tasks);
            let barrier = Arc::new(AsyncBarrier::new(num_tasks));
            for task_id in 0..num_tasks {
                let barrier = barrier.clone();
                let hashmap = hashmap.clone();
                tasks.push(tokio::task::spawn(async move {
                    barrier.wait().await;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashmap.insert_async(id, id).await;
                        assert!(result.is_ok());
                    }
                    for id in range.clone() {
                        assert!(hashmap.read_async(&id, |_, _| ()).await.is_some());
                    }

                    let mut in_range = 0;
                    let mut entry = hashmap.begin_async().await;
                    while let Some(current_entry) = entry.take() {
                        if range.contains(current_entry.key()) {
                            in_range += 1;
                        }
                        entry = current_entry.next_async().await;
                    }
                    assert!(in_range >= workload_size, "{in_range} {workload_size}");

                    let mut removed = 0;
                    hashmap
                        .retain_async(|k, _| {
                            if range.contains(k) {
                                removed += 1;
                                false
                            } else {
                                true
                            }
                        })
                        .await;
                    assert_eq!(removed, workload_size);

                    let mut entry = hashmap.begin_async().await;
                    while let Some(current_entry) = entry.take() {
                        assert!(!range.contains(current_entry.key()));
                        entry = current_entry.next_async().await;
                    }
                }));
            }

            for task in join_all(tasks).await {
                assert!(task.is_ok());
            }

            assert_eq!(hashmap.len(), 0);
        }
    }

    #[test]
    fn entry_read_next_sync() {
        let num_iter = if cfg!(miri) { 1 } else { 64 };
        let hashmap: Arc<HashMap<usize, usize>> = Arc::new(HashMap::default());
        for _ in 0..num_iter {
            let num_threads = if cfg!(miri) { 2 } else { 8 };
            let workload_size = 256;
            let mut threads = Vec::with_capacity(num_threads);
            let barrier = Arc::new(Barrier::new(num_threads));
            for thread_id in 0..num_threads {
                let barrier = barrier.clone();
                let hashmap = hashmap.clone();
                threads.push(thread::spawn(move || {
                    barrier.wait();
                    let range = (thread_id * workload_size)..((thread_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashmap.insert_sync(id, id);
                        assert!(result.is_ok());
                    }
                    for id in range.clone() {
                        assert!(hashmap.read_sync(&id, |_, _| ()).is_some());
                    }

                    let mut in_range = 0;
                    let mut entry = hashmap.begin_sync();
                    while let Some(current_entry) = entry.take() {
                        if range.contains(current_entry.key()) {
                            in_range += 1;
                        }
                        entry = current_entry.next_sync();
                    }
                    assert!(in_range >= workload_size, "{in_range} {workload_size}");

                    let mut removed = 0;
                    hashmap.retain_sync(|k, _| {
                        if range.contains(k) {
                            removed += 1;
                            false
                        } else {
                            true
                        }
                    });
                    assert_eq!(removed, workload_size);

                    let mut entry = hashmap.begin_sync();
                    while let Some(current_entry) = entry.take() {
                        assert!(!range.contains(current_entry.key()));
                        entry = current_entry.next_sync();
                    }
                }));
            }

            for thread in threads {
                assert!(thread.join().is_ok());
            }

            assert_eq!(hashmap.len(), 0);
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn insert_any_async() {
        let hashmap: Arc<HashMap<usize, usize>> = Arc::new(HashMap::default());
        for _ in 0..64 {
            let num_tasks = 8;
            let workload_size = 512;
            let mut tasks = Vec::with_capacity(num_tasks);
            let barrier = Arc::new(AsyncBarrier::new(num_tasks));
            for task_id in 0..num_tasks {
                let barrier = barrier.clone();
                let hashmap = hashmap.clone();
                tasks.push(tokio::task::spawn(async move {
                    barrier.wait().await;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashmap.insert_async(id, id).await;
                        assert!(result.is_ok());
                    }
                    for id in range.clone() {
                        let result = hashmap.insert_async(id, id).await;
                        assert_eq!(result, Err((id, id)));
                    }
                    let mut iterated = 0;
                    hashmap
                        .retain_async(|k, _| {
                            if range.contains(k) {
                                iterated += 1;
                            }
                            true
                        })
                        .await;
                    assert!(iterated >= workload_size);

                    let mut removed = 0;
                    hashmap
                        .retain_async(|k, _| {
                            if range.contains(k) {
                                removed += 1;
                                false
                            } else {
                                true
                            }
                        })
                        .await;
                    assert_eq!(removed, workload_size);
                    assert!(hashmap.iter_async(|k, _| !range.contains(k)).await);
                }));
            }

            for task in join_all(tasks).await {
                assert!(task.is_ok());
            }

            assert_eq!(hashmap.len(), 0);
        }
    }

    #[test]
    fn insert_any_sync() {
        let num_iter = if cfg!(miri) { 2 } else { 64 };
        let hashmap: Arc<HashMap<usize, usize>> = Arc::new(HashMap::default());
        for _ in 0..num_iter {
            let num_threads = if cfg!(miri) { 2 } else { 8 };
            let workload_size = 256;
            let mut threads = Vec::with_capacity(num_threads);
            let barrier = Arc::new(Barrier::new(num_threads));
            for thread_id in 0..num_threads {
                let barrier = barrier.clone();
                let hashmap = hashmap.clone();
                threads.push(thread::spawn(move || {
                    barrier.wait();
                    let range = (thread_id * workload_size)..((thread_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashmap.insert_sync(id, id);
                        assert!(result.is_ok());
                    }
                    for id in range.clone() {
                        let result = hashmap.insert_sync(id, id);
                        assert_eq!(result, Err((id, id)));
                    }
                    let mut iterated = 0;
                    hashmap.retain_sync(|k, _| {
                        if range.contains(k) {
                            iterated += 1;
                        }
                        true
                    });
                    assert!(iterated >= workload_size);

                    let mut removed = 0;
                    hashmap.retain_sync(|k, _| {
                        if range.contains(k) {
                            removed += 1;
                            false
                        } else {
                            true
                        }
                    });
                    assert_eq!(removed, workload_size);
                    assert!(hashmap.iter_sync(|k, _| !range.contains(k)));
                }));
            }

            for thread in threads {
                assert!(thread.join().is_ok());
            }

            assert_eq!(hashmap.len(), 0);
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn insert_retain_remove_async() {
        let data_size = 4096;
        for _ in 0..16 {
            let hashmap: Arc<HashMap<u64, u64>> = Arc::new(HashMap::default());
            let hashmap_clone = hashmap.clone();
            let barrier = Arc::new(AsyncBarrier::new(2));
            let barrier_clone = barrier.clone();
            let inserted = Arc::new(AtomicU64::new(0));
            let inserted_clone = inserted.clone();
            let removed = Arc::new(AtomicU64::new(data_size));
            let removed_clone = removed.clone();
            let task = tokio::task::spawn(async move {
                // test insert
                barrier_clone.wait().await;
                let mut iterated = 0;
                let mut checker = BTreeSet::new();
                let mut max = inserted_clone.load(Acquire);
                hashmap_clone.retain_sync(|k, _| {
                    iterated += 1;
                    checker.insert(*k);
                    true
                });
                for key in 0..max {
                    assert!(checker.contains(&key));
                }

                barrier_clone.wait().await;
                iterated = 0;
                checker = BTreeSet::new();
                max = inserted_clone.load(Acquire);
                hashmap_clone
                    .retain_async(|k, _| {
                        iterated += 1;
                        checker.insert(*k);
                        true
                    })
                    .await;
                for key in 0..max {
                    assert!(checker.contains(&key));
                }

                // test remove
                barrier_clone.wait().await;
                iterated = 0;
                max = removed_clone.load(Acquire);
                hashmap_clone.retain_sync(|k, _| {
                    iterated += 1;
                    assert!(*k < max);
                    true
                });

                barrier_clone.wait().await;
                iterated = 0;
                max = removed_clone.load(Acquire);
                hashmap_clone
                    .retain_async(|k, _| {
                        iterated += 1;
                        assert!(*k < max);
                        true
                    })
                    .await;
            });

            // insert
            barrier.wait().await;
            for i in 0..data_size {
                if i == data_size / 2 {
                    barrier.wait().await;
                }
                assert!(hashmap.insert_sync(i, i).is_ok());
                inserted.store(i, Release);
            }

            // remove
            barrier.wait().await;
            for i in (0..data_size).rev() {
                if i == data_size / 2 {
                    barrier.wait().await;
                }
                assert!(hashmap.remove_sync(&i).is_some());
                removed.store(i, Release);
            }

            assert!(task.await.is_ok());
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn insert_prune_any_async() {
        let hashmap: Arc<HashMap<usize, usize>> = Arc::new(HashMap::default());
        for _ in 0..256 {
            let num_tasks = 8;
            let workload_size = 256;
            let mut tasks = Vec::with_capacity(num_tasks);
            let barrier = Arc::new(AsyncBarrier::new(num_tasks));
            for task_id in 0..num_tasks {
                let barrier = barrier.clone();
                let hashmap = hashmap.clone();
                tasks.push(tokio::task::spawn(async move {
                    barrier.wait().await;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashmap.insert_async(id, id).await;
                        assert!(result.is_ok());
                    }
                    let mut removed = 0;
                    hashmap
                        .iter_mut_async(|entry| {
                            if range.contains(&entry.0) {
                                let (k, v) = entry.consume();
                                assert_eq!(k, v);
                                removed += 1;
                            }
                            true
                        })
                        .await;
                    assert_eq!(removed, workload_size);
                    assert!(hashmap.iter_async(|k, _| !range.contains(k)).await);
                }));
            }

            for task in join_all(tasks).await {
                assert!(task.is_ok());
            }

            assert_eq!(hashmap.len(), 0);
        }
    }

    #[test]
    fn insert_prune_any_sync() {
        let num_threads = if cfg!(miri) { 2 } else { 8 };
        let num_iters = if cfg!(miri) { 1 } else { 256 };
        let hashmap: Arc<HashMap<usize, usize>> = Arc::new(HashMap::default());
        for _ in 0..num_iters {
            let workload_size = 256;
            let mut threads = Vec::with_capacity(num_threads);
            let barrier = Arc::new(Barrier::new(num_threads));
            for thread_id in 0..num_threads {
                let barrier = barrier.clone();
                let hashmap = hashmap.clone();
                threads.push(thread::spawn(move || {
                    barrier.wait();
                    let range = (thread_id * workload_size)..((thread_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashmap.insert_sync(id, id);
                        assert!(result.is_ok());
                    }
                    let mut removed = 0;
                    hashmap.iter_mut_sync(|entry| {
                        if range.contains(&entry.0) {
                            let (k, v) = entry.consume();
                            assert_eq!(k, v);
                            removed += 1;
                        }
                        true
                    });
                    assert_eq!(removed, workload_size);
                    assert!(hashmap.iter_sync(|k, _| !range.contains(k)));
                }));
            }

            for thread in threads {
                assert!(thread.join().is_ok());
            }

            assert_eq!(hashmap.len(), 0);
        }
    }

    proptest! {
        #[cfg_attr(miri, ignore)]
        #[test]
        fn insert_prop(key in 0_usize..16) {
            let range = 256;
            let checker = Arc::new(AtomicUsize::new(0));
            let hashmap: HashMap<Data, Data> = HashMap::default();
            for d in key..(key + range) {
                assert!(
                    hashmap
                        .insert_sync(Data::new(d, checker.clone()), Data::new(d, checker.clone()))
                        .is_ok()
                );
                *hashmap
                    .entry_sync(Data::new(d, checker.clone()))
                    .or_insert(Data::new(d + 1, checker.clone()))
                    .get_mut() = Data::new(d + 2, checker.clone());
            }

            for d in (key + range)..(key + range + range) {
                assert!(
                    hashmap
                        .insert_sync(Data::new(d, checker.clone()), Data::new(d, checker.clone()))
                        .is_ok()
                );
                *hashmap
                    .entry_sync(Data::new(d, checker.clone()))
                    .or_insert(Data::new(d + 1, checker.clone()))
                    .get_mut() = Data::new(d + 2, checker.clone());
            }

            let mut removed = 0;
            hashmap.retain_sync(|k, _| {
                if k.data >= key + range {
                    removed += 1;
                    false
                } else {
                    true
                }
            });
            assert_eq!(removed, range);

            assert_eq!(hashmap.len(), range);
            let mut found_keys = 0;
            hashmap.retain_sync(|k, v| {
                assert!(k.data < key + range);
                assert!(v.data >= key);
                found_keys += 1;
                true
            });
            assert_eq!(found_keys, range);
            assert_eq!(checker.load(Relaxed), range * 2);
            for d in key..(key + range) {
                assert!(hashmap.contains_sync(&Data::new(d, checker.clone())));
            }
            for d in key..(key + range) {
                assert!(hashmap.remove_sync(&Data::new(d, checker.clone())).is_some());
            }
            assert_eq!(checker.load(Relaxed), 0);

            for d in key..(key + range) {
                assert!(
                    hashmap
                        .insert_sync(Data::new(d, checker.clone()), Data::new(d, checker.clone()))
                        .is_ok()
                );
                *hashmap
                    .entry_sync(Data::new(d, checker.clone()))
                    .or_insert(Data::new(d + 1, checker.clone()))
                    .get_mut() = Data::new(d + 2, checker.clone());
            }
            hashmap.clear_sync();
            assert_eq!(checker.load(Relaxed), 0);

            for d in key..(key + range) {
                assert!(
                    hashmap
                        .insert_sync(Data::new(d, checker.clone()), Data::new(d, checker.clone()))
                        .is_ok()
                );
                *hashmap
                    .entry_sync(Data::new(d, checker.clone()))
                    .or_insert(Data::new(d + 1, checker.clone()))
                    .get_mut() = Data::new(d + 2, checker.clone());
            }
            assert_eq!(checker.load(Relaxed), range * 2);
            drop(hashmap);
            assert_eq!(checker.load(Relaxed), 0);
        }
    }
}

mod hashindex {
    use std::collections::BTreeSet;
    use std::panic::UnwindSafe;
    use std::rc::Rc;
    use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
    use std::sync::atomic::{AtomicU64, AtomicUsize, fence};
    use std::sync::{Arc, Barrier};
    use std::thread;

    use futures::future::join_all;
    use proptest::strategy::{Strategy, ValueTree};
    use proptest::test_runner::TestRunner;
    use sdd::Guard;
    use tokio::sync::Barrier as AsyncBarrier;

    use super::common::{EqTest, R};
    use crate::HashIndex;
    use crate::hash_index::{self, Iter};

    static_assertions::assert_not_impl_any!(HashIndex<Rc<String>, Rc<String>>: Send, Sync);
    static_assertions::assert_not_impl_any!(hash_index::Entry<Rc<String>, Rc<String>>: Send, Sync);
    static_assertions::assert_impl_all!(HashIndex<String, String>: Send, Sync, UnwindSafe);
    static_assertions::assert_impl_all!(Iter<'static, String, String>: UnwindSafe);
    static_assertions::assert_not_impl_any!(HashIndex<String, *const String>: Send, Sync);
    static_assertions::assert_not_impl_any!(Iter<'static, String, *const String>: Send, Sync);

    #[test]
    fn equivalent() {
        let hashindex: HashIndex<EqTest, usize> = HashIndex::default();
        assert!(
            hashindex
                .insert_sync(EqTest("HELLO".to_owned(), 1), 1)
                .is_ok()
        );
        assert!(!hashindex.contains("NO"));
        assert!(hashindex.contains("HELLO"));
    }

    #[test]
    fn compare() {
        let hashindex1: HashIndex<String, usize> = HashIndex::new();
        let hashindex2: HashIndex<String, usize> = HashIndex::new();
        assert_eq!(hashindex1, hashindex2);

        assert!(hashindex1.insert_sync("Hi".to_string(), 1).is_ok());
        assert_ne!(hashindex1, hashindex2);

        assert!(hashindex2.insert_sync("Hello".to_string(), 2).is_ok());
        assert_ne!(hashindex1, hashindex2);

        assert!(hashindex1.insert_sync("Hello".to_string(), 2).is_ok());
        assert_ne!(hashindex1, hashindex2);

        assert!(hashindex2.insert_sync("Hi".to_string(), 1).is_ok());
        assert_eq!(hashindex1, hashindex2);

        assert!(hashindex1.remove_sync("Hi"));
        assert_ne!(hashindex1, hashindex2);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn clear_async() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);

        let workload_size = ((1_usize << 18) / 16) * 15;
        for _ in 0..2 {
            let hashindex: HashIndex<usize, R> = HashIndex::default();
            for k in 0..workload_size {
                assert!(hashindex.insert_async(k, R::new(&INST_CNT)).await.is_ok());
            }
            assert!(INST_CNT.load(Relaxed) >= workload_size);
            assert_eq!(hashindex.len(), workload_size);
            hashindex.clear_async().await;
            drop(hashindex);
            assert_eq!(INST_CNT.load(Relaxed), 0);
        }
    }

    #[test]
    fn clear_sync() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);

        for workload_size in 2..64 {
            let hashindex: HashIndex<usize, R> = HashIndex::default();
            for k in 0..workload_size {
                assert!(hashindex.insert_sync(k, R::new(&INST_CNT)).is_ok());
            }
            assert!(INST_CNT.load(Relaxed) >= workload_size);
            assert_eq!(hashindex.len(), workload_size);
            hashindex.clear_sync();
            drop(hashindex);
            assert_eq!(INST_CNT.load(Relaxed), 0);
        }
    }

    #[test]
    fn from_iter() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);

        let workload_size = 256;
        let hashindex = (0..workload_size)
            .map(|k| (k / 2, R::new(&INST_CNT)))
            .collect::<HashIndex<usize, R>>();
        assert_eq!(hashindex.len(), workload_size / 2);
        drop(hashindex);

        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[test]
    fn clone() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let hashindex: HashIndex<usize, R> = HashIndex::with_capacity(1024);

        let workload_size = 256;

        for k in 0..workload_size {
            assert!(hashindex.insert_sync(k, R::new(&INST_CNT)).is_ok());
        }
        let hashindex_clone = hashindex.clone();
        drop(hashindex);
        for k in 0..workload_size {
            assert!(hashindex_clone.peek_with(&k, |_, _| ()).is_some());
        }
        drop(hashindex_clone);
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[test]
    fn string_key() {
        let num_iter = if cfg!(miri) { 4 } else { 4096 };
        let hashindex1: HashIndex<String, u32> = HashIndex::default();
        let hashindex2: HashIndex<u32, String> = HashIndex::default();
        let mut checker1 = BTreeSet::new();
        let mut checker2 = BTreeSet::new();
        let mut runner = TestRunner::default();
        for i in 0..num_iter {
            let prop_str = "[a-z]{1,16}".new_tree(&mut runner).unwrap();
            let str_val = prop_str.current();
            if hashindex1.insert_sync(str_val.clone(), i).is_ok() {
                checker1.insert((str_val.clone(), i));
            }
            let str_borrowed = str_val.as_str();
            if !cfg!(miri) {
                // `Miri` complains about concurrent access to `partial_hash`.
                assert!(hashindex1.peek_with(str_borrowed, |_, _| ()).is_some());
            }

            if hashindex2.insert_sync(i, str_val.clone()).is_ok() {
                checker2.insert((i, str_val.clone()));
            }
        }
        assert_eq!(hashindex1.len(), checker1.len());
        assert_eq!(hashindex2.len(), checker2.len());
        for iter in checker1 {
            assert!(hashindex1.remove_sync(iter.0.as_str()));
        }
        for iter in checker2 {
            assert!(hashindex2.remove_sync(&iter.0));
        }
        assert_eq!(hashindex1.len(), 0);
        assert_eq!(hashindex2.len(), 0);
    }

    #[test]
    fn local_ref() {
        struct L<'a>(&'a AtomicUsize);
        impl<'a> L<'a> {
            fn new(cnt: &'a AtomicUsize) -> Self {
                cnt.fetch_add(1, Relaxed);
                L(cnt)
            }
        }
        impl Drop for L<'_> {
            fn drop(&mut self) {
                self.0.fetch_sub(1, Relaxed);
            }
        }

        let workload_size = 256;
        let cnt = AtomicUsize::new(0);
        let hashindex: HashIndex<usize, L> = HashIndex::default();

        for k in 0..workload_size {
            assert!(hashindex.insert_sync(k, L::new(&cnt)).is_ok());
        }
        hashindex.retain_sync(|k, _| {
            assert!(*k < workload_size);
            true
        });
        assert_eq!(cnt.load(Relaxed), workload_size);

        for k in 0..workload_size / 2 {
            assert!(hashindex.remove_sync(&k));
        }
        hashindex.retain_sync(|k, _| {
            assert!(*k >= workload_size / 2);
            true
        });

        drop(hashindex);
        assert_eq!(cnt.load(Relaxed), 0);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn insert_peek_remove_async() {
        let hashindex: Arc<HashIndex<usize, usize>> = Arc::new(HashIndex::default());
        let num_tasks = 4;
        let workload_size = 1024 * 1024;

        for k in 0..num_tasks {
            assert!(hashindex.insert_async(k, k).await.is_ok());
        }

        let mut tasks = Vec::with_capacity(num_tasks);
        let barrier = Arc::new(AsyncBarrier::new(num_tasks));
        for task_id in 0..num_tasks {
            let barrier = barrier.clone();
            let hashindex = hashindex.clone();
            tasks.push(tokio::task::spawn(async move {
                barrier.wait().await;
                if task_id == 0 {
                    for k in num_tasks..workload_size {
                        assert!(hashindex.insert_async(k, k).await.is_ok());
                    }
                    for k in num_tasks..workload_size {
                        assert!(hashindex.remove_async(&k).await);
                    }
                } else {
                    for k in 0..num_tasks {
                        assert!(hashindex.peek_with(&k, |_, _| ()).is_some());
                    }
                }
            }));
        }

        for task in join_all(tasks).await {
            assert!(task.is_ok());
        }
    }

    #[test]
    fn insert_peek_remove_sync() {
        let hashindex: Arc<HashIndex<usize, usize>> = Arc::new(HashIndex::default());
        let num_threads = if cfg!(miri) { 2 } else { 4 };
        let workload_size = if cfg!(miri) { 1024 } else { 1024 * 1024 };

        for k in 0..num_threads {
            assert!(hashindex.insert_sync(k, k).is_ok());
        }

        let mut threads = Vec::with_capacity(num_threads);
        let barrier = Arc::new(Barrier::new(num_threads));
        for task_id in 0..num_threads {
            let barrier = barrier.clone();
            let hashindex = hashindex.clone();
            threads.push(thread::spawn(move || {
                barrier.wait();
                if task_id == 0 {
                    for k in num_threads..workload_size {
                        assert!(hashindex.insert_sync(k, k).is_ok());
                    }
                    for k in num_threads..workload_size {
                        assert!(hashindex.remove_sync(&k));
                    }
                } else if !cfg!(miri) {
                    // See notes about `Miri` in `peek*`.
                    for k in 0..num_threads {
                        assert!(hashindex.peek_with(&k, |_, _| ()).is_some());
                    }
                }
            }));
        }

        for thread in threads {
            assert!(thread.join().is_ok());
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn peek_remove_insert_async() {
        let hashindex: Arc<HashIndex<usize, String>> = Arc::new(HashIndex::default());
        let num_tasks = 4;
        let num_iter = 64;
        let workload_size = 256;

        let str = "HOW ARE YOU HOW ARE YOU";
        for k in 0..num_tasks * workload_size {
            assert!(hashindex.insert_async(k, str.to_string()).await.is_ok());
        }

        let mut tasks = Vec::with_capacity(num_tasks);
        let barrier = Arc::new(AsyncBarrier::new(num_tasks));
        for task_id in 0..num_tasks {
            let barrier = barrier.clone();
            let hashindex = hashindex.clone();
            tasks.push(tokio::task::spawn(async move {
                barrier.wait().await;
                let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                if task_id == 0 {
                    for _ in 0..num_iter {
                        let v = {
                            let guard = Guard::new();
                            let v = hashindex.peek(&(task_id * workload_size), &guard).unwrap();
                            assert_eq!(str, v);
                            v.to_owned()
                        };
                        fence(Acquire);
                        for id in range.clone() {
                            assert!(hashindex.remove_async(&id).await);
                            assert!(hashindex.insert_async(id, str.to_string()).await.is_ok());
                        }
                        fence(Acquire);
                        assert_eq!(str, v);
                    }
                } else {
                    for _ in 0..num_iter {
                        for id in range.clone() {
                            assert!(hashindex.remove_async(&id).await);
                            assert!(hashindex.insert_async(id, str.to_string()).await.is_ok());
                        }
                    }
                }
            }));
        }

        for task in join_all(tasks).await {
            assert!(task.is_ok());
        }

        assert_eq!(hashindex.len(), num_tasks * workload_size);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn rebuild_async() {
        let hashindex: Arc<HashIndex<usize, usize>> = Arc::new(HashIndex::default());
        let num_tasks = 4;
        let num_iter = 64;
        let workload_size = 256;

        for k in 0..num_tasks * workload_size {
            assert!(hashindex.insert_sync(k, k).is_ok());
        }

        let mut tasks = Vec::with_capacity(num_tasks);
        let barrier = Arc::new(AsyncBarrier::new(num_tasks));
        for task_id in 0..num_tasks {
            let barrier = barrier.clone();
            let hashindex = hashindex.clone();
            tasks.push(tokio::task::spawn(async move {
                barrier.wait().await;
                let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                for _ in 0..num_iter {
                    for id in range.clone() {
                        assert!(hashindex.remove_async(&id).await);
                        assert!(hashindex.insert_async(id, id).await.is_ok());
                    }
                }
            }));
        }

        for task in join_all(tasks).await {
            assert!(task.is_ok());
        }

        assert_eq!(hashindex.len(), num_tasks * workload_size);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn entry_read_next_async() {
        let hashindex: Arc<HashIndex<usize, usize>> = Arc::new(HashIndex::default());
        for _ in 0..64 {
            let num_tasks = 8;
            let workload_size = 512;
            let mut tasks = Vec::with_capacity(num_tasks);
            let barrier = Arc::new(AsyncBarrier::new(num_tasks));
            for task_id in 0..num_tasks {
                let barrier = barrier.clone();
                let hashindex = hashindex.clone();
                tasks.push(tokio::task::spawn(async move {
                    barrier.wait().await;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashindex.insert_async(id, id).await;
                        assert!(result.is_ok());
                    }
                    for id in range.clone() {
                        assert!(hashindex.peek_with(&id, |_, _| ()).is_some());
                    }

                    let mut in_range = 0;
                    let mut entry = hashindex.begin_async().await;
                    while let Some(current_entry) = entry.take() {
                        if range.contains(current_entry.key()) {
                            in_range += 1;
                        }
                        entry = current_entry.next_async().await;
                    }
                    assert!(in_range >= workload_size, "{in_range} {workload_size}");

                    let mut removed = 0;
                    hashindex
                        .retain_async(|k, _| {
                            if range.contains(k) {
                                removed += 1;
                                false
                            } else {
                                true
                            }
                        })
                        .await;
                    assert_eq!(removed, workload_size);

                    let mut entry = hashindex.begin_async().await;
                    while let Some(current_entry) = entry.take() {
                        assert!(!range.contains(current_entry.key()));
                        entry = current_entry.next_async().await;
                    }
                }));
            }

            for task in join_all(tasks).await {
                assert!(task.is_ok());
            }

            assert_eq!(hashindex.len(), 0);
        }
    }

    #[test]
    fn entry_read_next_sync() {
        let num_iter = if cfg!(miri) { 2 } else { 64 };
        let hashindex: Arc<HashIndex<usize, usize>> = Arc::new(HashIndex::default());
        for _ in 0..num_iter {
            let num_threads = if cfg!(miri) { 3 } else { 8 };
            let workload_size = 256;
            let mut threads = Vec::with_capacity(num_threads);
            let barrier = Arc::new(Barrier::new(num_threads));
            for thread_id in 0..num_threads {
                let barrier = barrier.clone();
                let hashindex = hashindex.clone();
                threads.push(thread::spawn(move || {
                    barrier.wait();
                    let range = (thread_id * workload_size)..((thread_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashindex.insert_sync(id, id);
                        assert!(result.is_ok());
                    }
                    for id in range.clone() {
                        if !cfg!(miri) {
                            // See notes about `Miri` in `peek*`.
                            assert!(hashindex.peek_with(&id, |_, _| ()).is_some());
                        }
                    }

                    let mut in_range = 0;
                    let mut entry = hashindex.begin_sync();
                    while let Some(current_entry) = entry.take() {
                        if range.contains(current_entry.key()) {
                            in_range += 1;
                        }
                        entry = current_entry.next_sync();
                    }
                    assert!(in_range >= workload_size, "{in_range} {workload_size}");

                    let mut removed = 0;
                    hashindex.retain_sync(|k, _| {
                        if range.contains(k) {
                            removed += 1;
                            false
                        } else {
                            true
                        }
                    });
                    assert_eq!(removed, workload_size);

                    let mut entry = hashindex.begin_sync();
                    while let Some(current_entry) = entry.take() {
                        assert!(!range.contains(current_entry.key()));
                        entry = current_entry.next_sync();
                    }
                }));
            }

            for thread in threads {
                assert!(thread.join().is_ok());
            }

            assert_eq!(hashindex.len(), 0);
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn insert_retain_remove_async() {
        let data_size = 4096;
        for _ in 0..16 {
            let hashindex: Arc<HashIndex<u64, u64>> = Arc::new(HashIndex::default());
            let hashindex_clone = hashindex.clone();
            let barrier = Arc::new(AsyncBarrier::new(2));
            let barrier_clone = barrier.clone();
            let inserted = Arc::new(AtomicU64::new(0));
            let inserted_clone = inserted.clone();
            let removed = Arc::new(AtomicU64::new(data_size));
            let removed_clone = removed.clone();
            let task = tokio::task::spawn(async move {
                // test insert
                barrier_clone.wait().await;
                let mut iterated = 0;
                let mut checker = BTreeSet::new();
                let mut max = inserted_clone.load(Acquire);
                hashindex_clone.retain_sync(|k, _| {
                    iterated += 1;
                    checker.insert(*k);
                    true
                });
                for key in 0..max {
                    assert!(checker.contains(&key));
                }

                barrier_clone.wait().await;
                iterated = 0;
                checker = BTreeSet::new();
                max = inserted_clone.load(Acquire);
                hashindex_clone
                    .retain_async(|k, _| {
                        iterated += 1;
                        checker.insert(*k);
                        true
                    })
                    .await;
                for key in 0..max {
                    assert!(checker.contains(&key));
                }

                // test remove
                barrier_clone.wait().await;
                iterated = 0;
                max = removed_clone.load(Acquire);
                hashindex_clone.retain_sync(|k, _| {
                    iterated += 1;
                    assert!(*k < max);
                    true
                });

                barrier_clone.wait().await;
                iterated = 0;
                max = removed_clone.load(Acquire);
                hashindex_clone
                    .retain_async(|k, _| {
                        iterated += 1;
                        assert!(*k < max);
                        true
                    })
                    .await;
            });

            // insert
            barrier.wait().await;
            for i in 0..data_size {
                if i == data_size / 2 {
                    barrier.wait().await;
                }
                assert!(hashindex.insert_sync(i, i).is_ok());
                inserted.store(i, Release);
            }

            // remove
            barrier.wait().await;
            for i in (0..data_size).rev() {
                if i == data_size / 2 {
                    barrier.wait().await;
                }
                assert!(hashindex.remove_sync(&i));
                removed.store(i, Release);
            }

            assert!(task.await.is_ok());
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn update_get_async() {
        let hashindex: Arc<HashIndex<usize, usize>> = Arc::new(HashIndex::default());
        for _ in 0..256 {
            let num_tasks = 8;
            let workload_size = 256;
            let mut tasks = Vec::with_capacity(num_tasks);
            let barrier = Arc::new(AsyncBarrier::new(num_tasks));
            for task_id in 0..num_tasks {
                let barrier = barrier.clone();
                let hashindex = hashindex.clone();
                tasks.push(tokio::task::spawn(async move {
                    barrier.wait().await;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashindex.insert_async(id, id).await;
                        assert!(result.is_ok());
                        let entry = hashindex.get_async(&id).await.unwrap();
                        assert_eq!(*entry.get(), id);
                    }
                    for id in range.clone() {
                        hashindex.peek_with(&id, |k, v| assert_eq!(k, v));
                        let mut entry = hashindex.get_async(&id).await.unwrap();
                        assert_eq!(*entry.get(), id);
                        entry.update(usize::MAX);
                    }
                    for id in range.clone() {
                        hashindex.peek_with(&id, |_, v| assert_eq!(*v, usize::MAX));
                        let entry = hashindex.get_async(&id).await.unwrap();
                        assert_eq!(*entry.get(), usize::MAX);
                        entry.remove_entry();
                    }
                }));
            }

            for task in join_all(tasks).await {
                assert!(task.is_ok());
            }

            assert_eq!(hashindex.len(), 0);
        }
    }

    #[test]
    fn update_get_sync() {
        let num_threads = if cfg!(miri) { 2 } else { 8 };
        let num_iters = if cfg!(miri) { 2 } else { 256 };
        let hashindex: Arc<HashIndex<usize, usize>> = Arc::new(HashIndex::default());
        for _ in 0..num_iters {
            let workload_size = 256;
            let mut threads = Vec::with_capacity(num_threads);
            let barrier = Arc::new(Barrier::new(num_threads));
            for thread_id in 0..num_threads {
                let barrier = barrier.clone();
                let hashindex = hashindex.clone();
                threads.push(thread::spawn(move || {
                    barrier.wait();
                    let range = (thread_id * workload_size)..((thread_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashindex.insert_sync(id, id);
                        assert!(result.is_ok());
                        let entry = hashindex.get_sync(&id).unwrap();
                        assert_eq!(*entry.get(), id);
                    }
                    for id in range.clone() {
                        if !cfg!(miri) {
                            // See notes about `Miri` in `peek*`.
                            hashindex.peek_with(&id, |k, v| assert_eq!(k, v));
                        }
                        let mut entry = hashindex.get_sync(&id).unwrap();
                        assert_eq!(*entry.get(), id);
                        entry.update(usize::MAX);
                    }
                    for id in range.clone() {
                        if !cfg!(miri) {
                            // See notes about `Miri` in `peek*`.
                            hashindex.peek_with(&id, |_, v| assert_eq!(*v, usize::MAX));
                        }
                        let entry = hashindex.get_sync(&id).unwrap();
                        assert_eq!(*entry.get(), usize::MAX);
                        entry.remove_entry();
                    }
                }));
            }

            for thread in threads {
                assert!(thread.join().is_ok());
            }

            assert_eq!(hashindex.len(), 0);
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn insert_retain_async() {
        let hashindex: Arc<HashIndex<usize, usize>> = Arc::new(HashIndex::default());
        for _ in 0..256 {
            let num_tasks = 8;
            let workload_size = 256;
            let mut tasks = Vec::with_capacity(num_tasks);
            let barrier = Arc::new(AsyncBarrier::new(num_tasks));
            for task_id in 0..num_tasks {
                let barrier = barrier.clone();
                let hashindex = hashindex.clone();
                tasks.push(tokio::task::spawn(async move {
                    barrier.wait().await;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    for id in range.clone() {
                        let result = hashindex.insert_async(id, id).await;
                        assert!(result.is_ok());
                    }
                    for id in range.clone() {
                        let result = hashindex.insert_async(id, id).await;
                        assert_eq!(result, Err((id, id)));
                    }
                    let mut iterated = 0;
                    hashindex.iter(&Guard::new()).for_each(|(k, _)| {
                        if range.contains(k) {
                            iterated += 1;
                        }
                    });
                    assert!(iterated >= workload_size);
                    assert!(
                        hashindex
                            .iter(&Guard::new())
                            .any(|(k, _)| range.contains(k))
                    );

                    let mut removed = 0;
                    hashindex
                        .retain_async(|k, _| {
                            if range.contains(k) {
                                removed += 1;
                                false
                            } else {
                                true
                            }
                        })
                        .await;
                    assert_eq!(removed, workload_size);
                    assert!(
                        !hashindex
                            .iter(&Guard::new())
                            .any(|(k, _)| range.contains(k))
                    );
                }));
            }

            for task in join_all(tasks).await {
                assert!(task.is_ok());
            }

            assert_eq!(hashindex.len(), 0);
        }
    }
}

mod hashset {
    use std::panic::UnwindSafe;
    use std::rc::Rc;

    use super::common::EqTest;
    use crate::HashSet;

    static_assertions::assert_not_impl_any!(HashSet<Rc<String>>: Send, Sync);
    static_assertions::assert_impl_all!(HashSet<String>: Send, Sync, UnwindSafe);
    static_assertions::assert_not_impl_any!(HashSet<*const String>: Send, Sync);

    #[test]
    fn equivalent() {
        let hashset: HashSet<EqTest> = HashSet::default();
        assert!(hashset.insert_sync(EqTest("HELLO".to_owned(), 1)).is_ok());
        assert!(!hashset.contains_sync("NO"));
        assert!(hashset.contains_sync("HELLO"));
    }

    #[test]
    fn from_iter() {
        let workload_size = 256;
        let hashset = (0..workload_size)
            .map(|k| k / 2)
            .collect::<HashSet<usize>>();
        assert_eq!(hashset.len(), workload_size / 2);
    }

    #[test]
    fn compare() {
        let hashset1: HashSet<String> = HashSet::new();
        let hashset2: HashSet<String> = HashSet::new();
        assert_eq!(hashset1, hashset2);

        assert!(hashset1.insert_sync("Hi".to_string()).is_ok());
        assert_ne!(hashset1, hashset2);

        assert!(hashset2.insert_sync("Hello".to_string()).is_ok());
        assert_ne!(hashset1, hashset2);

        assert!(hashset1.insert_sync("Hello".to_string()).is_ok());
        assert_ne!(hashset1, hashset2);

        assert!(hashset2.insert_sync("Hi".to_string()).is_ok());
        assert_eq!(hashset1, hashset2);

        assert!(hashset1.remove_sync("Hi").is_some());
        assert_ne!(hashset1, hashset2);
    }
}

mod hashcache {
    use futures::future::join_all;
    use std::panic::UnwindSafe;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering::Relaxed;
    use tokio::sync::Barrier as AsyncBarrier;

    use proptest::prelude::*;

    use super::common::{EqTest, MaybeEq, R};
    use crate::HashCache;
    use crate::hash_cache::{self, ReplaceResult};

    static_assertions::assert_not_impl_any!(HashCache<Rc<String>, Rc<String>>: Send, Sync);
    static_assertions::assert_not_impl_any!(hash_cache::Entry<Rc<String>, Rc<String>>: Send, Sync);
    static_assertions::assert_impl_all!(HashCache<String, String>: Send, Sync, UnwindSafe);
    static_assertions::assert_not_impl_any!(HashCache<String, *const String>: Send, Sync);
    static_assertions::assert_impl_all!(hash_cache::OccupiedEntry<String, String>: Send, Sync);
    static_assertions::assert_not_impl_any!(hash_cache::OccupiedEntry<String, *const String>: Send, Sync, UnwindSafe);
    static_assertions::assert_impl_all!(hash_cache::VacantEntry<String, String>: Send, Sync);
    static_assertions::assert_not_impl_any!(hash_cache::VacantEntry<String, *const String>: Send, Sync, UnwindSafe);

    #[test]
    fn equivalent() {
        let hashcache: HashCache<EqTest, usize> = HashCache::default();
        assert!(hashcache.put_sync(EqTest("HELLO".to_owned(), 1), 1).is_ok());
        assert!(!hashcache.contains_sync("NO"));
        assert!(hashcache.contains_sync("HELLO"));
    }

    #[test]
    fn put_drop() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let hashcache: HashCache<usize, R> = HashCache::default();

        let workload_size = 256;
        for k in 0..workload_size {
            assert!(hashcache.put_sync(k, R::new(&INST_CNT)).is_ok());
        }
        assert!(INST_CNT.load(Relaxed) <= hashcache.capacity());
        drop(hashcache);
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn put_drop_async() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let hashcache: HashCache<usize, R> = HashCache::default();

        let workload_size = 1024;
        for k in 0..workload_size {
            assert!(hashcache.put_async(k, R::new(&INST_CNT)).await.is_ok());
        }
        assert!(INST_CNT.load(Relaxed) <= hashcache.capacity());
        drop(hashcache);
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn replace_async() {
        let hashcache: HashCache<MaybeEq, u64> = HashCache::default();
        let workload_size = 256;
        for k in 0..workload_size {
            let ReplaceResult::NotReplaced(v) = hashcache.replace_async(MaybeEq(k, 0)).await else {
                unreachable!();
            };
            v.put_entry(k);
        }
        for k in 0..workload_size {
            let ReplaceResult::Replaced(o, p) = hashcache.replace_async(MaybeEq(k, 1)).await else {
                continue;
            };
            assert_eq!(p.1, 0);
            assert_eq!(o.get(), &k);
        }
        assert!(hashcache.iter_async(|k, _| k.1 == 1).await);
    }

    #[test]
    fn replace_sync() {
        let hashcache: HashCache<MaybeEq, u64> = HashCache::default();
        let workload_size = 256;
        for k in 0..workload_size {
            let ReplaceResult::NotReplaced(v) = hashcache.replace_sync(MaybeEq(k, 0)) else {
                unreachable!();
            };
            v.put_entry(k);
        }
        for k in 0..workload_size {
            let ReplaceResult::Replaced(o, p) = hashcache.replace_sync(MaybeEq(k, 1)) else {
                continue;
            };
            assert_eq!(p.1, 0);
            assert_eq!(o.get(), &k);
        }
        assert!(hashcache.iter_sync(|k, _| k.1 == 1));
    }

    #[test]
    fn put_full_clear_put() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let capacity = 256;
        let hashcache: HashCache<usize, R> = HashCache::with_capacity(capacity, capacity);

        let mut max_key = 0;
        for k in 0..=capacity {
            if let Ok(Some(_)) = hashcache.put_sync(k, R::new(&INST_CNT)) {
                max_key = k;
                break;
            }
        }

        hashcache.clear_sync();
        for k in 0..=capacity {
            if let Ok(Some(_)) = hashcache.put_sync(k, R::new(&INST_CNT)) {
                assert_eq!(max_key, k);
                break;
            }
        }

        assert!(INST_CNT.load(Relaxed) <= hashcache.capacity());
        drop(hashcache);
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn put_full_clear_put_async() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let capacity = 256;
        let hashcache: HashCache<usize, R> = HashCache::with_capacity(capacity, capacity);

        let mut max_key = 0;
        for k in 0..=capacity {
            if let Ok(Some(_)) = hashcache.put_async(k, R::new(&INST_CNT)).await {
                max_key = k;
                break;
            }
        }

        for i in 0..4 {
            if i % 2 == 0 {
                hashcache.clear_async().await;
            } else {
                hashcache.clear_sync();
            }
            for k in 0..=capacity {
                if let Ok(Some(_)) = hashcache.put_async(k, R::new(&INST_CNT)).await {
                    assert_eq!(max_key, k);
                    break;
                }
            }
        }

        assert!(INST_CNT.load(Relaxed) <= hashcache.capacity());
        drop(hashcache);
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[test]
    fn put_retain_get() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let workload_size = 256;
        let retain_limit = 64;
        let hashcache: HashCache<usize, R> = HashCache::with_capacity(0, 256);

        for k in 0..workload_size {
            assert!(hashcache.put_sync(k, R::new(&INST_CNT)).is_ok());
        }

        hashcache.retain_sync(|k, _| *k <= retain_limit);
        for k in 0..workload_size {
            if hashcache.get_sync(&k).is_some() {
                assert!(k <= retain_limit);
            }
        }
        for k in 0..workload_size {
            if hashcache.put_sync(k, R::new(&INST_CNT)).is_err() {
                assert!(k <= retain_limit);
            }
        }

        hashcache.retain_sync(|k, _| *k > retain_limit);
        for k in 0..workload_size {
            if hashcache.get_sync(&k).is_some() {
                assert!(k > retain_limit);
            }
        }
        for k in 0..workload_size {
            if hashcache.put_sync(k, R::new(&INST_CNT)).is_err() {
                assert!(k > retain_limit);
            }
        }

        assert!(INST_CNT.load(Relaxed) <= hashcache.capacity());
        drop(hashcache);

        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[test]
    fn sparse_cache() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let hashcache = HashCache::<usize, R>::with_capacity(64, 64);
        for s in 0..16 {
            for k in s * 4..s * 4 + 4 {
                assert!(hashcache.put_sync(k, R::new(&INST_CNT)).is_ok());
            }
            hashcache.retain_sync(|k, _| *k % 2 == 0);
            for k in s * 4..s * 4 + 4 {
                if hashcache.put_sync(k, R::new(&INST_CNT)).is_err() {
                    assert!(k % 2 == 0);
                }
            }
            hashcache.clear_sync();
        }

        drop(hashcache);

        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn put_get_remove() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let hashcache: Arc<HashCache<usize, R>> = Arc::new(HashCache::default());
        for _ in 0..256 {
            let num_tasks = 8;
            let workload_size = 256;
            let mut tasks = Vec::with_capacity(num_tasks);
            let barrier = Arc::new(AsyncBarrier::new(num_tasks));
            for task_id in 0..num_tasks {
                let barrier = barrier.clone();
                let hashcache = hashcache.clone();
                tasks.push(tokio::task::spawn(async move {
                    barrier.wait().await;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    for id in range.clone() {
                        if id % 4 == 0 {
                            hashcache.entry_async(id).await.or_put(R::new(&INST_CNT));
                        } else {
                            assert!(hashcache.put_async(id, R::new(&INST_CNT)).await.is_ok());
                        }
                    }
                    let mut hit_count = 0;
                    for id in range.clone() {
                        let hit = hashcache.get_async(&id).await.is_some();
                        if hit {
                            hit_count += 1;
                        }
                    }
                    assert!(hit_count <= *hashcache.capacity_range().end());
                    let mut remove_count = 0;
                    for id in range.clone() {
                        let removed = hashcache.remove_async(&id).await.is_some();
                        if removed {
                            remove_count += 1;
                        }
                    }
                    assert!(remove_count <= hit_count);
                }));
            }

            for task in join_all(tasks).await {
                assert!(task.is_ok());
            }

            assert_eq!(hashcache.len(), 0);
        }
        drop(hashcache);
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn put_remove_maintain() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        for _ in 0..64 {
            let hashcache: Arc<HashCache<usize, R>> = Arc::new(HashCache::with_capacity(256, 1024));
            let num_tasks = 8;
            let workload_size = 2048;
            let mut tasks = Vec::with_capacity(num_tasks);
            let barrier = Arc::new(AsyncBarrier::new(num_tasks));
            let evicted = Arc::new(AtomicUsize::new(0));
            let removed = Arc::new(AtomicUsize::new(0));
            for task_id in 0..num_tasks {
                let barrier = barrier.clone();
                let evicted = evicted.clone();
                let removed = removed.clone();
                let hashcache = hashcache.clone();
                tasks.push(tokio::task::spawn(async move {
                    barrier.wait().await;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    let mut cnt = 0;
                    for key in range.clone() {
                        let result = hashcache.put_async(key, R::new(&INST_CNT)).await;
                        match result {
                            Ok(Some(_)) => cnt += 1,
                            Ok(_) => (),
                            Err(_) => unreachable!(),
                        }
                    }
                    evicted.fetch_add(cnt, Relaxed);
                    cnt = 0;
                    for key in range.clone() {
                        let result = hashcache.remove_async(&key).await;
                        if result.is_some() {
                            cnt += 1;
                        }
                    }
                    removed.fetch_add(cnt, Relaxed);
                }));
            }

            for task in join_all(tasks).await {
                assert!(task.is_ok());
            }
            assert_eq!(
                evicted.load(Relaxed) + removed.load(Relaxed),
                workload_size * num_tasks
            );
            assert_eq!(hashcache.len(), 0);
            assert_eq!(INST_CNT.load(Relaxed), 0);
        }
    }

    proptest! {
        #[cfg_attr(miri, ignore)]
        #[test]
        fn capacity(xs in 0_usize..256) {
            let hashcache: HashCache<usize, usize> = HashCache::with_capacity(0, 64);
            for k in 0..xs {
                assert!(hashcache.put_sync(k, k).is_ok());
            }
            assert!(hashcache.capacity() <= 64);

            let hashcache: HashCache<usize, usize> = HashCache::with_capacity(xs, xs * 2);
            for k in 0..xs {
                assert!(hashcache.put_sync(k, k).is_ok());
            }
            if xs == 0 {
                assert_eq!(hashcache.capacity_range(), 0..=64);
            } else {
                assert_eq!(hashcache.capacity_range(), xs.next_power_of_two().max(64)..=(xs * 2).next_power_of_two().max(64));
            }
         }
    }
}

mod treeindex {
    use std::borrow::Borrow;
    use std::cmp::Ordering;
    use std::collections::BTreeSet;
    use std::ops::RangeInclusive;
    use std::panic::UnwindSafe;
    use std::rc::Rc;
    use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
    use std::sync::atomic::{AtomicBool, AtomicUsize};
    use std::sync::{Arc, Barrier};
    use std::thread;

    use futures::future::join_all;
    use proptest::prelude::*;
    use proptest::strategy::ValueTree;
    use proptest::test_runner::TestRunner;
    use sdd::Guard;
    use sdd::suspend;
    use tokio::sync::Barrier as AsyncBarrier;
    use tokio::task;

    use super::common::R;
    use crate::tree_index::{Iter, Proximity, Range};
    use crate::{Comparable, Equivalent, TreeIndex};

    static_assertions::assert_not_impl_any!(TreeIndex<Rc<String>, Rc<String>>: Send, Sync);
    static_assertions::assert_impl_all!(TreeIndex<String, String>: Send, Sync, UnwindSafe);
    static_assertions::assert_impl_all!(Iter<'static, 'static, String, String>: UnwindSafe);
    static_assertions::assert_impl_all!(Range<'static, 'static, String, String, String, RangeInclusive<String>>: UnwindSafe);
    static_assertions::assert_not_impl_any!(TreeIndex<String, *const String>: Send, Sync);
    static_assertions::assert_not_impl_any!(Iter<'static, 'static, String, *const String>: Send, Sync);
    static_assertions::assert_not_impl_any!(Range<'static, 'static, String, *const String, String, RangeInclusive<String>>: Send, Sync);

    #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct CmpTest(String, usize);

    impl Comparable<CmpTest> for &str {
        fn compare(&self, key: &CmpTest) -> Ordering {
            self.cmp(&key.0.borrow())
        }
    }

    impl Equivalent<CmpTest> for &str {
        fn equivalent(&self, key: &CmpTest) -> bool {
            key.0.eq(self)
        }
    }

    #[test]
    fn comparable() {
        let tree: TreeIndex<CmpTest, usize> = TreeIndex::default();

        assert!(tree.insert_sync(CmpTest("A".to_owned(), 0), 0).is_ok());
        assert!(tree.peek_with(&"A", |_, _| true).unwrap());
        assert!(tree.insert_sync(CmpTest("B".to_owned(), 0), 0).is_ok());
        assert!(tree.peek_with(&"B", |_, _| true).unwrap());
        assert!(tree.insert_sync(CmpTest("C".to_owned(), 0), 0).is_ok());
        assert!(tree.peek_with(&"C", |_, _| true).unwrap());

        assert!(tree.insert_sync(CmpTest("Z".to_owned(), 0), 0).is_ok());
        assert!(tree.peek_with(&"Z", |_, _| true).unwrap());
        assert!(tree.insert_sync(CmpTest("Y".to_owned(), 0), 0).is_ok());
        assert!(tree.peek_with(&"Y", |_, _| true).unwrap());
        assert!(tree.insert_sync(CmpTest("X".to_owned(), 0), 0).is_ok());
        assert!(tree.peek_with(&"X", |_, _| true).unwrap());

        let guard = Guard::new();
        let mut range = tree.range("C".."Y", &guard);
        assert_eq!(range.next().unwrap().0.0, "C");
        assert_eq!(range.next_back().unwrap().0.0, "X");
        assert!(range.next_back().is_none());
        assert!(range.next().is_none());

        tree.remove_range_sync("C".."Y");

        assert!(tree.peek_with(&"A", |_, _| true).unwrap());
        assert!(tree.peek_with(&"B", |_, _| true).unwrap());
        assert!(tree.peek_with(&"C", |_, _| true).is_none());
        assert!(tree.peek_with(&"X", |_, _| true).is_none());
        assert!(tree.peek_with(&"Y", |_, _| true).unwrap());
        assert!(tree.peek_with(&"Z", |_, _| true).unwrap());
    }

    #[test]
    fn insert_drop() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let tree: TreeIndex<usize, R> = TreeIndex::default();

        let workload_size = 256;
        for k in 0..workload_size {
            assert!(tree.insert_sync(k, R::new(&INST_CNT)).is_ok());
        }
        assert!(INST_CNT.load(Relaxed) >= workload_size);
        assert_eq!(tree.len(), workload_size);
        drop(tree);

        while INST_CNT.load(Relaxed) != 0 {
            Guard::new().accelerate();
            thread::yield_now();
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn insert_drop_async() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let tree: TreeIndex<usize, R> = TreeIndex::default();

        let workload_size = 1024;
        for k in 0..workload_size {
            assert!(tree.insert_async(k, R::new(&INST_CNT)).await.is_ok());
        }
        assert!(INST_CNT.load(Relaxed) >= workload_size);
        assert_eq!(tree.len(), workload_size);
        drop(tree);

        while INST_CNT.load(Relaxed) != 0 {
            Guard::new().accelerate();
            tokio::task::yield_now().await;
        }
    }

    #[test]
    fn insert_remove() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let tree: TreeIndex<usize, R> = TreeIndex::default();

        let workload_size = 256;
        for k in 0..workload_size {
            assert!(tree.insert_sync(k, R::new(&INST_CNT)).is_ok());
        }
        assert!(INST_CNT.load(Relaxed) >= workload_size);
        assert_eq!(tree.len(), workload_size);
        for k in 0..workload_size {
            assert!(tree.remove_sync(&k));
        }
        assert_eq!(tree.len(), 0);

        while INST_CNT.load(Relaxed) != 0 {
            Guard::new().accelerate();
            thread::yield_now();
        }
    }

    #[test]
    fn insert_remove_clear() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);

        let num_threads = 3;
        let num_iter = if cfg!(miri) { 1 } else { 16 };
        let workload_size = if cfg!(miri) { 32 } else { 1024 };
        let tree: Arc<TreeIndex<String, R>> = Arc::new(TreeIndex::default());
        let mut threads = Vec::with_capacity(num_threads);
        let barrier = Arc::new(Barrier::new(num_threads));
        for task_id in 0..num_threads {
            let barrier = barrier.clone();
            let tree = tree.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..num_iter {
                    barrier.wait();
                    match task_id {
                        0 => {
                            for k in 0..workload_size {
                                assert!(
                                    tree.insert_sync(format!("{k}"), R::new(&INST_CNT)).is_ok()
                                );
                            }
                        }
                        1 => {
                            for k in 0..workload_size / 8 {
                                tree.remove_sync(format!("{}", k * 4).as_str());
                            }
                        }
                        _ => {
                            for _ in 0..workload_size / 64 {
                                if tree.len() >= workload_size / 4 {
                                    tree.clear();
                                }
                            }
                        }
                    }
                    tree.clear();
                    assert!(suspend());
                }
                drop(tree);
            }));
        }

        for thread in threads {
            assert!(thread.join().is_ok());
        }

        drop(tree);

        while INST_CNT.load(Relaxed) != 0 {
            Guard::new().accelerate();
            thread::yield_now();
        }
    }

    #[test]
    fn clear() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let tree: TreeIndex<usize, R> = TreeIndex::default();

        let workload_size = if cfg!(miri) { 256 } else { 1024 * 1024 };
        for k in 0..workload_size {
            assert!(tree.insert_sync(k, R::new(&INST_CNT)).is_ok());
        }
        assert!(INST_CNT.load(Relaxed) >= workload_size);
        assert_eq!(tree.len(), workload_size);
        tree.clear();

        while INST_CNT.load(Relaxed) != 0 {
            Guard::new().accelerate();
            thread::yield_now();
        }
    }

    #[test]
    fn reclaim() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        struct R(usize);
        impl R {
            fn new() -> R {
                R(INST_CNT.fetch_add(1, Relaxed))
            }
        }
        impl Clone for R {
            fn clone(&self) -> Self {
                INST_CNT.fetch_add(1, Relaxed);
                R(self.0)
            }
        }
        impl Drop for R {
            fn drop(&mut self) {
                INST_CNT.fetch_sub(1, Relaxed);
            }
        }

        let data_size = 256;
        let tree: TreeIndex<usize, R> = TreeIndex::new();
        for k in 0..data_size {
            assert!(tree.insert_sync(k, R::new()).is_ok());
        }
        for k in (0..data_size).rev() {
            assert!(tree.remove_sync(&k));
        }

        let mut cnt = 0;
        while INST_CNT.load(Relaxed) > 0 {
            Guard::new().accelerate();
            cnt += 1;
        }
        assert!(cnt >= INST_CNT.load(Relaxed));

        let tree: TreeIndex<usize, R> = TreeIndex::new();
        for k in 0..(data_size / 16) {
            assert!(tree.insert_sync(k, R::new()).is_ok());
        }
        tree.clear();

        while INST_CNT.load(Relaxed) > 0 {
            Guard::new().accelerate();
        }
    }

    #[test]
    fn clone() {
        static INST_CNT: AtomicUsize = AtomicUsize::new(0);
        let tree: TreeIndex<usize, R> = TreeIndex::default();

        let workload_size = 256;
        for k in 0..workload_size {
            assert!(tree.insert_sync(k, R::new(&INST_CNT)).is_ok());
        }
        let tree_clone = tree.clone();
        tree.clear();
        for k in 0..workload_size {
            assert!(tree_clone.peek_with(&k, |_, _| ()).is_some());
        }
        tree_clone.clear();

        while INST_CNT.load(Relaxed) != 0 {
            Guard::new().accelerate();
            thread::yield_now();
        }
    }

    #[test]
    fn compare() {
        let tree1: TreeIndex<String, usize> = TreeIndex::new();
        let tree2: TreeIndex<String, usize> = TreeIndex::new();
        assert_eq!(tree1, tree2);

        assert!(tree1.insert_sync("Hi".to_string(), 1).is_ok());
        assert_ne!(tree1, tree2);

        assert!(tree2.insert_sync("Hello".to_string(), 2).is_ok());
        assert_ne!(tree1, tree2);

        assert!(tree1.insert_sync("Hello".to_string(), 2).is_ok());
        assert_ne!(tree1, tree2);

        assert!(tree2.insert_sync("Hi".to_string(), 1).is_ok());
        assert_eq!(tree1, tree2);

        assert!(tree1.remove_sync("Hi"));
        assert_ne!(tree1, tree2);
    }

    #[test]
    fn insert_peek() {
        let num_iter = if cfg!(miri) { 2 } else { 1024 };
        let tree1: TreeIndex<String, u32> = TreeIndex::default();
        let tree2: TreeIndex<u32, String> = TreeIndex::default();
        let mut checker1 = BTreeSet::new();
        let mut checker2 = BTreeSet::new();
        let mut runner = TestRunner::default();
        for i in 0..num_iter {
            let prop_str = "[a-z]{1,16}".new_tree(&mut runner).unwrap();
            let str_val = prop_str.current();
            if tree1.insert_sync(str_val.clone(), i).is_ok() {
                checker1.insert((str_val.clone(), i));
            }
            let str_borrowed = str_val.as_str();
            assert!(tree1.peek_with(str_borrowed, |_, _| ()).is_some());

            if tree2.insert_sync(i, str_val.clone()).is_ok() {
                checker2.insert((i, str_val.clone()));
            }
        }
        for iter in &checker1 {
            let v = tree1.peek_with(iter.0.as_str(), |_, v| *v);
            assert_eq!(v.unwrap(), iter.1);
        }
        for iter in &checker2 {
            let v = tree2.peek_with(&iter.0, |_, v| v.clone());
            assert_eq!(v.unwrap(), iter.1);
        }
    }

    #[test]
    fn double_ended_iter() {
        let workload_size = if cfg!(miri) { 1024 } else { 16384 };
        let tree: TreeIndex<usize, ()> = TreeIndex::default();
        for k in 0..workload_size {
            assert!(tree.insert_sync(k, ()).is_ok());
        }

        let guard = Guard::new();
        let mut de_iter = tree.iter(&guard);
        for k in 0..workload_size / 4 {
            assert_eq!(de_iter.next_back(), Some((&(workload_size - k - 1), &())));
        }
        for k in 0..workload_size / 4 {
            assert_eq!(de_iter.next(), Some((&k, &())));
        }
        for k in 0..workload_size / 4 {
            assert_eq!(
                de_iter.next_back(),
                Some((&(workload_size - k - 1 - workload_size / 4), &()))
            );
        }
        for k in 0..workload_size / 4 {
            assert_eq!(de_iter.next(), Some((&(k + workload_size / 4), &())));
        }
        assert!(de_iter.next().is_none());
        assert!(de_iter.next_back().is_none());
        assert_eq!(tree.iter(&guard).rev().count(), workload_size);
        assert_eq!(tree.iter(&guard).rev().rev().count(), workload_size);

        // Only one end of the iterator was exhausted.
        for rev in [true, false] {
            let mut de_iter = tree.iter(&guard);
            if rev {
                let mut prev = usize::MAX;
                while let Some((key, ())) = de_iter.next_back() {
                    assert!(prev > *key);
                    prev = *key;
                }
                assert!(de_iter.next().is_none());
                assert!(!de_iter.flip());
            } else {
                let mut prev = 0;
                for (key, ()) in de_iter.by_ref() {
                    assert!(prev == 0 || prev < *key);
                    prev = *key;
                }
                assert!(de_iter.next_back().is_none());
                assert!(!de_iter.flip());
            }
        }
    }

    #[test]
    fn double_ended_range_iter() {
        let workload_size = if cfg!(miri) { 1024 } else { 16384 };
        let tree: TreeIndex<usize, ()> = TreeIndex::default();
        for k in 0..workload_size {
            assert!(tree.insert_sync(k, ()).is_ok());
        }

        let guard = Guard::new();
        let start = workload_size / 4;
        let end = (workload_size / 4) * 3;
        let mut de_range = tree.range(start..end, &guard);
        for k in 0..workload_size / 8 {
            assert_eq!(de_range.next_back(), Some((&(end - k - 1), &())));
        }
        for k in 0..workload_size / 8 {
            assert_eq!(de_range.next(), Some((&(start + k), &())));
        }
        for k in 0..workload_size / 8 {
            assert_eq!(
                de_range.next_back(),
                Some((&(end - k - 1 - workload_size / 8), &()))
            );
        }
        for k in 0..workload_size / 8 {
            assert_eq!(
                de_range.next(),
                Some((&(start + k + workload_size / 8), &()))
            );
        }
        assert!(de_range.next().is_none());
        assert!(de_range.next_back().is_none());
        assert_eq!(
            tree.range(0..=usize::MAX, &guard).rev().count(),
            workload_size
        );
        assert_eq!(
            tree.range(..usize::MAX, &guard).rev().rev().count(),
            workload_size
        );

        // Only one end of the iterator was exhausted.
        for rev in [true, false] {
            let mut de_range = tree.range(..usize::MAX, &guard);
            if rev {
                let mut prev = usize::MAX;
                while let Some((key, ())) = de_range.next_back() {
                    assert!(prev > *key);
                    prev = *key;
                }
                assert!(de_range.next().is_none());
                assert!(!de_range.flip());
            } else {
                let mut prev = 0;
                for (key, ()) in de_range.by_ref() {
                    assert!(prev == 0 || prev < *key);
                    prev = *key;
                }
                assert!(de_range.next_back().is_none());
                assert!(!de_range.flip());
            }
        }
    }

    #[test]
    fn range() {
        let tree: TreeIndex<String, usize> = TreeIndex::default();
        assert!(tree.insert_sync("Ape".to_owned(), 0).is_ok());
        assert!(tree.insert_sync("Apple".to_owned(), 1).is_ok());
        assert!(tree.insert_sync("Banana".to_owned(), 3).is_ok());
        assert!(tree.insert_sync("Badezimmer".to_owned(), 2).is_ok());
        assert_eq!(tree.range(..="Ball".to_owned(), &Guard::new()).count(), 3);
        assert_eq!(
            tree.range("Ape".to_owned()..="Ball".to_owned(), &Guard::new())
                .count(),
            3
        );
        assert_eq!(
            tree.range("Apex".to_owned()..="Ball".to_owned(), &Guard::new())
                .rev()
                .count(),
            2
        );
        assert_eq!(
            tree.range("Ace".to_owned()..="Ball".to_owned(), &Guard::new())
                .count(),
            3
        );
        assert_eq!(tree.range(..="Z".to_owned(), &Guard::new()).count(), 4);
        assert_eq!(
            tree.range("Ape".to_owned()..="Z".to_owned(), &Guard::new())
                .rev()
                .count(),
            4
        );
        assert_eq!(
            tree.range("Apex".to_owned()..="Z".to_owned(), &Guard::new())
                .count(),
            3
        );
        assert_eq!(
            tree.range("Ace".to_owned()..="Z".to_owned(), &Guard::new())
                .rev()
                .count(),
            4
        );
        assert_eq!(tree.range(.."Banana".to_owned(), &Guard::new()).count(), 3);
        assert_eq!(
            tree.range("Ape".to_owned().."Banana".to_owned(), &Guard::new())
                .count(),
            3
        );
        assert_eq!(
            tree.range("Apex".to_owned().."Banana".to_owned(), &Guard::new())
                .rev()
                .count(),
            2
        );
        assert_eq!(
            tree.range("Ace".to_owned().."Banana".to_owned(), &Guard::new())
                .count(),
            3
        );
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn insert_peek_remove_async() {
        let num_tasks = 8;
        let workload_size = 256;
        for _ in 0..256 {
            let tree: Arc<TreeIndex<usize, usize>> = Arc::new(TreeIndex::default());
            let mut tasks = Vec::with_capacity(num_tasks);
            let barrier = Arc::new(AsyncBarrier::new(num_tasks));
            for task_id in 0..num_tasks {
                let barrier = barrier.clone();
                let tree = tree.clone();
                tasks.push(tokio::task::spawn(async move {
                    barrier.wait().await;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    for id in range.clone() {
                        assert!(tree.insert_async(id, id).await.is_ok());
                        assert!(tree.insert_async(id, id).await.is_err());
                    }
                    for id in range.clone() {
                        let result = tree.peek_with(&id, |_, v| *v);
                        assert_eq!(result, Some(id));
                    }
                    for id in range.clone() {
                        assert!(tree.remove_if_async(&id, |v| *v == id).await);
                    }
                    for id in range {
                        assert!(!tree.remove_if_async(&id, |v| *v == id).await);
                    }
                }));
            }

            for task in join_all(tasks).await {
                assert!(task.is_ok());
            }
            assert_eq!(tree.len(), 0);
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn remove_range_async() {
        let num_tasks = 2;
        let workload_size = 4096;
        for _ in 0..16 {
            let tree: Arc<TreeIndex<usize, usize>> = Arc::new(TreeIndex::default());
            let mut tasks = Vec::with_capacity(num_tasks);
            let barrier = Arc::new(AsyncBarrier::new(num_tasks));
            let data = Arc::new(AtomicUsize::default());
            for task_id in 0..num_tasks {
                let barrier = barrier.clone();
                let data = data.clone();
                let tree = tree.clone();
                tasks.push(tokio::task::spawn(async move {
                    barrier.wait().await;
                    if task_id == 0 {
                        for k in 1..=workload_size {
                            assert!(tree.insert_async(k, k).await.is_ok());
                            assert!(tree.peek(&k, &Guard::new()).is_some());
                            data.store(k, Release);
                        }
                    } else {
                        loop {
                            let end_bound = data.load(Acquire);
                            if end_bound == workload_size {
                                break;
                            } else if end_bound <= 1 {
                                task::yield_now().await;
                                continue;
                            }
                            if end_bound % 2 == 0 {
                                let keys = tree
                                    .range(..end_bound, &Guard::new())
                                    .rev()
                                    .map(|(k, v)| (*k, *v))
                                    .collect::<Vec<_>>();
                                for (k, _) in keys {
                                    tree.remove_async(&k).await;
                                }
                            } else {
                                tree.remove_range_async(..end_bound).await;
                            }
                            if end_bound % 5 == 0 {
                                for (k, v) in tree.iter(&Guard::new()) {
                                    assert_eq!(k, v);
                                    assert!(!(..end_bound).contains(k), "{k}");
                                }
                            } else {
                                assert!(
                                    tree.peek(&(end_bound - 1), &Guard::new()).is_none(),
                                    "{end_bound} {}",
                                    data.load(Relaxed)
                                );
                                assert!(tree.peek(&end_bound, &Guard::new()).is_some());
                            }
                            // Give the scheduler a chance to run the other task.
                            task::yield_now().await;
                        }
                    }
                }));
            }

            for task in join_all(tasks).await {
                assert!(task.is_ok());
            }

            tree.remove_range_sync(..workload_size);
            assert!(tree.peek(&(workload_size - 1), &Guard::new()).is_none());
            assert!(tree.peek(&workload_size, &Guard::new()).is_some());
            assert_eq!(tree.len(), 1);
            assert_eq!(tree.depth(), 1);
        }
    }

    #[test]
    fn insert_peek_iter_sync() {
        let range = if cfg!(miri) { 64 } else { 4096 };
        let num_threads = if cfg!(miri) { 2 } else { 16 };
        let tree: Arc<TreeIndex<usize, usize>> = Arc::new(TreeIndex::new());
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut threads = Vec::with_capacity(num_threads);
        for thread_id in 0..num_threads {
            let tree = tree.clone();
            let barrier = barrier.clone();
            threads.push(thread::spawn(move || {
                let first_key = thread_id * range;
                barrier.wait();
                for key in first_key..(first_key + range / 2) {
                    assert!(tree.insert_sync(key, key).is_ok());
                }
                for key in first_key..(first_key + range / 2) {
                    assert!(
                        tree.peek_with(&key, |key, val| assert_eq!(key, val))
                            .is_some()
                    );
                }
                for key in (first_key + range / 2)..(first_key + range) {
                    assert!(tree.insert_sync(key, key).is_ok());
                }
                for key in (first_key + range / 2)..(first_key + range) {
                    assert!(
                        tree.peek_with(&key, |key, val| assert_eq!(key, val))
                            .is_some()
                    );
                }
            }));
        }
        for thread in threads {
            assert!(thread.join().is_ok());
        }

        let mut found = 0;
        for key in 0..num_threads * range {
            if tree
                .peek_with(&key, |key, val| assert_eq!(key, val))
                .is_some()
            {
                found += 1;
            }
        }
        assert_eq!(found, num_threads * range);
        for key in 0..num_threads * range {
            assert!(
                tree.peek_with(&key, |key, val| assert_eq!(key, val))
                    .is_some()
            );
        }

        let guard = Guard::new();
        let iter = tree.iter(&guard);
        let mut prev = 0;
        for entry in iter {
            assert!(prev == 0 || prev < *entry.0);
            assert_eq!(*entry.0, *entry.1);
            prev = *entry.0;
        }
    }

    #[test]
    fn insert_peek_remove_range_sync() {
        let (num_threads, range) = if cfg!(miri) { (2, 8) } else { (8, 4096) };
        let tree: Arc<TreeIndex<usize, usize>> = Arc::new(TreeIndex::new());
        for t in 0..num_threads {
            // Fixed points.
            assert!(tree.insert_sync(t * range, t * range).is_ok());
        }
        let stopped: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let barrier = Arc::new(Barrier::new(num_threads + 1));
        let mut threads = Vec::with_capacity(num_threads);
        for thread_id in 0..num_threads {
            let (tree, stopped, barrier) = (tree.clone(), stopped.clone(), barrier.clone());
            threads.push(thread::spawn(move || {
                let first = thread_id * range;
                barrier.wait();
                while !stopped.load(Relaxed) {
                    for k in (first + 1)..(first + range) {
                        assert!(tree.insert_sync(k, k).is_ok());
                    }
                    for k in (first + 1)..(first + range) {
                        assert!(tree.peek_with(&k, |k, v| assert_eq!(k, v)).is_some());
                    }
                    {
                        let guard = Guard::new();
                        let mut range_iter = tree.range(first.., &guard);
                        let mut entry = range_iter.next().unwrap();
                        assert_eq!(entry, (&first, &first));
                        entry = range_iter.next().unwrap();
                        assert_eq!(entry, (&(first + 1), &(first + 1)));
                        entry = range_iter.next().unwrap();
                        assert_eq!(entry, (&(first + 2), &(first + 2)));
                        entry = range_iter.next().unwrap();
                        assert_eq!(entry, (&(first + 3), &(first + 3)));
                    }

                    let key_at_halfway = first + range / 2;
                    for key in (first + 1)..(first + range) {
                        if key == key_at_halfway {
                            let guard = Guard::new();
                            let mut range_iter = tree.range((first + 1).., &guard);
                            let entry = range_iter.next().unwrap();
                            assert_eq!(entry, (&key_at_halfway, &key_at_halfway));
                            let entry = range_iter.next().unwrap();
                            assert_eq!(entry, (&(key_at_halfway + 1), &(key_at_halfway + 1)));
                        }
                        assert!(tree.remove_sync(&key));
                        assert!(!tree.remove_sync(&key));
                        assert!(tree.peek_with(&(first + 1), |_, _| ()).is_none());
                        assert!(tree.peek_with(&key, |_, _| ()).is_none());
                    }
                    for k in (first + 1)..(first + range) {
                        assert!(tree.peek_with(&k, |k, v| assert_eq!(k, v)).is_none());
                    }
                }
            }));
        }
        barrier.wait();

        let repeat = if cfg!(miri) { 16 } else { 512 };
        for _ in 0..repeat {
            let (mut found_0, mut found_markers) = (false, 0);
            let (mut prev_marker, mut prev_marker_rev) = (0, 0);
            let (mut prev, mut prev_rev) = (0, usize::MAX);
            let guard = Guard::new();
            let (mut iter, mut iter_ended, mut iter_rev_ended, mut rev) =
                (tree.iter(&guard), false, false, false);
            while !iter_ended || !iter_rev_ended {
                rev = iter_ended || (!iter_rev_ended && rev);
                let current = if rev { iter.next_back() } else { iter.next() };
                let Some((key, _)) = current else {
                    if rev {
                        iter_rev_ended = true;
                    } else {
                        iter_ended = true;
                    }
                    continue;
                };
                if key % range == 0 {
                    found_markers += 1;
                    if *key == 0 {
                        found_0 = true;
                    } else if rev {
                        assert!(prev_marker_rev == 0 || prev_marker_rev - range == *key);
                        prev_marker_rev = *key;
                    } else {
                        assert_eq!(prev_marker + range, *key);
                        prev_marker = *key;
                    }
                }
                assert!((rev && prev_rev > *key) || (!rev && (prev == 0 || prev < *key)));
                if rev {
                    prev_rev = *key;
                } else {
                    prev = *key;
                }
                rev = !rev;
            }
            assert!(found_0);
            assert_eq!(found_markers, num_threads);
        }

        stopped.store(true, Release);
        for thread in threads {
            assert!(thread.join().is_ok());
        }
    }

    #[test]
    fn insert_peek_remove_sync() {
        let num_threads = if cfg!(miri) { 2 } else { 16 };
        let tree: Arc<TreeIndex<usize, usize>> = Arc::new(TreeIndex::new());
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut threads = Vec::with_capacity(num_threads);
        for thread_id in 0..num_threads {
            let tree = tree.clone();
            let barrier = barrier.clone();
            threads.push(thread::spawn(move || {
                barrier.wait();
                let data_size = if cfg!(miri) { 16 } else { 4096 };
                for _ in 0..data_size {
                    let range = 0..32;
                    let inserted = range
                        .clone()
                        .filter(|i| tree.insert_sync(*i, thread_id).is_ok())
                        .count();
                    let found = range
                        .clone()
                        .filter(|i| tree.peek_with(i, |_, v| *v == thread_id).is_some_and(|t| t))
                        .count();
                    let removed = range
                        .clone()
                        .filter(|i| tree.remove_if_sync(i, |v| *v == thread_id))
                        .count();
                    let removed_again = range
                        .clone()
                        .filter(|i| tree.remove_if_sync(i, |v| *v == thread_id))
                        .count();
                    assert_eq!(removed_again, 0);
                    assert_eq!(found, removed, "{inserted} {found} {removed}");
                    assert_eq!(inserted, found, "{inserted} {found} {removed}");
                }
            }));
        }
        for thread in threads {
            assert!(thread.join().is_ok());
        }
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn insert_remove_iter_sync() {
        let num_iter = if cfg!(miri) { 4 } else { 64 };
        let data_size = if cfg!(miri) { 128 } else { 4096 };
        for _ in 0..num_iter {
            let tree: Arc<TreeIndex<usize, u64>> = Arc::new(TreeIndex::default());
            let barrier = Arc::new(Barrier::new(3));
            let inserted = Arc::new(AtomicUsize::new(0));
            let removed = Arc::new(AtomicUsize::new(data_size));
            let mut threads = Vec::new();
            for _ in 0..2 {
                let tree = tree.clone();
                let barrier = barrier.clone();
                let inserted = inserted.clone();
                let removed = removed.clone();
                let thread = thread::spawn(move || {
                    // test insert
                    for _ in 0..2 {
                        barrier.wait();
                        let max = inserted.load(Acquire);
                        let mut prev = 0;
                        let mut iterated = 0;
                        let guard = Guard::new();
                        for iter in tree.iter(&guard) {
                            assert!(
                                prev == 0
                                    || (*iter.0 <= max && prev + 1 == *iter.0)
                                    || *iter.0 > prev
                            );
                            prev = *iter.0;
                            iterated += 1;
                        }
                        assert!(iterated >= max);
                    }
                    // test remove
                    for _ in 0..2 {
                        barrier.wait();
                        let mut prev = 0;
                        let max = removed.load(Acquire);
                        let guard = Guard::new();
                        for iter in tree.iter(&guard) {
                            let current = *iter.0;
                            assert!(current < max);
                            assert!(prev + 1 == current || prev == 0);
                            prev = current;
                        }
                    }
                });
                threads.push(thread);
            }
            // insert
            barrier.wait();
            for i in 0..data_size {
                if i == data_size / 2 {
                    barrier.wait();
                }
                assert!(tree.insert_sync(i, 0).is_ok());
                inserted.store(i, Release);
            }
            // remove
            barrier.wait();
            for i in (0..data_size).rev() {
                if i == data_size / 2 {
                    barrier.wait();
                }
                assert!(tree.remove_sync(&i));
                removed.store(i, Release);
            }
            for thread in threads {
                assert!(thread.join().is_ok());
            }
        }
    }

    #[test]
    fn insert_locate_remove_sync() {
        let (num_threads, range) = if cfg!(miri) { (2, 8) } else { (8, 1024) };
        let repeat = if cfg!(miri) { 4 } else { 512 };
        let offset = 7;
        let tree: Arc<TreeIndex<usize, usize>> = Arc::new(TreeIndex::new());

        for t in 0..num_threads {
            // Fixed points.
            assert!(
                tree.insert_sync(offset + t * range, offset + t * range)
                    .is_ok()
            );
        }

        let barrier = Arc::new(Barrier::new(num_threads));
        let mut threads = Vec::with_capacity(num_threads);
        for t in 0..num_threads {
            let (tree, barrier) = (tree.clone(), barrier.clone());
            threads.push(thread::spawn(move || {
                let first = offset + t * range;
                barrier.wait();
                for _ in 0..repeat {
                    let guard = Guard::new();
                    for k in (first + 1)..(first + range) {
                        assert!(tree.insert_sync(k, k).is_ok());
                    }
                    let Proximity::Exact(iter) = tree.locate(&first, &guard) else {
                        unreachable!("{:?}", tree.locate(&first, &guard));
                    };
                    assert_eq!(iter.get(), Some((&first, &first)));

                    for key in (first + 1)..(first + range) {
                        assert!(tree.remove_sync(&key));
                        assert!(!tree.remove_sync(&key));
                        assert!(tree.peek_with(&(first + 1), |_, _| ()).is_none());
                        assert!(tree.peek_with(&key, |_, _| ()).is_none());

                        if key != first + range - 1 {
                            let Proximity::Between(iter) = tree.locate(&key, &guard) else {
                                unreachable!();
                            };
                            assert_eq!(iter.get(), Some((&first, &first)));
                            assert_eq!(iter.get_back(), Some((&(key + 1), &(key + 1))));
                        }
                    }
                    let Proximity::Smaller(iter) = tree.locate(&usize::MAX, &guard) else {
                        unreachable!();
                    };
                    assert!(*iter.get_back().unwrap().0 <= offset + num_threads * range);

                    let Proximity::Larger(iter) = tree.locate(&0, &guard) else {
                        unreachable!();
                    };
                    assert_eq!(*iter.get().unwrap().0, offset);
                }
            }));
        }
        for thread in threads {
            assert!(thread.join().is_ok());
        }
    }

    proptest! {
        #[cfg_attr(miri, ignore)]
        #[test]
        fn remove_range_prop(lower in 0_usize..4096_usize, range in 0_usize..4096_usize) {
            let remove_range = lower..lower + range;
            let insert_range = (256_usize, 4095_usize);
            let tree = TreeIndex::default();
            for k in insert_range.0..=insert_range.1 {
                prop_assert!(tree.insert_sync(k, k).is_ok());
            }
            if usize::BITS == 32 {
                prop_assert_eq!(tree.depth(), 4);
            } else {
                prop_assert_eq!(tree.depth(), 3);
            }
            tree.remove_range_sync(remove_range.clone());
            if remove_range.contains(&insert_range.0) && remove_range.contains(&insert_range.1) {
                prop_assert!(tree.is_empty());
            }
            for (k, v) in tree.iter(&Guard::new()) {
                prop_assert_eq!(k, v);
                prop_assert!(!remove_range.contains(k), "{k}");
            }
            for k in 0_usize..4096_usize {
                if tree.peek_with(&k, |_, _| ()).is_some() {
                    prop_assert!(!remove_range.contains(&k), "{k}");
                }
            }
            for k in remove_range.clone() {
                prop_assert!(tree.insert_sync(k, k).is_ok());
            }
            let mut cnt = 0;
            for (k, v) in tree.iter(&Guard::new()) {
                prop_assert_eq!(k, v);
                if remove_range.contains(k) {
                    cnt += 1;
                }
            }
            assert_eq!(cnt, range);
        }
    }
}

#[cfg(feature = "serde")]
mod serde {
    use serde_test::{Token, assert_tokens};

    use crate::{HashCache, HashIndex, HashMap, HashSet, TreeIndex};

    #[test]
    fn hashmap() {
        let hashmap: HashMap<u64, i16> = HashMap::new();
        assert!(hashmap.insert_sync(2, -6).is_ok());
        assert_tokens(
            &hashmap,
            &[
                Token::Map { len: Some(1) },
                Token::U64(2),
                Token::I16(-6),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn hashset() {
        let hashset: HashSet<u64> = HashSet::new();
        assert!(hashset.insert_sync(2).is_ok());
        assert_tokens(
            &hashset,
            &[Token::Seq { len: Some(1) }, Token::U64(2), Token::SeqEnd],
        );
    }

    #[test]
    fn hashindex() {
        let hashindex: HashIndex<u64, i16> = HashIndex::new();
        assert!(hashindex.insert_sync(2, -6).is_ok());
        assert_tokens(
            &hashindex,
            &[
                Token::Map { len: Some(1) },
                Token::U64(2),
                Token::I16(-6),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn hashcache() {
        let hashcache: HashCache<u64, i16> = HashCache::new();
        let capacity_range = hashcache.capacity_range();
        assert!(hashcache.put_sync(2, -6).is_ok());
        assert_tokens(
            &hashcache,
            &[
                Token::Map {
                    len: Some(*capacity_range.end()),
                },
                Token::U64(2),
                Token::I16(-6),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn treeindex() {
        let treeindex: TreeIndex<u64, i16> = TreeIndex::new();
        assert!(treeindex.insert_sync(4, -4).is_ok());
        assert!(treeindex.insert_sync(2, -6).is_ok());
        assert!(treeindex.insert_sync(3, -5).is_ok());
        assert_tokens(
            &treeindex,
            &[
                Token::Map { len: Some(3) },
                Token::U64(2),
                Token::I16(-6),
                Token::U64(3),
                Token::I16(-5),
                Token::U64(4),
                Token::I16(-4),
                Token::MapEnd,
            ],
        );
    }
}
