use crate::element::PointElement;
use crate::style::{ShapeStyle, SizeDesc};

/// The point plot object, which takes an iterator of points in guest coordinate system
/// and create an element for each point
pub struct PointSeries<'a, Coord, I: IntoIterator<Item = Coord>, E, Size: SizeDesc + Clone> {
    style: ShapeStyle,
    size: Size,
    data_iter: I::IntoIter,
    make_point: &'a dyn Fn(Coord, Size, ShapeStyle) -> E,
}

impl<'a, Coord, I: IntoIterator<Item = Coord>, E, Size: SizeDesc + Clone> Iterator
    for PointSeries<'a, Coord, I, E, Size>
{
    type Item = E;
    fn next(&mut self) -> Option<Self::Item> {
        self.data_iter
            .next()
            .map(|x| (self.make_point)(x, self.size.clone(), self.style))
    }
}

impl<'a, Coord, I: IntoIterator<Item = Coord>, E, Size: SizeDesc + Clone>
    PointSeries<'a, Coord, I, E, Size>
where
    E: PointElement<Coord, Size>,
{
    /// Create a new point series with the element that implements point trait.
    /// You may also use a more general way to create a point series with `of_element`
    /// function which allows a customized element construction function
    pub fn new<S: Into<ShapeStyle>>(iter: I, size: Size, style: S) -> Self {
        Self {
            data_iter: iter.into_iter(),
            size,
            style: style.into(),
            make_point: &|a, b, c| E::make_point(a, b, c),
        }
    }
}

impl<'a, Coord, I: IntoIterator<Item = Coord>, E, Size: SizeDesc + Clone>
    PointSeries<'a, Coord, I, E, Size>
{
    /// Create a new point series. Similar to `PointSeries::new` but it doesn't
    /// requires the element implements point trait. So instead of using the point
    /// constructor, it uses the customized function for element creation
    pub fn of_element<S: Into<ShapeStyle>, F: Fn(Coord, Size, ShapeStyle) -> E>(
        iter: I,
        size: Size,
        style: S,
        cons: &'a F,
    ) -> Self {
        Self {
            data_iter: iter.into_iter(),
            size,
            style: style.into(),
            make_point: cons,
        }
    }
}
