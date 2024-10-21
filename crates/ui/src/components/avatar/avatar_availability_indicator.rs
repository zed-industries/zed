#![allow(missing_docs)]
use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Availability {
    Free,
    Busy,
}

#[derive(IntoElement)]
pub struct AvatarAvailabilityIndicator {
    availability: Availability,
    avatar_size: Option<Pixels>,
}

impl AvatarAvailabilityIndicator {
    pub fn new(availability: Availability) -> Self {
        Self {
            availability,
            avatar_size: None,
        }
    }

    /// Sets the size of the [`Avatar`](crate::Avatar) this indicator appears on.
    pub fn avatar_size(mut self, size: impl Into<Option<Pixels>>) -> Self {
        self.avatar_size = size.into();
        self
    }
}

impl RenderOnce for AvatarAvailabilityIndicator {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let avatar_size = self.avatar_size.unwrap_or_else(|| cx.rem_size());

        // HACK: non-integer sizes result in oval indicators.
        let indicator_size = (avatar_size * 0.4).round();

        div()
            .absolute()
            .bottom_0()
            .right_0()
            .size(indicator_size)
            .rounded(indicator_size)
            .bg(match self.availability {
                Availability::Free => cx.theme().status().created,
                Availability::Busy => cx.theme().status().deleted,
            })
    }
}
