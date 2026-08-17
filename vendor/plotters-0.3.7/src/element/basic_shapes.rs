use super::{Drawable, PointCollection};
use crate::style::{Color, ShapeStyle, SizeDesc};
use plotters_backend::{BackendCoord, DrawingBackend, DrawingErrorKind};

#[inline]
fn to_i((x, y): (f32, f32)) -> (i32, i32) {
    (x.round() as i32, y.round() as i32)
}

#[inline]
fn to_f((x, y): (i32, i32)) -> (f32, f32) {
    (x as f32, y as f32)
}

/**
An element representing a single pixel.

See [`crate::element::EmptyElement`] for more information and examples.
*/
pub struct Pixel<Coord> {
    pos: Coord,
    style: ShapeStyle,
}

impl<Coord> Pixel<Coord> {
    /**
    Creates a new pixel.

    See [`crate::element::EmptyElement`] for more information and examples.
    */
    pub fn new<P: Into<Coord>, S: Into<ShapeStyle>>(pos: P, style: S) -> Self {
        Self {
            pos: pos.into(),
            style: style.into(),
        }
    }
}

impl<'a, Coord> PointCollection<'a, Coord> for &'a Pixel<Coord> {
    type Point = &'a Coord;
    type IntoIter = std::iter::Once<&'a Coord>;
    fn point_iter(self) -> Self::IntoIter {
        std::iter::once(&self.pos)
    }
}

impl<Coord, DB: DrawingBackend> Drawable<DB> for Pixel<Coord> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        if let Some((x, y)) = points.next() {
            return backend.draw_pixel((x, y), self.style.color.to_backend_color());
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_pixel_element() {
    use crate::prelude::*;
    let da = crate::create_mocked_drawing_area(300, 300, |m| {
        m.check_draw_pixel(|c, (x, y)| {
            assert_eq!(x, 150);
            assert_eq!(y, 152);
            assert_eq!(c, RED.to_rgba());
        });

        m.drop_check(|b| {
            assert_eq!(b.num_draw_pixel_call, 1);
            assert_eq!(b.draw_count, 1);
        });
    });
    da.draw(&Pixel::new((150, 152), RED))
        .expect("Drawing Failure");
}

/// This is a deprecated type. Please use new name [`PathElement`] instead.
#[deprecated(note = "Use new name PathElement instead")]
pub type Path<Coord> = PathElement<Coord>;

/// An element of a series of connected lines
pub struct PathElement<Coord> {
    points: Vec<Coord>,
    style: ShapeStyle,
}
impl<Coord> PathElement<Coord> {
    /// Create a new path
    /// - `points`: The iterator of the points
    /// - `style`: The shape style
    /// - returns the created element
    pub fn new<P: Into<Vec<Coord>>, S: Into<ShapeStyle>>(points: P, style: S) -> Self {
        Self {
            points: points.into(),
            style: style.into(),
        }
    }
}

impl<'a, Coord> PointCollection<'a, Coord> for &'a PathElement<Coord> {
    type Point = &'a Coord;
    type IntoIter = &'a [Coord];
    fn point_iter(self) -> &'a [Coord] {
        &self.points
    }
}

impl<Coord, DB: DrawingBackend> Drawable<DB> for PathElement<Coord> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        backend.draw_path(points, &self.style)
    }
}

#[cfg(test)]
#[test]
fn test_path_element() {
    use crate::prelude::*;
    let da = crate::create_mocked_drawing_area(300, 300, |m| {
        m.check_draw_path(|c, s, path| {
            assert_eq!(c, BLUE.to_rgba());
            assert_eq!(s, 5);
            assert_eq!(path, vec![(100, 101), (105, 107), (150, 157)]);
        });
        m.drop_check(|b| {
            assert_eq!(b.num_draw_path_call, 1);
            assert_eq!(b.draw_count, 1);
        });
    });
    da.draw(&PathElement::new(
        vec![(100, 101), (105, 107), (150, 157)],
        Into::<ShapeStyle>::into(BLUE).stroke_width(5),
    ))
    .expect("Drawing Failure");
}

/// An element of a series of connected lines in dash style.
///
/// It's similar to [`PathElement`] but has a dash style.
pub struct DashedPathElement<I: Iterator + Clone, Size: SizeDesc> {
    points: I,
    size: Size,
    spacing: Size,
    style: ShapeStyle,
}

