/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::context::Input;
use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
use aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;
use aws_smithy_runtime_api::client::ser_de::{SerializeRequest, SharedRequestSerializer};
use aws_smithy_types::config_bag::{ConfigBag, FrozenLayer, Layer};
use std::sync::Mutex;

/// Test [`SerializeRequest`] that returns a canned request.
#[derive(Default, Debug)]
pub struct CannedRequestSerializer {
    inner: Mutex<Option<Result<HttpRequest, BoxError>>>,
}

impl CannedRequestSerializer {
    /// Create a new [`CannedRequestSerializer`] with a successful canned request.
    pub fn success(request: HttpRequest) -> Self {
        Self {
            inner: Mutex::new(Some(Ok(request))),
        }
    }

    /// Create a new [`CannedRequestSerializer`] with a canned error.
    pub fn failure(error: BoxError) -> Self {
        Self {
            inner: Mutex::new(Some(Err(error))),
        }
    }

    fn take(&self) -> Option<Result<HttpRequest, BoxError>> {
        match self.inner.lock() {
            Ok(mut guard) => guard.take(),
            Err(_) => None,
        }
    }
}

impl SerializeRequest for CannedRequestSerializer {
    fn serialize_input(
        &self,
        _input: Input,
        _cfg: &mut ConfigBag,
    ) -> Result<HttpRequest, BoxError> {
        self.take()
            .ok_or("CannedRequestSerializer's inner value has already been taken.")?
    }
}

impl RuntimePlugin for CannedRequestSerializer {
    fn config(&self) -> Option<FrozenLayer> {
        let mut cfg = Layer::new("CannedRequest");
        cfg.store_put(SharedRequestSerializer::new(Self {
            inner: Mutex::new(self.take()),
        }));
        Some(cfg.freeze())
    }
}
