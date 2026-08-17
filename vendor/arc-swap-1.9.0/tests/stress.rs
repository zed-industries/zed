//! Stress-tests
//!
//! The tests in here try to torture the implementation with multiple threads, in an attempt to
//! discover any possible race condition.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use adaptive_barrier::{Barrier, PanicMode};
use arc_swap::strategy::{CaS, DefaultStrategy, IndependentStrategy, Strategy};
use arc_swap::ArcSwapAny;
use crossbeam_utils::thread;
use itertools::Itertools;
use once_cell::sync::Lazy;

static LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

/// We want to prevent these tests from running concurrently, because they run multi-threaded.
fn lock() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(PoisonError::into_inner)
}

struct LLNode<S: Strategy<Option<Arc<LLNode<S>>>>> {
    next: ArcSwapAny<Option<Arc<LLNode<S>>>, S>,
    num: usize,
    owner: usize,
}

/// A test that repeatedly builds a linked list concurrently with multiple threads.
///
/// The idea here is to stress-test the RCU implementation and see that no items get lost and that
/// the ref counts are correct afterwards.
fn storm_link_list<S>(node_cnt: usize, iters: usize)
where
    S: Default + CaS<Option<Arc<LLNode<S>>>> + Send + Sync,
{
    let _lock = lock();
    let head = ArcSwapAny::<_, S>::from(None::<Arc<LLNode<S>>>);
    #[cfg(not(miri))]
    let cpus = num_cpus::get();
    #[cfg(miri)]
    let cpus = 2;
    let barr = Barrier::new(PanicMode::Poison);
    thread::scope(|scope| {
        for thread in 0..cpus {
            // We want to borrow these, but that kind-of conflicts with the move closure mode
            let mut barr = barr.clone();
            let head = &head;
            scope.spawn(move |_| {
                let nodes = (0..node_cnt)
                    .map(|i| LLNode {
                        next: ArcSwapAny::from(None),
                        num: i,
                        owner: thread,
                    })
                    .map(Arc::new)
                    .collect::<Vec<_>>();
                for iter in 0..iters {
                    barr.wait(); // Start synchronously
                    for n in nodes.iter().rev() {
                        head.rcu(|head| {
                            n.next.store(head.clone()); // Cloning the optional Arc
                            Some(Arc::clone(n))
                        });
                    }
                    // And do the checks once everyone finishes
                    barr.wait();
                    // First, check that all our numbers are increasing by one and all are present
                    let mut node = head.load();
                    let mut expecting = 0;
                    while node.is_some() {
                        // A bit of gymnastics, we don't have NLL yet and we need to persuade the
                        // borrow checker this is safe.
                        let next = {
                            let inner = node.as_ref().unwrap();
                            if inner.owner == thread {
                                assert_eq!(expecting, inner.num);
                                expecting += 1;
                            }
                            inner.next.load()
                        };
                        node = next;
                    }
                    assert_eq!(node_cnt, expecting);
                    // We don't want to count the ref-counts while someone still plays around with
                    // them and loading.
                    barr.wait();
                    // Now that we've checked we have everything, check that all the nodes have ref
                    // count 2 ‒ once in the vector, once in the linked list.
                    for n in &nodes {
                        assert_eq!(
                            2,
                            Arc::strong_count(n),
                            "Wrong number of counts in item {} in iteration {}",
                            n.num,
                            iter,
                        );
                    }
                    // Reset the head so we don't mix the runs together, which would create a mess.
                    // Also, the tails might disturb the ref counts.
                    barr.wait();
                    head.store(None);
                    nodes.last().unwrap().next.store(None);
                }
                barr.wait();
                // We went through all the iterations. Dismantle the list and see that everything
                // has ref count 1.
                head.store(None);
                for n in &nodes {
                    n.next.store(None);
                }
                barr.wait(); // Wait until everyone resets their own nexts
                for n in &nodes {
                    assert_eq!(1, Arc::strong_count(n));
                }
            });
        }

        drop(barr);
    })
    .unwrap();
}

struct LLNodeCnt<'a> {
    next: Option<Arc<LLNodeCnt<'a>>>,
    num: usize,
    owner: usize,
    live_cnt: &'a AtomicUsize,
}

impl<'a> Drop for LLNodeCnt<'a> {
    fn drop(&mut self) {
        self.live_cnt.fetch_sub(1, Ordering::Relaxed);
    }
}

