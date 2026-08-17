mod array;
#[cfg(feature = "alloc")]
mod vec;

pub(crate) use array::FutureArray;
#[cfg(feature = "alloc")]
pub(crate) use vec::FutureVec;
