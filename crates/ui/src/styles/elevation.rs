use std::fmt::{self, Display, Formatter};

use gpui::{App, BoxShadow, Hsla, hsla, point, px};
use smallvec::{SmallVec, smallvec};
use theme::{ActiveTheme, Appearance};

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
    /// A surface above the [ElevationIndex::ElevatedSurface] that is used for dialogs, alerts, modals, etc.
    ModalSurface,
}

impl Display for ElevationIndex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ElevationIndex::Background => write!(f, "Background"),
            ElevationIndex::Surface => write!(f, "Surface"),
            ElevationIndex::EditorSurface => write!(f, "Editor Surface"),
            ElevationIndex::ElevatedSurface => write!(f, "Elevated Surface"),
            ElevationIndex::ModalSurface => write!(f, "Modal Surface"),
        }
    }
}

impl ElevationIndex {
    /// Returns an appropriate shadow for the given elevation index.
    pub fn shadow(self, cx: &App) -> SmallVec<[BoxShadow; 2]> {
        let is_light = cx.theme().appearance() == Appearance::Light;

        match self {
            ElevationIndex::Surface => smallvec![],
            ElevationIndex::EditorSurface => smallvec![],

            ElevationIndex::ElevatedSurface => smallvec![
                BoxShadow {
                    color: hsla(0., 0., 0., 0.12),
                    offset: point(px(0.), px(2.)),
                    blur_radius: px(3.),
                    spread_radius: px(0.),
                },
                BoxShadow {
                    color: hsla(0., 0., 0., if is_light { 0.03 } else { 0.06 }),
                    offset: point(px(1.), px(1.)),
                    blur_radius: px(0.),
                    spread_radius: px(0.),
                }
            ],

            ElevationIndex::ModalSurface => smallvec![
                BoxShadow {
                    color: hsla(0., 0., 0., if is_light { 0.06 } else { 0.12 }),
                    offset: point(px(0.), px(2.)),
                    blur_radius: px(3.),
                    spread_radius: px(0.),
                },
                BoxShadow {
                    color: hsla(0., 0., 0., if is_light { 0.06 } else { 0.08 }),
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
                BoxShadow {
                    color: hsla(0., 0., 0., if is_light { 0.04 } else { 0.12 }),
                    offset: point(px(1.), px(1.)),
                    blur_radius: px(0.),
                    spread_radius: px(0.),
                },
            ],

            _ => smallvec![],
        }
    }

    /// Returns the background color for the given elevation index.
    pub fn bg(&self, cx: &mut App) -> Hsla {
        match self {
            ElevationIndex::Background => cx.theme().colors().background,
            ElevationIndex::Surface => cx.theme().colors().surface_background,
            ElevationIndex::EditorSurface => cx.theme().colors().editor_background,
            ElevationIndex::ElevatedSurface => cx.theme().colors().elevated_surface_background,
            ElevationIndex::ModalSurface => cx.theme().colors().elevated_surface_background,
        }
    }

    /// Returns a color that is appropriate a filled element on this elevation
    pub fn on_elevation_bg(&self, cx: &App) -> Hsla {
        match self {
            ElevationIndex::Background => cx.theme().colors().surface_background,
            ElevationIndex::Surface => cx.theme().colors().background,
            ElevationIndex::EditorSurface => cx.theme().colors().surface_background,
            ElevationIndex::ElevatedSurface => cx.theme().colors().background,
            ElevationIndex::ModalSurface => cx.theme().colors().background,
        }
    }

    /// Attempts to return a darker background color than the current elevation index's background.
    ///
    /// If the current background color is already dark, it will return a lighter color instead.
    pub fn darker_bg(&self, cx: &App) -> Hsla {
        match self {
            ElevationIndex::Background => cx.theme().colors().surface_background,
            ElevationIndex::Surface => cx.theme().colors().editor_background,
            ElevationIndex::EditorSurface => cx.theme().colors().surface_background,
            ElevationIndex::ElevatedSurface => cx.theme().colors().editor_background,
            ElevationIndex::ModalSurface => cx.theme().colors().editor_background,
        }
    }
}
