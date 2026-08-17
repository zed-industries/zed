#![cfg(all(test, feature = "sync"))]

use std::{
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
    sync::{Arc, Barrier},
    thread,
};

use moka::{
    sync::{Cache, SegmentedCache},
    Entry,
};

const NUM_THREADS: u8 = 16;
const FILE: &str = "./Cargo.toml";

macro_rules! generate_test_get_with {
    ($test_fn_name:ident, $cache_init:expr) => {
        #[test]
        fn $test_fn_name() {
            const TEN_MIB: usize = 10 * 1024 * 1024; // 10MiB

            let cache = $cache_init;
            let call_counter = Arc::new(AtomicUsize::default());
            let barrier = Arc::new(Barrier::new(NUM_THREADS as usize));

            let threads: Vec<_> = (0..NUM_THREADS)
                .map(|thread_id| {
                    let my_cache = cache.clone();
                    let my_call_counter = Arc::clone(&call_counter);
                    let my_barrier = Arc::clone(&barrier);

                    thread::spawn(move || {
                        my_barrier.wait();

                        println!("Thread {thread_id} started.");

                        let key = "key1".to_string();
                        let value = match thread_id % 4 {
                            0 => my_cache.get_with(key.clone(), || {
                                println!("Thread {thread_id} inserting a value.");
                                my_call_counter.fetch_add(1, Ordering::AcqRel);
                                Arc::new(vec![0u8; TEN_MIB])
                            }),
                            1 => my_cache.get_with_by_ref(key.as_str(), || {
                                println!("Thread {thread_id} inserting a value.");
                                my_call_counter.fetch_add(1, Ordering::AcqRel);
                                Arc::new(vec![0u8; TEN_MIB])
                            }),
                            2 => my_cache
                                .entry(key.clone())
                                .or_insert_with(|| {
                                    println!("Thread {thread_id} inserting a value.");
                                    my_call_counter.fetch_add(1, Ordering::AcqRel);
                                    Arc::new(vec![0u8; TEN_MIB])
                                })
                                .into_value(),
                            3 => my_cache
                                .entry_by_ref(key.as_str())
                                .or_insert_with(|| {
                                    println!("Thread {thread_id} inserting a value.");
                                    my_call_counter.fetch_add(1, Ordering::AcqRel);
                                    Arc::new(vec![0u8; TEN_MIB])
                                })
                                .into_value(),
                            _ => unreachable!(),
                        };

                        assert_eq!(value.len(), TEN_MIB);
                        assert!(my_cache.get(key.as_str()).is_some());

                        println!("Thread {thread_id} got the value. (len: {})", value.len());
                    })
                })
                .collect();

            threads
                .into_iter()
                .for_each(|t| t.join().expect("Thread failed"));

            assert_eq!(call_counter.load(Ordering::Acquire), 1);
        }
    };
}

macro_rules! generate_test_optionally_get_with {
    ($test_fn_name:ident, $cache_init:expr) => {
        #[test]
        fn $test_fn_name() {
            let cache = $cache_init;
            let call_counter = Arc::new(AtomicUsize::default());
            let barrier = Arc::new(Barrier::new(NUM_THREADS as usize));

            fn get_file_size(
                thread_id: u8,
                path: impl AsRef<Path>,
                call_counter: &AtomicUsize,
            ) -> Option<u64> {
                println!("get_file_size() called by thread {thread_id}.");
                call_counter.fetch_add(1, Ordering::AcqRel);
                std::fs::metadata(path).ok().map(|m| m.len())
            }

            let threads: Vec<_> = (0..NUM_THREADS)
                .map(|thread_id| {
                    let my_cache = cache.clone();
                    let my_call_counter = Arc::clone(&call_counter);
                    let my_barrier = Arc::clone(&barrier);

                    thread::spawn(move || {
                        my_barrier.wait();

                        println!("Thread {thread_id} started.");

                        let key = "key1".to_string();
                        let value = match thread_id % 4 {
                            0 => my_cache.optionally_get_with(key.clone(), || {
                                get_file_size(thread_id, FILE, &my_call_counter)
                            }),
                            1 => my_cache.optionally_get_with_by_ref(key.as_str(), || {
                                get_file_size(thread_id, FILE, &my_call_counter)
                            }),
                            2 => my_cache
                                .entry(key.clone())
                                .or_optionally_insert_with(|| {
                                    get_file_size(thread_id, FILE, &my_call_counter)
                                })
                                .map(Entry::into_value),
                            3 => my_cache
                                .entry_by_ref(key.as_str())
                                .or_optionally_insert_with(|| {
                                    get_file_size(thread_id, FILE, &my_call_counter)
                                })
                                .map(Entry::into_value),
                            _ => unreachable!(),
                        };

                        assert!(value.is_some());
                        assert!(my_cache.get(key.as_str()).is_some());

                        println!(
                            "Thread {thread_id} got the value. (len: {})",
                            value.unwrap()
                        );
                    })
                })
                .collect();

            threads
                .into_iter()
                .for_each(|t| t.join().expect("Thread failed"));

            assert_eq!(call_counter.load(Ordering::Acquire), 1);
        }
    };
}

