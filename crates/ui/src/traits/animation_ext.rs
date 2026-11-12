use std::time::Duration;

use gpui::AnimationExt;

use crate::traits::transformable::Transformable;

/// An extension trait for adding common animations to animatable components.
pub trait CommonAnimationExt: AnimationExt {
    /// Render this component as rotating, one full turn every `period` seconds.
    ///
    /// The view isn't re-rendered to advance the rotation. All spinners share one phase.
    fn with_rotate_animation(self, period: u64) -> Self
    where
        Self: Transformable + Sized,
    {
        self.animate_rotation(Duration::from_secs(period))
    }
}

impl<T: AnimationExt> CommonAnimationExt for T {}
