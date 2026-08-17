use std::sync::atomic::{AtomicBool, Ordering};

use async_std::task;
use async_std::task_local;

#[cfg(not(target_os = "unknown"))]
use async_std::task::spawn;
#[cfg(target_os = "unknown")]
use async_std::task::spawn_local as spawn;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn drop_local() {
    static DROP_LOCAL: AtomicBool = AtomicBool::new(false);

    struct Local;

    impl Drop for Local {
        fn drop(&mut self) {
            DROP_LOCAL.store(true, Ordering::SeqCst);
        }
    }

    task_local! {
        static LOCAL: Local = Local;
    }

    // Spawn a task that just touches its task-local.
    let handle = spawn(async {
        LOCAL.with(|_| ());
    });
    let task = handle.task().clone();

    // Wait for the task to finish and make sure its task-local has been dropped.
    task::block_on(async {
        handle.await;
        assert!(DROP_LOCAL.load(Ordering::SeqCst));
        drop(task);
    });
}
