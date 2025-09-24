//! # Convex Client
//! The official Rust client for [Convex](https://convex.dev).
//!
//! Convex is the backend application platform with everything you need to build
//! your product. Convex clients can subscribe to queries and perform mutations
//! and actions. Check out the [Convex Documentation](https://docs.convex.dev) for more information.
//!
//! # Usage
//! ## Native Rust development
//! To use Convex to create native Rust applications with [`tokio`], you can use
//! the [`ConvexClient`] struct directly. All you need is your deployment URL
//! from your existing project, and you can subscribe to queries and call
//! mutations. To make a new project, check out our [getting started guide](https://docs.convex.dev/get-started).
//!
//! ```no_run
//! use convex::ConvexClient;
//! use futures::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut client = ConvexClient::new("https://cool-music-123.convex.cloud").await?;
//!     client.mutation("sendMessage", maplit::btreemap!{
//!         "body".into() => "Let it be.".into(),
//!         "author".into() => "The Beatles".into(),
//!     }).await?;
//!     let mut sub = client.subscribe("listMessages", maplit::btreemap!{}).await?;
//!     while let Some(result) = sub.next().await {
//!         println!("{result:?}");
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Extending client for other programming languages or frameworks.
//! To extend Convex into non-[`tokio`] frameworks,
//! you can use the [`base_client::BaseConvexClient`] to build something similar
//! to a [`ConvexClient`].
//!
//! Detailed examples of both use cases are documented for each struct.

#![cfg_attr(not(test), warn(missing_docs))]
#![warn(rustdoc::missing_crate_level_docs)]

mod value;
#[cfg(any(test, feature = "testing"))]
pub use value::export::roundtrip::ExportContext;
pub use value::{
    ConvexError,
    Value,
};

mod client;
pub use client::{
    subscription::{
        QuerySetSubscription,
        QuerySubscription,
    },
    ConvexClient,
    ConvexClientBuilder,
};
pub use sync::WebSocketState;

pub mod base_client;
#[doc(inline)]
pub use base_client::{
    FunctionResult,
    QueryResults,
    SubscriberId,
};

mod sync;
