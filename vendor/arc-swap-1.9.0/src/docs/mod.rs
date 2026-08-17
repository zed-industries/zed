//! Additional documentation.
//!
//! Here we have some more general topics that might be good to know that just don't fit to the
//! crate level intro.
//!
//! Also, there were some previous blog posts about the crate which you might find interesting.
//!
//! # Atomic orderings
//!
//! Each operation on the [`ArcSwapAny`] with [`DefaultStrategy`] type callable concurrently (eg.
//! [`load`], but not [`into_inner`]) contains at least one [`SeqCst`] atomic read-write operation,
//! therefore even operations on different instances have a defined global order of operations.
//!
//! # Features
//!
//! The `weak` feature adds the ability to use arc-swap with the [`Weak`] pointer too,
//! through the [`ArcSwapWeak`] type. The needed std support is stabilized in rust version 1.45 (as
//! of now in beta).
//!
//! The `experimental-strategies` enables few more strategies that can be used. Note that these
//! **are not** part of the API stability guarantees and they may be changed, renamed or removed at
//! any time.
//!
//! The `experimental-thread-local` feature can be used to build arc-swap for `no_std` targets, by
//! replacing occurences of [`std::thread_local!`] with the `#[thread_local]` directive. This
//! requires a nightly Rust compiler as it makes use of the experimental
//! [`thread_local`](https://doc.rust-lang.org/unstable-book/language-features/thread-local.html)
//! feature. Using this features, thread-local variables are compiled using LLVM built-ins, which
//! have [several underlying modes of
//! operation](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/tls-model.html).  To add
//! support for thread-local variables on a platform that does not have OS or linker support, the
//! easiest way is to use `-Ztls-model=emulated` and to implement `__emutls_get_address` by hand,
//! as in [this
//! example](https://opensource.apple.com/source/clang/clang-800.0.38/src/projects/compiler-rt/lib/builtins/emutls.c.auto.html)
//! from Clang.
//!
//! # Minimal compiler version
//!
//! The `1` versions will compile on all compilers supporting the 2018 edition. Note that this
//! applies only if no additional feature flags are enabled and does not apply to compiling or
//! running tests.
//!
//! [`ArcSwapAny`]: crate::ArcSwapAny
//! [`ArcSwapWeak`]: crate::ArcSwapWeak
//! [`load`]: crate::ArcSwapAny::load
//! [`into_inner`]: crate::ArcSwapAny::into_inner
//! [`DefaultStrategy`]: crate::DefaultStrategy
//! [`SeqCst`]: std::sync::atomic::Ordering::SeqCst
//! [`Weak`]: std::sync::Weak

pub mod internal;
pub mod limitations;
pub mod patterns;
pub mod performance;
