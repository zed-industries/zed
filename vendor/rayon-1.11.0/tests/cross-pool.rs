use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn cross_pool_busy() {
    let pool1 = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
    let pool2 = ThreadPoolBuilder::new().num_threads(1).build().unwrap();

    let n: i32 = 100;
    let sum: i32 = pool1.install(move || {
        // Each item will block on pool2, but pool1 can continue processing other work from the
        // parallel iterator in the meantime. There's a chance that pool1 will still be awake to
        // see the latch set without being tickled, and then it will drop that stack job. The latch
        // internals must not assume that the job will still be alive after it's set!
        (1..=n)
            .into_par_iter()
            .map(|i| pool2.install(move || i))
            .sum()
    });
    assert_eq!(sum, n * (n + 1) / 2);
}
