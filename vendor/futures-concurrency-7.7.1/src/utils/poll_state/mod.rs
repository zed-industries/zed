#![allow(clippy::module_inception)]

mod array;
#[cfg(feature = "alloc")]
mod maybe_done;
mod poll_state;
#[cfg(feature = "alloc")]
mod vec;

pub(crate) use array::PollArray;
#[cfg(feature = "alloc")]
pub(crate) use maybe_done::MaybeDone;
pub(crate) use poll_state::PollState;
#[cfg(feature = "alloc")]
pub(crate) use vec::PollVec;
