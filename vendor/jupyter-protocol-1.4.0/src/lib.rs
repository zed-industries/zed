//! # Jupyter Protocol
//!
//! This crate provides a complete implementation of the Jupyter messaging protocol,
//! as specified in the [Jupyter Client documentation](https://jupyter-client.readthedocs.io/en/latest/messaging.html).
//!
//! It includes types and structures for all Jupyter message types, as well as
//! utilities for working with Jupyter kernels and clients.
//!
//! ## Main Components
//!
//! - [`JupyterMessage`]: The main message type, encompassing all Jupyter protocol messages.
//! - [`JupyterMessageContent`]: An enum representing the various content types for Jupyter messages.
//! - [`Media`]: Represents rich media content (MIME bundles) in Jupyter messages.
//! - [`ConnectionInfo`]: Contains information needed to connect to a Jupyter kernel.
//!
//! ## Usage
//!
//! Here's a basic example of creating and working with Jupyter messages:
//!
//! ```rust
//! use jupyter_protocol::{JupyterMessage, ExecuteRequest, JupyterMessageContent};
//!
//! // Create an execute request
//! let execute_request = ExecuteRequest::new("print('Hello, world!')".to_string());
//!
//! // Convert it to a JupyterMessage
//! let message: JupyterMessage = execute_request.into();
//!
//! // You can then send this message using your preferred transport layer
//!
//! // When receiving messages, you can match on the content type:
//! match message.content {
//!     JupyterMessageContent::ExecuteRequest(req) => {
//!         println!("Received execute request with code: {}", req.code);
//!     },
//!     _ => println!("Received other message type"),
//! }
//! ```
//!
//! For more detailed examples and usage information, see the documentation for
//! individual modules and types.
pub mod messaging;
pub use messaging::*;

pub mod connection_info;
pub use connection_info::{ConnectionInfo, Transport};

mod error;
pub use error::JupyterError;

mod time;

mod execution_count;
pub use execution_count::*;

mod kernelspec;
pub use kernelspec::*;

pub mod media;
pub use media::*;

use async_trait::async_trait;
use futures::{Sink, Stream};

#[async_trait]
pub trait JupyterConnection:
    Sink<JupyterMessage> + Stream<Item = core::result::Result<JupyterMessage, Self::Error>>
{
}
