use rand::distr::Uniform;
use rand::{rng, Rng};
use rayon::prelude::*;
use std::cell::Cell;
use std::cmp::Ordering;
use std::panic;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

const LEN: usize = 20_000;

static VERSIONS: AtomicUsize = AtomicUsize::new(0);

static DROP_COUNTS: [AtomicUsize; LEN] = [const { AtomicUsize::new(0) }; LEN];

#[derive(Clone, Eq)]
struct DropCounter {
    x: u32,
    id: usize,
    version: Cell<usize>,
}

impl PartialEq for DropCounter {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for DropCounter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.version.set(self.version.get() + 1);
        other.version.set(other.version.get() + 1);
        VERSIONS.fetch_add(2, Relaxed);
        self.x.partial_cmp(&other.x)
    }
}

impl Ord for DropCounter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        DROP_COUNTS[self.id].fetch_add(1, Relaxed);
        VERSIONS.fetch_sub(self.version.get(), Relaxed);
    }
}

macro_rules! test {
    ($input:ident, $func:ident) => {
        let len = $input.len();

        // Work out the total number of comparisons required to sort
        // this array...
        let count = AtomicUsize::new(0);
        $input.to_owned().$func(|a, b| {
            count.fetch_add(1, Relaxed);
            a.cmp(b)
        });

        let mut panic_countdown = count.load(Relaxed);
        let step = if len <= 100 {
            1
        } else {
            Ord::max(1, panic_countdown / 10)
        };

        // ... and then panic after each `step` comparisons.
        loop {
            // Refresh the counters.
            VERSIONS.store(0, Relaxed);
            for i in 0..len {
                DROP_COUNTS[i].store(0, Relaxed);
            }

            let v = $input.to_owned();
            let _ = thread::spawn(move || {
                let mut v = v;
                let panic_countdown = AtomicUsize::new(panic_countdown);
                v.$func(|a, b| {
                    if panic_countdown.fetch_sub(1, Relaxed) == 1 {
                        SILENCE_PANIC.set(true);
                        panic!();
                    }
                    a.cmp(b)
                })
            })
            .join();

            // Check that the number of things dropped is exactly
            // what we expect (i.e. the contents of `v`).
            for (i, c) in DROP_COUNTS.iter().enumerate().take(len) {
                let count = c.load(Relaxed);
                assert!(
                    count == 1,
                    "found drop count == {} for i == {}, len == {}",
                    count,
                    i,
                    len
                );
            }

            // Check that the most recent versions of values were dropped.
            assert_eq!(VERSIONS.load(Relaxed), 0);

            if panic_countdown < step {
                break;
            }
            panic_countdown -= step;
        }
    };
}

thread_local!(static SILENCE_PANIC: Cell<bool> = const { Cell::new(false) });

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn sort_panic_safe() {
    let prev = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        if !SILENCE_PANIC.get() {
            prev(info);
        }
    }));

    for &len in &[1, 2, 3, 4, 5, 10, 20, 100, 500, 5_000, 20_000] {
        let len_dist = Uniform::new(0, len).unwrap();
        for &modulus in &[5, 30, 1_000, 20_000] {
            for &has_runs in &[false, true] {
                let mut rng = rng();
                let mut input = (0..len)
                    .map(|id| DropCounter {
                        x: rng.random_range(0..modulus),
                        id,
                        version: Cell::new(0),
                    })
                    .collect::<Vec<_>>();

                if has_runs {
                    for c in &mut input {
                        c.x = c.id as u32;
                    }

                    for _ in 0..5 {
                        let a = rng.sample(len_dist);
                        let b = rng.sample(len_dist);
                        if a < b {
                            input[a..b].reverse();
                        } else {
                            input.swap(a, b);
                        }
                    }
                }

                test!(input, par_sort_by);
                test!(input, par_sort_unstable_by);
            }
        }
    }
}
