//! Path styles.

/// Describes the visual style of a fill.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Fill {
    /// The non-zero fill rule.
    NonZero,
    /// The even-odd fill rule.
    EvenOdd,
}

/// Defines the connection between two segments of a stroke.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Join {
    /// A straight line connecting the segments.
    Bevel,
    /// The segments are extended to their natural intersection point.
    Miter,
    /// An arc between the segments.
    Round,
}

/// Defines the shape to be drawn at the beginning or end of a stroke.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Cap {
    /// Flat cap.
    Butt,
    /// Square cap with dimensions equal to half the stroke width.
    Square,
    /// Rounded cap with radius equal to half the stroke width.
    Round,
}

/// Describes the visual style of a stroke.
#[derive(Copy, Clone, Debug)]
pub struct Stroke<'a> {
    /// Width of the stroke.
    pub width: f32,
    /// Style for connecting segments of the stroke.
    pub join: Join,
    /// Limit for miter joins.
    pub miter_limit: f32,
    /// Style for capping the beginning of an open subpath.
    pub start_cap: Cap,
    /// Style for capping the end of an open subpath.
    pub end_cap: Cap,
    /// Lengths of dashes in alternating on/off order.
    pub dashes: &'a [f32],
    /// Offset of the first dash.
    pub offset: f32,
    /// True if the stroke width should be affected by the scale of a transform.
    pub scale: bool,
}

impl Default for Stroke<'_> {
    fn default() -> Self {
        Self {
            width: 1.,
            join: Join::Miter,
            miter_limit: 4.,
            start_cap: Cap::Butt,
            end_cap: Cap::Butt,
            dashes: &[],
            offset: 0.,
            scale: true,
        }
    }
}

impl<'a> Stroke<'a> {
    /// Creates a new stroke style with the specified width.
    #[allow(clippy::field_reassign_with_default)]
    pub fn new(width: f32) -> Self {
        let mut s = Self::default();
        s.width = width;
        s
    }

    /// Sets the width of the stroke. The default is 1.
    pub fn width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }

    /// Sets the join style that determines how individual segments of the path
    /// will be connected. The default is miter.
    pub fn join(&mut self, join: Join) -> &mut Self {
        self.join = join;
        self
    }

    /// Sets the limit for miter joins beyond which a bevel will be generated.
    /// The default is 4.
    pub fn miter_limit(&mut self, limit: f32) -> &mut Self {
        self.miter_limit = limit;
        self
    }

    /// Sets the cap style that will be generated at the start and end of the
    /// stroke. Note that this will override the individual start and end cap
    /// options. The default is butt.
    pub fn cap(&mut self, cap: Cap) -> &mut Self {
        self.start_cap = cap;
        self.end_cap = cap;
        self
    }

    /// Sets both the start and end cap styles for the stroke.
    pub fn caps(&mut self, start: Cap, end: Cap) -> &mut Self {
        self.start_cap = start;
        self.end_cap = end;
        self
    }

    /// Sets the dash array and offset of the stroke. The default is an empty
    /// array, meaning that the stroke will be drawn as a continuous line.
    pub fn dash(&mut self, dashes: &'a [f32], offset: f32) -> &mut Self {
        self.dashes = dashes;
        self.offset = offset;
        self
    }

    /// Sets whether or not scaling is applied to the stroke. The default is true.
    pub fn scale(&mut self, scale: bool) -> &mut Self {
        self.scale = scale;
        self
    }
}

/// Represents the style of a path for rendering or hit testing.
#[derive(Copy, Clone, Debug)]
pub enum Style<'a> {
    Fill(Fill),
    Stroke(Stroke<'a>),
}

impl Default for Style<'_> {
    fn default() -> Self {
        Self::Fill(Fill::NonZero)
    }
}

impl Style<'_> {
    /// Returns true if the style is a stroke.
    pub fn is_stroke(&self) -> bool {
        matches!(self, Self::Stroke(_))
    }
}

impl From<Fill> for Style<'_> {
    fn from(style: Fill) -> Self {
        Self::Fill(style)
    }
}

impl<'a> From<Stroke<'a>> for Style<'a> {
    fn from(style: Stroke<'a>) -> Self {
        Self::Stroke(style)
    }
}

impl<'a> From<&'a Stroke<'a>> for Style<'a> {
    fn from(style: &'a Stroke<'a>) -> Self {
        Self::Stroke(*style)
    }
}

impl<'a> From<&'a mut Stroke<'a>> for Style<'a> {
    fn from(style: &'a mut Stroke<'a>) -> Self {
        Self::Stroke(*style)
    }
}
