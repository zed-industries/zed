# Loom

Loom is a testing tool for concurrent Rust code. It runs a test many
times, permuting the possible concurrent executions of that test under
the [C11 memory model][spec]. It uses [state reduction
techniques][cdschecker] to avoid combinatorial explosion.

[![Crates.io](https://img.shields.io/crates/v/loom.svg)](https://crates.io/crates/loom)
[![Documentation](https://docs.rs/loom/badge.svg)][docs]
[![Build Status](https://github.com/tokio-rs/loom/actions/workflows/ci.yml/badge.svg)](https://github.com/tokio-rs/loom/actions)
[![Discord chat](https://img.shields.io/discord/500028886025895936.svg?logo=discord&style=flat-square)](https://discord.com/channels/500028886025895936/628283088555737089)

[docs]: https://docs.rs/loom
[spec]: https://en.cppreference.com/w/cpp/atomic/memory_order
[cdschecker]: http://plrg.eecs.uci.edu/publications/toplas16.pdf

## Quickstart

The [loom documentation][docs] has significantly more documentation on
how to use loom. But if you just want a jump-start, first add this to
your `Cargo.toml`.

```toml
[target.'cfg(loom)'.dependencies]
loom = "0.7"
```

Next, create a test file and add a test:

```rust
use loom::sync::Arc;
use loom::sync::atomic::AtomicUsize;
use loom::sync::atomic::Ordering::{Acquire, Release, Relaxed};
use loom::thread;

#[test]
#[should_panic]
fn buggy_concurrent_inc() {
    loom::model(|| {
        let num = Arc::new(AtomicUsize::new(0));

        let ths: Vec<_> = (0..2)
            .map(|_| {
                let num = num.clone();
                thread::spawn(move || {
                    let curr = num.load(Acquire);
                    num.store(curr + 1, Release);
                })
            })
            .collect();

        for th in ths {
            th.join().unwrap();
        }

        assert_eq!(2, num.load(Relaxed));
    });
}
```

Then, run the test with

```console
RUSTFLAGS="--cfg loom" cargo test --test buggy_concurrent_inc --release
```

## Unsupported features
Loom currently does not implement the full C11 memory model.
Here is the (incomplete) list of unsupported features.
* `SeqCst` accesses (e.g. `load`, `store`, ..):
  They are are regarded as `AcqRel`. That is, they impose weaker
  synchronization, causing Loom to generate false alarms (not complete). See
  [#180](https://github.com/tokio-rs/loom/issues/180) for example. On the other
  hand, `fence(SeqCst)` is supported.
* Load buffering behavior:
  Loom does not explore some executions that are possible in the C11 memory
  model. That is, there can be a bug in the checked code even if Loom says
  there is no bug (not sound).  See the `load_buffering` test case in
  `tests/litmus.rs`.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in `loom` by you, shall be licensed as MIT,
without any additional terms or conditions.
