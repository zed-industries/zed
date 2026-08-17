//! Limitations and common pitfalls.
//!
//! # Sized types
//!
//! This currently works only for `Sized` types. Unsized types have „fat pointers“, which are twice
//! as large as the normal ones. The [`AtomicPtr`] doesn't support them. One could use something
//! like `AtomicU128` for them. The catch is this doesn't exist and the difference would make it
//! really hard to implement the debt storage/stripped down hazard pointers.
//!
//! A workaround is to use double indirection:
//!
//! ```rust
//! # use arc_swap::ArcSwap;
//! // This doesn't work:
//! // let data: ArcSwap<[u8]> = ArcSwap::new(Arc::from([1, 2, 3]));
//!
//! // But this does:
//! let data: ArcSwap<Box<[u8]>> = ArcSwap::from_pointee(Box::new([1, 2, 3]));
//! # drop(data);
//! ```
//!
//! It also may be possible to use `ArcSwap` with the [`triomphe::ThinArc`] (that crate needs
//! enabling a feature flag to cooperate with `ArcSwap`).
//!
//! # Too many [`Guard`]s
//!
//! There's only limited number of "fast" slots for borrowing from [`ArcSwap`] for each single
//! thread (currently 8, but this might change in future versions). If these run out, the algorithm
//! falls back to slower path.
//!
//! If too many [`Guard`]s are kept around, the performance might be poor. These are not intended
//! to be stored in data structures or used across async yield points.
//!
//! [`ArcSwap`]: crate::ArcSwap
//! [`Guard`]: crate::Guard
//! [`AtomicPtr`]: std::sync::atomic::AtomicPtr
//!
//! # No `Clone` implementation
//!
//! Previous version implemented [`Clone`], but it turned out to be very confusing to people, since
//! it created fully independent [`ArcSwap`]. Users expected the instances to be tied to each
//! other, that store in one would change the result of future load of the other.
//!
//! To emulate the original behaviour, one can do something like this:
//!
//! ```rust
//! # use arc_swap::ArcSwap;
//! # let old = ArcSwap::from_pointee(42);
//! let new = ArcSwap::new(old.load_full());
//! # let _ = new;
//! ```
//!
//! [`triomphe::ThinArc`]: https://docs.rs/triomphe/latest/triomphe/struct.ThinArc.html
