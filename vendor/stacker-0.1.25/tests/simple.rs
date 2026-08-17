extern crate stacker;

const RED_ZONE: usize = 100 * 1024; // 100k
const STACK_PER_RECURSION: usize = 1 * 1024 * 1024; // 1MB

pub fn ensure_sufficient_stack<R, F: FnOnce() -> R + std::panic::UnwindSafe>(f: F) -> R {
    stacker::maybe_grow(RED_ZONE, STACK_PER_RECURSION, f)
}

#[inline(never)]
fn recurse(n: usize) {
    let x = [42u8; 50000];
    if n != 0 {
        ensure_sufficient_stack(|| recurse(n - 1));
    }
    #[allow(dropping_copy_types)]
    drop(x);
}

#[test]
#[cfg_attr(miri, ignore)] // Too slow under Miri's interpreter
fn foo() {
    let limit = if cfg!(target_arch = "wasm32") {
        2000
    } else {
        10000
    };
    recurse(limit);
}
