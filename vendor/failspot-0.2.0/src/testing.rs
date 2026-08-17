//! Allows testing code to enable and disable failspots
//!
//! When testing a crate, failspots can be enabled through a [Client] object. This can be retrieved
//! by using the `fn testing_client() -> Client<'static, Self>` method that exists as part of
//! every enum that was declared using the [`failspot_name!()`][crate::failspot_name] macro.
//!
//! The [`Client::set_enabled()`][Client::set_enabled] method can be used to set or unset a
//! failspot. [`Client::reset()`][Client::reset] will unset all failspots.
//!
//! Example usage:
//!
//! ```
//! # failspot::failspot_name! { pub enum FailSpotName { Name1 } }
//! # fn run_tests() {}
//! let mut client = FailSpotName::testing_client();
//! client.set_enabled(FailSpotName::Name1, true);
//! // When the `Client` object drops, all the failspots will be reset to disabled
//! // Must ensure it stays alive while tests are running.
//! run_tests();
//! ```
//!
//! # Concurrency -- Important!!!
//!
//! **TL;DR -- Put all your integration tests that use failspot in a separate source file!**
//!
//! ## The problem
//!
//! In Rust, **tests are run concurrently by default**. Since the configuration for the failspots
//! is a global variable that will be shared by all threads, that would create a problem -- Tests
//! that don't use failspots will suddenly start failing because another concurrent test enabled
//! them, and tests that do use failspots would clobber each other's configuration.
//!
//! To prevent this, the [Client] returned by `testing_client()` is **protected by a mutex** --
//! Only one test at a time can configure the failspots through the `Client` methods. When the
//! client is dropped, all the failspots are reset to disabled state and the mutex is released so
//! the next test can start with a fresh state.
//!
//! This means **every test that may run concurrently with a failspot test must hold the [Client]
//! object the entire time the test is running**, even if that test doesn't actually use failspots.
//! If there are multiple enums declared with [`failspot_name!()`][crate::failspot_name] then a
//! [Client] object for each enum must be held by every test that may run concurrently.
//!
//! For tests that use failspots, this is intuitive -- Most tests that use failspots will create a
//! [Client] as part of their setup.
//!
//! ## Stopping regular tests from breaking
//!
//! For tests that don't use failspots, there are 2 choices:
//!
//! 1.  **Put failspot tests in their own source file (recommended)** Integration tests in
//!     different source files are run in different processes, so separating failspot and non
//!     failspot tests eliminates the concurrency issue.
//!
//! 2.  **Force tests to run serially** By setting `RUST_TEST_THREADS=1` in the enviroment, the
//!     tests will run one-at-a-time and there will be no interference.
//!
//! Obviously, the first one should be preferred unless there is a good reason not to.

use {
    flagset::FlagSet,
    std::{
        ops::{Deref, DerefMut},
        sync::{Mutex, MutexGuard, RwLock},
    },
};

/// Config object for an enum declared with [`failspot_name!()`][crate::failspot_name]
///
/// Every failspot enum has one of these attached. It tracks which failspots are currently
/// enabled for that enum, and contains the mutex that ensures that only one [Client] at a time
/// is running. It is not normally used directly by user code, but is instead used by the
/// [`failspot!()`][crate::failspot] macro for testing failpoints, and by the `testing_client()`
/// method to obtain a [Client] for testing code.
#[derive(Debug)]
pub struct Config<T: flagset::Flags> {
    inner: RwLock<ConfigInner<T>>,
    client_mutex: Mutex<()>,
}

#[derive(Debug, Eq, PartialEq)]
struct ConfigInner<T: flagset::Flags> {
    enabled_spots: FlagSet<T>,
}

impl<T: flagset::Flags> Default for Config<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            client_mutex: Default::default(),
        }
    }
}

impl<T: flagset::Flags> Default for ConfigInner<T> {
    fn default() -> Self {
        Self {
            enabled_spots: Default::default(),
        }
    }
}

impl<T: flagset::Flags> Config<T> {
    /// Returns whether or not the given failspot is enabled
    pub fn enabled(&self, spot: T) -> bool {
        self.inner().enabled_spots.contains(spot)
    }
    /// Returns a client for this failspot config
    pub fn client(&self) -> Client<'_, T> {
        Client::new(self)
    }
    fn inner(&self) -> impl Deref<Target = ConfigInner<T>> + '_ {
        self.inner.read().unwrap()
    }
    fn inner_mut(&self) -> impl DerefMut<Target = ConfigInner<T>> + '_ {
        self.inner.write().unwrap()
    }
}

/// Client for testing code
///
/// See [module-level docs][self], especially the part about concurrency.
#[derive(Debug)]
pub struct Client<'a, T: flagset::Flags> {
    config: &'a Config<T>,
    _guard: MutexGuard<'a, ()>,
}

impl<'a, T: flagset::Flags> Client<'a, T> {
    /// Create a new [Client].
    ///
    /// Normally not used directly -- Use `EnumName::testing_client()` instead
    pub fn new(config: &'a Config<T>) -> Self {
        let _guard = config
            .client_mutex
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        assert_eq!(
            *config.inner(),
            ConfigInner::default(),
            "somehow failed to reset config to default after last client"
        );

        Client { config, _guard }
    }
    /// Set whether the given failspot is enabled or disabled
    pub fn set_enabled(&mut self, spot: T, enabled: bool) -> &mut Self {
        if enabled {
            self.config.inner_mut().enabled_spots |= spot;
        } else {
            self.config.inner_mut().enabled_spots -= spot;
        }
        self
    }
    /// Reset all failspots to disabled
    pub fn reset(&mut self) -> &mut Self {
        *self.config.inner_mut() = ConfigInner::default();
        self
    }
    /// Finish with a [Client], resetting all failspots to disabled and releasing the mutex
    ///
    /// Identical to dropping the [Client], but a bit more explicit about intent.
    pub fn finish(self) {
        drop(self)
    }
}

impl<'a, T: flagset::Flags> Drop for Client<'a, T> {
    fn drop(&mut self) {
        self.reset();
    }
}
