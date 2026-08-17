#![warn(rust_2018_idioms)]
#![cfg(feature = "join-map")]

use std::panic::AssertUnwindSafe;

use futures::future::{pending, FutureExt};
use tokio::sync::oneshot;
use tokio::task::LocalSet;
use tokio::time::Duration;
use tokio_util::task::JoinMap;

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
}

// Spawn `N` tasks that return their index (`i`).
fn spawn_index_tasks(map: &mut JoinMap<usize, usize>, n: usize, on: Option<&LocalSet>) {
    for i in 0..n {
        let rc = std::rc::Rc::new(i);
        match on {
            None => map.spawn_local(i, async move { *rc }),
            Some(local) => map.spawn_local_on(i, async move { *rc }, local),
        };
    }
}

// Spawn `N` “pending” tasks that own a `oneshot::Sender`.
// When the task is aborted the sender is dropped, which is observed
// via the returned `Receiver`s.
fn spawn_pending_tasks(
    map: &mut JoinMap<usize, ()>,
    receivers: &mut Vec<oneshot::Receiver<()>>,
    n: usize,
    on: Option<&LocalSet>,
) {
    for i in 0..n {
        let (tx, rx) = oneshot::channel::<()>();
        receivers.push(rx);

        let fut = async move {
            pending::<()>().await;
            drop(tx);
        };
        match on {
            None => map.spawn_local(i, fut),
            Some(local) => map.spawn_local_on(i, fut, local),
        };
    }
}

/// Await every task in JoinMap and assert every task returns its own key.
async fn drain_joinmap_and_assert(mut map: JoinMap<usize, usize>, n: usize) {
    let mut seen = vec![false; n];
    while let Some((k, res)) = map.join_next().await {
        let v = res.expect("task panicked");
        assert_eq!(k, v);
        seen[v] = true;
    }
    assert!(seen.into_iter().all(|b| b));
    assert!(map.is_empty());
}

// Await every receiver and assert they all return `Err` because the
// corresponding sender (inside an aborted task) was dropped.
async fn await_receivers_and_assert(receivers: Vec<oneshot::Receiver<()>>) {
    for rx in receivers {
        assert!(
            rx.await.is_err(),
            "task should have been aborted and sender dropped"
        );
    }
}

#[tokio::test(start_paused = true)]
async fn test_with_sleep() {
    let mut map = JoinMap::new();

    for i in 0..10 {
        map.spawn(i, async move { i });
        assert_eq!(map.len(), 1 + i);
    }
    map.detach_all();
    assert_eq!(map.len(), 0);

    assert!(map.join_next().await.is_none());

    for i in 0..10 {
        map.spawn(i, async move {
            tokio::time::sleep(Duration::from_secs(i as u64)).await;
            i
        });
        assert_eq!(map.len(), 1 + i);
    }

    let mut seen = [false; 10];
    while let Some((k, res)) = map.join_next().await {
        seen[k] = true;
        assert_eq!(res.expect("task should have completed successfully"), k);
    }

    for was_seen in &seen {
        assert!(was_seen);
    }
    assert!(map.join_next().await.is_none());

    // Do it again.
    for i in 0..10 {
        map.spawn(i, async move {
            tokio::time::sleep(Duration::from_secs(i as u64)).await;
            i
        });
    }

    let mut seen = [false; 10];
    while let Some((k, res)) = map.join_next().await {
        seen[k] = true;
        assert_eq!(res.expect("task should have completed successfully"), k);
    }

    for was_seen in &seen {
        assert!(was_seen);
    }
    assert!(map.join_next().await.is_none());
}

#[tokio::test]
async fn test_abort_on_drop() {
    let mut map = JoinMap::new();

    let mut recvs = Vec::new();

    for i in 0..16 {
        let (send, recv) = oneshot::channel::<()>();
        recvs.push(recv);

        map.spawn(i, async {
            // This task will never complete on its own.
            futures::future::pending::<()>().await;
            drop(send);
        });
    }

    drop(map);

    for recv in recvs {
        // The task is aborted soon and we will receive an error.
        assert!(recv.await.is_err());
    }
}

#[tokio::test]
async fn alternating() {
    let mut map = JoinMap::new();

    assert_eq!(map.len(), 0);
    map.spawn(1, async {});
    assert_eq!(map.len(), 1);
    map.spawn(2, async {});
    assert_eq!(map.len(), 2);

    for i in 0..16 {
        let (_, res) = map.join_next().await.unwrap();
        assert!(res.is_ok());
        assert_eq!(map.len(), 1);
        map.spawn(i, async {});
        assert_eq!(map.len(), 2);
    }
}

