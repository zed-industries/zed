//! A collection of the most used traits, structs and enums

pub use crate::candlestick::Candlesticks;
pub use crate::curve::Curve::{Dots, Impulses, Lines, LinesPoints, Points, Steps};
pub use crate::errorbar::ErrorBar::{XErrorBars, XErrorLines, YErrorBars, YErrorLines};
pub use crate::filledcurve::FilledCurve;
pub use crate::key::{Boxed, Horizontal, Justification, Order, Position, Stacked, Vertical};
pub use crate::proxy::{Font, Label, Output, Title};
pub use crate::traits::{Configure, Plot, Set};
pub use crate::{
    Axes, Axis, BoxWidth, Color, Figure, FontSize, Grid, Key, LineType, LineWidth, Opacity,
    PointSize, PointType, Range, Scale, ScaleFactor, Size, Terminal, TicLabels,
};