macro_rules! generate_test_try_get_with {
    ($test_fn_name:ident, $cache_init:expr) => {
        #[test]
        fn $test_fn_name() {
            let cache = $cache_init;
            let call_counter = Arc::new(AtomicUsize::default());
            let barrier = Arc::new(Barrier::new(NUM_THREADS as usize));

            fn get_file_size(
                thread_id: u8,
                path: impl AsRef<Path>,
                call_counter: &AtomicUsize,
            ) -> Result<u64, std::io::Error> {
                println!("get_file_size() called by thread {thread_id}.");
                call_counter.fetch_add(1, Ordering::AcqRel);
                Ok(std::fs::metadata(path)?.len())
            }

            let threads: Vec<_> = (0..NUM_THREADS)
                .map(|thread_id| {
                    let my_cache = cache.clone();
                    let my_call_counter = Arc::clone(&call_counter);
                    let my_barrier = Arc::clone(&barrier);

                    thread::spawn(move || {
                        my_barrier.wait();

                        println!("Thread {thread_id} started.");

                        let key = "key1".to_string();
                        let value = match thread_id % 4 {
                            0 => my_cache.try_get_with(key.clone(), || {
                                get_file_size(thread_id, FILE, &my_call_counter)
                            }),
                            1 => my_cache.try_get_with_by_ref(key.as_str(), || {
                                get_file_size(thread_id, FILE, &my_call_counter)
                            }),
                            2 => my_cache
                                .entry(key.clone())
                                .or_try_insert_with(|| {
                                    get_file_size(thread_id, FILE, &my_call_counter)
                                })
                                .map(Entry::into_value),
                            3 => my_cache
                                .entry_by_ref(key.as_str())
                                .or_try_insert_with(|| {
                                    get_file_size(thread_id, FILE, &my_call_counter)
                                })
                                .map(Entry::into_value),
                            _ => unreachable!(),
                        };

                        assert!(value.is_ok());
                        assert!(my_cache.get(key.as_str()).is_some());

                        println!(
                            "Thread {thread_id} got the value. (len: {})",
                            value.unwrap()
                        );
                    })
                })
                .collect();

            threads
                .into_iter()
                .for_each(|t| t.join().expect("Thread failed"));

            assert_eq!(call_counter.load(Ordering::Acquire), 1);
        }
    };
}

generate_test_get_with!(test_cache_get_with, Cache::<String, Arc<Vec<u8>>>::new(100));
generate_test_get_with!(
    test_seg_cache_get_with,
    SegmentedCache::<String, Arc<Vec<u8>>>::new(100, 4)
);
generate_test_optionally_get_with!(
    test_cache_optionally_get_with,
    Cache::<String, u64>::new(100)
);
generate_test_optionally_get_with!(
    test_seg_cache_optionally_get_with,
    SegmentedCache::<String, u64>::new(100, 4)
);
generate_test_try_get_with!(test_cache_try_get_with, Cache::<String, u64>::new(100));
generate_test_try_get_with!(
    test_seg_cache_try_get_with,
    SegmentedCache::<String, u64>::new(100, 4)
);
