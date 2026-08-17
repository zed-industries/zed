/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Module to extend the functionality of types in `patination_stream` module to allow for
//! collecting elements of the stream into collection.
//!
//! Majority of the code is borrowed from
//! <https://github.com/tokio-rs/tokio/blob/fc9518b62714daac9a38b46c698b94ac5d5b1ca2/tokio-stream/src/stream_ext/collect.rs>

pub(crate) mod sealed {
    /// A trait that signifies that elements can be collected into `T`.
    ///
    /// Currently the trait may not be implemented by clients so we can make changes in the future
    /// without breaking code depending on it.
    pub trait Collectable<T> {
        type Collection;

        fn initialize() -> Self::Collection;

        fn extend(collection: &mut Self::Collection, item: T) -> bool;

        fn finalize(collection: Self::Collection) -> Self;
    }
}

impl<T> sealed::Collectable<T> for Vec<T> {
    type Collection = Self;

    fn initialize() -> Self::Collection {
        Vec::default()
    }

    fn extend(collection: &mut Self::Collection, item: T) -> bool {
        collection.push(item);
        true
    }

    fn finalize(collection: Self::Collection) -> Self {
        collection
    }
}

impl<T, U, E> sealed::Collectable<Result<T, E>> for Result<U, E>
where
    U: sealed::Collectable<T>,
{
    type Collection = Result<U::Collection, E>;

    fn initialize() -> Self::Collection {
        Ok(U::initialize())
    }

    fn extend(collection: &mut Self::Collection, item: Result<T, E>) -> bool {
        match item {
            Ok(item) => {
                let collection = collection.as_mut().ok().expect("invalid state");
                U::extend(collection, item)
            }
            Err(e) => {
                *collection = Err(e);
                false
            }
        }
    }

    fn finalize(collection: Self::Collection) -> Self {
        match collection {
            Ok(collection) => Ok(U::finalize(collection)),
            err @ Err(_) => Err(err.map(drop).unwrap_err()),
        }
    }
}
