//! Error bar plots

use std::borrow::Cow;
use std::iter::IntoIterator;

use crate::data::Matrix;
use crate::traits::{self, Data, Set};
use crate::{
    Color, Display, ErrorBarDefault, Figure, Label, LineType, LineWidth, Plot, PointSize,
    PointType, Script,
};

/// Properties common to error bar plots
pub struct Properties {
    color: Option<Color>,
    label: Option<Cow<'static, str>>,
    line_type: LineType,
    linewidth: Option<f64>,
    point_size: Option<f64>,
    point_type: Option<PointType>,
    style: Style,
}

impl ErrorBarDefault<Style> for Properties {
    fn default(style: Style) -> Properties {
        Properties {
            color: None,
            label: None,
            line_type: LineType::Solid,
            linewidth: None,
            point_type: None,
            point_size: None,
            style,
        }
    }
}

impl Script for Properties {
    // Allow clippy::format_push_string even with older versions of rust (<1.62) which
    // don't have it defined.
    #[allow(clippy::all)]
    fn script(&self) -> String {
        let mut script = format!("with {} ", self.style.display());

        script.push_str(&format!("lt {} ", self.line_type.display()));

        if let Some(lw) = self.linewidth {
            script.push_str(&format!("lw {} ", lw))
        }

        if let Some(color) = self.color {
            script.push_str(&format!("lc rgb '{}' ", color.display()))
        }

        if let Some(pt) = self.point_type {
            script.push_str(&format!("pt {} ", pt.display()))
        }

        if let Some(ps) = self.point_size {
            script.push_str(&format!("ps {} ", ps))
        }

        if let Some(ref label) = self.label {
            script.push_str("title '");
            script.push_str(label);
            script.push('\'')
        } else {
            script.push_str("notitle")
        }

        script
    }
}

impl Set<Color> for Properties {
    /// Changes the color of the error bars
    fn set(&mut self, color: Color) -> &mut Properties {
        self.color = Some(color);
        self
    }
}

impl Set<Label> for Properties {
    /// Sets the legend label
    fn set(&mut self, label: Label) -> &mut Properties {
        self.label = Some(label.0);
        self
    }
}

impl Set<LineType> for Properties {
    /// Change the line type
    ///
    /// **Note** By default `Solid` lines are used
    fn set(&mut self, lt: LineType) -> &mut Properties {
        self.line_type = lt;
        self
    }
}

impl Set<LineWidth> for Properties {
    /// Changes the linewidth
    ///
    /// # Panics
    ///
    /// Panics if `lw` is a non-positive value
    fn set(&mut self, lw: LineWidth) -> &mut Properties {
        let lw = lw.0;

        assert!(lw > 0.);

        self.linewidth = Some(lw);
        self
    }
}

impl Set<PointSize> for Properties {
    /// Changes the size of the points
    ///
    /// # Panics
    ///
    /// Panics if `size` is a non-positive value
    fn set(&mut self, ps: PointSize) -> &mut Properties {
        let ps = ps.0;

        assert!(ps > 0.);

        self.point_size = Some(ps);
        self
    }
}

impl Set<PointType> for Properties {
    /// Changes the point type
    fn set(&mut self, pt: PointType) -> &mut Properties {
        self.point_type = Some(pt);
        self
    }
}

#[derive(Clone, Copy)]
enum Style {
    XErrorBars,
    XErrorLines,
    YErrorBars,
    YErrorLines,
}

impl Display<&'static str> for Style {
    fn display(&self) -> &'static str {
        match *self {
            Style::XErrorBars => "xerrorbars",
            Style::XErrorLines => "xerrorlines",
            Style::YErrorBars => "yerrorbars",
            Style::YErrorLines => "yerrorlines",
        }
    }
}

/// Asymmetric error bar plots
pub enum ErrorBar<X, Y, L, H> {
    /// Horizontal error bars
    XErrorBars {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
        /// X coordinate of the left end of the error bar
        x_low: L,
        /// Y coordinate of the right end of the error bar
        x_high: H,
    },
    /// Horizontal error bars, where each point is joined by a line
    XErrorLines {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
        /// X coordinate of the left end of the error bar
        x_low: L,
        /// Y coordinate of the right end of the error bar
        x_high: H,
    },
    /// Vertical error bars
    YErrorBars {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
        /// Y coordinate of the bottom of the error bar
        y_low: L,
        /// Y coordinate of the top of the error bar
        y_high: H,
    },
    /// Vertical error bars, where each point is joined by a line
    YErrorLines {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
        /// Y coordinate of the bottom of the error bar
        y_low: L,
        /// Y coordinate of the top of the error bar
        y_high: H,
    },
}

impl<X, Y, L, H> ErrorBar<X, Y, L, H> {
    fn style(&self) -> Style {
        match *self {
            ErrorBar::XErrorBars { .. } => Style::XErrorBars,
            ErrorBar::XErrorLines { .. } => Style::XErrorLines,
            ErrorBar::YErrorBars { .. } => Style::YErrorBars,
            ErrorBar::YErrorLines { .. } => Style::YErrorLines,
        }
    }
}

impl<X, Y, L, H> traits::Plot<ErrorBar<X, Y, L, H>> for Figure
where
    H: IntoIterator,
    H::Item: Data,
    L: IntoIterator,
    L::Item: Data,
    X: IntoIterator,
    X::Item: Data,
    Y: IntoIterator,
    Y::Item: Data,
{
    type Properties = Properties;

    fn plot<F>(&mut self, e: ErrorBar<X, Y, L, H>, configure: F) -> &mut Figure
    where
        F: FnOnce(&mut Properties) -> &mut Properties,
    {
        let (x_factor, y_factor) = crate::scale_factor(&self.axes, crate::Axes::BottomXLeftY);

        let style = e.style();
        let (x, y, length, height, e_factor) = match e {
            ErrorBar::XErrorBars {
                x,
                y,
                x_low,
                x_high,
            }
            | ErrorBar::XErrorLines {
                x,
                y,
                x_low,
                x_high,
            } => (x, y, x_low, x_high, x_factor),
            ErrorBar::YErrorBars {
                x,
                y,
                y_low,
                y_high,
            }
            | ErrorBar::YErrorLines {
                x,
                y,
                y_low,
                y_high,
            } => (x, y, y_low, y_high, y_factor),
        };
        let data = Matrix::new(
            izip!(x, y, length, height),
            (x_factor, y_factor, e_factor, e_factor),
        );
        self.plots.push(Plot::new(
            data,
            configure(&mut ErrorBarDefault::default(style)),
        ));
        self
    }
}

// TODO XY error bar
// pub struct XyErrorBar<X, Y, XL, XH, YL, YH> {
// x: X,
// y: Y,
// x_low: XL,
// x_high: XH,
// y_low: YL,
// y_high: YH,
// }

// TODO Symmetric error bars
// pub enum SymmetricErrorBar {
// XSymmetricErrorBar { x: X, y: Y, x_delta: D },
// XSymmetricErrorLines { x: X, y: Y, x_delta: D },
// YSymmetricErrorBar { x: X, y: Y, y_delta: D },
// YSymmetricErrorLines { x: X, y: Y, y_delta: D },
// }