#[tokio::test]
async fn test_keys() {
    use std::collections::HashSet;

    let mut map = JoinMap::new();

    assert_eq!(map.len(), 0);
    map.spawn(1, async {});
    assert_eq!(map.len(), 1);
    map.spawn(2, async {});
    assert_eq!(map.len(), 2);

    let keys = map.keys().collect::<HashSet<&u32>>();
    assert!(keys.contains(&1));
    assert!(keys.contains(&2));

    let _ = map.join_next().await.unwrap();
    let _ = map.join_next().await.unwrap();

    assert_eq!(map.len(), 0);
    let keys = map.keys().collect::<HashSet<&u32>>();
    assert!(keys.is_empty());
}

#[tokio::test(start_paused = true)]
async fn abort_by_key() {
    let mut map = JoinMap::new();
    let mut num_canceled = 0;
    let mut num_completed = 0;
    for i in 0..16 {
        map.spawn(i, async move {
            tokio::time::sleep(Duration::from_secs(i as u64)).await;
        });
    }

    for i in 0..16 {
        if i % 2 != 0 {
            // abort odd-numbered tasks.
            map.abort(&i);
        }
    }

    while let Some((key, res)) = map.join_next().await {
        match res {
            Ok(()) => {
                num_completed += 1;
                assert_eq!(key % 2, 0);
                assert!(!map.contains_key(&key));
            }
            Err(e) => {
                num_canceled += 1;
                assert!(e.is_cancelled());
                assert_ne!(key % 2, 0);
                assert!(!map.contains_key(&key));
            }
        }
    }

    assert_eq!(num_canceled, 8);
    assert_eq!(num_completed, 8);
}

#[tokio::test(start_paused = true)]
async fn abort_by_predicate() {
    let mut map = JoinMap::new();
    let mut num_canceled = 0;
    let mut num_completed = 0;
    for i in 0..16 {
        map.spawn(i, async move {
            tokio::time::sleep(Duration::from_secs(i as u64)).await;
        });
    }

    // abort odd-numbered tasks.
    map.abort_matching(|key| key % 2 != 0);

    while let Some((key, res)) = map.join_next().await {
        match res {
            Ok(()) => {
                num_completed += 1;
                assert_eq!(key % 2, 0);
                assert!(!map.contains_key(&key));
            }
            Err(e) => {
                num_canceled += 1;
                assert!(e.is_cancelled());
                assert_ne!(key % 2, 0);
                assert!(!map.contains_key(&key));
            }
        }
    }

    assert_eq!(num_canceled, 8);
    assert_eq!(num_completed, 8);
}

#[test]
fn runtime_gone() {
    let mut map = JoinMap::new();
    {
        let rt = rt();
        map.spawn_on("key", async { 1 }, rt.handle());
        drop(rt);
    }

    let (key, res) = rt().block_on(map.join_next()).unwrap();
    assert_eq!(key, "key");
    assert!(res.unwrap_err().is_cancelled());
}

// This ensures that `join_next` works correctly when the coop budget is
// exhausted.
#[tokio::test(flavor = "current_thread")]
async fn join_map_coop() {
    // Large enough to trigger coop.
    const TASK_NUM: u32 = 1000;

    static SEM: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(0);

    let mut map = JoinMap::new();

    for i in 0..TASK_NUM {
        map.spawn(i, async move {
            SEM.add_permits(1);
            i
        });
    }

    // Wait for all tasks to complete.
    //
    // Since this is a `current_thread` runtime, there's no race condition
    // between the last permit being added and the task completing.
    let _ = SEM.acquire_many(TASK_NUM).await.unwrap();

    let mut count = 0;
    let mut coop_count = 0;
    loop {
        match map.join_next().now_or_never() {
            Some(Some((key, Ok(i)))) => assert_eq!(key, i),
            Some(Some((key, Err(err)))) => panic!("failed[{key}]: {err}"),
            None => {
                coop_count += 1;
                tokio::task::yield_now().await;
                continue;
            }
            Some(None) => break,
        }

        count += 1;
    }
    assert!(coop_count >= 1);
    assert_eq!(count, TASK_NUM);
}

