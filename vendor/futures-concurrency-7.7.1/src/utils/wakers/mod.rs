mod array;
#[cfg(all(test, feature = "alloc"))]
mod dummy;
#[cfg(feature = "alloc")]
mod vec;

#[cfg(all(test, feature = "alloc"))]
pub(crate) use dummy::DummyWaker;

pub(crate) use array::*;
#[cfg(feature = "alloc")]
pub(crate) use vec::*;
