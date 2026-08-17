//! This module corresponds to `mach/sync_policy.h`

pub type sync_policy_t = libc::c_int;

pub const SYNC_POLICY_FIFO: sync_policy_t = 0x0;
pub const SYNC_POLICY_FIXED_PRIORITY: sync_policy_t = 0x1;
pub const SYNC_POLICY_REVERSED: sync_policy_t = 0x2;
pub const SYNC_POLICY_ORDER_MASK: sync_policy_t = 0x3;
pub const SYNC_POLICY_LIFO: sync_policy_t = SYNC_POLICY_FIFO | SYNC_POLICY_REVERSED;