#[tokio::test(start_paused = true)]
async fn abort_all() {
    let mut map: JoinMap<usize, ()> = JoinMap::new();

    for i in 0..5 {
        map.spawn(i, futures::future::pending());
    }
    for i in 5..10 {
        map.spawn(i, async {
            tokio::time::sleep(Duration::from_secs(1)).await;
        });
    }

    // The join map will now have 5 pending tasks and 5 ready tasks.
    tokio::time::sleep(Duration::from_secs(2)).await;

    map.abort_all();
    assert_eq!(map.len(), 10);

    let mut count = 0;
    let mut seen = [false; 10];
    while let Some((k, res)) = map.join_next().await {
        seen[k] = true;
        if let Err(err) = res {
            assert!(err.is_cancelled());
        }
        count += 1;
    }
    assert_eq!(count, 10);
    assert_eq!(map.len(), 0);
    for was_seen in &seen {
        assert!(was_seen);
    }
}

#[tokio::test]
async fn duplicate_keys() {
    let mut map = JoinMap::new();
    map.spawn(1, async { 1 });
    map.spawn(1, async { 2 });

    assert_eq!(map.len(), 1);

    let (key, res) = map.join_next().await.unwrap();
    assert_eq!(key, 1);
    assert_eq!(res.unwrap(), 2);

    assert!(map.join_next().await.is_none());
}

#[tokio::test]
async fn duplicate_keys2() {
    let (send, recv) = oneshot::channel::<()>();

    let mut map = JoinMap::new();
    map.spawn(1, async { 1 });
    map.spawn(1, async {
        recv.await.unwrap();
        2
    });

    assert_eq!(map.len(), 1);

    tokio::select! {
        biased;
        res = map.join_next() => match res {
            Some((_key, res)) => panic!("Task {res:?} exited."),
            None => panic!("Phantom task completion."),
        },
        () = tokio::task::yield_now() => {},
    }

    send.send(()).unwrap();

    let (key, res) = map.join_next().await.unwrap();
    assert_eq!(key, 1);
    assert_eq!(res.unwrap(), 2);

    assert!(map.join_next().await.is_none());
}

#[cfg_attr(not(panic = "unwind"), ignore)]
#[tokio::test]
async fn duplicate_keys_drop() {
    #[derive(Hash, Debug, PartialEq, Eq)]
    struct Key;
    impl Drop for Key {
        fn drop(&mut self) {
            panic!("drop called for key");
        }
    }

    let (send, recv) = oneshot::channel::<()>();

    let mut map = JoinMap::new();

    map.spawn(Key, async { recv.await.unwrap() });

    // replace the task, force it to drop the key and abort the task
    // we should expect it to panic when dropping the key.
    let _ = std::panic::catch_unwind(AssertUnwindSafe(|| map.spawn(Key, async {}))).unwrap_err();

    // don't panic when this key drops.
    let (key, _) = map.join_next().await.unwrap();
    std::mem::forget(key);

    // original task should have been aborted, so the sender should be dangling.
    assert!(send.is_closed());

    assert!(map.join_next().await.is_none());
}

mod spawn_local {
    use super::*;

    #[test]
    #[should_panic(
        expected = "`spawn_local` called from outside of a `task::LocalSet` or `runtime::LocalRuntime`"
    )]
    fn panic_outside_any_runtime() {
        let mut map = JoinMap::new();
        map.spawn_local((), async {});
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic(
        expected = "`spawn_local` called from outside of a `task::LocalSet` or `runtime::LocalRuntime`"
    )]
    async fn panic_in_multi_thread_runtime() {
        let mut map = JoinMap::new();
        map.spawn_local((), async {});
    }

    #[cfg(tokio_unstable)]
    mod local_runtime {
        use super::*;

        /// Spawn several tasks, and then join all tasks.
        #[tokio::test(flavor = "local")]
        async fn spawn_then_join_next() {
            const N: usize = 8;

            let mut map = JoinMap::new();
            spawn_index_tasks(&mut map, N, None);

            assert!(map.join_next().now_or_never().is_none());
            drain_joinmap_and_assert(map, N).await;
        }

        /// Spawn several pending-forever tasks, and then shutdown the [`JoinMap`].
        #[tokio::test(flavor = "local")]
        async fn spawn_then_shutdown() {
            const N: usize = 8;

            let mut map = JoinMap::new();
            let mut receivers = Vec::new();

            spawn_pending_tasks(&mut map, &mut receivers, N, None);
            assert!(map.join_next().now_or_never().is_none());

            map.shutdown().await;
            assert!(map.is_empty());
            await_receivers_and_assert(receivers).await;
        }

        /// Spawn several pending-forever tasks, and then drop the [`JoinMap`].
        #[tokio::test(flavor = "local")]
        async fn spawn_then_drop() {
            const N: usize = 8;

            let mut map = JoinMap::new();
            let mut receivers = Vec::new();

            spawn_pending_tasks(&mut map, &mut receivers, N, None);
            assert!(map.join_next().now_or_never().is_none());

            drop(map);
            await_receivers_and_assert(receivers).await;
        }
    }

    mod local_set {
        use super::*;

        /// Spawn several tasks, and then join all tasks.
        #[tokio::test(flavor = "current_thread")]
        async fn spawn_then_join_next() {
            const N: usize = 8;
            let local = LocalSet::new();

            local
                .run_until(async move {
                    let mut map = JoinMap::new();
                    spawn_index_tasks(&mut map, N, None);
                    drain_joinmap_and_assert(map, N).await;
                })
                .await;
        }

        /// Spawn several pending-forever tasks, and then shutdown the [`JoinMap`].
        #[tokio::test(flavor = "current_thread")]
        async fn spawn_then_shutdown() {
            const N: usize = 8;
            let local = LocalSet::new();

            local
                .run_until(async {
                    let mut map = JoinMap::new();
                    let mut receivers = Vec::new();

                    spawn_pending_tasks(&mut map, &mut receivers, N, None);
                    assert!(map.join_next().now_or_never().is_none());

                    map.shutdown().await;
                    assert!(map.is_empty());
                    await_receivers_and_assert(receivers).await;
                })
                .await;
        }

        /// Spawn several pending-forever tasks, and then drop the [`JoinMap`].
        #[tokio::test(flavor = "current_thread")]
        async fn spawn_then_drop() {
            const N: usize = 8;
            let local = LocalSet::new();

            local
                .run_until(async {
                    let mut map = JoinMap::new();
                    let mut receivers = Vec::new();

                    spawn_pending_tasks(&mut map, &mut receivers, N, None);
                    assert!(map.join_next().now_or_never().is_none());

                    drop(map);
                    await_receivers_and_assert(receivers).await;
                })
                .await;
        }
    }
}

