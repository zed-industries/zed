/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::client::interceptors::context::{Error, Output};
use aws_smithy_runtime_api::client::orchestrator::{HttpResponse, OrchestratorError};
use aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;
use aws_smithy_runtime_api::client::ser_de::{DeserializeResponse, SharedResponseDeserializer};
use aws_smithy_types::config_bag::{FrozenLayer, Layer};
use std::sync::Mutex;

/// Test response deserializer that always returns the same canned response.
#[derive(Default, Debug)]
pub struct CannedResponseDeserializer {
    inner: Mutex<Option<Result<Output, OrchestratorError<Error>>>>,
}

impl CannedResponseDeserializer {
    /// Creates a new `CannedResponseDeserializer` with the given canned response.
    pub fn new(output: Result<Output, OrchestratorError<Error>>) -> Self {
        Self {
            inner: Mutex::new(Some(output)),
        }
    }

    fn take(&self) -> Option<Result<Output, OrchestratorError<Error>>> {
        match self.inner.lock() {
            Ok(mut guard) => guard.take(),
            Err(_) => None,
        }
    }
}

impl DeserializeResponse for CannedResponseDeserializer {
    fn deserialize_nonstreaming(
        &self,
        _response: &HttpResponse,
    ) -> Result<Output, OrchestratorError<Error>> {
        self.take()
            .ok_or("CannedResponseDeserializer's inner value has already been taken.")
            .unwrap()
    }
}

impl RuntimePlugin for CannedResponseDeserializer {
    fn config(&self) -> Option<FrozenLayer> {
        let mut cfg = Layer::new("CannedResponse");
        cfg.store_put(SharedResponseDeserializer::new(Self {
            inner: Mutex::new(self.take()),
        }));
        Some(cfg.freeze())
    }
}
