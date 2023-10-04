pub use gpui3::{
    div, Element, IntoAnyElement, ParentElement, ScrollState, StyleHelpers, ViewContext,
};

pub use crate::ui::{HackyChildren, HackyChildrenPayload};

use strum::EnumIter;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum Shape {
    #[default]
    Circle,
    RoundedRectangle,
}
