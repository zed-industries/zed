use gpui::{hsla, point, px, BoxShadow};
use smallvec::{smallvec, SmallVec};

#[doc = include_str!("docs/elevation.md")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Elevation {
    ElevationIndex(ElevationIndex),
    LayerIndex(LayerIndex),
    ElementIndex(ElementIndex),
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
    pub fn z_index(self) -> u16 {
        match self {
            ElevationIndex::Background => 0,
            ElevationIndex::Surface => 42,
            ElevationIndex::ElevatedSurface => 84,
            ElevationIndex::Wash => 126,
            ElevationIndex::ModalSurface => 168,
            ElevationIndex::DraggedElement => 210,
        }
    }

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

impl LayerIndex {
    pub fn usize(&self) -> usize {
        match *self {
            LayerIndex::BehindElement => 0,
            LayerIndex::Element => 100,
            LayerIndex::ElevatedElement => 200,
        }
    }
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

impl ElementIndex {
    pub fn usize(&self) -> usize {
        match *self {
            ElementIndex::Effect => 0,
            ElementIndex::Background => 100,
            ElementIndex::Tint => 200,
            ElementIndex::Highlight => 300,
            ElementIndex::Content => 400,
            ElementIndex::Overlay => 500,
        }
    }
}
