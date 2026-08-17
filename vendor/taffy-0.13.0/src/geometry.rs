//! Geometric primitives useful for layout

use crate::util::sys::f32_max;
use crate::CompactLength;
use crate::{style::Dimension, util::sys::f32_min};
use core::ops::{Add, Sub};

#[cfg(feature = "flexbox")]
use crate::style::FlexDirection;

/// The simple absolute horizontal and vertical axis
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AbsoluteAxis {
    /// The horizontal axis
    Horizontal,
    /// The vertical axis
    Vertical,
}

impl AbsoluteAxis {
    /// Returns the other variant of the enum
    #[inline]
    pub const fn other_axis(&self) -> Self {
        match *self {
            AbsoluteAxis::Horizontal => AbsoluteAxis::Vertical,
            AbsoluteAxis::Vertical => AbsoluteAxis::Horizontal,
        }
    }
}

impl<T> Size<T> {
    #[inline(always)]
    /// Get either the width or height depending on the AbsoluteAxis passed in
    pub fn get_abs(self, axis: AbsoluteAxis) -> T {
        match axis {
            AbsoluteAxis::Horizontal => self.width,
            AbsoluteAxis::Vertical => self.height,
        }
    }
}

impl<T: Add> Rect<T> {
    #[inline(always)]
    /// Get either the width or height depending on the AbsoluteAxis passed in
    pub fn grid_axis_sum(self, axis: AbsoluteAxis) -> <T as Add>::Output {
        match axis {
            AbsoluteAxis::Horizontal => self.left + self.right,
            AbsoluteAxis::Vertical => self.top + self.bottom,
        }
    }
}

/// The CSS abstract axis
/// <https://www.w3.org/TR/css-writing-modes-3/#abstract-axes>
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AbstractAxis {
    /// The axis in the inline dimension, i.e. the horizontal axis in horizontal writing modes and the vertical axis in vertical writing modes.
    Inline,
    /// The axis in the block dimension, i.e. the vertical axis in horizontal writing modes and the horizontal axis in vertical writing modes.
    Block,
}

impl AbstractAxis {
    /// Returns the other variant of the enum
    #[inline]
    pub const fn other(&self) -> AbstractAxis {
        match *self {
            AbstractAxis::Inline => AbstractAxis::Block,
            AbstractAxis::Block => AbstractAxis::Inline,
        }
    }

    /// Convert an `AbstractAxis` into an `AbsoluteAxis` naively assuming that the Inline axis is Horizontal
    /// This is currently always true, but will change if Taffy ever implements the `writing_mode` property
    #[inline]
    pub const fn as_abs_naive(&self) -> AbsoluteAxis {
        match self {
            AbstractAxis::Inline => AbsoluteAxis::Horizontal,
            AbstractAxis::Block => AbsoluteAxis::Vertical,
        }
    }
}

/// Container that holds an item in each absolute axis without specifying
/// what kind of item it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct InBothAbsAxis<T> {
    /// The item in the horizontal axis
    pub horizontal: T,
    /// The item in the vertical axis
    pub vertical: T,
}

impl<T: Copy> InBothAbsAxis<T> {
    #[cfg(feature = "grid")]
    /// Get the contained item based on the AbsoluteAxis passed
    pub const fn get(&self, axis: AbsoluteAxis) -> T {
        match axis {
            AbsoluteAxis::Horizontal => self.horizontal,
            AbsoluteAxis::Vertical => self.vertical,
        }
    }
}

/// An axis-aligned UI rectangle
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect<T> {
    /// This can represent either the x-coordinate of the starting edge,
    /// or the amount of padding on the starting side.
    ///
    /// The starting edge is the left edge when working with LTR text,
    /// and the right edge when working with RTL text.
    pub left: T,
    /// This can represent either the x-coordinate of the ending edge,
    /// or the amount of padding on the ending side.
    ///
    /// The ending edge is the right edge when working with LTR text,
    /// and the left edge when working with RTL text.
    pub right: T,
    /// This can represent either the y-coordinate of the top edge,
    /// or the amount of padding on the top side.
    pub top: T,
    /// This can represent either the y-coordinate of the bottom edge,
    /// or the amount of padding on the bottom side.
    pub bottom: T,
}

impl<U, T: Add<U>> Add<Rect<U>> for Rect<T> {
    type Output = Rect<T::Output>;

    fn add(self, rhs: Rect<U>) -> Self::Output {
        Rect {
            left: self.left + rhs.left,
            right: self.right + rhs.right,
            top: self.top + rhs.top,
            bottom: self.bottom + rhs.bottom,
        }
    }
}

