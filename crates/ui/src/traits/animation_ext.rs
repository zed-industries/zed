use std::time::Duration;

use gpui::{Animation, AnimationElement, AnimationExt, Transformation, percentage};

use crate::{prelude::*, traits::transformable::Transformable};

/// An extension trait for adding common animations to animatable components.
pub trait CommonAnimationExt: AnimationExt {
    /// Render this component as rotating over the given duration.
    #[track_caller]
    fn with_rotate_animation(self, duration: u64) -> AnimationElement<Self>
    where
        Self: Transformable + Sized,
    {
        self.with_keyed_rotate_animation(
            ElementId::CodeLocation(*std::panic::Location::caller()),
            duration,
        )
    }

    /// Render this component as rotating with the given element ID over the given duration.
    fn with_keyed_rotate_animation(
        self,
        id: impl Into<ElementId>,
        duration: u64,
    ) -> AnimationElement<Self>
    where
        Self: Transformable + Sized,
    {
        self.with_animation(
            id,
            Animation::new(Duration::from_secs(duration)).repeat(),
            |component, delta| component.transform(Transformation::rotate(percentage(delta))),
        )
    }
}

impl<T: AnimationExt> CommonAnimationExt for T {}
