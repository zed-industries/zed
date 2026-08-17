#![allow(dead_code)]

//! Utilities to implement the different futures of this crate.

mod array;
#[cfg(feature = "alloc")]
mod chunked_vec;
/// Indexer used for dynamic number of futures, like with `Vec`.
#[cfg(feature = "alloc")]
mod dyn_indexer;
mod futures;
/// Indexer used for static number of futures, like with arrays and tuples.
mod indexer;
mod output;
mod pin;
mod poll_state;
mod stream;
mod tuple;
mod wakers;

#[doc(hidden)]
pub mod private;

pub(crate) use self::futures::FutureArray;
#[cfg(feature = "alloc")]
pub(crate) use self::futures::FutureVec;
pub(crate) use array::array_assume_init;
#[cfg(feature = "alloc")]
pub(crate) use chunked_vec::ChunkedVec;
#[cfg(feature = "alloc")]
pub(crate) use dyn_indexer::DynIndexer;
pub(crate) use indexer::Indexer;
pub(crate) use output::OutputArray;
#[cfg(feature = "alloc")]
pub(crate) use output::OutputVec;
pub(crate) use pin::{get_pin_mut, iter_pin_mut};
#[cfg(feature = "alloc")]
pub(crate) use pin::{get_pin_mut_from_vec, iter_pin_mut_vec};
pub(crate) use poll_state::PollArray;
#[cfg(feature = "alloc")]
pub(crate) use poll_state::{MaybeDone, PollState, PollVec};
pub(crate) use tuple::{gen_conditions, tuple_len};
pub(crate) use wakers::WakerArray;
#[cfg(feature = "alloc")]
pub(crate) use wakers::WakerVec;

#[cfg(all(test, feature = "alloc"))]
pub(crate) use wakers::DummyWaker;

#[cfg(all(test, feature = "alloc"))]
pub(crate) mod channel;

#[cfg(feature = "alloc")]
pub(crate) use stream::{from_iter, FromIter};
