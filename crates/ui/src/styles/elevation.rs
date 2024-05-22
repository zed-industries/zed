use gpui::{hsla, point, px, BoxShadow};
use smallvec::{smallvec, SmallVec};

#[doc = include_str!("docs/elevation.md")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Elevation {
    ElevationIndex(ElevationIndex),
    LayerIndex(LayerIndex),
    ElementIndex(ElementIndex),
}

impl Into<Elevation> for ElevationIndex {
    fn into(self) -> Elevation {
        Elevation::ElevationIndex(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevationIndex {
    Background,
    Surface,
    ElevatedSurface,
    Wash,
    ModalSurface,
    DraggedElement,
}

impl ElevationIndex {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerIndex {
    BehindElement,
    Element,
    ElevatedElement,
}

/// An appropriate z-index for the given layer based on its intended usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementIndex {
    Effect,
    Background,
    Tint,
    Highlight,
    Content,
    Overlay,
}
