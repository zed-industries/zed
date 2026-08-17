/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Provides Sender/Receiver implementations for Event Stream codegen.

use std::error::Error as StdError;

mod receiver;
mod sender;

/// A generic, boxed error that's `Send`, `Sync`, and `'static`.
pub type BoxError = Box<dyn StdError + Send + Sync + 'static>;

#[doc(inline)]
pub use sender::{EventStreamSender, MessageStreamAdapter, MessageStreamError};

#[doc(inline)]
pub use receiver::{InitialMessageType, Receiver, ReceiverError};
