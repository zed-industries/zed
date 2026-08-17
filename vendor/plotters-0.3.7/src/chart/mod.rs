/*!
The high-level plotting abstractions.

Plotters uses `ChartContext`, a thin layer on the top of `DrawingArea`,  to provide
high-level chart specific drawing functionalities, like, mesh line, coordinate label
and other common components for the data chart.

To draw a series, `ChartContext::draw_series` is used to draw a series on the chart.
In Plotters, a series is abstracted as an iterator of elements.

`ChartBuilder` is used to construct a chart. To learn more detailed information, check the
detailed description for each struct.
*/

mod axes3d;
mod builder;
mod context;
mod dual_coord;
mod mesh;
mod series;
mod state;

pub use builder::{ChartBuilder, LabelAreaPosition};
pub use context::ChartContext;
pub use dual_coord::{DualCoordChartContext, DualCoordChartState};
pub use mesh::{MeshStyle, SecondaryMeshStyle};
pub use series::{SeriesAnno, SeriesLabelPosition, SeriesLabelStyle};
pub use state::ChartState;

use context::Coord3D;
