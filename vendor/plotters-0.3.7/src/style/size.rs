use crate::coord::CoordTranslate;
use crate::drawing::DrawingArea;
use plotters_backend::DrawingBackend;

/// The trait indicates that the type has a dimensional data.
/// This is the abstraction for the relative sizing model.
/// A relative sizing value is able to be converted into a concrete size
/// when coupling with a type with `HasDimension` type.
pub trait HasDimension {
    /// Get the dimensional data for this object
    fn dim(&self) -> (u32, u32);
}

impl<D: DrawingBackend, C: CoordTranslate> HasDimension for DrawingArea<D, C> {
    fn dim(&self) -> (u32, u32) {
        self.dim_in_pixel()
    }
}

impl HasDimension for (u32, u32) {
    fn dim(&self) -> (u32, u32) {
        *self
    }
}

/// The trait that describes a size, it may be a relative size which the
/// size is determined by the parent size, e.g., 10% of the parent width
pub trait SizeDesc {
    /// Convert the size into the number of pixels
    ///
    /// - `parent`: The reference to the parent container of this size
    /// - **returns**: The number of pixels
    fn in_pixels<T: HasDimension>(&self, parent: &T) -> i32;
}

impl SizeDesc for i32 {
    fn in_pixels<D: HasDimension>(&self, _parent: &D) -> i32 {
        *self
    }
}

impl SizeDesc for u32 {
    fn in_pixels<D: HasDimension>(&self, _parent: &D) -> i32 {
        *self as i32
    }
}

impl SizeDesc for f32 {
    fn in_pixels<D: HasDimension>(&self, _parent: &D) -> i32 {
        *self as i32
    }
}

impl SizeDesc for f64 {
    fn in_pixels<D: HasDimension>(&self, _parent: &D) -> i32 {
        *self as i32
    }
}

/// Describes a relative size, might be
///     1. portion of height
///     2. portion of width
///     3. portion of the minimal of height and weight
pub enum RelativeSize {
    /// Percentage height
    Height(f64),
    /// Percentage width
    Width(f64),
    /// Percentage of either height or width, which is smaller
    Smaller(f64),
}

impl RelativeSize {
    /// Set the lower bound of the relative size.
    ///
    /// - `min_sz`: The minimal size the relative size can be in pixels
    /// - **returns**: The relative size with the bound
    pub fn min(self, min_sz: i32) -> RelativeSizeWithBound {
        RelativeSizeWithBound {
            size: self,
            min: Some(min_sz),
            max: None,
        }
    }

    /// Set the upper bound of the relative size
    ///
    /// - `max_size`: The maximum size in pixels for this relative size
    /// - **returns** The relative size with the upper bound
    pub fn max(self, max_sz: i32) -> RelativeSizeWithBound {
        RelativeSizeWithBound {
            size: self,
            max: Some(max_sz),
            min: None,
        }
    }
}

impl SizeDesc for RelativeSize {
    fn in_pixels<D: HasDimension>(&self, parent: &D) -> i32 {
        let (w, h) = parent.dim();
        match self {
            RelativeSize::Width(p) => *p * f64::from(w),
            RelativeSize::Height(p) => *p * f64::from(h),
            RelativeSize::Smaller(p) => *p * f64::from(w.min(h)),
        }
        .round() as i32
    }
}

/// Allows a value turns into a relative size
pub trait AsRelative: Into<f64> {
    /// Make the value a relative size of percentage of width
    fn percent_width(self) -> RelativeSize {
        RelativeSize::Width(self.into() / 100.0)
    }
    /// Make the value a relative size of percentage of height
    fn percent_height(self) -> RelativeSize {
        RelativeSize::Height(self.into() / 100.0)
    }
    /// Make the value a relative size of percentage of minimal of height and width
    fn percent(self) -> RelativeSize {
        RelativeSize::Smaller(self.into() / 100.0)
    }
}

impl<T: Into<f64>> AsRelative for T {}

/// The struct describes a relative size with upper bound and lower bound
pub struct RelativeSizeWithBound {
    size: RelativeSize,
    min: Option<i32>,
    max: Option<i32>,
}

impl RelativeSizeWithBound {
    /// Set the lower bound of the bounded relative size
    ///
    /// - `min_sz`: The lower bound of this size description
    /// - **returns**: The newly created size description with the bound
    pub fn min(mut self, min_sz: i32) -> RelativeSizeWithBound {
        self.min = Some(min_sz);
        self
    }

    /// Set the upper bound of the bounded relative size
    ///
    /// - `min_sz`: The upper bound of this size description
    /// - **returns**: The newly created size description with the bound
    pub fn max(mut self, max_sz: i32) -> RelativeSizeWithBound {
        self.max = Some(max_sz);
        self
    }
}

impl SizeDesc for RelativeSizeWithBound {
    fn in_pixels<D: HasDimension>(&self, parent: &D) -> i32 {
        let size = self.size.in_pixels(parent);
        let size_lower_capped = self.min.map_or(size, |x| x.max(size));
        self.max.map_or(size_lower_capped, |x| x.min(size))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_relative_size() {
        let size = (10).percent_height();
        assert_eq!(size.in_pixels(&(100, 200)), 20);

        let size = (10).percent_width();
        assert_eq!(size.in_pixels(&(100, 200)), 10);

        let size = (-10).percent_width();
        assert_eq!(size.in_pixels(&(100, 200)), -10);

        let size = (10).percent_width().min(30);
        assert_eq!(size.in_pixels(&(100, 200)), 30);
        assert_eq!(size.in_pixels(&(400, 200)), 40);

        let size = (10).percent();
        assert_eq!(size.in_pixels(&(100, 200)), 10);
        assert_eq!(size.in_pixels(&(400, 200)), 20);
    }
}
