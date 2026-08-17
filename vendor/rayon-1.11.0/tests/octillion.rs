use rayon::prelude::*;

const OCTILLION: u128 = 1_000_000_000_000_000_000_000_000_000;

/// Produce a parallel iterator for 0u128..10²⁷
fn octillion() -> rayon::range::Iter<u128> {
    (0..OCTILLION).into_par_iter()
}

/// Produce a parallel iterator for 0u128..=10²⁷
fn octillion_inclusive() -> rayon::range_inclusive::Iter<u128> {
    (0..=OCTILLION).into_par_iter()
}

/// Produce a parallel iterator for 0u128..10²⁷ using `flat_map`
fn octillion_flat() -> impl ParallelIterator<Item = u128> {
    (0u32..1_000_000_000)
        .into_par_iter()
        .with_max_len(1_000)
        .map(|i| u64::from(i) * 1_000_000_000)
        .flat_map(|i| {
            (0u32..1_000_000_000)
                .into_par_iter()
                .with_max_len(1_000)
                .map(move |j| i + u64::from(j))
        })
        .map(|i| u128::from(i) * 1_000_000_000)
        .flat_map(|i| {
            (0u32..1_000_000_000)
                .into_par_iter()
                .with_max_len(1_000)
                .map(move |j| i + u128::from(j))
        })
}

// NOTE: `find_first` and `find_last` currently take too long on 32-bit targets,
// because the `AtomicUsize` match position has much too limited resolution.

#[test]
#[cfg_attr(not(target_pointer_width = "64"), ignore)]
fn find_first_octillion() {
    let x = octillion().find_first(|_| true);
    assert_eq!(x, Some(0));
}

#[test]
#[cfg_attr(not(target_pointer_width = "64"), ignore)]
fn find_first_octillion_inclusive() {
    let x = octillion_inclusive().find_first(|_| true);
    assert_eq!(x, Some(0));
}

#[test]
#[cfg_attr(not(target_pointer_width = "64"), ignore)]
fn find_first_octillion_flat() {
    let x = octillion_flat().find_first(|_| true);
    assert_eq!(x, Some(0));
}

fn two_threads<F: Send + FnOnce() -> R, R: Send>(f: F) -> R {
    // FIXME: If we don't use at least two threads, then we end up walking
    // through the entire iterator sequentially, without the benefit of any
    // short-circuiting.  We probably don't want testing to wait that long. ;)
    let builder = rayon::ThreadPoolBuilder::new().num_threads(2);
    let pool = builder.build().unwrap();

    pool.install(f)
}

#[test]
#[cfg_attr(
    any(
        not(target_pointer_width = "64"),
        target_os = "emscripten",
        target_family = "wasm"
    ),
    ignore
)]
fn find_last_octillion() {
    // It would be nice if `find_last` could prioritize the later splits,
    // basically flipping the `join` args, without needing indexed `rev`.
    // (or could we have an unindexed `rev`?)
    let x = two_threads(|| octillion().find_last(|_| true));
    assert_eq!(x, Some(OCTILLION - 1));
}

#[test]
#[cfg_attr(
    any(
        not(target_pointer_width = "64"),
        target_os = "emscripten",
        target_family = "wasm"
    ),
    ignore
)]
fn find_last_octillion_inclusive() {
    let x = two_threads(|| octillion_inclusive().find_last(|_| true));
    assert_eq!(x, Some(OCTILLION));
}

#[test]
#[cfg_attr(
    any(
        not(target_pointer_width = "64"),
        target_os = "emscripten",
        target_family = "wasm"
    ),
    ignore
)]
fn find_last_octillion_flat() {
    let x = two_threads(|| octillion_flat().find_last(|_| true));
    assert_eq!(x, Some(OCTILLION - 1));
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn find_any_octillion() {
    let x = two_threads(|| octillion().find_any(|x| *x > OCTILLION / 2));
    assert!(x.is_some());
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn find_any_octillion_flat() {
    let x = two_threads(|| octillion_flat().find_any(|x| *x > OCTILLION / 2));
    assert!(x.is_some());
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn filter_find_any_octillion() {
    let x = two_threads(|| {
        octillion()
            .filter(|x| *x > OCTILLION / 2)
            .find_any(|_| true)
    });
    assert!(x.is_some());
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn filter_find_any_octillion_flat() {
    let x = two_threads(|| {
        octillion_flat()
            .filter(|x| *x > OCTILLION / 2)
            .find_any(|_| true)
    });
    assert!(x.is_some());
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn fold_find_any_octillion_flat() {
    let x = two_threads(|| octillion_flat().fold(|| (), |_, _| ()).find_any(|_| true));
    assert!(x.is_some());
}
