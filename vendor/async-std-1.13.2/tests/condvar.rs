#![cfg(feature = "unstable")]
use std::sync::Arc;
use std::time::Duration;

use async_std::sync::{Condvar, Mutex};
use async_std::task::{self, JoinHandle};

#[cfg(not(target_os = "unknown"))]
use async_std::task::spawn;
#[cfg(target_os = "unknown")]
use async_std::task::spawn_local as spawn;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn wait_timeout_with_lock() {
    task::block_on(async {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();

        spawn(async move {
            let (m, c) = &*pair2;
            let _g = m.lock().await;
            task::sleep(Duration::from_millis(200)).await;
            c.notify_one();
        });

        let (m, c) = &*pair;
        let (_, wait_result) = c
            .wait_timeout(m.lock().await, Duration::from_millis(50))
            .await;
        assert!(wait_result.timed_out());
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn wait_timeout_without_lock() {
    task::block_on(async {
        let m = Mutex::new(false);
        let c = Condvar::new();

        let (_, wait_result) = c
            .wait_timeout(m.lock().await, Duration::from_millis(10))
            .await;
        assert!(wait_result.timed_out());
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn wait_timeout_until_timed_out() {
    task::block_on(async {
        let m = Mutex::new(false);
        let c = Condvar::new();

        let (_, wait_result) = c
            .wait_timeout_until(m.lock().await, Duration::from_millis(100), |&mut started| {
                started
            })
            .await;
        assert!(wait_result.timed_out());
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn notify_all() {
    task::block_on(async {
        let mut tasks: Vec<JoinHandle<()>> = Vec::new();
        let pair = Arc::new((Mutex::new(0u32), Condvar::new()));

        for _ in 0..10 {
            let pair = pair.clone();
            tasks.push(spawn(async move {
                let (m, c) = &*pair;
                let mut count = m.lock().await;
                while *count == 0 {
                    count = c.wait(count).await;
                }
                *count += 1;
            }));
        }

        // Give some time for tasks to start up
        task::sleep(Duration::from_millis(50)).await;

        let (m, c) = &*pair;
        {
            let mut count = m.lock().await;
            *count += 1;
            c.notify_all();
        }

        for t in tasks {
            t.await;
        }
        let count = m.lock().await;
        assert_eq!(11, *count);
    })
}
