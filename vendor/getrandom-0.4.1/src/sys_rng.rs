use crate::Error;
use rand_core::{TryCryptoRng, TryRng};

/// A [`TryRng`] interface over the system's preferred random number source
///
/// This is a zero-sized struct. It can be freely constructed with just `SysRng`.
///
/// This struct is also available as [`rand::rngs::SysRng`] when using [rand].
///
/// # Usage example
///
/// `SysRng` implements [`TryRng`]:
/// ```
/// use getrandom::{rand_core::TryRng, SysRng};
///
/// let mut key = [0u8; 32];
/// SysRng.try_fill_bytes(&mut key).unwrap();
/// ```
///
/// Using it as an [`Rng`] is possible using [`UnwrapErr`]:
/// ```
/// use getrandom::rand_core::{Rng, UnwrapErr};
/// use getrandom::SysRng;
///
/// let mut rng = UnwrapErr(SysRng);
/// let random_u64 = rng.next_u64();
/// ```
///
/// [rand]: https://crates.io/crates/rand
/// [`rand::rngs::SysRng`]: https://docs.rs/rand/latest/rand/rngs/struct.SysRng.html
/// [`Rng`]: rand_core::Rng
/// [`UnwrapErr`]: rand_core::UnwrapErr
#[derive(Clone, Copy, Debug, Default)]
pub struct SysRng;

impl TryRng for SysRng {
    type Error = Error;

    #[inline]
    fn try_next_u32(&mut self) -> Result<u32, Error> {
        crate::u32()
    }

    #[inline]
    fn try_next_u64(&mut self) -> Result<u64, Error> {
        crate::u64()
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        crate::fill(dest)
    }
}

impl TryCryptoRng for SysRng {}
