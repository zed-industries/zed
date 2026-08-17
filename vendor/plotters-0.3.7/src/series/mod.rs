/*!
  This module contains predefined types of series.
  The series in Plotters is actually an iterator of elements, which
  can be taken by `ChartContext::draw_series` function.

  This module defines some "iterator transformer", which transform the data
  iterator to the element iterator.

  Any type that implements iterator emitting drawable elements are acceptable series.
  So iterator combinator such as `map`, `zip`, etc can also be used.
*/

#[cfg(feature = "area_series")]
mod area_series;
#[cfg(feature = "histogram")]
mod histogram;
#[cfg(feature = "line_series")]
mod line_series;
#[cfg(feature = "point_series")]
mod point_series;
#[cfg(feature = "surface_series")]
mod surface;

#[cfg(feature = "area_series")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "area_series")))]
pub use area_series::AreaSeries;
#[cfg(feature = "histogram")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "histogram")))]
pub use histogram::Histogram;
#[cfg(feature = "line_series")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "line_series")))]
pub use line_series::{DashedLineSeries, DottedLineSeries, LineSeries};
#[cfg(feature = "point_series")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "point_series")))]
pub use point_series::PointSeries;
#[cfg(feature = "surface_series")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "surface_series")))]
pub use surface::SurfaceSeries;
