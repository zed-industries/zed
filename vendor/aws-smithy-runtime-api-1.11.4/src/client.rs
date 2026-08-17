/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/// Declares a new-type for a future that is returned from an async trait (prior to stable async trait).
///
/// To declare a future with a static lifetime:
/// ```ignore
/// new_type_future! {
///     doc = "some rustdoc for the future's struct",
///     pub struct NameOfFuture<'static, OutputType, ErrorType>;
/// }
/// ```
///
/// To declare a future with a non-static lifetime:
/// ```ignore
/// new_type_future! {
///     doc = "some rustdoc for the future's struct",
///     pub struct NameOfFuture<'a, OutputType, ErrorType>;
/// }
/// ```
macro_rules! new_type_future {
    (
        #[doc = $type_docs:literal]
        pub struct $future_name:ident<'static, $output:ty, $err:ty>;
    ) => {
        new_type_future!(@internal, $type_docs, $future_name, $output, $err, 'static,);
    };
    (
        #[doc = $type_docs:literal]
        pub struct $future_name:ident<$lifetime:lifetime, $output:ty, $err:ty>;
    ) => {
        new_type_future!(@internal, $type_docs, $future_name, $output, $err, $lifetime, <$lifetime>);
    };
    (@internal, $type_docs:literal, $future_name:ident, $output:ty, $err:ty, $lifetime:lifetime, $($decl_lifetime:tt)*) => {
        pin_project_lite::pin_project! {
            #[allow(clippy::type_complexity)]
            #[doc = $type_docs]
            pub struct $future_name$($decl_lifetime)* {
                #[pin]
                inner: aws_smithy_async::future::now_or_later::NowOrLater<
                    Result<$output, $err>,
                    aws_smithy_async::future::BoxFuture<$lifetime, $output, $err>
                >,
            }
        }

        impl$($decl_lifetime)* $future_name$($decl_lifetime)* {
            #[doc = concat!("Create a new `", stringify!($future_name), "` with the given future.")]
            pub fn new<F>(future: F) -> Self
            where
                F: std::future::Future<Output = Result<$output, $err>> + Send + $lifetime,
            {
                Self {
                    inner: aws_smithy_async::future::now_or_later::NowOrLater::new(Box::pin(future)),
                }
            }

            #[doc = concat!("
            Create a new `", stringify!($future_name), "` with the given boxed future.

            Use this if you already have a boxed future to avoid double boxing it.
            ")]
            pub fn new_boxed(
                future: std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<$output, $err>> + Send + $lifetime>,
                >,
            ) -> Self {
                Self {
                    inner: aws_smithy_async::future::now_or_later::NowOrLater::new(future),
                }
            }

            #[doc = concat!("Create a `", stringify!($future_name), "` that is immediately ready with the given result.")]
            pub fn ready(result: Result<$output, $err>) -> Self {
                Self {
                    inner: aws_smithy_async::future::now_or_later::NowOrLater::ready(result),
                }
            }
        }

        impl$($decl_lifetime)* std::future::Future for $future_name$($decl_lifetime)* {
            type Output = Result<$output, $err>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let this = self.project();
                this.inner.poll(cx)
            }
        }
    };
}

pub mod auth;

pub mod connection;

pub mod connector_metadata;

pub mod dns;

pub mod endpoint;

pub mod http;

/// Smithy identity used by auth and signing.
pub mod identity;

pub mod interceptors;

pub mod orchestrator;

pub mod result;

pub mod retries;

pub mod runtime_components;

pub mod runtime_plugin;

pub mod behavior_version;

pub mod ser_de;

pub mod stalled_stream_protection;

/// Smithy support-code for code generated waiters.
pub mod waiters;
