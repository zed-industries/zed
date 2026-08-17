//! The rayon prelude imports the various `ParallelIterator` traits.
//! The intention is that one can include `use rayon::prelude::*` and
//! have easy access to the various traits and methods you will need.

pub use crate::iter::FromParallelIterator;
pub use crate::iter::IndexedParallelIterator;
pub use crate::iter::IntoParallelIterator;
pub use crate::iter::IntoParallelRefIterator;
pub use crate::iter::IntoParallelRefMutIterator;
pub use crate::iter::ParallelBridge;
pub use crate::iter::ParallelDrainFull;
pub use crate::iter::ParallelDrainRange;
pub use crate::iter::ParallelExtend;
pub use crate::iter::ParallelIterator;
pub use crate::slice::ParallelSlice;
pub use crate::slice::ParallelSliceMut;
pub use crate::str::ParallelString;
