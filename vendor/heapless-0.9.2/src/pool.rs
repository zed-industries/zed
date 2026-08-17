//! Memory and object pools
//!
//! # Target support
//!
//! This module/API is only available on these compilation targets:
//!
//! - ARM architectures which instruction set include the LDREX, CLREX and STREX instructions, e.g.
//!   `thumbv7m-none-eabi` but not `thumbv6m-none-eabi`
//! - 32-bit and 64-bit x86.
//!
//! # Benchmarks
//!
//! - compilation settings
//!   - `codegen-units = 1`
//!   - `lto = 'fat'`
//!   - `opt-level = 'z'`
//! - compilation target: `thumbv7em-none-eabihf`
//! - CPU: ARM Cortex-M4F
//!
//! - test program:
//!
//! ```no_run
//! use heapless::box_pool;
//!
//! box_pool!(MyBoxPool: ()); // or `arc_pool!` or `object_pool!`
//!
//! bkpt();
//! let res = MyBoxPool.alloc(());
//! bkpt();
//!
//! if let Ok(boxed) = res {
//!     bkpt();
//!     drop(boxed);
//!     bkpt();
//! }
//! # fn bkpt() {}
//! ```
//!
//! - measurement method: the cycle counter (CYCCNT) register was sampled each time a breakpoint
//!   (`bkpt`) was hit. the difference between the "after" and the "before" value of CYCCNT yields
//!   the execution time in clock cycles.
//!
//! | API                          | clock cycles |
//! |------------------------------|--------------|
//! | `BoxPool::alloc`             | 23           |
//! | `pool::boxed::Box::drop`     | 23           |
//! | `ArcPool::alloc`             | 28           |
//! | `pool::arc::Arc::drop`       | 59           |
//! | `ObjectPool::request`        | 23           |
//! | `pool::object::Object::drop` | 23           |
//!
//! Note that the execution time won't include `T`'s initialization nor `T`'s destructor which will
//! be present in the general case for `Box` and `Arc`.

mod treiber;

#[cfg(any(feature = "portable-atomic", target_has_atomic = "ptr"))]
pub mod arc;
pub mod boxed;
pub mod object;
