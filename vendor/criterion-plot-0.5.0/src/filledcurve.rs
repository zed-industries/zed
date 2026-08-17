//! Filled curve plots

use std::borrow::Cow;
use std::iter::IntoIterator;

use crate::data::Matrix;
use crate::traits::{self, Data, Set};
use crate::{Axes, Color, Default, Display, Figure, Label, Opacity, Plot, Script};

/// Properties common to filled curve plots
pub struct Properties {
    axes: Option<Axes>,
    color: Option<Color>,
    label: Option<Cow<'static, str>>,
    opacity: Option<f64>,
}

impl Default for Properties {
    fn default() -> Properties {
        Properties {
            axes: None,
            color: None,
            label: None,
            opacity: None,
        }
    }
}

impl Script for Properties {
    // Allow clippy::format_push_string even with older versions of rust (<1.62) which
    // don't have it defined.
    #[allow(clippy::all)]
    fn script(&self) -> String {
        let mut script = if let Some(axes) = self.axes {
            format!("axes {} ", axes.display())
        } else {
            String::new()
        };
        script.push_str("with filledcurves ");

        script.push_str("fillstyle ");

        if let Some(opacity) = self.opacity {
            script.push_str(&format!("solid {} ", opacity))
        }

        // TODO border shoulde be configurable
        script.push_str("noborder ");

        if let Some(color) = self.color {
            script.push_str(&format!("lc rgb '{}' ", color.display()));
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

impl Set<Axes> for Properties {
    /// Select axes to plot against
    ///
    /// **Note** By default, the `BottomXLeftY` axes are used
    fn set(&mut self, axes: Axes) -> &mut Properties {
        self.axes = Some(axes);
        self
    }
}

impl Set<Color> for Properties {
    /// Sets the fill color
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

impl Set<Opacity> for Properties {
    /// Changes the opacity of the fill color
    ///
    /// **Note** By default, the fill color is totally opaque (`opacity = 1.0`)
    ///
    /// # Panics
    ///
    /// Panics if `opacity` is outside the range `[0, 1]`
    fn set(&mut self, opacity: Opacity) -> &mut Properties {
        self.opacity = Some(opacity.0);
        self
    }
}

/// Fills the area between two curves
pub struct FilledCurve<X, Y1, Y2> {
    /// X coordinate of the data points of both curves
    pub x: X,
    /// Y coordinate of the data points of the first curve
    pub y1: Y1,
    /// Y coordinate of the data points of the second curve
    pub y2: Y2,
}

impl<X, Y1, Y2> traits::Plot<FilledCurve<X, Y1, Y2>> for Figure
where
    X: IntoIterator,
    X::Item: Data,
    Y1: IntoIterator,
    Y1::Item: Data,
    Y2: IntoIterator,
    Y2::Item: Data,
{
    type Properties = Properties;

    fn plot<F>(&mut self, fc: FilledCurve<X, Y1, Y2>, configure: F) -> &mut Figure
    where
        F: FnOnce(&mut Properties) -> &mut Properties,
    {
        let FilledCurve { x, y1, y2 } = fc;

        let mut props = Default::default();
        configure(&mut props);

        let (x_factor, y_factor) =
            crate::scale_factor(&self.axes, props.axes.unwrap_or(crate::Axes::BottomXLeftY));

        let data = Matrix::new(izip!(x, y1, y2), (x_factor, y_factor, y_factor));
        self.plots.push(Plot::new(data, &props));
        self
    }
}
