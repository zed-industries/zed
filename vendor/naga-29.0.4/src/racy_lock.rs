#[cfg(no_std)]
use alloc::boxed::Box;
#[cfg(no_std)]
use once_cell::race::OnceBox;
#[cfg(std)]
use std::sync::LazyLock;

#[cfg(std)]
type Inner<T> = LazyLock<T, fn() -> T>;
#[cfg(no_std)]
type Inner<T> = OnceBox<T>;

/// Lazy static helper that uses [`LazyLock`] with `std` and [`OnceBox`] otherwise.
///
/// [`LazyLock`]: https://doc.rust-lang.org/stable/std/sync/struct.LazyLock.html
/// [`OnceBox`]: https://docs.rs/once_cell/latest/once_cell/race/struct.OnceBox.html
pub struct RacyLock<T: 'static> {
    inner: Inner<T>,
    #[cfg(no_std)]
    init: fn() -> T,
}

impl<T: 'static> RacyLock<T> {
    #[cfg(std)]
    /// Creates a new [`RacyLock`], which will initialize using the provided `init` function.
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            inner: LazyLock::new(init),
        }
    }

    #[cfg(no_std)]
    /// Creates a new [`RacyLock`], which will initialize using the provided `init` function.
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            inner: OnceBox::new(),
            init,
        }
    }
}

#[cfg(std)]
impl<T: 'static> core::ops::Deref for RacyLock<T> {
    type Target = T;

    /// Loads the internal value, initializing it if required.
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(no_std)]
impl<T: 'static> core::ops::Deref for RacyLock<T> {
    type Target = T;

    /// Loads the internal value, initializing it if required.
    fn deref(&self) -> &Self::Target {
        self.inner.get_or_init(|| Box::new((self.init)()))
    }
}
