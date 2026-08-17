/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Types that allow an access token provider to be created from a closure

use crate::provider::{future, token::ProvideToken};
use std::fmt::{self, Debug, Formatter};
use std::future::Future;
use std::marker::PhantomData;

/// A [`ProvideToken`] implemented by a closure.
///
/// See [`provide_token_fn`] for more details.
#[derive(Copy, Clone)]
pub struct ProvideTokenFn<'c, T> {
    f: T,
    phantom: PhantomData<&'c T>,
}

impl<T> Debug for ProvideTokenFn<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ProvideTokenFn")
    }
}

impl<'c, T, F> ProvideToken for ProvideTokenFn<'c, T>
where
    T: Fn() -> F + Send + Sync + 'c,
    F: Future<Output = crate::provider::token::Result> + Send + 'static,
{
    fn provide_token<'a>(&'a self) -> future::ProvideToken<'a>
    where
        Self: 'a,
    {
        future::ProvideToken::new((self.f)())
    }
}

/// Returns a new token provider built with the given closure. This allows you
/// to create an [`ProvideToken`] implementation from an async block that returns
/// a [`crate::provider::token::Result`].
///
/// # Examples
///
/// ```no_run
/// use aws_credential_types::Token;
/// use aws_credential_types::token_fn::provide_token_fn;
///
/// async fn load_token() -> Token {
///     todo!()
/// }
///
/// provide_token_fn(|| async {
///     // Async process to retrieve a token goes here
///     let token = load_token().await;
///     Ok(token)
/// });
/// ```
pub fn provide_token_fn<'c, T, F>(f: T) -> ProvideTokenFn<'c, T>
where
    T: Fn() -> F + Send + Sync + 'c,
    F: Future<Output = crate::provider::token::Result> + Send + 'static,
{
    ProvideTokenFn {
        f,
        phantom: Default::default(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Token;

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn creds_are_send_sync() {
        assert_send_sync::<Token>()
    }

    // Test that the closure passed to `provide_token_fn` is allowed to borrow things
    #[tokio::test]
    async fn provide_token_fn_closure_can_borrow() {
        fn check_is_str_ref(_input: &str) {}
        async fn test_async_provider(input: String) -> crate::provider::token::Result {
            Ok(Token::new(input, None))
        }

        let things_to_borrow = vec!["one".to_string(), "two".to_string()];

        let mut providers = Vec::new();
        for thing in &things_to_borrow {
            let provider = provide_token_fn(move || {
                check_is_str_ref(thing);
                test_async_provider(thing.into())
            });
            providers.push(provider);
        }

        let (two, one) = (providers.pop().unwrap(), providers.pop().unwrap());
        assert_eq!("one", one.provide_token().await.unwrap().token());
        assert_eq!("two", two.provide_token().await.unwrap().token());
    }
}
