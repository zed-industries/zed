mod array;
#[cfg(feature = "alloc")]
mod vec;

pub(crate) use array::OutputArray;
#[cfg(feature = "alloc")]
pub(crate) use vec::OutputVec;
