//! Float trait

use cast::From;
use num_traits::float;

/// This is an extension of `num_traits::float::Float` that adds safe
/// casting and Sync + Send. Once `num_traits` has these features this
/// can be removed.
pub trait Float:
    float::Float + From<usize, Output = Self> + From<f32, Output = Self> + Sync + Send
{
}

impl Float for f32 {}
impl Float for f64 {}
