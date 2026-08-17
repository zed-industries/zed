/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_http_client::{
    tls::{self, rustls_provider::CryptoMode},
    Builder,
};

#[tokio::main]
async fn main() {
    // feature = rustls-aws-lc
    let _client = Builder::new()
        .tls_provider(tls::Provider::Rustls(CryptoMode::AwsLc))
        .build_https();

    // feature = rustls-aws-lc-fips
    // A FIPS client can also be created. Note that this has a more complex build environment required.
    let _client = Builder::new()
        .tls_provider(tls::Provider::Rustls(CryptoMode::AwsLcFips))
        .build_https();
}
