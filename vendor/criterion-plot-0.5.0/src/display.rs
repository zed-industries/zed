use std::borrow::Cow;

use crate::key::{Horizontal, Justification, Order, Stacked, Vertical};
use crate::{Axes, Axis, Color, Display, Grid, LineType, PointType, Terminal};

impl Display<&'static str> for Axis {
    fn display(&self) -> &'static str {
        match *self {
            Axis::BottomX => "x",
            Axis::LeftY => "y",
            Axis::RightY => "y2",
            Axis::TopX => "x2",
        }
    }
}

impl Display<&'static str> for Axes {
    fn display(&self) -> &'static str {
        match *self {
            Axes::BottomXLeftY => "x1y1",
            Axes::BottomXRightY => "x1y2",
            Axes::TopXLeftY => "x2y1",
            Axes::TopXRightY => "x2y2",
        }
    }
}

impl Display<Cow<'static, str>> for Color {
    fn display(&self) -> Cow<'static, str> {
        match *self {
            Color::Black => Cow::from("black"),
            Color::Blue => Cow::from("blue"),
            Color::Cyan => Cow::from("cyan"),
            Color::DarkViolet => Cow::from("dark-violet"),
            Color::ForestGreen => Cow::from("forest-green"),
            Color::Gold => Cow::from("gold"),
            Color::Gray => Cow::from("gray"),
            Color::Green => Cow::from("green"),
            Color::Magenta => Cow::from("magenta"),
            Color::Red => Cow::from("red"),
            Color::Rgb(r, g, b) => Cow::from(format!("#{:02x}{:02x}{:02x}", r, g, b)),
            Color::White => Cow::from("white"),
            Color::Yellow => Cow::from("yellow"),
        }
    }
}

impl Display<&'static str> for Grid {
    fn display(&self) -> &'static str {
        match *self {
            Grid::Major => "",
            Grid::Minor => "m",
        }
    }
}

impl Display<&'static str> for Horizontal {
    fn display(&self) -> &'static str {
        match *self {
            Horizontal::Center => "center",
            Horizontal::Left => "left",
            Horizontal::Right => "right",
        }
    }
}

impl Display<&'static str> for Justification {
    fn display(&self) -> &'static str {
        match *self {
            Justification::Left => "Left",
            Justification::Right => "Right",
        }
    }
}

impl Display<&'static str> for LineType {
    fn display(&self) -> &'static str {
        match *self {
            LineType::Dash => "2",
            LineType::Dot => "3",
            LineType::DotDash => "4",
            LineType::DotDotDash => "5",
            LineType::SmallDot => "0",
            LineType::Solid => "1",
        }
    }
}

impl Display<&'static str> for Order {
    fn display(&self) -> &'static str {
        match *self {
            Order::TextSample => "noreverse",
            Order::SampleText => "reverse",
        }
    }
}

impl Display<&'static str> for PointType {
    fn display(&self) -> &'static str {
        match *self {
            PointType::Circle => "6",
            PointType::FilledCircle => "7",
            PointType::FilledSquare => "5",
            PointType::FilledTriangle => "9",
            PointType::Plus => "1",
            PointType::Square => "4",
            PointType::Star => "3",
            PointType::Triangle => "8",
            PointType::X => "2",
        }
    }
}

impl Display<&'static str> for Stacked {
    fn display(&self) -> &'static str {
        match *self {
            Stacked::Horizontally => "horizontal",
            Stacked::Vertically => "vertical",
        }
    }
}

impl Display<&'static str> for Terminal {
    fn display(&self) -> &'static str {
        match *self {
            Terminal::Svg => "svg dynamic",
        }
    }
}

impl Display<&'static str> for Vertical {
    fn display(&self) -> &'static str {
        match *self {
            Vertical::Bottom => "bottom",
            Vertical::Center => "center",
            Vertical::Top => "top",
        }
    }
}
