//! Coordinate axis

use std::borrow::Cow;
use std::iter::IntoIterator;

use crate::map;
use crate::traits::{Configure, Data, Set};
use crate::{
    grid, Axis, Default, Display, Grid, Label, Range, Scale, ScaleFactor, Script, TicLabels,
};

/// Properties of the coordinate axes
#[derive(Clone)]
pub struct Properties {
    grids: map::grid::Map<grid::Properties>,
    hidden: bool,
    label: Option<Cow<'static, str>>,
    logarithmic: bool,
    range: Option<(f64, f64)>,
    scale_factor: f64,
    tics: Option<String>,
}

impl Default for Properties {
    fn default() -> Properties {
        Properties {
            grids: map::grid::Map::new(),
            hidden: false,
            label: None,
            logarithmic: false,
            range: None,
            scale_factor: 1.,
            tics: None,
        }
    }
}

impl Properties {
    /// Hides the axis
    ///
    /// **Note** The `TopX` and `RightY` axes are hidden by default
    pub fn hide(&mut self) -> &mut Properties {
        self.hidden = true;
        self
    }

    /// Makes the axis visible
    ///
    /// **Note** The `BottomX` and `LeftY` axes are visible by default
    pub fn show(&mut self) -> &mut Properties {
        self.hidden = false;
        self
    }
}

impl Configure<Grid> for Properties {
    type Properties = grid::Properties;

    /// Configures the gridlines
    fn configure<F>(&mut self, grid: Grid, configure: F) -> &mut Properties
    where
        F: FnOnce(&mut grid::Properties) -> &mut grid::Properties,
    {
        if self.grids.contains_key(grid) {
            configure(self.grids.get_mut(grid).unwrap());
        } else {
            let mut properties = Default::default();
            configure(&mut properties);
            self.grids.insert(grid, properties);
        }

        self
    }
}

impl Set<Label> for Properties {
    /// Attaches a label to the axis
    fn set(&mut self, label: Label) -> &mut Properties {
        self.label = Some(label.0);
        self
    }
}

impl Set<Range> for Properties {
    /// Changes the range of the axis that will be shown
    ///
    /// **Note** All axes are auto-scaled by default
    fn set(&mut self, range: Range) -> &mut Properties {
        self.hidden = false;

        match range {
            Range::Auto => self.range = None,
            Range::Limits(low, high) => self.range = Some((low, high)),
        }

        self
    }
}

impl Set<Scale> for Properties {
    /// Sets the scale of the axis
    ///
    /// **Note** All axes use a linear scale by default
    fn set(&mut self, scale: Scale) -> &mut Properties {
        self.hidden = false;

        match scale {
            Scale::Linear => self.logarithmic = false,
            Scale::Logarithmic => self.logarithmic = true,
        }

        self
    }
}

impl Set<ScaleFactor> for Properties {
    /// Changes the *scale factor* of the axis.
    ///
    /// All the data plotted against this axis will have its corresponding coordinate scaled with
    /// this factor before being plotted.
    ///
    /// **Note** The default scale factor is `1`.
    fn set(&mut self, factor: ScaleFactor) -> &mut Properties {
        self.scale_factor = factor.0;

        self
    }
}

impl<P, L> Set<TicLabels<P, L>> for Properties
where
    L: IntoIterator,
    L::Item: AsRef<str>,
    P: IntoIterator,
    P::Item: Data,
{
    /// Attaches labels to the tics of an axis
    fn set(&mut self, tics: TicLabels<P, L>) -> &mut Properties {
        let TicLabels { positions, labels } = tics;

        let pairs = positions
            .into_iter()
            .zip(labels.into_iter())
            .map(|(pos, label)| format!("'{}' {}", label.as_ref(), pos.f64()))
            .collect::<Vec<_>>();

        if pairs.is_empty() {
            self.tics = None
        } else {
            self.tics = Some(pairs.join(", "));
        }

        self
    }
}

impl<'a> Script for (Axis, &'a Properties) {
    // Allow clippy::format_push_string even with older versions of rust (<1.62) which
    // don't have it defined.
    #[allow(clippy::all)]
    fn script(&self) -> String {
        let &(axis, properties) = self;
        let axis_ = axis.display();

        let mut script = if properties.hidden {
            return format!("unset {}tics\n", axis_);
        } else {
            format!("set {}tics nomirror ", axis_)
        };

        if let Some(ref tics) = properties.tics {
            script.push_str(&format!("({})", tics))
        }

        script.push('\n');

        if let Some(ref label) = properties.label {
            script.push_str(&format!("set {}label '{}'\n", axis_, label))
        }

        if let Some((low, high)) = properties.range {
            script.push_str(&format!("set {}range [{}:{}]\n", axis_, low, high))
        }

        if properties.logarithmic {
            script.push_str(&format!("set logscale {}\n", axis_));
        }

        for (grid, properties) in properties.grids.iter() {
            script.push_str(&(axis, grid, properties).script());
        }

        script
    }
}

impl crate::ScaleFactorTrait for Properties {
    fn scale_factor(&self) -> f64 {
        self.scale_factor
    }
}