/// Test where we build and then deconstruct a linked list using multiple threads.
fn storm_unroll<S>(node_cnt: usize, iters: usize)
where
    S: Default + Send + Sync,
    for<'a> S: CaS<Option<Arc<LLNodeCnt<'a>>>>,
{
    let _lock = lock();

    #[cfg(not(miri))]
    let cpus = num_cpus::get();
    #[cfg(miri)]
    let cpus = 2;
    let barr = Barrier::new(PanicMode::Poison);
    let global_cnt = AtomicUsize::new(0);
    // We plan to create this many nodes during the whole test.
    let live_cnt = AtomicUsize::new(cpus * node_cnt * iters);
    let head = ArcSwapAny::<_, S>::from(None);
    thread::scope(|scope| {
        for thread in 0..cpus {
            // Borrow these instead of moving.
            let head = &head;
            let mut barr = barr.clone();
            let global_cnt = &global_cnt;
            let live_cnt = &live_cnt;
            scope.spawn(move |_| {
                for iter in 0..iters {
                    barr.wait();
                    // Create bunch of nodes and put them into the list.
                    for i in 0..node_cnt {
                        let mut node = Arc::new(LLNodeCnt {
                            next: None,
                            num: i,
                            owner: thread,
                            live_cnt,
                        });
                        head.rcu(|head| {
                            // Clone Option<Arc>
                            Arc::get_mut(&mut node).unwrap().next = head.clone();
                            Arc::clone(&node)
                        });
                    }
                    if barr.wait().is_leader() {
                        let mut cnt = 0;
                        let mut node = head.load_full();
                        while let Some(n) = node.as_ref() {
                            cnt += 1;
                            node = n.next.clone();
                        }
                        assert_eq!(cnt, node_cnt * cpus);
                    }
                    barr.wait();
                    // Keep removing items, count how many there are and that they increase in each
                    // thread's list.
                    let mut last_seen = vec![node_cnt; cpus];
                    let mut cnt = 0;
                    while let Some(node) =
                        head.rcu(|head| head.as_ref().and_then(|h| h.next.clone()))
                    {
                        assert!(last_seen[node.owner] > node.num);
                        last_seen[node.owner] = node.num;
                        cnt += 1;
                    }
                    global_cnt.fetch_add(cnt, Ordering::Relaxed);
                    if barr.wait().is_leader() {
                        assert_eq!(node_cnt * cpus, global_cnt.swap(0, Ordering::Relaxed));
                    }
                    assert_eq!(
                        (iters - iter - 1) * node_cnt * cpus,
                        live_cnt.load(Ordering::Relaxed),
                    );
                }
            });
        }

        drop(barr);
    })
    .unwrap();
    // Everything got destroyed properly.
    assert_eq!(0, live_cnt.load(Ordering::Relaxed));
}

fn load_parallel<S>(iters: usize)
where
    S: Default + Strategy<Arc<usize>> + Send + Sync,
{
    let _lock = lock();
    #[cfg(not(miri))]
    let cpus = num_cpus::get();
    #[cfg(miri)]
    let cpus = 2;
    let shared = ArcSwapAny::<_, S>::from(Arc::new(0));
    thread::scope(|scope| {
        scope.spawn(|_| {
            for i in 0..iters {
                shared.store(Arc::new(i));
            }
        });
        for _ in 0..cpus {
            scope.spawn(|_| {
                for _ in 0..iters {
                    let guards = (0..256).map(|_| shared.load()).collect::<Vec<_>>();
                    for (l, h) in guards.iter().tuple_windows() {
                        assert!(**l <= **h, "{} > {}", l, h);
                    }
                }
            });
        }
    })
    .unwrap();
    let v = shared.load_full();
    assert_eq!(2, Arc::strong_count(&v));
}

#[cfg(not(miri))]
const ITER_SMALL: usize = 100;
#[cfg(not(miri))]
const ITER_MID: usize = 1000;

#[cfg(miri)]
const ITER_SMALL: usize = 2;
#[cfg(miri)]
const ITER_MID: usize = 5;

macro_rules! t {
    ($name: ident, $strategy: ty) => {
        mod $name {
            use super::*;

            #[allow(deprecated)] // We use some "deprecated" testing strategies
            type Strategy = $strategy;

            #[test]
            fn storm_link_list_small() {
                storm_link_list::<Strategy>(ITER_SMALL, 5);
            }

            #[test]
            #[ignore]
            fn storm_link_list_large() {
                storm_link_list::<Strategy>(10_000, 50);
            }

            #[test]
            fn storm_unroll_small() {
                storm_unroll::<Strategy>(ITER_SMALL, 5);
            }

            #[test]
            #[ignore]
            fn storm_unroll_large() {
                storm_unroll::<Strategy>(10_000, 50);
            }

            #[test]
            fn load_parallel_small() {
                load_parallel::<Strategy>(ITER_MID);
            }

            #[test]
            #[ignore]
            fn load_parallel_large() {
                load_parallel::<Strategy>(100_000);
            }
        }
    };
}

t!(default, DefaultStrategy);
t!(independent, IndependentStrategy);
#[cfg(feature = "internal-test-strategies")]
t!(
    full_slots,
    arc_swap::strategy::test_strategies::FillFastSlots
);
