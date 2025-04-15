//! The GPUI prelude is a collection of traits and types that are widely used
//! throughout the library. It is recommended to import this prelude into your
//! application to avoid having to import each trait individually.

pub use crate::{
    AppContext as _, BorrowAppContext, Context, Element, FocusableElement, InteractiveElement,
    IntoElement, ParentElement, Refineable, Render, RenderOnce, StatefulInteractiveElement, Styled,
    StyledImage, VisualContext, util::FluentBuilder,
};
