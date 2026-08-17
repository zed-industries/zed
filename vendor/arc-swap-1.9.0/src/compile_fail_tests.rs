// The doc tests allow us to do a compile_fail test, which is cool and what we want, but we don't
// want to expose this in the docs, so we use a private struct for that reason.
//
// Note we also bundle one that *does* compile with each, just to make sure they don't silently
// not-compile by some different reason.
//! ```rust,compile_fail
//! let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
//! std::thread::spawn(|| {
//!     drop(shared);
//! });
//! ```
//!
//! ```rust
//! let shared = arc_swap::ArcSwap::from_pointee(42);
//! std::thread::spawn(|| {
//!     drop(shared);
//! })
//! .join()
//! .unwrap();
//! ```
//!
//! ```rust,compile_fail
//! let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
//! let guard = shared.load();
//! std::thread::spawn(|| {
//!     drop(guard);
//! });
//! ```
//!
//! ```rust
//! let shared = arc_swap::ArcSwap::from_pointee(42);
//! let guard = shared.load();
//! std::thread::spawn(|| {
//!     drop(guard);
//! })
//! .join()
//! .unwrap();
//! ```
//!
//! ```rust,compile_fail
//! let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
//! crossbeam_utils::thread::scope(|scope| {
//!     scope.spawn(|_| {
//!         let _ = &shared;
//!     });
//! }).unwrap();
//! ```
//!
//! ```rust
//! let shared = arc_swap::ArcSwap::from_pointee(42);
//! crossbeam_utils::thread::scope(|scope| {
//!     scope.spawn(|_| {
//!         let _ = &shared;
//!     });
//! }).unwrap();
//! ```
//!
//! ```rust,compile_fail
//! let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
//! let guard = shared.load();
//! crossbeam_utils::thread::scope(|scope| {
//!     scope.spawn(|_| {
//!         let _ = &guard;
//!     });
//! }).unwrap();
//! ```
//!
//! ```rust
//! let shared = arc_swap::ArcSwap::from_pointee(42);
//! let guard = shared.load();
//! crossbeam_utils::thread::scope(|scope| {
//!     scope.spawn(|_| {
//!         let _ = &guard;
//!     });
//! }).unwrap();
//! ```
//!
//! See that `ArcSwapAny<Rc>` really isn't Send.
//! ```rust
//! use std::sync::Arc;
//! use arc_swap::ArcSwapAny;
//!
//! let a: ArcSwapAny<Arc<usize>> = ArcSwapAny::new(Arc::new(42));
//! std::thread::spawn(move || drop(a)).join().unwrap();
//! ```
//!
//! ```rust,compile_fail
//! use std::rc::Rc;
//! use arc_swap::ArcSwapAny;
//!
//! let a: ArcSwapAny<Rc<usize>> = ArcSwapAny::new(Rc::new(42));
//! std::thread::spawn(move || drop(a));
//! ```
