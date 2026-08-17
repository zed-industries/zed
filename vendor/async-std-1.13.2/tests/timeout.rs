use std::time::Duration;

use async_std::future::timeout;
use async_std::task;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn timeout_future_many() {
    task::block_on(async {
        let futures = (0..10)
            .map(|i| {
                timeout(Duration::from_millis(i * 50), async move {
                    task::sleep(Duration::from_millis(i)).await;
                    Ok::<(), async_std::future::TimeoutError>(())
                })
            })
            .collect::<Vec<_>>();

        for future in futures {
            future.await.unwrap().unwrap();
        }
    });
}