impl<T> Rect<T> {
    /// Applies the function `f` to all four sides of the rect
    ///
    /// When applied to the left and right sides, the width is used
    /// as the second parameter of `f`.
    /// When applied to the top or bottom sides, the height is used instead.
    #[cfg(any(feature = "flexbox", feature = "block_layout"))]
    pub(crate) fn zip_size<R, F, U>(self, size: Size<U>, f: F) -> Rect<R>
    where
        F: Fn(T, U) -> R,
        U: Copy,
    {
        Rect {
            left: f(self.left, size.width),
            right: f(self.right, size.width),
            top: f(self.top, size.height),
            bottom: f(self.bottom, size.height),
        }
    }

    /// Applies the function `f` to the left, right, top, and bottom properties
    ///
    /// This is used to transform a `Rect<T>` into a `Rect<R>`.
    pub fn map<R, F>(self, f: F) -> Rect<R>
    where
        F: Fn(T) -> R,
    {
        Rect { left: f(self.left), right: f(self.right), top: f(self.top), bottom: f(self.bottom) }
    }

    /// Returns a `Line<T>` representing the left and right properties of the Rect
    pub fn horizontal_components(self) -> Line<T> {
        Line { start: self.left, end: self.right }
    }

    /// Returns a `Line<T>` containing the top and bottom properties of the Rect
    pub fn vertical_components(self) -> Line<T> {
        Line { start: self.top, end: self.bottom }
    }
}

impl<T, U> Rect<T>
where
    T: Add<Output = U> + Copy + Clone,
{
    /// The sum of [`Rect.start`](Rect) and [`Rect.end`](Rect)
    ///
    /// This is typically used when computing total padding.
    ///
    /// **NOTE:** this is *not* the width of the rectangle.
    #[inline(always)]
    pub fn horizontal_axis_sum(&self) -> U {
        self.left + self.right
    }

    /// The sum of [`Rect.top`](Rect) and [`Rect.bottom`](Rect)
    ///
    /// This is typically used when computing total padding.
    ///
    /// **NOTE:** this is *not* the height of the rectangle.
    #[inline(always)]
    pub fn vertical_axis_sum(&self) -> U {
        self.top + self.bottom
    }

    /// Both horizontal_axis_sum and vertical_axis_sum as a `Size<T>`
    ///
    /// **NOTE:** this is *not* the width/height of the rectangle.
    #[inline(always)]
    #[allow(dead_code)] // Fixes spurious clippy warning: this function is used!
    pub fn sum_axes(&self) -> Size<U> {
        Size { width: self.horizontal_axis_sum(), height: self.vertical_axis_sum() }
    }

    /// The sum of the two fields of the [`Rect`] representing the main axis.
    ///
    /// This is typically used when computing total padding.
    ///
    /// If the [`FlexDirection`] is [`FlexDirection::Row`] or [`FlexDirection::RowReverse`], this is [`Rect::horizontal`].
    /// Otherwise, this is [`Rect::vertical`].
    #[cfg(feature = "flexbox")]
    pub(crate) fn main_axis_sum(&self, direction: FlexDirection) -> U {
        if direction.is_row() {
            self.horizontal_axis_sum()
        } else {
            self.vertical_axis_sum()
        }
    }

    /// The sum of the two fields of the [`Rect`] representing the cross axis.
    ///
    /// If the [`FlexDirection`] is [`FlexDirection::Row`] or [`FlexDirection::RowReverse`], this is [`Rect::vertical`].
    /// Otherwise, this is [`Rect::horizontal`].
    #[cfg(feature = "flexbox")]
    pub(crate) fn cross_axis_sum(&self, direction: FlexDirection) -> U {
        if direction.is_row() {
            self.vertical_axis_sum()
        } else {
            self.horizontal_axis_sum()
        }
    }
}

impl<T> Rect<T>
where
    T: Copy + Clone,
{
    /// The `start` or `top` value of the [`Rect`], from the perspective of the main layout axis
    #[cfg(feature = "flexbox")]
    pub(crate) const fn main_start(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.left
        } else {
            self.top
        }
    }

    /// The `end` or `bottom` value of the [`Rect`], from the perspective of the main layout axis
    #[cfg(feature = "flexbox")]
    pub(crate) const fn main_end(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.right
        } else {
            self.bottom
        }
    }

    /// The `start` or `top` value of the [`Rect`], from the perspective of the cross layout axis
    #[cfg(feature = "flexbox")]
    pub(crate) const fn cross_start(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.top
        } else {
            self.left
        }
    }

    /// The `end` or `bottom` value of the [`Rect`], from the perspective of the main layout axis
    #[cfg(feature = "flexbox")]
    pub(crate) const fn cross_end(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.bottom
        } else {
            self.right
        }
    }
}

