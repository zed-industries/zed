# Changelog

4.6.1 - 4.6.2

* Add support for `MIRIFLAGS="-Zmiri-strict-provenance"`.
 
4.6.0

* Add `{Shared, Unique}::{new_with, new_with_unchecked}`.

4.5.3

* Minor optimization of scanning remote thread-local variables.

4.5.2

* Adjust epoch countdown parameters.

4.5.1

* Migrate to [`codeberg`](https://codeberg.org/wvwwvwwv/scalable-delayed-dealloc).
* Remove `LinkedEntry::take_inner`: the method is highly dangerous to use.

4.5.0

* Add `Guard::set_has_garbage`.

4.4.0

* Add `Ptr::as_{ptr|ref}_unchecked`.
* Add `{Owned|Shared}::as_non_null_ptr`.

4.3.5

* Prepare for an upcoming Rust breaking change: [`Rust#136702`](https://github.com/rust-lang/rust/issues/136702).

4.3.4

* Add `Ptr::as_ref_unchecked`.

4.3.3

* Minor code cleanup.

4.3.2

* Add `Bag::try_push`.

4.3.1

* Add lock-free concurrent data structures: `Bag`, `LinkedList`, `Queue`, and `Stack`.

4.2.5

* Add `Guard::has_garbage`.

4.2.4

* Minor optimization.

4.2.2 - 4.2.3

* `Guard::accelerate` now only accelerates garbage collection of the current thread without affecting other threads.

4.2.1

* `u8` can be converted into `Epoch`.

4.2.0

* `Epoch` uses a range of `[0, 63]` `u8` values instead of rotating four values.

4.1.2

* More const functions.

4.1.1

* Let `miri` not execute Intel-specific code paths.

4.1.0

* The size of `Option<Guard>` is now that of `Guard`.

4.0.1

* Minor improvements to documentation.

4.0.0

* Bump MSRV to 1.85.0 / Edition 2024.

3.0.10

* Minor epoch update policy optimization.
* Minor `NonNull` optimization on `Owned` and `Shared`.

3.0.9

* Fix unsound `Sync` implementations of `AtomicShared` and `Shared`; previously, the `Sync` implementation allowed an arbitrary thread to own/drop the contained instance.

3.0.8

* Minor `const` optimization.

3.0.7

* Fix a use-after-free issue when thread-local storage is dropped.

3.0.5

* Fix minor linting errors.

3.0.4

* Adjust tests to be more `Miri` friendly.

3.0.3

* Fix a rare memory ordering issue when dropping thread-local storage.

3.0.2

* Make `SDD` much more friendly to `Miri`.

3.0.1

* Compatible with the [`Miri`](https://github.com/rust-lang/miri) memory leak checker.
* Make `Collectible` private since it is unsafe.
* Remove `Guard::defer` which depends on `Collectible`.
* Remove `prepare`.

2.1.0

* Minor performance optimization.
* Remove `Owned::release`.

2.0.0

* `{Owned, Shared}::release` no longer receives a `Guard`.
* `Link` is now public.

1.7.0

* Add `loom` support.

1.6.0

* Add `Guard::accelerate`.

1.5.0

* Fix `Guard::epoch` to return the correct epoch value.

1.4.0

* `Epoch` is now a 4-state type (3 -> 4).

1.3.0

* Add `Epoch`
* Add `Guard::epoch`.

1.2.0

* Remove `Collectible::drop_and_dealloc`.

1.1.0

* Add `prepare`.

1.0.1

* Relax trait bounds of `Guard::defer_execute`.

1.0.0

* Minor code cleanup.

0.2.0

* Make `Guard` `UnwindSafe`.

0.1.0

* Minor optimization.

0.0.1

* Initial commit: code copied from [`scalable-concurrent-containers`](https://github.com/wvwwvwwv/scalable-concurrent-containers).
