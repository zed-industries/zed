//! Capability-based random number generators
//!
//! This corresponds to [`rand`].
//!
//! Capability-based APIs represent access to external resources as values
//! which can be passed around between different parts of a program.
//!
//! Two notable features are the [`OsRng`] and [`CapRng`] types, which
//! wrap up access to the operating system entropy source in capability
//! values.
//!
//! This crate uses the existing `rand::SeedableRng` trait rather than having
//! its own version, however while `rand::SeedableRng` is mostly just a pure
//! interface, it provides a `from_entropy` function which directly reads
//! from the operating system entropy source. To preserve the
//! capability-based interface, avoid using `rand::SeedableRng`'s
//! `from_entropy` function on any of the types that implement that trait; use
//! [`std_rng_from_entropy`] instead.
//!
//! [`OsRng`]: crate::rngs::OsRng
//! [`CapRng`]: crate::rngs::CapRng

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.ico"
)]

#[doc(hidden)]
pub use ambient_authority::ambient_authority_known_at_compile_time;
pub use ambient_authority::{ambient_authority, AmbientAuthority};
pub use rand::{distributions, seq, CryptoRng, Error, Fill, Rng, RngCore, SeedableRng};

/// Convenience re-export of common members.
///
/// This corresponds to [`rand::prelude`].
pub mod prelude {
    pub use crate::distributions::Distribution;
    #[cfg(feature = "small_rng")]
    pub use crate::rngs::SmallRng;
    pub use crate::rngs::{CapRng, StdRng};
    pub use crate::seq::{IteratorRandom, SliceRandom};
    pub use crate::{random, thread_rng, CryptoRng, Rng, RngCore, SeedableRng};
}

/// Random number generators and adapters.
///
/// This corresponds to [`rand::rngs`].
pub mod rngs {
    use super::AmbientAuthority;

    pub use rand::rngs::{adapter, mock, StdRng};

    #[cfg(feature = "small_rng")]
    pub use rand::rngs::SmallRng;

    /// A random number generator that retrieves randomness from the operating
    /// system.
    ///
    /// This corresponds to [`rand::rngs::OsRng`], except instead of
    /// implementing `Default` it has an ambient-authority `default` function
    /// to access the operating system.
    #[derive(Clone, Copy, Debug)]
    pub struct OsRng(());

    impl OsRng {
        /// Returns an `OsRng` instance.
        ///
        /// # Ambient Authority
        ///
        /// This function makes use of ambient authority to access the platform
        /// entropy source.
        #[inline]
        pub const fn default(ambient_authority: AmbientAuthority) -> Self {
            let _ = ambient_authority;
            Self(())
        }
    }

    impl crate::RngCore for OsRng {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            rand::rngs::OsRng.next_u32()
        }

        #[inline]
        fn next_u64(&mut self) -> u64 {
            rand::rngs::OsRng.next_u64()
        }

        #[inline]
        fn fill_bytes(&mut self, bytes: &mut [u8]) {
            rand::rngs::OsRng.fill_bytes(bytes)
        }

        #[inline]
        fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), crate::Error> {
            rand::rngs::OsRng.try_fill_bytes(bytes)
        }
    }

    impl crate::CryptoRng for OsRng {}

    /// The type returned by `thread_rng`, essentially just a reference to a
    /// PRNG in memory.
    ///
    /// This corresponds to [`rand::rngs::ThreadRng`], except that it isn't
    /// tied to thread-local memory.
    #[derive(Clone, Debug)]
    pub struct CapRng {
        pub(super) inner: rand::rngs::ThreadRng,
    }

    impl CapRng {
        /// A convenience alias for calling `thread_rng`.
        ///
        /// # Ambient Authority
        ///
        /// This function makes use of ambient authority to access the platform
        /// entropy source.
        #[inline]
        pub fn default(ambient_authority: AmbientAuthority) -> Self {
            crate::thread_rng(ambient_authority)
        }
    }

    impl crate::RngCore for CapRng {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            self.inner.next_u32()
        }

        #[inline]
        fn next_u64(&mut self) -> u64 {
            self.inner.next_u64()
        }

        #[inline]
        fn fill_bytes(&mut self, bytes: &mut [u8]) {
            self.inner.fill_bytes(bytes)
        }

        #[inline]
        fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), crate::Error> {
            self.inner.try_fill_bytes(bytes)
        }
    }

    impl crate::CryptoRng for CapRng {}
}

/// Retrieve the lazily-initialized thread-local random number generator,
/// seeded by the system.
///
/// This corresponds to [`rand::thread_rng`].
///
/// # Ambient Authority
///
/// This function makes use of ambient authority to access the platform entropy
/// source.
#[inline]
pub fn thread_rng(ambient_authority: AmbientAuthority) -> rngs::CapRng {
    let _ = ambient_authority;
    rngs::CapRng {
        inner: rand::thread_rng(),
    }
}

/// Retrieve the standard random number generator, seeded by the system.
///
/// This corresponds to [`rand::rngs::StdRng::from_entropy`].
///
/// # Ambient Authority
///
/// This function makes use of ambient authority to access the platform entropy
/// source.
#[inline]
pub fn std_rng_from_entropy(ambient_authority: AmbientAuthority) -> rngs::StdRng {
    let _ = ambient_authority;
    rand::rngs::StdRng::from_entropy()
}

/// Generates a random value using the thread-local random number generator.
///
/// This corresponds to [`rand::random`].
///
/// # Ambient Authority
///
/// This function makes use of ambient authority to access the platform entropy
/// source.
#[inline]
pub fn random<T>(ambient_authority: AmbientAuthority) -> T
where
    crate::distributions::Standard: crate::distributions::Distribution<T>,
{
    let _ = ambient_authority;
    rand::random()
}
