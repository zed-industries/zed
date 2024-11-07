use std::fmt::{self, Display, Formatter};

use gpui::{hsla, point, px, BoxShadow, Hsla, WindowContext};
use smallvec::{smallvec, SmallVec};
use theme::ActiveTheme;

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
    /// The same elevation as the primary surface, but used for the editable areas, like buffers
    EditorSurface,
    /// A surface that is elevated above the primary surface. but below washes, models, and dragged elements.
    ElevatedSurface,
    /// A surface that is above all non-modal surfaces, and separates the app from focused intents, like dialogs, alerts, modals, etc.
    Wash,
    /// A surface above the [ElevationIndex::Wash] that is used for dialogs, alerts, modals, etc.
    ModalSurface,
    /// A surface above all other surfaces, reserved exclusively for dragged elements, like a dragged file, tab or other draggable element.
    DraggedElement,
}

impl Display for ElevationIndex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ElevationIndex::Background => write!(f, "Background"),
            ElevationIndex::Surface => write!(f, "Surface"),
            ElevationIndex::EditorSurface => write!(f, "Editor Surface"),
            ElevationIndex::ElevatedSurface => write!(f, "Elevated Surface"),
            ElevationIndex::Wash => write!(f, "Wash"),
            ElevationIndex::ModalSurface => write!(f, "Modal Surface"),
            ElevationIndex::DraggedElement => write!(f, "Dragged Element"),
        }
    }
}

impl ElevationIndex {
    /// Returns an appropriate shadow for the given elevation index.
    pub fn shadow(self) -> SmallVec<[BoxShadow; 2]> {
        match self {
            ElevationIndex::Surface => smallvec![],
            ElevationIndex::EditorSurface => smallvec![],

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

    /// Returns the background color for the given elevation index.
    pub fn bg(&self, cx: &WindowContext) -> Hsla {
        match self {
            ElevationIndex::Background => cx.theme().colors().background,
            ElevationIndex::Surface => cx.theme().colors().surface_background,
            ElevationIndex::EditorSurface => cx.theme().colors().editor_background,
            ElevationIndex::ElevatedSurface => cx.theme().colors().elevated_surface_background,
            ElevationIndex::Wash => gpui::transparent_black(),
            ElevationIndex::ModalSurface => cx.theme().colors().elevated_surface_background,
            ElevationIndex::DraggedElement => gpui::transparent_black(),
        }
    }
}