impl Rect<f32> {
    /// Creates a new Rect with `0.0` as all parameters
    pub const ZERO: Rect<f32> = Self { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 };

    /// Creates a new Rect
    #[must_use]
    pub const fn new(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Self { left: start, right: end, top, bottom }
    }
}

/// An abstract "line". Represents any type that has a start and an end
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Line<T> {
    /// The start position of a line
    pub start: T,
    /// The end position of a line
    pub end: T,
}

impl<T> Line<T> {
    /// Applies the function `f` to both the width and height
    ///
    /// This is used to transform a `Line<T>` into a `Line<R>`.
    pub fn map<R, F>(self, f: F) -> Line<R>
    where
        F: Fn(T) -> R,
    {
        Line { start: f(self.start), end: f(self.end) }
    }
}

impl Line<bool> {
    /// A `Line<bool>` with both start and end set to `true`
    pub const TRUE: Self = Line { start: true, end: true };
    /// A `Line<bool>` with both start and end set to `false`
    pub const FALSE: Self = Line { start: false, end: false };
}

impl<T: Add + Copy> Line<T> {
    /// Adds the start and end values together and returns the result
    pub fn sum(&self) -> <T as Add>::Output {
        self.start + self.end
    }
}

/// The width and height of a [`Rect`]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Size<T> {
    /// The x extent of the rectangle
    pub width: T,
    /// The y extent of the rectangle
    pub height: T,
}

// Generic Add impl for Size<T> + Size<U> where T + U has an Add impl
impl<U, T: Add<U>> Add<Size<U>> for Size<T> {
    type Output = Size<<T as Add<U>>::Output>;

    fn add(self, rhs: Size<U>) -> Self::Output {
        Size { width: self.width + rhs.width, height: self.height + rhs.height }
    }
}

// Generic Sub impl for Size<T> + Size<U> where T + U has an Sub impl
impl<U, T: Sub<U>> Sub<Size<U>> for Size<T> {
    type Output = Size<<T as Sub<U>>::Output>;

    fn sub(self, rhs: Size<U>) -> Self::Output {
        Size { width: self.width - rhs.width, height: self.height - rhs.height }
    }
}

// Note: we allow dead_code here as we want to provide a complete API of helpers that is symmetrical in all axes,
// but sometimes we only currently have a use for the helper in a single axis
#[allow(dead_code)]
impl<T> Size<T> {
    /// Applies the function `f` to both the width and height
    ///
    /// This is used to transform a `Size<T>` into a `Size<R>`.
    pub fn map<R, F>(self, f: F) -> Size<R>
    where
        F: Fn(T) -> R,
    {
        Size { width: f(self.width), height: f(self.height) }
    }

    /// Applies the function `f` to the width
    pub fn map_width<F>(self, f: F) -> Size<T>
    where
        F: Fn(T) -> T,
    {
        Size { width: f(self.width), height: self.height }
    }

    /// Applies the function `f` to the height
    pub fn map_height<F>(self, f: F) -> Size<T>
    where
        F: Fn(T) -> T,
    {
        Size { width: self.width, height: f(self.height) }
    }

    /// Applies the function `f` to both the width and height
    /// of this value and another passed value
    pub fn zip_map<Other, Ret, Func>(self, other: Size<Other>, f: Func) -> Size<Ret>
    where
        Func: Fn(T, Other) -> Ret,
    {
        Size { width: f(self.width, other.width), height: f(self.height, other.height) }
    }

    /// Sets the extent of the main layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    #[cfg(feature = "flexbox")]
    pub(crate) fn set_main(&mut self, direction: FlexDirection, value: T) {
        if direction.is_row() {
            self.width = value
        } else {
            self.height = value
        }
    }

    /// Sets the extent of the cross layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    #[cfg(feature = "flexbox")]
    pub(crate) fn set_cross(&mut self, direction: FlexDirection, value: T) {
        if direction.is_row() {
            self.height = value
        } else {
            self.width = value
        }
    }

    /// Creates a new value of type Self with the main axis set to value provided
    ///
    /// Whether this is the width or height depends on the `direction` provided
    #[cfg(feature = "flexbox")]
    pub(crate) fn with_main(self, direction: FlexDirection, value: T) -> Self {
        let mut new = self;
        if direction.is_row() {
            new.width = value
        } else {
            new.height = value
        }
        new
    }

