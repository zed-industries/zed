//! # Cranelift Control
//!
//! This is the home of the control plane of chaos mode, a compilation feature
//! intended to be turned on for certain fuzz targets. When the feature is
//! turned off, as is normally the case, [ControlPlane] will be a zero-sized
//! type and optimized away.
//!
//! While the feature is turned on, the struct [ControlPlane]
//! provides functionality to tap into pseudo-randomness at specific locations
//! in the code. It may be used for targeted fuzzing of compiler internals,
//! e.g. manipulate heuristic optimizations, clobber undefined register bits
//! etc.
//!
//! There are two ways to acquire a [ControlPlane]:
//! - [arbitrary] for the real deal (requires the `fuzz` feature, enabled by default)
//! - [default] for an "empty" control plane which always returns default
//!   values
//!
//! ## Fuel Limit
//!
//! Controls the number of mutations or optimizations that the compiler will
//! perform before stopping.
//!
//! When a perturbation introduced by chaos mode triggers a bug, it may not be
//! immediately clear which of the introduced perturbations was the trigger. The
//! fuel limit can then be used to binary-search for the trigger. It limits the
//! number of perturbations introduced by the control plane. The fuel limit will
//! typically be set with a command line argument passed to a fuzz target. For
//! example:
//! ```sh
//! cargo fuzz run --features chaos $TARGET -- --fuel=16
//! ```
//!
//! ## `no_std` support
//!
//! This crate compiles in `no_std` environments, although both the `fuzz`
//! or `chaos` features have a dependency on `std`. This means that on `no_std`
//! you can't use [arbitrary] to initialize [ControlPlane] and can't enable
//! chaos mode, although the rest of the usual [ControlPlane] API is available.
//!
//! [arbitrary]: ControlPlane#method.arbitrary
//! [default]: ControlPlane#method.default

#![no_std]

// The `alloc` crate is only needed by chaos mode, which guarantees that
// `alloc` is present because of its dependency on `std`.
#[cfg(feature = "chaos")]
extern crate alloc;

#[cfg(not(feature = "chaos"))]
mod zero_sized;
#[cfg(not(feature = "chaos"))]
pub use zero_sized::*;

#[cfg(feature = "chaos")]
mod chaos;
#[cfg(feature = "chaos")]
pub use chaos::*;
