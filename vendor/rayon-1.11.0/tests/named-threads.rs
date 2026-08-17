use std::collections::HashSet;

use rayon::prelude::*;
use rayon::*;

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn named_threads() {
    ThreadPoolBuilder::new()
        .thread_name(|i| format!("hello-name-test-{i}"))
        .build_global()
        .unwrap();

    const N: usize = 10000;

    let thread_names = (0..N)
        .into_par_iter()
        .flat_map(|_| ::std::thread::current().name().map(str::to_owned))
        .collect::<HashSet<String>>();

    let all_contains_name = thread_names
        .iter()
        .all(|name| name.starts_with("hello-name-test-"));
    assert!(all_contains_name);
}
