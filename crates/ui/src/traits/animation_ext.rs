use std::time::Duration;

use crate::traits::transformable::Transformable;

/// An extension trait for adding common animations to animatable components.
pub trait CommonAnimationExt: Transformable + Sized {
    /// Render this component as rotating, one full turn every `period` seconds.
    ///
    /// The view isn't re-rendered to advance the rotation. All spinners share one phase.
    fn with_rotate_animation(self, period: u64) -> Self {
        self.animate_rotation(Duration::from_secs(period))
    }
}

impl<T: Transformable> CommonAnimationExt for T {}
