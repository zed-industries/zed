/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Built-in DNS resolver implementations.

#[cfg(all(feature = "rt-tokio", not(target_family = "wasm")))]
mod tokio {
    use aws_smithy_runtime_api::client::dns::{DnsFuture, ResolveDns, ResolveDnsError};
    use std::io::Error as IoError;
    use std::net::ToSocketAddrs;

    /// DNS resolver that uses `tokio::spawn_blocking` to resolve DNS using the standard library.
    ///
    /// This implementation isn't available for WASM targets.
    #[non_exhaustive]
    #[derive(Debug, Default, Clone)]
    pub struct TokioDnsResolver;

    impl TokioDnsResolver {
        /// Creates a new Tokio DNS resolver
        pub fn new() -> Self {
            Self
        }
    }

    impl ResolveDns for TokioDnsResolver {
        fn resolve_dns<'a>(&'a self, name: &'a str) -> DnsFuture<'a> {
            let name = name.to_string();
            DnsFuture::new(async move {
                let result = tokio::task::spawn_blocking(move || (name, 0).to_socket_addrs()).await;
                match result {
                    Err(join_failure) => Err(ResolveDnsError::new(IoError::other(join_failure))),
                    Ok(Ok(dns_result)) => {
                        Ok(dns_result.into_iter().map(|addr| addr.ip()).collect())
                    }
                    Ok(Err(dns_failure)) => Err(ResolveDnsError::new(dns_failure)),
                }
            })
        }
    }
}

#[cfg(all(feature = "rt-tokio", not(target_family = "wasm")))]
pub use self::tokio::TokioDnsResolver;
