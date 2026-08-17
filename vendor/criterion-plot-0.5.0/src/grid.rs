//! Gridline

use crate::{Axis, Default, Display, Grid, Script};

/// Gridline properties
#[derive(Clone, Copy)]
pub struct Properties {
    hidden: bool,
}

impl Default for Properties {
    fn default() -> Properties {
        Properties { hidden: true }
    }
}

// TODO Lots of configuration pending: linetype, linewidth, etc
impl Properties {
    /// Hides the gridlines
    ///
    /// **Note** Both `Major` and `Minor` gridlines are hidden by default
    pub fn hide(&mut self) -> &mut Properties {
        self.hidden = true;
        self
    }

    /// Shows the gridlines
    pub fn show(&mut self) -> &mut Properties {
        self.hidden = false;
        self
    }
}

impl<'a> Script for (Axis, Grid, &'a Properties) {
    fn script(&self) -> String {
        let &(axis, grid, properties) = self;
        let axis = axis.display();
        let grid = grid.display();

        if properties.hidden {
            String::new()
        } else {
            format!("set grid {}{}tics\n", grid, axis)
        }
    }
}
