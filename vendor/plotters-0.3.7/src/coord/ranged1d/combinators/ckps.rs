// The customized coordinate combinators.
// This file contains a set of coorindate combinators that allows you determine the
// keypoint by your own code.
use std::ops::Range;

use crate::coord::ranged1d::{AsRangedCoord, DiscreteRanged, KeyPointHint, Ranged};

/// The coordinate decorator that binds a key point vector.
/// Normally, all the ranged coordinate implements its own keypoint algorithm
/// to determine how to render the tick mark and mesh grid.
/// This decorator allows customized tick mark specifiied by vector.
/// See [BindKeyPoints::with_key_points](trait.BindKeyPoints.html#tymethod.with_key_points)
/// for details.
/// Note: For any coordinate spec wrapped by this decorator, the maximum number of labels configured by
/// MeshStyle will be ignored and the key point function will always returns the entire vector
pub struct WithKeyPoints<Inner: Ranged> {
    inner: Inner,
    bold_points: Vec<Inner::ValueType>,
    light_points: Vec<Inner::ValueType>,
}

impl<I: Ranged> WithKeyPoints<I> {
    /// Specify the light key points, which is used to render the light mesh line
    pub fn with_light_points<T: IntoIterator<Item = I::ValueType>>(mut self, iter: T) -> Self {
        self.light_points.clear();
        self.light_points.extend(iter);
        self
    }

    /// Get a reference to the bold points
    pub fn bold_points(&self) -> &[I::ValueType] {
        self.bold_points.as_ref()
    }

    /// Get a mut reference to the bold points
    pub fn bold_points_mut(&mut self) -> &mut [I::ValueType] {
        self.bold_points.as_mut()
    }

    /// Get a reference to light key points
    pub fn light_points(&self) -> &[I::ValueType] {
        self.light_points.as_ref()
    }

    /// Get a mut reference to the light key points
    pub fn light_points_mut(&mut self) -> &mut [I::ValueType] {
        self.light_points.as_mut()
    }
}

impl<R: Ranged> Ranged for WithKeyPoints<R>
where
    R::ValueType: Clone,
{
    type ValueType = R::ValueType;
    type FormatOption = R::FormatOption;

    fn range(&self) -> Range<Self::ValueType> {
        self.inner.range()
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        self.inner.map(value, limit)
    }

    fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<Self::ValueType> {
        if hint.weight().allow_light_points() {
            self.light_points.clone()
        } else {
            self.bold_points.clone()
        }
    }

    fn axis_pixel_range(&self, limit: (i32, i32)) -> Range<i32> {
        self.inner.axis_pixel_range(limit)
    }
}

impl<R: DiscreteRanged> DiscreteRanged for WithKeyPoints<R>
where
    R::ValueType: Clone,
{
    fn size(&self) -> usize {
        self.inner.size()
    }
    fn index_of(&self, value: &Self::ValueType) -> Option<usize> {
        self.inner.index_of(value)
    }
    fn from_index(&self, index: usize) -> Option<Self::ValueType> {
        self.inner.from_index(index)
    }
}

/// Bind a existing coordinate spec with a given key points vector. See [WithKeyPoints](struct.WithKeyPoints.html ) for more details.
pub trait BindKeyPoints
where
    Self: AsRangedCoord,
{
    /// Bind a existing coordinate spec with a given key points vector. See [WithKeyPoints](struct.WithKeyPoints.html ) for more details.
    /// Example:
    /// ```
    ///use plotters::prelude::*;
    ///use plotters_bitmap::BitMapBackend;
    ///let mut buffer = vec![0;1024*768*3];
    /// let root = BitMapBackend::with_buffer(&mut buffer, (1024, 768)).into_drawing_area();
    /// let mut chart = ChartBuilder::on(&root)
    ///    .build_cartesian_2d(
    ///        (0..100).with_key_points(vec![1,20,50,90]),   // <= This line will make the plot shows 4 tick marks at 1, 20, 50, 90
    ///        0..10
    /// ).unwrap();
    /// chart.configure_mesh().draw().unwrap();
    ///```
    fn with_key_points(self, points: Vec<Self::Value>) -> WithKeyPoints<Self::CoordDescType> {
        WithKeyPoints {
            inner: self.into(),
            bold_points: points,
            light_points: vec![],
        }
    }
}

impl<T: AsRangedCoord> BindKeyPoints for T {}

/// The coordinate decorator that allows customized keypoint algorithms.
/// Normally, all the coordinate spec implements its own key point algorithm
/// But this decorator allows you override the pre-defined key point algorithm.
///
/// To use this decorator, see [BindKeyPointMethod::with_key_point_func](trait.BindKeyPointMethod.html#tymethod.with_key_point_func)
pub struct WithKeyPointMethod<R: Ranged> {
    inner: R,
    bold_func: Box<dyn Fn(usize) -> Vec<R::ValueType>>,
    light_func: Box<dyn Fn(usize) -> Vec<R::ValueType>>,
}

