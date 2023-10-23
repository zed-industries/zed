#[doc = include_str!("elevation.md")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Elevation {
    ElevationIndex(ElevationIndex),
    LayerIndex(LayerIndex),
    ElementIndex(ElementIndex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevationIndex {
    AppBackground,
    UISurface,
    ElevatedSurface,
    Wash,
    ModalSurfaces,
    DraggedElement,
}

impl ElevationIndex {
    pub fn usize(&self) -> usize {
        match *self {
            ElevationIndex::AppBackground => 0,
            ElevationIndex::UISurface => 100,
            ElevationIndex::ElevatedSurface => 200,
            ElevationIndex::Wash => 300,
            ElevationIndex::ModalSurfaces => 400,
            ElevationIndex::DraggedElement => 900,
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