    /// Creates a new value of type Self with the cross axis set to value provided
    ///
    /// Whether this is the width or height depends on the `direction` provided
    #[cfg(feature = "flexbox")]
    pub(crate) fn with_cross(self, direction: FlexDirection, value: T) -> Self {
        let mut new = self;
        if direction.is_row() {
            new.height = value
        } else {
            new.width = value
        }
        new
    }

    /// Creates a new value of type Self with the main axis modified by the callback provided
    ///
    /// Whether this is the width or height depends on the `direction` provided
    #[cfg(feature = "flexbox")]
    pub(crate) fn map_main(self, direction: FlexDirection, mapper: impl FnOnce(T) -> T) -> Self {
        let mut new = self;
        if direction.is_row() {
            new.width = mapper(new.width);
        } else {
            new.height = mapper(new.height);
        }
        new
    }

    /// Creates a new value of type Self with the cross axis modified by the callback provided
    ///
    /// Whether this is the width or height depends on the `direction` provided
    #[cfg(feature = "flexbox")]
    pub(crate) fn map_cross(self, direction: FlexDirection, mapper: impl FnOnce(T) -> T) -> Self {
        let mut new = self;
        if direction.is_row() {
            new.height = mapper(new.height);
        } else {
            new.width = mapper(new.width);
        }
        new
    }

    /// Gets the extent of the main layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    #[cfg(feature = "flexbox")]
    pub(crate) fn main(self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.width
        } else {
            self.height
        }
    }

    /// Gets the extent of the cross layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    #[cfg(feature = "flexbox")]
    pub(crate) fn cross(self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.height
        } else {
            self.width
        }
    }

    /// Gets the extent of the specified layout axis
    /// Whether this is the width or height depends on the `GridAxis` provided
    #[cfg(feature = "grid")]
    pub(crate) fn get(self, axis: AbstractAxis) -> T {
        match axis {
            AbstractAxis::Inline => self.width,
            AbstractAxis::Block => self.height,
        }
    }

    /// Sets the extent of the specified layout axis
    /// Whether this is the width or height depends on the `GridAxis` provided
    #[cfg(feature = "grid")]
    pub(crate) fn set(&mut self, axis: AbstractAxis, value: T) {
        match axis {
            AbstractAxis::Inline => self.width = value,
            AbstractAxis::Block => self.height = value,
        }
    }

    /// Sets the extent of the specified layout axis
    /// Whether this is the width or height depends on the `GridAxis` provided
    #[cfg(feature = "grid")]
    pub(crate) fn with(mut self, axis: AbstractAxis, value: T) -> Self {
        match axis {
            AbstractAxis::Inline => self.width = value,
            AbstractAxis::Block => self.height = value,
        }
        self
    }
}

impl Size<f32> {
    /// A [`Size`] with zero width and height
    pub const ZERO: Size<f32> = Self { width: 0.0, height: 0.0 };

    /// Applies f32_max to each component separately
    #[inline(always)]
    pub fn f32_max(self, rhs: Size<f32>) -> Size<f32> {
        Size { width: f32_max(self.width, rhs.width), height: f32_max(self.height, rhs.height) }
    }

    /// Applies f32_min to each component separately
    #[inline(always)]
    pub fn f32_min(self, rhs: Size<f32>) -> Size<f32> {
        Size { width: f32_min(self.width, rhs.width), height: f32_min(self.height, rhs.height) }
    }

    /// Return true if both width and height are greater than 0 else false
    #[inline(always)]
    pub fn has_non_zero_area(self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }
}

impl Size<Option<f32>> {
    /// A [`Size`] with `None` width and height
    pub const NONE: Size<Option<f32>> = Self { width: None, height: None };

    /// A [`Size<Option<f32>>`] with `Some(width)` and `Some(height)` as parameters
    #[must_use]
    pub const fn new(width: f32, height: f32) -> Self {
        Size { width: Some(width), height: Some(height) }
    }

    /// Creates a new [`Size<Option<f32>>`] with either the width or height set based on the provided `direction`
    #[cfg(feature = "flexbox")]
    pub const fn from_cross(direction: FlexDirection, value: Option<f32>) -> Self {
        let mut new = Self::NONE;
        if direction.is_row() {
            new.height = value
        } else {
            new.width = value
        }
        new
    }

