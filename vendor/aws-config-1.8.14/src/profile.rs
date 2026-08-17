/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Load configuration from AWS Profiles
//!
//! AWS profiles are typically stored in `~/.aws/config` and `~/.aws/credentials`. For more details
//! see the [`load`] function.

pub mod parser;

pub mod credentials;
pub mod profile_file;
pub mod region;

#[cfg(feature = "sso")]
pub mod token;
#[cfg(feature = "sso")]
#[doc(inline)]
pub use token::ProfileFileTokenProvider;

#[doc(inline)]
pub use aws_runtime::env_config::error::EnvConfigFileLoadError as ProfileFileLoadError;
#[doc(inline)]
pub use aws_runtime::env_config::parse::EnvConfigParseError as ProfileParseError;
#[doc(inline)]
pub use aws_runtime::env_config::property::Property;
#[doc(inline)]
pub use aws_runtime::env_config::section::{EnvConfigSections as ProfileSet, Profile};
#[doc(inline)]
pub use credentials::ProfileFileCredentialsProvider;
#[doc(inline)]
pub use parser::load;
#[doc(inline)]
pub use region::ProfileFileRegionProvider;

mod cell {
    use std::future::Future;
    use std::sync::{Arc, Mutex};
    use tokio::sync::OnceCell;

    /// Once cell with a result where the error can be taken.
    ///
    /// The profile providers need to cache their inner provider value (specifically for SSO)
    /// in order to preserve the SSO token cache. This wrapper around [`OnceCell`] allows
    /// for initializing the inner provider once in a way that if it fails, the error can
    /// be taken so that it doesn't need to implement `Clone`.
    #[derive(Debug)]
    pub(super) struct ErrorTakingOnceCell<T, E> {
        cell: OnceCell<Result<Arc<T>, Mutex<E>>>,
    }

    impl<T, E> ErrorTakingOnceCell<T, E> {
        pub(super) fn new() -> Self {
            Self {
                cell: OnceCell::new(),
            }
        }

        pub(super) async fn get_or_init<F, Fut>(
            &self,
            init: F,
            mut taken_error: E,
        ) -> Result<Arc<T>, E>
        where
            F: FnOnce() -> Fut,
            Fut: Future<Output = Result<T, E>>,
        {
            let init = || async move { (init)().await.map(Arc::new).map_err(Mutex::new) };
            match self.cell.get_or_init(init).await {
                Ok(value) => Ok(value.clone()),
                Err(err) => {
                    let mut locked = err.lock().unwrap();
                    std::mem::swap(&mut *locked, &mut taken_error);
                    Err(taken_error)
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::profile::cell::ErrorTakingOnceCell;
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };

        #[derive(Debug)]
        enum Error {
            InitError,
            Taken,
        }

        #[tokio::test]
        async fn taken_error() {
            let cell = ErrorTakingOnceCell::new();
            let calls = AtomicUsize::new(0);
            let init = || async {
                calls.fetch_add(1, Ordering::SeqCst);
                Result::<String, _>::Err(Error::InitError)
            };

            let result = cell.get_or_init(init, Error::Taken).await;
            assert!(matches!(result, Err(Error::InitError)));

            let result = cell.get_or_init(init, Error::Taken).await;
            assert!(matches!(result, Err(Error::Taken)));

            let result = cell.get_or_init(init, Error::Taken).await;
            assert!(matches!(result, Err(Error::Taken)));
            assert_eq!(1, calls.load(Ordering::SeqCst));
        }

        #[tokio::test]
        async fn value_initialized_once() {
            let cell = ErrorTakingOnceCell::new();
            let calls = AtomicUsize::new(0);
            let init = || async {
                calls.fetch_add(1, Ordering::SeqCst);
                Result::<_, Error>::Ok("test".to_string())
            };

            let original = cell.get_or_init(init, Error::Taken).await.unwrap();
            let next = cell.get_or_init(init, Error::Taken).await.unwrap();
            assert!(Arc::ptr_eq(&original, &next));
            assert_eq!(1, calls.load(Ordering::SeqCst));
        }
    }
}
