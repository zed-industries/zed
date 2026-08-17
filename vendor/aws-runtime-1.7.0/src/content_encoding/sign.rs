/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_sigv4::http_request::SigningError;
use aws_smithy_runtime_api::http::Headers;
use aws_smithy_types::config_bag::{Storable, StoreReplace};
use bytes::Bytes;
use std::sync::{mpsc, Arc, Mutex};

// Trait for signing chunks and trailers
//
// Trait methods take `&mut self`` because they keep track of running signature as they sign each chunk.
pub(crate) trait SignChunk: std::fmt::Debug {
    fn chunk_signature(&mut self, chunk: &Bytes) -> Result<String, SigningError>;

    fn trailer_signature(&mut self, trailing_headers: &Headers) -> Result<String, SigningError>;
}

/// Deferred chunk signer that allows a signer to be wired up later.
///
/// Signing chunks and trailers occurs after HTTP request signing and requires
/// signing context from the initial HTTP signature operation.
///
/// This signer establishes an MPSC channel with a sender placed in the request
/// configuration. The HTTP signer implementation retrieves the sender from the
/// config and sends an actual signing implementation with the required context.
#[derive(Clone, Debug)]
#[allow(clippy::type_complexity)]
pub struct DeferredSigner {
    // The outer `Arc` enables cloning `DeferredSigner`, making `AwsChunkedBody` retryable.
    // The inner trait objects are boxed to enable calling mutable trait methods.
    signer: Arc<Mutex<Option<Box<dyn SignChunk + Send + Sync>>>>,
    rx: Arc<Mutex<Option<mpsc::Receiver<Box<dyn SignChunk + Send + Sync>>>>>,
}

impl Storable for DeferredSigner {
    type Storer = StoreReplace<Self>;
}

impl DeferredSigner {
    /// Create a new `DeferredSigner` and its associated sender.
    pub fn new() -> (Self, DeferredSignerSender) {
        let (tx, rx) = mpsc::channel();
        (
            Self {
                signer: Default::default(),
                rx: Arc::new(Mutex::new(Some(rx))),
            },
            DeferredSignerSender { tx: Mutex::new(tx) },
        )
    }

    /// Create an empty `DeferredSigner`, typically used as a placeholder for `std::mem::replace`
    pub fn empty() -> Self {
        Self {
            rx: Default::default(),
            signer: Default::default(),
        }
    }

    fn acquire(&self) -> Box<dyn SignChunk + Send + Sync> {
        let mut rx = self.rx.lock().unwrap();
        rx.take()
            .and_then(|receiver| receiver.try_recv().ok())
            .expect("signer should be available")
    }
}

/// A sender placed in the config bag to wire up a signer for signing chunks and trailers.
#[derive(Debug)]
pub struct DeferredSignerSender {
    tx: Mutex<mpsc::Sender<Box<dyn SignChunk + Send + Sync>>>,
}

impl DeferredSignerSender {
    pub(crate) fn send(
        &self,
        signer: Box<dyn SignChunk + Send + Sync>,
    ) -> Result<(), mpsc::SendError<Box<dyn SignChunk + Send + Sync>>> {
        self.tx.lock().unwrap().send(signer)
    }
}

impl Storable for DeferredSignerSender {
    type Storer = StoreReplace<Self>;
}

impl SignChunk for DeferredSigner {
    fn chunk_signature(&mut self, chunk: &Bytes) -> Result<String, SigningError> {
        let mut signer = self.signer.lock().unwrap();
        let signer = signer.get_or_insert_with(|| self.acquire());
        signer.chunk_signature(chunk)
    }

    fn trailer_signature(&mut self, trailing_headers: &Headers) -> Result<String, SigningError> {
        let mut signer = self.signer.lock().unwrap();
        let signer = signer.get_or_insert_with(|| self.acquire());
        signer.trailer_signature(trailing_headers)
    }
}
