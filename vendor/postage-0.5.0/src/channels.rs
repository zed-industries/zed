pub mod barrier;
pub mod broadcast;
pub mod dispatch;
pub mod mpsc;
pub mod oneshot;
pub mod watch;

use std::{cell::Cell, marker::Sync};

use static_assertions::{assert_impl_all, assert_not_impl_all};

// Testing types for static assertions on channel endpoints
// Some channel implementations have unsafe Sync impls,
//   even if their generic type does not impl Sync (or &T impl Send)
// In order to pin down this behavior, these testing messages
//   are used as generics in static assertions.
#[allow(dead_code)]
struct SendMessage {
    cell: Cell<u8>,
}

assert_impl_all!(SendMessage: Send);
assert_not_impl_all!(SendMessage: Sync);

#[allow(dead_code)]
struct SendSyncMessage;
assert_impl_all!(SendSyncMessage: Send, Sync);
