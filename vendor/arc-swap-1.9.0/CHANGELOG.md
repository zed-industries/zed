# 1.9.0

* Promote certain orderings to SeqCst. Original proofs based on wrong reading of
  standard :-(. Expect some performance degradation (#198, #200).

# 1.8.2

* Proper gate of `Pin` (since 1.39 - we are not using only `Pin`, but also
  `Pin::into_inner`, #197).

# 1.8.1

* Some more careful orderings (#195).

# 1.8.0

* Support for Pin (#185, #183).
* Fix (hopefully) crash on ARM (#164).
* Fix Miri check (#186, #156).
* Fix support for Rust 1.31.0.
* Some minor clippy lints.

# 1.7.1

* Fix docs build (mutually exclusive features, #112).

# 1.7.0

* Support for no-std builds with the `experimental-thread-local`. Needs nightly
  compiler. No stability guarantees with this feature (#93).

# 1.6.0

* Fix a data race reported by MIRI.
* Avoid violating stacked borrows (AFAIK these are still experimental and not
  normative, but better safe than sorry). (#80).
* The `AccessConvert` wrapper is needed less often in practice (#77).

# 1.5.1

* bug: Insufficient synchronization on weak platforms (#76).

  Never observed in practice (it's suspected practical weak platforms like ARM
  are still stronger than the model), but still technically UB.
* docs: Mention triomphe's `ThinArc` around the fat-pointer limitations.

# 1.5.0

* Support serde (by a feature).

# 1.4.0

* Allow const-initializing ArcSwapOption (`const_empty` method).

# 1.3.2

* More helpful description of the `AsRaw` trait (isn't implemented for owned
  `Arc`/`Option<Arc>`).

# 1.3.1

* Cache doc improvements.

# 1.3.0

* Allow mapping of DynAccess.
* Fix some lints.
* Don't leave threads running in tests/doctests. It's a bad form and annoys
  miri.

# 1.2.0

* Miri and 32 bit tests in CI.
* Making the writers lock-free. Soft-removing the IndependentStrategy, as it is
  no longer needed (hidden and the same as the DafultStrategy).

# 1.1.0

* Fix soundness bug around access::Map. Technically a breaking change, but
  unlikely to bite and breaking seems to be the least bad option. #45.

# 1.0.0

* Remove Clone implementation. People are often confused by it and it is easy to
  emulate by hand in the rare case it is actually needed.

# 1.0.0-rc1

* Get rid of the `load_signal_safe`. It only complicates things and it is niche;
  signal-hook-registry has its own simplified version.
* Avoid `from_ptr(as_ptr())`. Slight change in `RefCnt::inc` which technically
  is API breaking change, but this one should not matter in practice.
* Extend documentation about clone behaviour.
* Few more traits for Guard (`From<T: RefCnt>`, `Default`).
* Get rid of `rcu_unwap`, the whole concept is a trap.
* Hide the whole gen lock thing.
* Introduce the `Strategy`, as a high level way to choose how exactly the
  locking happens.
  - Not possible to implement by downstream users just yet, or call them.
  - The CaS is its own trait for flexibility.
* Adding the SimpleGenLock experimental strategy.
  - Not part of stability guarantees.

# 0.4.7

* Rename the `unstable-weak` to `weak` feature. The support is now available on
  1.45 (currently in beta).

# 0.4.6

* Adjust to `Weak::as_ptr` from std (the weak pointer support, relying on
  unstable features).
* Support running on miri (without some optimizations), so dependencies may run
  miri tests.
* Little optimization when waiting out the contention on write operations.

# 0.4.5

* Added `Guard::from_inner`.

# 0.4.4

* Top-level docs rewrite (less rambling, hopefully more readable).

# 0.4.3

* Fix the `Display` implementation on `Guard` to correctly delegate to the
  underlying `Display` implementation.

# 0.4.2

* The Access functionality ‒ ability to pass a handle to subpart of held data to
  somewhere with the ability to update itself.
* Mapped cache can take `FnMut` as well as `Fn`.

# 0.4.1

* Mapped caches ‒ to allow giving access to parts of config only.

# 0.4.0

* Support for Weak pointers.
* RefCnt implemented for Rc.
* Breaking: Big API cleanups.
  - Peek is gone.
  - Terminology of getting the data unified to `load`.
  - There's only one kind of `Guard` now.
  - Guard derefs to the `Arc`/`Option<Arc>` or similar.
  - `Cache` got moved to top level of the crate.
  - Several now unneeded semi-internal traits and trait methods got removed.
* Splitting benchmarks into a separate sub-crate.
* Minor documentation improvements.

# 0.3.11

* Prevention against UB due to dropping Guards and overflowing the guard
  counter (aborting instead, such problem is very degenerate anyway and wouldn't
  work in the first place).

# 0.3.10

* Tweak slot allocation to take smaller performance hit if some leases are held.
* Increase the number of lease slots per thread to 8.
* Added a cache for faster access by keeping an already loaded instance around.

# 0.3.9

* Fix Send/Sync for Guard and Lease (they were broken in the safe but
  uncomfortable direction ‒ not implementing them even if they could).

# 0.3.8

* `Lease<Option<_>>::unwrap()`, `expect()` and `into_option()` for convenient
  use.

# 0.3.7

* Use the correct `#[deprecated]` syntax.

# 0.3.6

* Another locking store (`PrivateSharded`) to complement the global and private
  unsharded ones.
* Comparison to other crates/approaches in the docs.

# 0.3.5

* Updates to documentation, made it hopefully easier to digest.
* Added the ability to separate gen-locks of one ArcSwapAny from others.
* Some speed improvements by inlining.
* Simplified the `lease` method internally, making it faster in optimistic
  cases.

# 0.3.4

* Another potentially weak ordering discovered (with even less practical effect
  than the previous).

# 0.3.3

* Increased potentially weak ordering (probably without any practical effect).

# 0.3.2

* Documentation link fix.

# 0.3.1

* Few convenience constructors.
* More tests (some randomized property testing).

# 0.3.0

* `compare_and_swap` no longer takes `&Guard` as current as that is a sure way
  to create a deadlock.
* Introduced `Lease` for temporary storage, which doesn't suffer from contention
  like `load`, but doesn't block writes like `Guard`. The downside is it slows
  down with number of held by the current thread.
* `compare_and_swap` and `rcu` uses leases.
* Made the `ArcSwap` as small as the pointer itself, by making the
  shards/counters and generation ID global. This comes at a theoretical cost of
  more contention when different threads use different instances.

# 0.2.0

* Added an `ArcSwapOption`, which allows storing NULL values (as None) as well
  as a valid pointer.
* `compare_and_swap` accepts borrowed `Arc` as `current` and doesn't consume one
  ref count.
* Sharding internal counters, to improve performance on read-mostly contented
  scenarios.
* Providing `peek_signal_safe` as the only async signal safe method to use
  inside signal handlers. This removes the footgun with dropping the `Arc`
  returned from `load` inside a signal handler.

# 0.1.4

* The `peek` method to use the `Arc` inside without incrementing the reference
  count.
* Some more (and hopefully better) benchmarks.

# 0.1.3

* Documentation fix (swap is *not* lock-free in current implementation).

# 0.1.2

* More freedom in the `rcu` and `rcu_unwrap` return types.

# 0.1.1

* `rcu` support.
* `compare_and_swap` support.
* Added some primitive benchmarks.

# 0.1.0

* Initial implementation.
