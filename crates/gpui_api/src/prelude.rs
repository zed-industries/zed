//! The GPUI prelude is a collection of traits and types that are widely used
//! throughout the library. It is recommended to import this prelude into your
//! application to avoid having to import each trait individually.

pub use crate::{
    AppContext as _, BorrowAppContext, BoundsExt, Context, Element, FluentBuilder, ImageExt,
    InteractiveElement, IntoElement, LineLayoutExt, ParentElement, PlatformInputHandlerExt,
    Refineable, Render, RenderOnce, StatefulInteractiveElement, Styled, StyledImage, TaskExt as _,
    VisualContext, WindowBoundsExt,
};