mod spawn_local_on {
    use super::*;

    #[cfg(tokio_unstable)]
    mod local_runtime {
        use super::*;

        /// Spawn several tasks, and then join all tasks.
        #[tokio::test(flavor = "local")]
        async fn spawn_then_join_next() {
            const N: usize = 8;

            let local = LocalSet::new();
            let mut map = JoinMap::new();

            spawn_index_tasks(&mut map, N, Some(&local));
            assert!(map.join_next().now_or_never().is_none());

            local
                .run_until(async move {
                    drain_joinmap_and_assert(map, N).await;
                })
                .await;
        }
    }

    mod local_set {
        use super::*;

        /// Spawn several tasks, and then join all tasks.
        #[tokio::test(flavor = "current_thread")]
        async fn spawn_then_join_next() {
            const N: usize = 8;
            let local = LocalSet::new();
            let mut pending_map = JoinMap::new();

            spawn_index_tasks(&mut pending_map, N, Some(&local));
            assert!(pending_map.join_next().now_or_never().is_none());

            local
                .run_until(async move {
                    drain_joinmap_and_assert(pending_map, N).await;
                })
                .await;
        }

        /// Spawn several pending-forever tasks, and then shutdown the [`JoinMap`].
        #[tokio::test(flavor = "current_thread")]
        async fn spawn_then_shutdown() {
            const N: usize = 8;
            let local = LocalSet::new();
            let mut map = JoinMap::new();
            let mut receivers = Vec::new();

            spawn_pending_tasks(&mut map, &mut receivers, N, Some(&local));
            assert!(map.join_next().now_or_never().is_none());

            local
                .run_until(async move {
                    map.shutdown().await;
                    assert!(map.is_empty());
                    await_receivers_and_assert(receivers).await;
                })
                .await;
        }

        /// Spawn several pending-forever tasks and then drop the [`JoinMap`]
        /// before the `LocalSet` is driven and while the `LocalSet` is already driven.
        #[tokio::test(flavor = "current_thread")]
        async fn spawn_then_drop() {
            const N: usize = 8;

            {
                let local = LocalSet::new();
                let mut map = JoinMap::new();
                let mut receivers = Vec::new();

                spawn_pending_tasks(&mut map, &mut receivers, N, Some(&local));
                assert!(map.join_next().now_or_never().is_none());

                drop(map);
                local
                    .run_until(async move { await_receivers_and_assert(receivers).await })
                    .await;
            }

            {
                let local = LocalSet::new();
                let mut map = JoinMap::new();
                let mut receivers = Vec::new();

                spawn_pending_tasks(&mut map, &mut receivers, N, Some(&local));
                assert!(map.join_next().now_or_never().is_none());

                local
                    .run_until(async move {
                        drop(map);
                        await_receivers_and_assert(receivers).await;
                    })
                    .await;
            }
        }
    }
}
