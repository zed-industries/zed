use gpui::{hsla, point, px, BoxShadow};
use smallvec::{smallvec, SmallVec};

/// Today, elevation is primarily used to add shadows to elements, and set the correct background for elements like buttons.
///
/// Elevation can be thought of as the physical closeness of an element to the
/// user. Elements with lower elevations are physically further away on the
/// z-axis and appear to be underneath elements with higher elevations.
///
/// In the future, a more complete approach to elevation may be added.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevationIndex {
    /// On the layer of the app background. This is under panels, panes, and
    /// other surfaces.
    Background,
    /// The primary surface – Contains panels, panes, containers, etc.
    Surface,
    /// A surface that is elevated above the primary surface. but below washes, models, and dragged elements.
    ElevatedSurface,
    /// A surface that is above all non-modal surfaces, and separates the app from focused intents, like dialogs, alerts, modals, etc.
    Wash,
    /// A surface above the [ElevationIndex::Wash] that is used for dialogs, alerts, modals, etc.
    ModalSurface,
    /// A surface above all other surfaces, reserved exclusively for dragged elements, like a dragged file, tab or other draggable element.
    DraggedElement,
}

impl ElevationIndex {
    /// Returns an appropriate shadow for the given elevation index.
    pub fn shadow(self) -> SmallVec<[BoxShadow; 2]> {
        match self {
            ElevationIndex::Surface => smallvec![],

            ElevationIndex::ElevatedSurface => smallvec![BoxShadow {
                color: hsla(0., 0., 0., 0.12),
                offset: point(px(0.), px(2.)),
                blur_radius: px(3.),
                spread_radius: px(0.),
            }],

            ElevationIndex::ModalSurface => smallvec![
                BoxShadow {
                    color: hsla(0., 0., 0., 0.12),
                    offset: point(px(0.), px(2.)),
                    blur_radius: px(3.),
                    spread_radius: px(0.),
                },
                BoxShadow {
                    color: hsla(0., 0., 0., 0.08),
                    offset: point(px(0.), px(3.)),
                    blur_radius: px(6.),
                    spread_radius: px(0.),
                },
                BoxShadow {
                    color: hsla(0., 0., 0., 0.04),
                    offset: point(px(0.), px(6.)),
                    blur_radius: px(12.),
                    spread_radius: px(0.),
                },
            ],

            _ => smallvec![],
        }
    }
}
