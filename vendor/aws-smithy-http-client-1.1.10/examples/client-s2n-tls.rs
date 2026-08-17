/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_http_client::{tls, Builder};

fn main() {
    let _client = Builder::new()
        .tls_provider(tls::Provider::S2nTls)
        .build_https();
}
