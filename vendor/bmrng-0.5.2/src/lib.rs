#![forbid(unsafe_code)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(feature = "clippy", warn(cast_possible_truncation))]
#![cfg_attr(feature = "clippy", warn(cast_possible_wrap))]
#![cfg_attr(feature = "clippy", warn(cast_precision_loss))]
#![cfg_attr(feature = "clippy", warn(cast_sign_loss))]
#![cfg_attr(feature = "clippy", warn(missing_docs_in_private_items))]
#![cfg_attr(feature = "clippy", warn(mut_mut))]
#![cfg_attr(feature = "clippy", warn(print_stdout))]
#![cfg_attr(all(not(test), feature = "clippy"), warn(result_unwrap_used))]
#![cfg_attr(feature = "clippy", warn(unseparated_literal_suffix))]
#![cfg_attr(feature = "clippy", warn(wrong_pub_self_convention))]

//! An async MPSC request-response channel for Tokio, where you can send a response to the sender.
//!
//! See [`bmrng::channel()`](crate::channel()) for a channel with backpressure and
//! [`bmrng::unbounded::channel()`](crate::unbounded::channel()) for a channel without backpressure.

mod bounded;
pub use self::bounded::{
    channel, channel_with_timeout, Payload, RequestReceiver, RequestReceiverStream, RequestSender,
    Responder, ResponseReceiver,
};
/// The errors produced by this crate
pub mod error;
/// The unbounded channel alternative
pub mod unbounded;
pub use unbounded::channel as unbounded_channel;
pub use unbounded::channel_with_timeout as unbounded_channel_with_timeout;
