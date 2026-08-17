pub mod sink;
pub mod stream;
mod test_messages;
pub use test_messages::*;

use crate::Context;
use std::time::Duration;

pub const CHANNEL_TEST_ITERATIONS: usize = 2000;
pub const CHANNEL_TEST_SENDERS: usize = 10;
pub const CHANNEL_TEST_RECEIVERS: usize = 5;
pub const TEST_TIMEOUT: Duration = Duration::from_secs(100);

pub fn noop_context() -> crate::Context<'static> {
    Context::empty()
}

pub fn panic_context() -> crate::Context<'static> {
    futures_test::task::panic_context().into()
}