impl<I: Iterator + Clone, Size: SizeDesc> DashedPathElement<I, Size> {
    /// Create a new path
    /// - `points`: The iterator of the points
    /// - `size`: The dash size
    /// - `spacing`: The dash-to-dash spacing (gap size)
    /// - `style`: The shape style
    /// - returns the created element
    pub fn new<I0, S>(points: I0, size: Size, spacing: Size, style: S) -> Self
    where
        I0: IntoIterator<IntoIter = I>,
        S: Into<ShapeStyle>,
    {
        Self {
            points: points.into_iter(),
            size,
            spacing,
            style: style.into(),
        }
    }
}

impl<'a, I: Iterator + Clone, Size: SizeDesc> PointCollection<'a, I::Item>
    for &'a DashedPathElement<I, Size>
{
    type Point = I::Item;
    type IntoIter = I;
    fn point_iter(self) -> Self::IntoIter {
        self.points.clone()
    }
}

impl<I0: Iterator + Clone, Size: SizeDesc, DB: DrawingBackend> Drawable<DB>
    for DashedPathElement<I0, Size>
{
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut points: I,
        backend: &mut DB,
        ps: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let mut start = match points.next() {
            Some(c) => to_f(c),
            None => return Ok(()),
        };
        let size = self.size.in_pixels(&ps).max(0) as f32;
        if size == 0. {
            return Ok(());
        }
        let spacing = self.spacing.in_pixels(&ps).max(0) as f32;
        let mut dist = 0.;
        let mut is_solid = true;
        let mut queue = vec![to_i(start)];
        for curr in points {
            let end = to_f(curr);
            // Loop for solid and spacing
            while start != end {
                let (dx, dy) = (end.0 - start.0, end.1 - start.1);
                let d = dx.hypot(dy);
                let size = if is_solid { size } else { spacing };
                let left = size - dist;
                // Set next point to `start`
                if left < d {
                    let t = left / d;
                    start = (start.0 + dx * t, start.1 + dy * t);
                    dist += left;
                } else {
                    start = end;
                    dist += d;
                }
                // Draw if needed
                if is_solid {
                    queue.push(to_i(start));
                }
                if size <= dist {
                    if is_solid {
                        backend.draw_path(queue.drain(..), &self.style)?;
                    } else {
                        queue.push(to_i(start));
                    }
                    dist = 0.;
                    is_solid = !is_solid;
                }
            }
        }
        if queue.len() > 1 {
            backend.draw_path(queue, &self.style)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_dashed_path_element() {
    use crate::prelude::*;
    let check_list = std::cell::RefCell::new(vec![
        vec![(100, 100), (100, 103), (100, 105)],
        vec![(100, 107), (100, 112)],
        vec![(100, 114), (100, 119)],
        vec![(100, 119), (100, 120)],
    ]);
    let da = crate::create_mocked_drawing_area(300, 300, |m| {
        m.check_draw_path(move |c, s, path| {
            assert_eq!(c, BLUE.to_rgba());
            assert_eq!(s, 7);
            assert_eq!(path, check_list.borrow_mut().remove(0));
        });
        m.drop_check(|b| {
            assert_eq!(b.num_draw_path_call, 3);
            assert_eq!(b.draw_count, 3);
        });
    });
    da.draw(&DashedPathElement::new(
        vec![(100, 100), (100, 103), (100, 120)],
        5.,
        2.,
        BLUE.stroke_width(7),
    ))
    .expect("Drawing Failure");
}

/// An element of a series of connected lines in dot style for any markers.
///
/// It's similar to [`PathElement`] but use a marker function to draw markers with spacing.
pub struct DottedPathElement<I: Iterator + Clone, Size: SizeDesc, Marker> {
    points: I,
    shift: Size,
    spacing: Size,
    func: Box<dyn Fn(BackendCoord) -> Marker>,
}

impl<I: Iterator + Clone, Size: SizeDesc, Marker> DottedPathElement<I, Size, Marker> {
    /// Create a new path
    /// - `points`: The iterator of the points
    /// - `shift`: The shift of the first marker
    /// - `spacing`: The spacing between markers
    /// - `func`: The marker function
    /// - returns the created element
    pub fn new<I0, F>(points: I0, shift: Size, spacing: Size, func: F) -> Self
    where
        I0: IntoIterator<IntoIter = I>,
        F: Fn(BackendCoord) -> Marker + 'static,
    {
        Self {
            points: points.into_iter(),
            shift,
            spacing,
            func: Box::new(func),
        }
    }
}

impl<'a, I: Iterator + Clone, Size: SizeDesc, Marker> PointCollection<'a, I::Item>
    for &'a DottedPathElement<I, Size, Marker>
{
    type Point = I::Item;
    type IntoIter = I;
    fn point_iter(self) -> Self::IntoIter {
        self.points.clone()
    }
}

impl<I0, Size, DB, Marker> Drawable<DB> for DottedPathElement<I0, Size, Marker>
where
    I0: Iterator + Clone,
    Size: SizeDesc,
    DB: DrawingBackend,
    Marker: crate::element::IntoDynElement<'static, DB, BackendCoord>,
{
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut points: I,
        backend: &mut DB,
        ps: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let mut shift = self.shift.in_pixels(&ps).max(0) as f32;
        let mut start = match points.next() {
            Some(start_i) => {
                // Draw the first marker if no shift
                if shift == 0. {
                    let mk = (self.func)(start_i).into_dyn();
                    mk.draw(mk.point_iter().iter().copied(), backend, ps)?;
                }
                to_f(start_i)
            }
            None => return Ok(()),
        };
        let spacing = self.spacing.in_pixels(&ps).max(0) as f32;
        let mut dist = 0.;
        for curr in points {
            let end = to_f(curr);
            // Loop for spacing
            while start != end {
                let (dx, dy) = (end.0 - start.0, end.1 - start.1);
                let d = dx.hypot(dy);
                let spacing = if shift == 0. { spacing } else { shift };
                let left = spacing - dist;
                // Set next point to `start`
                if left < d {
                    let t = left / d;
                    start = (start.0 + dx * t, start.1 + dy * t);
                    dist += left;
                } else {
                    start = end;
                    dist += d;
                }
                // Draw if needed
                if spacing <= dist {
                    let mk = (self.func)(to_i(start)).into_dyn();
                    mk.draw(mk.point_iter().iter().copied(), backend, ps)?;
                    shift = 0.;
                    dist = 0.;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_dotted_path_element() {
    use crate::prelude::*;
    let da = crate::create_mocked_drawing_area(300, 300, |m| {
        m.drop_check(|b| {
            assert_eq!(b.num_draw_path_call, 0);
            assert_eq!(b.draw_count, 7);
        });
    });
    da.draw(&DottedPathElement::new(
        vec![(100, 100), (105, 105), (150, 150)],
        5,
        10,
        |c| Circle::new(c, 5, Into::<ShapeStyle>::into(RED).filled()),
    ))
    .expect("Drawing Failure");
}

/// A rectangle element
pub struct Rectangle<Coord> {
    points: [Coord; 2],
    style: ShapeStyle,
    margin: (u32, u32, u32, u32),
}

impl<Coord> Rectangle<Coord> {
    /// Create a new path
    /// - `points`: The left upper and right lower corner of the rectangle
    /// - `style`: The shape style
    /// - returns the created element
    pub fn new<S: Into<ShapeStyle>>(points: [Coord; 2], style: S) -> Self {
        Self {
            points,
            style: style.into(),
            margin: (0, 0, 0, 0),
        }
    }

    /// Set the margin of the rectangle
    /// - `t`: The top margin
    /// - `b`: The bottom margin
    /// - `l`: The left margin
    /// - `r`: The right margin
    pub fn set_margin(&mut self, t: u32, b: u32, l: u32, r: u32) -> &mut Self {
        self.margin = (t, b, l, r);
        self
    }

    /// Get the points of the rectangle
    /// - returns the element points
    pub fn get_points(&self) -> (&Coord, &Coord) {
        (&self.points[0], &self.points[1])
    }

    /// Set the style of the rectangle
    /// - `style`: The shape style
    /// - returns a mut reference to the rectangle
    pub fn set_style<S: Into<ShapeStyle>>(&mut self, style: S) -> &mut Self {
        self.style = style.into();
        self
    }
}

impl<'a, Coord> PointCollection<'a, Coord> for &'a Rectangle<Coord> {
    type Point = &'a Coord;
    type IntoIter = &'a [Coord];
    fn point_iter(self) -> &'a [Coord] {
        &self.points
    }
}

impl<Coord, DB: DrawingBackend> Drawable<DB> for Rectangle<Coord> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        match (points.next(), points.next()) {
            (Some(a), Some(b)) => {
                let (mut a, mut b) = ((a.0.min(b.0), a.1.min(b.1)), (a.0.max(b.0), a.1.max(b.1)));
                a.1 += self.margin.0 as i32;
                b.1 -= self.margin.1 as i32;
                a.0 += self.margin.2 as i32;
                b.0 -= self.margin.3 as i32;
                backend.draw_rect(a, b, &self.style, self.style.filled)
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
#[test]
fn test_rect_element() {
    use crate::prelude::*;
    {
        let da = crate::create_mocked_drawing_area(300, 300, |m| {
            m.check_draw_rect(|c, s, f, u, d| {
                assert_eq!(c, BLUE.to_rgba());
                assert!(!f);
                assert_eq!(s, 5);
                assert_eq!([u, d], [(100, 101), (105, 107)]);
            });
            m.drop_check(|b| {
                assert_eq!(b.num_draw_rect_call, 1);
                assert_eq!(b.draw_count, 1);
            });
        });
        da.draw(&Rectangle::new(
            [(100, 101), (105, 107)],
            Color::stroke_width(&BLUE, 5),
        ))
        .expect("Drawing Failure");
    }

    {
        let da = crate::create_mocked_drawing_area(300, 300, |m| {
            m.check_draw_rect(|c, _, f, u, d| {
                assert_eq!(c, BLUE.to_rgba());
                assert!(f);
                assert_eq!([u, d], [(100, 101), (105, 107)]);
            });
            m.drop_check(|b| {
                assert_eq!(b.num_draw_rect_call, 1);
                assert_eq!(b.draw_count, 1);
            });
        });
        da.draw(&Rectangle::new([(100, 101), (105, 107)], BLUE.filled()))
            .expect("Drawing Failure");
    }
}

/// A circle element
pub struct Circle<Coord, Size: SizeDesc> {
    center: Coord,
    size: Size,
    style: ShapeStyle,
}

impl<Coord, Size: SizeDesc> Circle<Coord, Size> {
    /// Create a new circle element
    /// - `coord` The center of the circle
    /// - `size` The radius of the circle
    /// - `style` The style of the circle
    /// - Return: The newly created circle element
    pub fn new<S: Into<ShapeStyle>>(coord: Coord, size: Size, style: S) -> Self {
        Self {
            center: coord,
            size,
            style: style.into(),
        }
    }
}

impl<'a, Coord, Size: SizeDesc> PointCollection<'a, Coord> for &'a Circle<Coord, Size> {
    type Point = &'a Coord;
    type IntoIter = std::iter::Once<&'a Coord>;
    fn point_iter(self) -> std::iter::Once<&'a Coord> {
        std::iter::once(&self.center)
    }
}

impl<Coord, DB: DrawingBackend, Size: SizeDesc> Drawable<DB> for Circle<Coord, Size> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut points: I,
        backend: &mut DB,
        ps: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        if let Some((x, y)) = points.next() {
            let size = self.size.in_pixels(&ps).max(0) as u32;
            return backend.draw_circle((x, y), size, &self.style, self.style.filled);
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_circle_element() {
    use crate::prelude::*;
    let da = crate::create_mocked_drawing_area(300, 300, |m| {
        m.check_draw_circle(|c, _, f, s, r| {
            assert_eq!(c, BLUE.to_rgba());
            assert!(!f);
            assert_eq!(s, (150, 151));
            assert_eq!(r, 20);
        });
        m.drop_check(|b| {
            assert_eq!(b.num_draw_circle_call, 1);
            assert_eq!(b.draw_count, 1);
        });
    });
    da.draw(&Circle::new((150, 151), 20, BLUE))
        .expect("Drawing Failure");
}

/// An element of a filled polygon
pub struct Polygon<Coord> {
    points: Vec<Coord>,
    style: ShapeStyle,
}
impl<Coord> Polygon<Coord> {
    /// Create a new polygon
    /// - `points`: The iterator of the points
    /// - `style`: The shape style
    /// - returns the created element
    pub fn new<P: Into<Vec<Coord>>, S: Into<ShapeStyle>>(points: P, style: S) -> Self {
        Self {
            points: points.into(),
            style: style.into(),
        }
    }
}

impl<'a, Coord> PointCollection<'a, Coord> for &'a Polygon<Coord> {
    type Point = &'a Coord;
    type IntoIter = &'a [Coord];
    fn point_iter(self) -> &'a [Coord] {
        &self.points
    }
}

impl<Coord, DB: DrawingBackend> Drawable<DB> for Polygon<Coord> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        backend.fill_polygon(points, &self.style.color.to_backend_color())
    }
}

#[cfg(test)]
#[test]
fn test_polygon_element() {
    use crate::prelude::*;
    let points = vec![(100, 100), (50, 500), (300, 400), (200, 300), (550, 200)];
    let expected_points = points.clone();

    let da = crate::create_mocked_drawing_area(800, 800, |m| {
        m.check_fill_polygon(move |c, p| {
            assert_eq!(c, BLUE.to_rgba());
            assert_eq!(expected_points.len(), p.len());
            assert_eq!(expected_points, p);
        });
        m.drop_check(|b| {
            assert_eq!(b.num_fill_polygon_call, 1);
            assert_eq!(b.draw_count, 1);
        });
    });

    da.draw(&Polygon::new(points.clone(), BLUE))
        .expect("Drawing Failure");
}