    /// Applies aspect_ratio (if one is supplied) to the Size:
    ///   - If width is `Some` but height is `None`, then height is computed from width and aspect_ratio
    ///   - If height is `Some` but width is `None`, then width is computed from height and aspect_ratio
    ///
    /// If aspect_ratio is `None` then this function simply returns self.
    pub fn maybe_apply_aspect_ratio(self, aspect_ratio: Option<f32>) -> Size<Option<f32>> {
        match aspect_ratio {
            Some(ratio) => match (self.width, self.height) {
                (Some(width), None) => Size { width: Some(width), height: Some(width / ratio) },
                (None, Some(height)) => Size { width: Some(height * ratio), height: Some(height) },
                _ => self,
            },
            None => self,
        }
    }
}

impl<T> Size<Option<T>> {
    /// Performs Option::unwrap_or on each component separately
    pub fn unwrap_or(self, alt: Size<T>) -> Size<T> {
        Size { width: self.width.unwrap_or(alt.width), height: self.height.unwrap_or(alt.height) }
    }

    /// Performs Option::or on each component separately
    pub fn or(self, alt: Size<Option<T>>) -> Size<Option<T>> {
        Size { width: self.width.or(alt.width), height: self.height.or(alt.height) }
    }

    /// Return true if both components are Some, else false.
    #[inline(always)]
    pub fn both_axis_defined(&self) -> bool {
        self.width.is_some() && self.height.is_some()
    }
}

impl Size<Dimension> {
    /// Generates a [`Size<Dimension>`] using length values
    #[must_use]
    pub const fn from_lengths(width: f32, height: f32) -> Self {
        Size { width: Dimension(CompactLength::length(width)), height: Dimension(CompactLength::length(height)) }
    }

    /// Generates a [`Size<Dimension>`] using percentage values
    #[must_use]
    pub const fn from_percent(width: f32, height: f32) -> Self {
        Size { width: Dimension(CompactLength::percent(width)), height: Dimension(CompactLength::percent(height)) }
    }
}

/// A 2-dimensional coordinate.
///
/// When used in association with a [`Rect`], represents the top-left corner.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point<T> {
    /// The x-coordinate
    pub x: T,
    /// The y-coordinate
    pub y: T,
}

impl Point<f32> {
    /// A [`Point`] with values (0,0), representing the origin
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
}

impl Point<Option<f32>> {
    /// A [`Point`] with values (None, None)
    pub const NONE: Self = Self { x: None, y: None };
}

// Generic Add impl for Point<T> + Point<U> where T + U has an Add impl
impl<U, T: Add<U>> Add<Point<U>> for Point<T> {
    type Output = Point<<T as Add<U>>::Output>;

    fn add(self, rhs: Point<U>) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T> Point<T> {
    /// Applies the function `f` to both the x and y
    ///
    /// This is used to transform a `Point<T>` into a `Point<R>`.
    pub fn map<R, F>(self, f: F) -> Point<R>
    where
        F: Fn(T) -> R,
    {
        Point { x: f(self.x), y: f(self.y) }
    }

    /// Gets the extent of the specified layout axis
    /// Whether this is the width or height depends on the `GridAxis` provided
    #[cfg(feature = "grid")]
    pub fn get(self, axis: AbstractAxis) -> T {
        match axis {
            AbstractAxis::Inline => self.x,
            AbstractAxis::Block => self.y,
        }
    }

    /// Swap x and y components
    pub fn transpose(self) -> Point<T> {
        Point { x: self.y, y: self.x }
    }

    /// Sets the extent of the specified layout axis
    /// Whether this is the width or height depends on the `GridAxis` provided
    #[cfg(feature = "grid")]
    pub fn set(&mut self, axis: AbstractAxis, value: T) {
        match axis {
            AbstractAxis::Inline => self.x = value,
            AbstractAxis::Block => self.y = value,
        }
    }

    /// Gets the component in the main layout axis
    ///
    /// Whether this is the x or y depends on the `direction` provided
    #[cfg(feature = "flexbox")]
    pub(crate) fn main(self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.x
        } else {
            self.y
        }
    }

    /// Gets the component in the cross layout axis
    ///
    /// Whether this is the x or y depends on the `direction` provided
    #[cfg(feature = "flexbox")]
    pub(crate) fn cross(self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.y
        } else {
            self.x
        }
    }
}

impl<T> From<Point<T>> for Size<T> {
    fn from(value: Point<T>) -> Self {
        Size { width: value.x, height: value.y }
    }
}

/// Generic struct which holds a "min" value and a "max" value
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MinMax<Min, Max> {
    /// The value representing the minimum
    pub min: Min,
    /// The value representing the maximum
    pub max: Max,
}