/// Bind an existing coordinate spec with a given key points algorithm. See [WithKeyPointMethod](struct.WithKeyMethod.html ) for more details.
pub trait BindKeyPointMethod
where
    Self: AsRangedCoord,
{
    /// Bind a existing coordinate spec with a given key points algorithm. See [WithKeyPointMethod](struct.WithKeyMethod.html ) for more details.
    /// Example:
    /// ```
    ///use plotters::prelude::*;
    ///use plotters_bitmap::BitMapBackend;
    ///let mut buffer = vec![0;1024*768*3];
    /// let root = BitMapBackend::with_buffer(&mut buffer, (1024, 768)).into_drawing_area();
    /// let mut chart = ChartBuilder::on(&root)
    ///    .build_cartesian_2d(
    ///        (0..100).with_key_point_func(|n| (0..100 / n as i32).map(|x| x * 100 / n as i32).collect()),
    ///        0..10
    /// ).unwrap();
    /// chart.configure_mesh().draw().unwrap();
    ///```
    fn with_key_point_func<F: Fn(usize) -> Vec<Self::Value> + 'static>(
        self,
        func: F,
    ) -> WithKeyPointMethod<Self::CoordDescType> {
        WithKeyPointMethod {
            inner: self.into(),
            bold_func: Box::new(func),
            light_func: Box::new(|_| Vec::new()),
        }
    }
}

impl<T: AsRangedCoord> BindKeyPointMethod for T {}

impl<R: Ranged> WithKeyPointMethod<R> {
    /// Define the light key point algorithm, by default this returns an empty set
    pub fn with_light_point_func<F: Fn(usize) -> Vec<R::ValueType> + 'static>(
        mut self,
        func: F,
    ) -> Self {
        self.light_func = Box::new(func);
        self
    }
}

impl<R: Ranged> Ranged for WithKeyPointMethod<R> {
    type ValueType = R::ValueType;
    type FormatOption = R::FormatOption;

    fn range(&self) -> Range<Self::ValueType> {
        self.inner.range()
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        self.inner.map(value, limit)
    }

    fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<Self::ValueType> {
        if hint.weight().allow_light_points() {
            (self.light_func)(hint.max_num_points())
        } else {
            (self.bold_func)(hint.max_num_points())
        }
    }

    fn axis_pixel_range(&self, limit: (i32, i32)) -> Range<i32> {
        self.inner.axis_pixel_range(limit)
    }
}

impl<R: DiscreteRanged> DiscreteRanged for WithKeyPointMethod<R> {
    fn size(&self) -> usize {
        self.inner.size()
    }
    fn index_of(&self, value: &Self::ValueType) -> Option<usize> {
        self.inner.index_of(value)
    }
    fn from_index(&self, index: usize) -> Option<Self::ValueType> {
        self.inner.from_index(index)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::coord::ranged1d::{BoldPoints, LightPoints};
    #[test]
    fn test_with_key_points() {
        let range = (0..100).with_key_points(vec![1, 2, 3]);
        assert_eq!(range.map(&3, (0, 1000)), 30);
        assert_eq!(range.range(), 0..100);
        assert_eq!(range.key_points(BoldPoints(100)), vec![1, 2, 3]);
        assert_eq!(range.key_points(LightPoints::new(100, 100)), vec![]);
        let range = range.with_light_points(5..10);
        assert_eq!(range.key_points(BoldPoints(10)), vec![1, 2, 3]);
        assert_eq!(
            range.key_points(LightPoints::new(10, 10)),
            (5..10).collect::<Vec<_>>()
        );

        assert_eq!(range.size(), 101);
        assert_eq!(range.index_of(&10), Some(10));
        assert_eq!(range.from_index(10), Some(10));

        assert_eq!(range.axis_pixel_range((0, 1000)), 0..1000);

        let mut range = range;

        assert_eq!(range.light_points().len(), 5);
        assert_eq!(range.light_points_mut().len(), 5);
        assert_eq!(range.bold_points().len(), 3);
        assert_eq!(range.bold_points_mut().len(), 3);
    }

    #[test]
    fn test_with_key_point_method() {
        let range = (0..100).with_key_point_func(|_| vec![1, 2, 3]);
        assert_eq!(range.map(&3, (0, 1000)), 30);
        assert_eq!(range.range(), 0..100);
        assert_eq!(range.key_points(BoldPoints(100)), vec![1, 2, 3]);
        assert_eq!(range.key_points(LightPoints::new(100, 100)), vec![]);
        let range = range.with_light_point_func(|_| (5..10).collect());
        assert_eq!(range.key_points(BoldPoints(10)), vec![1, 2, 3]);
        assert_eq!(
            range.key_points(LightPoints::new(10, 10)),
            (5..10).collect::<Vec<_>>()
        );

        assert_eq!(range.size(), 101);
        assert_eq!(range.index_of(&10), Some(10));
        assert_eq!(range.from_index(10), Some(10));

        assert_eq!(range.axis_pixel_range((0, 1000)), 0..1000);
    }
}
