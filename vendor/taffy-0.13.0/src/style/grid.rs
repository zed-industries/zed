//! Style types for CSS Grid layout
use super::{
    AlignContent, AlignItems, AlignSelf, CheapCloneStr, CompactLength, CoreStyle, Dimension, JustifyContent,
    LengthPercentage, LengthPercentageAuto, Style,
};
use crate::compute::grid::{GridCoordinate, GridLine, OriginZeroLine, MAX_GRID_TRACKS};
use crate::geometry::{AbsoluteAxis, AbstractAxis, Line, MinMax, Size};
use crate::style_helpers::*;
use crate::sys::{DefaultCheapStr, Vec};
use core::cmp::{max, min};
use core::fmt::Debug;

#[cfg(feature = "parse")]
use crate::util::parse::{
    from_str_from_css, parse_css_str_entirely, CssParseResult, FromCss, ParseError, Parser, Token,
};

/// Defines the value of the `grid-template-areas` property: the named areas plus the overall
/// size (in tracks) of the area template.
///
/// The template may be larger than the extents of the named areas due to unnamed (`.`) cells,
/// so the size is stored explicitly.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridTemplateAreas<CustomIdent: CheapCloneStr> {
    /// The named grid areas
    pub areas: crate::util::sys::GridTrackVec<GridTemplateArea<CustomIdent>>,
    /// The number of rows in the area template
    pub row_count: u16,
    /// The number of columns in the area template
    pub column_count: u16,
}

/// Defines a grid area
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridTemplateArea<CustomIdent: CheapCloneStr> {
    /// The name of the grid area which
    #[cfg_attr(feature = "serde", serde(deserialize_with = "crate::util::deserialize_from_str"))]
    pub name: CustomIdent,
    /// The index of the row at which the grid area starts in grid coordinates.
    pub row_start: u16,
    /// The index of the row at which the grid area ends in grid coordinates.
    pub row_end: u16,
    /// The index of the column at which the grid area starts in grid coordinates.
    pub column_start: u16,
    /// The index of the column at which the grid area end in grid coordinates.
    pub column_end: u16,
}

/// Defines a named grid line
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NamedGridLine<CustomIdent: CheapCloneStr> {
    /// The name of the grid area which
    #[cfg_attr(feature = "serde", serde(deserialize_with = "crate::util::deserialize_from_str"))]
    pub name: CustomIdent,
    /// The index of the row at which the grid area starts in grid coordinates.
    pub index: u16,
}

/// Axis as `Row` or `Column`
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum GridAreaAxis {
    /// The `Row` axis
    Row,
    /// The `Column` axis
    Column,
}

/// Logical end (`Start` or `End`)
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum GridAreaEnd {
    /// The `Start` end
    Start,
    /// The `End` end
    End,
}

/// A trait to represent a `repeat()` clause in a `grid-template-*` definition
pub trait GenericRepetition {
    /// The type that represents `<custom-ident>`s (for named lines)
    type CustomIdent: CheapCloneStr;
    /// The type which represents an iterator over the list of repeated tracks
    type RepetitionTrackList<'a>: Iterator<Item = TrackSizingFunction> + ExactSizeIterator + Clone
    where
        Self: 'a;

    /// A nested iterator of line names (nested because each line may have multiple associated names)
    type TemplateLineNames<'a>: TemplateLineNames<'a, Self::CustomIdent>
    where
        Self: 'a;
    /// The repetition count (integer, auto-fill, or auto-fit)
    fn count(&self) -> RepetitionCount;
    /// Get an iterator over the repeated tracks
    fn tracks(&self) -> Self::RepetitionTrackList<'_>;
    /// Returns the number of repeated tracks
    fn track_count(&self) -> u16 {
        self.tracks().len().min(u16::MAX as usize) as u16
    }
    /// Returns an iterator over the lines names
    fn lines_names(&self) -> Self::TemplateLineNames<'_>;
}

/// A nested list of line names. This is effectively a generic representation of `Vec<Vec<String>>` that allows
/// both the collection and string type to be customised.
#[rustfmt::skip]
pub trait TemplateLineNames<'a, S: CheapCloneStr> : Iterator<Item = Self::LineNameSet<'a>> + ExactSizeIterator + Clone where Self: 'a {
    /// A simple list line names. This is effectively a generic representation of `VecString>` that allows
    /// both the collection and string type to be customised.
    type LineNameSet<'b>: Iterator<Item = &'b S> + ExactSizeIterator + Clone where Self: 'b;
}

impl<'a, S: CheapCloneStr> TemplateLineNames<'a, S>
    for core::iter::Map<core::slice::Iter<'a, Vec<S>>, fn(&Vec<S>) -> core::slice::Iter<'_, S>>
{
    type LineNameSet<'b>
        = core::slice::Iter<'b, S>
    where
        Self: 'b;
}

#[derive(Copy, Clone)]
/// A type representing a component in a `grid-template-*` defintion where the type
/// representing `repeat()`s is generic
pub enum GenericGridTemplateComponent<S, Repetition>
where
    S: CheapCloneStr,
    Repetition: GenericRepetition<CustomIdent = S>,
{
    /// A single track sizing function
    Single(TrackSizingFunction),
    /// A `repeat()`
    Repeat(Repetition),
}

impl<S, Repetition> GenericGridTemplateComponent<S, Repetition>
where
    S: CheapCloneStr,
    Repetition: GenericRepetition<CustomIdent = S>,
{
    /// Whether the track definition is a auto-repeated fragment
    pub fn is_auto_repetition(&self) -> bool {
        match self {
            Self::Single(_) => false,
            Self::Repeat(repeat) => matches!(repeat.count(), RepetitionCount::AutoFit | RepetitionCount::AutoFill),
        }
    }
}

/// The set of styles required for a CSS Grid container
pub trait GridContainerStyle: CoreStyle {
    /// The type for a `repeat()` within a grid_template_rows or grid_template_columns
    type Repetition<'a>: GenericRepetition<CustomIdent = Self::CustomIdent>
    where
        Self: 'a;

    /// The type returned by grid_template_rows and grid_template_columns
    type TemplateTrackList<'a>: Iterator<Item = GenericGridTemplateComponent<Self::CustomIdent, Self::Repetition<'a>>>
        + ExactSizeIterator
        + Clone
    where
        Self: 'a;

    /// The type returned by grid_auto_rows and grid_auto_columns
    type AutoTrackList<'a>: Iterator<Item = TrackSizingFunction> + ExactSizeIterator + Clone
    where
        Self: 'a;

    /// The type returned by grid_template_row_names and grid_template_column_names
    //IntoIterator<Item = &'a Self::LineNameSet<'a>>
    type TemplateLineNames<'a>: TemplateLineNames<'a, Self::CustomIdent>
    where
        Self: 'a;

    /// The type of custom identifiers used to identify named grid lines and areas
    type GridTemplateAreas<'a>: IntoIterator<Item = GridTemplateArea<Self::CustomIdent>>
    where
        Self: 'a;

    // FIXME: re-add default implemenations for grid_{template,auto}_{rows,columns} once the
    // associated_type_defaults feature (https://github.com/rust-lang/rust/issues/29661) is stabilised.

    /// Defines the track sizing functions (heights) of the grid rows
    fn grid_template_rows(&self) -> Option<Self::TemplateTrackList<'_>>;
    /// Defines the track sizing functions (widths) of the grid columns
    fn grid_template_columns(&self) -> Option<Self::TemplateTrackList<'_>>;
    /// Defines the size of implicitly created rows
    fn grid_auto_rows(&self) -> Self::AutoTrackList<'_>;
    /// Defined the size of implicitly created columns
    fn grid_auto_columns(&self) -> Self::AutoTrackList<'_>;

    /// Named grid areas
    fn grid_template_areas(&self) -> Option<Self::GridTemplateAreas<'_>>;
    /// The number of rows in the `grid-template-areas` template (0 if there is no template).
    /// May be greater than the extent of the named areas due to unnamed (`.`) cells.
    fn grid_template_area_row_count(&self) -> u16 {
        self.grid_template_areas()
            .map(|areas| areas.into_iter().map(|area| area.row_end.max(1) - 1).max().unwrap_or(0))
            .unwrap_or(0)
    }
    /// The number of columns in the `grid-template-areas` template (0 if there is no template).
    /// May be greater than the extent of the named areas due to unnamed (`.`) cells.
    fn grid_template_area_column_count(&self) -> u16 {
        self.grid_template_areas()
            .map(|areas| areas.into_iter().map(|area| area.column_end.max(1) - 1).max().unwrap_or(0))
            .unwrap_or(0)
    }
    /// Defines the line names for row lines
    fn grid_template_column_names(&self) -> Option<Self::TemplateLineNames<'_>>;
    /// Defines the size of implicitly created rows
    fn grid_template_row_names(&self) -> Option<Self::TemplateLineNames<'_>>;

    /// Controls how items get placed into the grid for auto-placed items
    #[inline(always)]
    fn grid_auto_flow(&self) -> GridAutoFlow {
        Style::<Self::CustomIdent>::DEFAULT.grid_auto_flow
    }

    /// How large should the gaps between items in a grid or flex container be?
    #[inline(always)]
    fn gap(&self) -> Size<LengthPercentage> {
        Style::<Self::CustomIdent>::DEFAULT.gap
    }

    // Alignment properties

    /// How should content contained within this item be aligned in the cross/block axis
    #[inline(always)]
    fn align_content(&self) -> Option<AlignContent> {
        Style::<Self::CustomIdent>::DEFAULT.align_content
    }
    /// How should contained within this item be aligned in the main/inline axis
    #[inline(always)]
    fn justify_content(&self) -> Option<JustifyContent> {
        Style::<Self::CustomIdent>::DEFAULT.justify_content
    }
    /// How this node's children aligned in the cross/block axis?
    #[inline(always)]
    fn align_items(&self) -> Option<AlignItems> {
        Style::<Self::CustomIdent>::DEFAULT.align_items
    }
    /// How this node's children should be aligned in the inline axis
    #[inline(always)]
    fn justify_items(&self) -> Option<AlignItems> {
        Style::<Self::CustomIdent>::DEFAULT.justify_items
    }

    /// Get a grid item's row or column placement depending on the axis passed
    #[inline(always)]
    fn grid_template_tracks(&self, axis: AbsoluteAxis) -> Option<Self::TemplateTrackList<'_>> {
        match axis {
            AbsoluteAxis::Horizontal => self.grid_template_columns(),
            AbsoluteAxis::Vertical => self.grid_template_rows(),
        }
    }

    /// Get a grid container's align-content or justify-content alignment depending on the axis passed
    #[inline(always)]
    fn grid_align_content(&self, axis: AbstractAxis) -> AlignContent {
        match axis {
            AbstractAxis::Inline => self.justify_content().unwrap_or(AlignContent::STRETCH),
            AbstractAxis::Block => self.align_content().unwrap_or(AlignContent::STRETCH),
        }
    }
}

/// The set of styles required for a CSS Grid item (child of a CSS Grid container)
pub trait GridItemStyle: CoreStyle {
    /// Defines which row in the grid the item should start and end at
    #[inline(always)]
    fn grid_row(&self) -> Line<GridPlacement<Self::CustomIdent>> {
        Default::default()
    }
    /// Defines which column in the grid the item should start and end at
    #[inline(always)]
    fn grid_column(&self) -> Line<GridPlacement<Self::CustomIdent>> {
        Default::default()
    }

    /// How this node should be aligned in the cross/block axis
    /// Falls back to the parents [`AlignItems`] if not set
    #[inline(always)]
    fn align_self(&self) -> Option<AlignSelf> {
        Style::<Self::CustomIdent>::DEFAULT.align_self
    }
    /// How this node should be aligned in the inline axis
    /// Falls back to the parents [`super::JustifyItems`] if not set
    #[inline(always)]
    fn justify_self(&self) -> Option<AlignSelf> {
        Style::<Self::CustomIdent>::DEFAULT.justify_self
    }

    /// Get a grid item's row or column placement depending on the axis passed
    #[inline(always)]
    fn grid_placement(&self, axis: AbsoluteAxis) -> Line<GridPlacement<Self::CustomIdent>> {
        match axis {
            AbsoluteAxis::Horizontal => self.grid_column(),
            AbsoluteAxis::Vertical => self.grid_row(),
        }
    }
}

/// Controls whether grid items are placed row-wise or column-wise. And whether the sparse or dense packing algorithm is used.
///
/// The "dense" packing algorithm attempts to fill in holes earlier in the grid, if smaller items come up later. This may cause items to appear out-of-order, when doing so would fill in holes left by larger items.
///
/// Defaults to [`GridAutoFlow::Row`]
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow)
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GridAutoFlow {
    /// Items are placed by filling each row in turn, adding new rows as necessary
    #[default]
    Row,
    /// Items are placed by filling each column in turn, adding new columns as necessary.
    Column,
    /// Combines `Row` with the dense packing algorithm.
    RowDense,
    /// Combines `Column` with the dense packing algorithm.
    ColumnDense,
}

#[cfg(feature = "parse")]
impl FromCss for GridAutoFlow {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        let mut axis: Option<&'static str> = None;
        let mut dense = false;

        for _ in 0..2 {
            if let Ok(ident) = parser.try_parse(|parser| parser.expect_ident_cloned()) {
                match &*ident {
                    "row" => {
                        axis = Some("row");
                    }
                    "column" => {
                        axis = Some("column");
                    }
                    "dense" => dense = true,
                    _ => {
                        return Err(parser.new_unexpected_token_error(Token::Ident(ident)));
                    }
                }
            } else {
                break;
            }
        }

        match (axis, dense) {
            (Some("row"), false) => Ok(Self::Row),
            (Some("row") | None, true) => Ok(Self::RowDense),
            (Some("column"), false) => Ok(Self::Column),
            (Some("column"), true) => Ok(Self::ColumnDense),
            (None, false) => {
                let token = parser.next().cloned()?;
                Err(parser.new_unexpected_token_error(token))
            }
            _ => unreachable!(),
        }
    }
}
#[cfg(feature = "parse")]
from_str_from_css!(GridAutoFlow);

impl GridAutoFlow {
    /// Whether grid auto placement uses the sparse placement algorithm or the dense placement algorithm
    /// See: <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow#values>
    pub const fn is_dense(&self) -> bool {
        match self {
            Self::Row | Self::Column => false,
            Self::RowDense | Self::ColumnDense => true,
        }
    }

    /// Whether grid auto placement fills areas row-wise or column-wise
    /// See: <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow#values>
    pub const fn primary_axis(&self) -> AbsoluteAxis {
        match self {
            Self::Row | Self::RowDense => AbsoluteAxis::Horizontal,
            Self::Column | Self::ColumnDense => AbsoluteAxis::Vertical,
        }
    }
}

/// A grid line placement specification which is generic over the coordinate system that it uses to define
/// grid line positions.
///
/// `GenericGridPlacement<GridLine>` is aliased as GridPlacement and is exposed to users of Taffy to define styles.
/// `GenericGridPlacement<OriginZeroLine>` is aliased as OriginZeroGridPlacement and is used internally for placement computations.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GenericGridPlacement<LineType: GridCoordinate> {
    /// Place item according to the auto-placement algorithm, and the parent's grid_auto_flow property
    Auto,
    /// Place item at specified line (column or row) index
    Line(LineType),
    /// Item should span specified number of tracks (columns or rows)
    Span(u16),
}

/// A grid line placement using the normalized OriginZero coordinates to specify line positions.
pub(crate) type OriginZeroGridPlacement = GenericGridPlacement<OriginZeroLine>;

/// A grid line placement using CSS grid line coordinates to specify line positions. This uses the same coordinate
/// system as the public `GridPlacement` type but doesn't support named lines (these are expected to have already
/// been resolved by the time values of this type are constructed).
pub(crate) type NonNamedGridPlacement = GenericGridPlacement<GridLine>;

/// A grid line placement specification. Used for grid-[row/column]-[start/end]. Named tracks are not implemented.
///
/// Defaults to `GridPlacement::Auto`
///
/// [Specification](https://www.w3.org/TR/css3-grid-layout/#typedef-grid-row-start-grid-line)
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GridPlacement<S: CheapCloneStr = DefaultCheapStr> {
    /// Place item according to the auto-placement algorithm, and the parent's grid_auto_flow property
    #[default]
    Auto,
    /// Place item at specified line (column or row) index
    Line(GridLine),
    /// Place item at specified named line (column or row)
    NamedLine(S, i16),
    /// Item should span specified number of tracks (columns or rows)
    Span(u16),
    /// Item should span until the nth line named `<name>`.
    ///
    /// If there are less than n lines named `<name>` in the specified direction then
    /// all implicit lines will be counted.
    NamedSpan(S, u16),
}
impl<S: CheapCloneStr> TaffyAuto for GridPlacement<S> {
    const AUTO: Self = Self::Auto;
}
impl<S: CheapCloneStr> TaffyGridLine for GridPlacement<S> {
    fn from_line_index(index: i16) -> Self {
        GridPlacement::<S>::Line(GridLine::from(index))
    }
}
impl<S: CheapCloneStr> TaffyGridLine for Line<GridPlacement<S>> {
    fn from_line_index(index: i16) -> Self {
        Line { start: GridPlacement::<S>::from_line_index(index), end: GridPlacement::<S>::Auto }
    }
}
impl<S: CheapCloneStr> TaffyGridSpan for GridPlacement<S> {
    fn from_span(span: u16) -> Self {
        GridPlacement::<S>::Span(span)
    }
}
impl<S: CheapCloneStr> TaffyGridSpan for Line<GridPlacement<S>> {
    fn from_span(span: u16) -> Self {
        Line { start: GridPlacement::<S>::from_span(span), end: GridPlacement::<S>::Auto }
    }
}

#[cfg(feature = "parse")]
/// Saturates an `i32` to the range representable by `i16`.
fn saturating_i16(value: i32) -> i16 {
    value.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

#[cfg(feature = "parse")]
/// Saturates an `i32` to the range representable by `u16`.
fn saturating_u16(value: i32) -> u16 {
    value.clamp(u16::MIN as i32, u16::MAX as i32) as u16
}

#[cfg(feature = "parse")]
impl<S: CheapCloneStr> FromCss for GridPlacement<S> {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        let mut span = false;
        let mut number = None;
        let mut ident = None;

        while !parser.is_exhausted() {
            let token = parser.next()?.clone();
            match &token {
                Token::Ident(s) => match s.as_ref() {
                    "auto" => {
                        if span || number.is_some() || ident.is_some() {
                            return Err(parser.new_unexpected_token_error(token));
                        }
                        parser.expect_exhausted()?;
                        return Ok(Self::Auto);
                    }
                    "span" => {
                        if span {
                            return Err(parser.new_unexpected_token_error(token));
                        }
                        span = true;
                    }
                    other => {
                        if ident.is_some() {
                            return Err(parser.new_unexpected_token_error(token));
                        }
                        ident = Some(S::from(other));
                    }
                },
                Token::Number { int_value: Some(value), .. } if *value != 0 => {
                    if number.is_some() {
                        return Err(parser.new_unexpected_token_error(token));
                    }
                    number = Some(*value);
                }
                _ => return Err(parser.new_unexpected_token_error(token)),
            };
        }

        match (span, number, ident) {
            (true, None, None) => Ok(Self::Span(0)),
            (true, Some(number), None) => Ok(Self::Span(saturating_u16(number))),
            (true, None, Some(ident)) => Ok(Self::NamedSpan(ident, 0)),
            (true, Some(number), Some(ident)) => Ok(Self::NamedSpan(ident, saturating_u16(number))),
            (false, Some(number), None) => Ok(Self::Line(GridLine::from(saturating_i16(number)))),
            (false, Some(number), Some(ident)) => Ok(Self::NamedLine(ident, saturating_i16(number))),
            (false, None, Some(ident)) => Ok(Self::NamedLine(ident, 0)),
            (false, None, None) => Err(parser.new_error(cssparser::BasicParseErrorKind::EndOfInput)),
        }
    }
}

#[cfg(feature = "parse")]
impl<S: CheapCloneStr> core::str::FromStr for GridPlacement<S> {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_css_str_entirely(input)
    }
}

impl<S: CheapCloneStr> GridPlacement<S> {
    /// Apply a mapping function if the [`GridPlacement`] is a `Line`. Otherwise return `self` unmodified.
    pub fn into_origin_zero_placement_ignoring_named(&self, explicit_track_count: u16) -> OriginZeroGridPlacement {
        match self {
            Self::Auto => OriginZeroGridPlacement::Auto,
            // Spans are clamped to the maximum track limit
            // https://www.w3.org/TR/css-grid-1/#overlarge-grids
            Self::Span(span) => OriginZeroGridPlacement::Span(min(*span, MAX_GRID_TRACKS)),
            // Grid line zero is an invalid index, so it gets treated as Auto
            // See: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row-start#values
            Self::Line(line) => match line.as_i16() {
                0 => OriginZeroGridPlacement::Auto,
                _ => OriginZeroGridPlacement::Line(line.into_origin_zero_line(explicit_track_count)),
            },
            Self::NamedLine(_, _) => OriginZeroGridPlacement::Auto,
            Self::NamedSpan(_, _) => OriginZeroGridPlacement::Auto,
        }
    }
}

impl<S: CheapCloneStr> Line<GridPlacement<S>> {
    /// Apply a mapping function if the [`GridPlacement`] is a `Line`. Otherwise return `self` unmodified.
    pub fn into_origin_zero_ignoring_named(&self, explicit_track_count: u16) -> Line<OriginZeroGridPlacement> {
        Line {
            start: self.start.into_origin_zero_placement_ignoring_named(explicit_track_count),
            end: self.end.into_origin_zero_placement_ignoring_named(explicit_track_count),
        }
    }
}

impl NonNamedGridPlacement {
    /// Apply a mapping function if the [`GridPlacement`] is a `Track`. Otherwise return `self` unmodified.
    pub fn into_origin_zero_placement(
        &self,
        explicit_track_count: u16,
        // resolve_named: impl Fn(&str) -> Option<GridLine>
    ) -> OriginZeroGridPlacement {
        match self {
            Self::Auto => OriginZeroGridPlacement::Auto,
            // Spans are clamped to the maximum track limit
            // https://www.w3.org/TR/css-grid-1/#overlarge-grids
            Self::Span(span) => OriginZeroGridPlacement::Span(min(*span, MAX_GRID_TRACKS)),
            // Grid line zero is an invalid index, so it gets treated as Auto
            // See: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row-start#values
            Self::Line(line) => match line.as_i16() {
                0 => OriginZeroGridPlacement::Auto,
                _ => OriginZeroGridPlacement::Line(line.into_origin_zero_line(explicit_track_count)),
            },
        }
    }
}

impl<T: GridCoordinate> Line<GenericGridPlacement<T>> {
    /// Resolves the span for an indefinite placement (a placement that does not consist of two `Track`s).
    /// Panics if called on a definite placement
    pub const fn indefinite_span(&self) -> u16 {
        use GenericGridPlacement as GP;
        match (self.start, self.end) {
            (GP::Line(_), GP::Auto) => 1,
            (GP::Auto, GP::Line(_)) => 1,
            (GP::Auto, GP::Auto) => 1,
            (GP::Line(_), GP::Span(span)) => span,
            (GP::Span(span), GP::Line(_)) => span,
            (GP::Span(span), GP::Auto) => span,
            (GP::Auto, GP::Span(span)) => span,
            (GP::Span(span), GP::Span(_)) => span,
            (GP::Line(_), GP::Line(_)) => panic!("indefinite_span should only be called on indefinite grid tracks"),
        }
    }
}

impl<S: CheapCloneStr> Line<GridPlacement<S>> {
    #[inline]
    /// Whether the track position is definite in this axis (or the item will need auto placement)
    /// The track position is definite if least one of the start and end positions is a NON-ZERO track index
    /// (0 is an invalid line in GridLine coordinates, and falls back to "auto" which is indefinite)
    pub fn is_definite(&self) -> bool {
        match (&self.start, &self.end) {
            (GridPlacement::Line(line), _) if line.as_i16() != 0 => true,
            (_, GridPlacement::Line(line)) if line.as_i16() != 0 => true,
            (GridPlacement::NamedLine(_, _), _) => true,
            (_, GridPlacement::NamedLine(_, _)) => true,
            _ => false,
        }
    }
}

impl Line<NonNamedGridPlacement> {
    #[inline]
    /// Whether the track position is definite in this axis (or the item will need auto placement)
    /// The track position is definite if least one of the start and end positions is a NON-ZERO track index
    /// (0 is an invalid line in GridLine coordinates, and falls back to "auto" which is indefinite)
    pub fn is_definite(&self) -> bool {
        match (&self.start, &self.end) {
            (GenericGridPlacement::Line(line), _) if line.as_i16() != 0 => true,
            (_, GenericGridPlacement::Line(line)) if line.as_i16() != 0 => true,
            _ => false,
        }
    }

    /// Apply a mapping function if the [`GridPlacement`] is a `Track`. Otherwise return `self` unmodified.
    pub fn into_origin_zero(&self, explicit_track_count: u16) -> Line<OriginZeroGridPlacement> {
        Line {
            start: self.start.into_origin_zero_placement(explicit_track_count),
            end: self.end.into_origin_zero_placement(explicit_track_count),
        }
    }
}

impl Line<OriginZeroGridPlacement> {
    #[inline]
    /// Whether the track position is definite in this axis (or the item will need auto placement)
    /// The track position is definite if least one of the start and end positions is a track index
    pub const fn is_definite(&self) -> bool {
        matches!((self.start, self.end), (GenericGridPlacement::Line(_), _) | (_, GenericGridPlacement::Line(_)))
    }

    /// If at least one of the of the start and end positions is a track index then the other end can be resolved
    /// into a track index purely based on the information contained with the placement specification
    pub fn resolve_definite_grid_lines(&self) -> Line<OriginZeroLine> {
        use OriginZeroGridPlacement as GP;
        match (self.start, self.end) {
            (GP::Line(line1), GP::Line(line2)) => {
                if line1 == line2 {
                    Line { start: line1, end: line1 + 1 }
                } else {
                    Line { start: min(line1, line2), end: max(line1, line2) }
                }
            }
            (GP::Line(line), GP::Span(span)) => Line { start: line, end: line + span },
            (GP::Line(line), GP::Auto) => Line { start: line, end: line + 1 },
            (GP::Span(span), GP::Line(line)) => Line { start: line - span, end: line },
            (GP::Auto, GP::Line(line)) => Line { start: line - 1, end: line },
            _ => panic!("resolve_definite_grid_tracks should only be called on definite grid tracks"),
        }
    }

    /// For absolutely positioned items:
    ///   - Tracks resolve to definite tracks
    ///   - For Spans:
    ///      - If the other position is a Track, they resolve to a definite track relative to the other track
    ///      - Else resolve to None
    ///   - Auto resolves to None
    ///
    /// When finally positioning the item, a value of None means that the item's grid area is bounded by the grid
    /// container's border box on that side.
    pub fn resolve_absolutely_positioned_grid_tracks(&self) -> Line<Option<OriginZeroLine>> {
        use OriginZeroGridPlacement as GP;
        match (self.start, self.end) {
            (GP::Line(track1), GP::Line(track2)) => {
                if track1 == track2 {
                    Line { start: Some(track1), end: Some(track1 + 1) }
                } else {
                    Line { start: Some(min(track1, track2)), end: Some(max(track1, track2)) }
                }
            }
            (GP::Line(track), GP::Span(span)) => Line { start: Some(track), end: Some(track + span) },
            (GP::Line(track), GP::Auto) => Line { start: Some(track), end: None },
            (GP::Span(span), GP::Line(track)) => Line { start: Some(track - span), end: Some(track) },
            (GP::Auto, GP::Line(track)) => Line { start: None, end: Some(track) },
            _ => Line { start: None, end: None },
        }
    }

    /// If neither of the start and end positions is a track index then the other end can be resolved
    /// into a track index if a definite start position is supplied externally
    pub fn resolve_indefinite_grid_tracks(&self, start: OriginZeroLine) -> Line<OriginZeroLine> {
        use OriginZeroGridPlacement as GP;
        match (self.start, self.end) {
            (GP::Auto, GP::Auto) => Line { start, end: start + 1 },
            (GP::Span(span), GP::Auto) => Line { start, end: start + span },
            (GP::Auto, GP::Span(span)) => Line { start, end: start + span },
            (GP::Span(span), GP::Span(_)) => Line { start, end: start + span },
            _ => panic!("resolve_indefinite_grid_tracks should only be called on indefinite grid tracks"),
        }
    }
}

/// Represents the start and end points of a GridItem within a given axis
impl<S: CheapCloneStr> Default for Line<GridPlacement<S>> {
    fn default() -> Self {
        Line { start: GridPlacement::<S>::Auto, end: GridPlacement::<S>::Auto }
    }
}

/// Maximum track sizing function
///
/// Specifies the maximum size of a grid track. A grid track will automatically size between it's minimum and maximum size based
/// on the size of it's contents, the amount of available space, and the sizing constraint the grid is being size under.
/// See <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MaxTrackSizingFunction(pub(crate) CompactLength);
impl TaffyZero for MaxTrackSizingFunction {
    const ZERO: Self = Self(CompactLength::ZERO);
}
impl TaffyAuto for MaxTrackSizingFunction {
    const AUTO: Self = Self(CompactLength::AUTO);
}
impl TaffyMinContent for MaxTrackSizingFunction {
    const MIN_CONTENT: Self = Self(CompactLength::MIN_CONTENT);
}
impl TaffyMaxContent for MaxTrackSizingFunction {
    const MAX_CONTENT: Self = Self(CompactLength::MAX_CONTENT);
}
impl FromLength for MaxTrackSizingFunction {
    fn from_length<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::length(value.into() as f32)
    }
}
impl FromPercent for MaxTrackSizingFunction {
    fn from_percent<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::percent(value.into() as f32)
    }
}
impl TaffyFitContent for MaxTrackSizingFunction {
    fn fit_content(argument: LengthPercentage) -> Self {
        Self(CompactLength::fit_content(argument))
    }
}
impl FromFr for MaxTrackSizingFunction {
    fn from_fr<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::fr(value.into() as f32)
    }
}
impl From<LengthPercentage> for MaxTrackSizingFunction {
    fn from(input: LengthPercentage) -> Self {
        Self(input.0)
    }
}
impl From<LengthPercentageAuto> for MaxTrackSizingFunction {
    fn from(input: LengthPercentageAuto) -> Self {
        Self(input.0)
    }
}
impl From<Dimension> for MaxTrackSizingFunction {
    fn from(input: Dimension) -> Self {
        Self(input.0)
    }
}
impl From<MinTrackSizingFunction> for MaxTrackSizingFunction {
    fn from(input: MinTrackSizingFunction) -> Self {
        Self(input.0)
    }
}

#[cfg(feature = "parse")]
impl FromCss for MaxTrackSizingFunction {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        let token = parser.next()?.clone();
        match token {
            Token::Percentage { unit_value, .. } => Ok(Self::percent(unit_value)),
            Token::Dimension { unit, value, .. } if unit == "px" => Ok(Self::length(value)),
            Token::Dimension { unit, value, .. } if unit == "fr" && value.is_sign_positive() => Ok(Self::fr(value)),
            Token::Ident(ref ident) => match ident.as_ref() {
                "auto" => Ok(Self::auto()),
                "min-content" => Ok(Self::min_content()),
                "max-content" => Ok(Self::max_content()),
                _ => Err(parser.new_unexpected_token_error(token))?,
            },
            Token::Function(ref name) if name.as_ref() == "fit-content" => parser.parse_nested_block(|parser| {
                let token = parser.next()?.clone();
                match token {
                    Token::Percentage { unit_value, .. } => Ok(Self::fit_content_percent(unit_value)),
                    Token::Dimension { unit, value, .. } if unit == "px" => Ok(Self::fit_content_px(value)),
                    token => Err(parser.new_unexpected_token_error(token))?,
                }
            }),
            token => Err(parser.new_unexpected_token_error(token))?,
        }
    }
}

#[cfg(feature = "parse")]
from_str_from_css!(MaxTrackSizingFunction);

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MaxTrackSizingFunction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = CompactLength::deserialize(deserializer)?;
        // Note: validation intentionally excludes the CALC_TAG as deserializing calc() values is not supported
        if matches!(
            inner.tag(),
            CompactLength::LENGTH_TAG
                | CompactLength::PERCENT_TAG
                | CompactLength::AUTO_TAG
                | CompactLength::MIN_CONTENT_TAG
                | CompactLength::MAX_CONTENT_TAG
                | CompactLength::FIT_CONTENT_PX_TAG
                | CompactLength::FIT_CONTENT_PERCENT_TAG
                | CompactLength::FR_TAG
        ) {
            Ok(Self(inner))
        } else {
            Err(serde::de::Error::custom("Invalid tag"))
        }
    }
}

impl MaxTrackSizingFunction {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    #[inline(always)]
    pub const fn length(val: f32) -> Self {
        Self(CompactLength::length(val))
    }

    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    #[inline(always)]
    pub const fn percent(val: f32) -> Self {
        Self(CompactLength::percent(val))
    }

    /// The dimension should be automatically computed according to algorithm-specific rules
    /// regarding the default size of boxes.
    #[inline(always)]
    pub const fn auto() -> Self {
        Self(CompactLength::auto())
    }

    /// The size should be the "min-content" size.
    /// This is the smallest size that can fit the item's contents with ALL soft line-wrapping opportunities taken
    #[inline(always)]
    pub const fn min_content() -> Self {
        Self(CompactLength::min_content())
    }

    /// The size should be the "max-content" size.
    /// This is the smallest size that can fit the item's contents with NO soft line-wrapping opportunities taken
    #[inline(always)]
    pub const fn max_content() -> Self {
        Self(CompactLength::max_content())
    }

    /// The size should be computed according to the "fit content" formula:
    ///    `max(min_content, min(max_content, limit))`
    /// where:
    ///    - `min_content` is the [min-content](Self::min_content) size
    ///    - `max_content` is the [max-content](Self::max_content) size
    ///    - `limit` is a LENGTH value passed to this function
    ///
    /// The effect of this is that the item takes the size of `limit` clamped
    /// by the min-content and max-content sizes.
    #[inline(always)]
    pub const fn fit_content_px(limit: f32) -> Self {
        Self(CompactLength::fit_content_px(limit))
    }

    /// The size should be computed according to the "fit content" formula:
    ///    `max(min_content, min(max_content, limit))`
    /// where:
    ///    - `min_content` is the [min-content](Self::min_content) size
    ///    - `max_content` is the [max-content](Self::max_content) size
    ///    - `limit` is a PERCENTAGE value passed to this function
    ///
    /// The effect of this is that the item takes the size of `limit` clamped
    /// by the min-content and max-content sizes.
    #[inline(always)]
    pub const fn fit_content_percent(limit: f32) -> Self {
        Self(CompactLength::fit_content_percent(limit))
    }

    /// The dimension as a fraction of the total available grid space (`fr` units in CSS)
    /// Specified value is the numerator of the fraction. Denominator is the sum of all fraction specified in that grid dimension
    /// Spec: <https://www.w3.org/TR/css3-grid-layout/#fr-unit>
    #[inline(always)]
    pub const fn fr(val: f32) -> Self {
        Self(CompactLength::fr(val))
    }

    /// A `calc()` value. The value passed here is treated as an opaque handle to
    /// the actual calc representation and may be a pointer, index, etc.
    ///
    /// The low 3 bits are used as a tag value and will be returned as 0.
    #[inline]
    #[cfg(feature = "calc")]
    pub fn calc(ptr: *const ()) -> Self {
        Self(CompactLength::calc(ptr))
    }

    /// Create a LengthPercentageAuto from a raw `CompactLength`.
    /// # Safety
    /// CompactLength must represent a valid variant for LengthPercentageAuto
    #[allow(unsafe_code)]
    pub unsafe fn from_raw(val: CompactLength) -> Self {
        Self(val)
    }

    /// Get the underlying `CompactLength` representation of the value
    pub fn into_raw(self) -> CompactLength {
        self.0
    }

    /// Returns true if the max track sizing function is `MinContent`, `MaxContent`, `FitContent` or `Auto`, else false.
    #[inline(always)]
    pub fn is_intrinsic(&self) -> bool {
        self.0.is_intrinsic()
    }

    /// Returns true if the max track sizing function is `MaxContent`, `FitContent` or `Auto` else false.
    /// "In all cases, treat auto and fit-content() as max-content, except where specified otherwise for fit-content()."
    /// See: <https://www.w3.org/TR/css-grid-1/#algo-terms>
    #[inline(always)]
    pub fn is_max_content_alike(&self) -> bool {
        self.0.is_max_content_alike()
    }

    /// Returns true if the an Fr value, else false.
    #[inline(always)]
    pub fn is_fr(&self) -> bool {
        self.0.is_fr()
    }

    /// Returns true if the is `Auto`, else false.
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        self.0.is_auto()
    }

    /// Returns true if value is MinContent
    #[inline(always)]
    pub fn is_min_content(&self) -> bool {
        self.0.is_min_content()
    }

    /// Returns true if value is MaxContent
    #[inline(always)]
    pub fn is_max_content(&self) -> bool {
        self.0.is_max_content()
    }

    /// Returns true if value is FitContent(...)
    #[inline(always)]
    pub fn is_fit_content(&self) -> bool {
        self.0.is_fit_content()
    }

    /// Returns true if value is MaxContent or FitContent(...)
    #[inline(always)]
    pub fn is_max_or_fit_content(&self) -> bool {
        self.0.is_max_or_fit_content()
    }

    /// Returns whether the value can be resolved using `Self::definite_value`
    #[inline(always)]
    pub fn has_definite_value(self, parent_size: Option<f32>) -> bool {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => true,
            CompactLength::PERCENT_TAG => parent_size.is_some(),
            #[cfg(feature = "calc")]
            _ if self.0.is_calc() => parent_size.is_some(),
            _ => false,
        }
    }

    /// Returns fixed point values directly. Attempts to resolve percentage values against
    /// the passed available_space and returns if this results in a concrete value (which it
    /// will if the available_space is `Some`). Otherwise returns None.
    #[inline(always)]
    pub fn definite_value(
        self,
        parent_size: Option<f32>,
        calc_resolver: impl Fn(*const (), f32) -> f32,
    ) -> Option<f32> {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => Some(self.0.value()),
            CompactLength::PERCENT_TAG => parent_size.map(|size| self.0.value() * size),
            #[cfg(feature = "calc")]
            _ if self.0.is_calc() => parent_size.map(|size| calc_resolver(self.0.calc_value(), size)),
            _ => None,
        }
    }

    /// Resolve the maximum size of the track as defined by either:
    ///     - A fixed track sizing function
    ///     - A percentage track sizing function (with definite available space)
    ///     - A fit-content sizing function with fixed argument
    ///     - A fit-content sizing function with percentage argument (with definite available space)
    /// All other kinds of track sizing function return None.
    #[inline(always)]
    pub fn definite_limit(
        self,
        parent_size: Option<f32>,
        calc_resolver: impl Fn(*const (), f32) -> f32,
    ) -> Option<f32> {
        match self.0.tag() {
            CompactLength::FIT_CONTENT_PX_TAG => Some(self.0.value()),
            CompactLength::FIT_CONTENT_PERCENT_TAG => parent_size.map(|size| self.0.value() * size),
            _ => self.definite_value(parent_size, calc_resolver),
        }
    }

    /// Resolve percentage values against the passed parent_size, returning Some(value)
    /// Non-percentage values always return None.
    #[inline(always)]
    pub fn resolved_percentage_size(
        self,
        parent_size: f32,
        calc_resolver: impl Fn(*const (), f32) -> f32,
    ) -> Option<f32> {
        self.0.resolved_percentage_size(parent_size, calc_resolver)
    }

    /// Whether the track sizing functions depends on the size of the parent node
    #[inline(always)]
    pub fn uses_percentage(self) -> bool {
        self.0.uses_percentage()
    }
}

/// Minimum track sizing function
///
/// Specifies the minimum size of a grid track. A grid track will automatically size between it's minimum and maximum size based
/// on the size of it's contents, the amount of available space, and the sizing constraint the grid is being size under.
/// See <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MinTrackSizingFunction(pub(crate) CompactLength);
impl TaffyZero for MinTrackSizingFunction {
    const ZERO: Self = Self(CompactLength::ZERO);
}
impl TaffyAuto for MinTrackSizingFunction {
    const AUTO: Self = Self(CompactLength::AUTO);
}
impl TaffyMinContent for MinTrackSizingFunction {
    const MIN_CONTENT: Self = Self(CompactLength::MIN_CONTENT);
}
impl TaffyMaxContent for MinTrackSizingFunction {
    const MAX_CONTENT: Self = Self(CompactLength::MAX_CONTENT);
}
impl FromLength for MinTrackSizingFunction {
    fn from_length<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::length(value.into() as f32)
    }
}
impl FromPercent for MinTrackSizingFunction {
    fn from_percent<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::percent(value.into() as f32)
    }
}
impl From<LengthPercentage> for MinTrackSizingFunction {
    fn from(input: LengthPercentage) -> Self {
        Self(input.0)
    }
}
impl From<LengthPercentageAuto> for MinTrackSizingFunction {
    fn from(input: LengthPercentageAuto) -> Self {
        Self(input.0)
    }
}
impl From<Dimension> for MinTrackSizingFunction {
    fn from(input: Dimension) -> Self {
        Self(input.0)
    }
}

impl From<MaxTrackSizingFunction> for MinTrackSizingFunction {
    fn from(input: MaxTrackSizingFunction) -> Self {
        if input.is_fr() || input.is_fit_content() {
            return Self::auto();
        }
        Self(input.0)
    }
}

#[cfg(feature = "parse")]
impl FromCss for MinTrackSizingFunction {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        let token = parser.next()?.clone();
        match token {
            Token::Percentage { unit_value, .. } => Ok(Self::percent(unit_value)),
            Token::Dimension { unit, value, .. } if unit == "px" => Ok(Self::length(value)),
            Token::Ident(ref ident) => match ident.as_ref() {
                "auto" => Ok(Self::auto()),
                "min-content" => Ok(Self::min_content()),
                "max-content" => Ok(Self::max_content()),
                _ => Err(parser.new_unexpected_token_error(token))?,
            },
            token => Err(parser.new_unexpected_token_error(token))?,
        }
    }
}

#[cfg(feature = "parse")]
from_str_from_css!(MinTrackSizingFunction);

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MinTrackSizingFunction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = CompactLength::deserialize(deserializer)?;
        // Note: validation intentionally excludes the CALC_TAG as deserializing calc() values is not supported
        if matches!(
            inner.tag(),
            CompactLength::LENGTH_TAG
                | CompactLength::PERCENT_TAG
                | CompactLength::AUTO_TAG
                | CompactLength::MIN_CONTENT_TAG
                | CompactLength::MAX_CONTENT_TAG
                | CompactLength::FIT_CONTENT_PX_TAG
                | CompactLength::FIT_CONTENT_PERCENT_TAG
        ) {
            Ok(Self(inner))
        } else {
            Err(serde::de::Error::custom("Invalid tag"))
        }
    }
}

impl MinTrackSizingFunction {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    #[inline(always)]
    pub const fn length(val: f32) -> Self {
        Self(CompactLength::length(val))
    }

    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    #[inline(always)]
    pub const fn percent(val: f32) -> Self {
        Self(CompactLength::percent(val))
    }

    /// The dimension should be automatically computed according to algorithm-specific rules
    /// regarding the default size of boxes.
    #[inline(always)]
    pub const fn auto() -> Self {
        Self(CompactLength::auto())
    }

    /// The size should be the "min-content" size.
    /// This is the smallest size that can fit the item's contents with ALL soft line-wrapping opportunities taken
    #[inline(always)]
    pub const fn min_content() -> Self {
        Self(CompactLength::min_content())
    }

    /// The size should be the "max-content" size.
    /// This is the smallest size that can fit the item's contents with NO soft line-wrapping opportunities taken
    #[inline(always)]
    pub const fn max_content() -> Self {
        Self(CompactLength::max_content())
    }

    /// A `calc()` value. The value passed here is treated as an opaque handle to
    /// the actual calc representation and may be a pointer, index, etc.
    ///
    /// The low 3 bits are used as a tag value and will be returned as 0.
    #[inline]
    #[cfg(feature = "calc")]
    pub fn calc(ptr: *const ()) -> Self {
        Self(CompactLength::calc(ptr))
    }

    /// Create a LengthPercentageAuto from a raw `CompactLength`.
    /// # Safety
    /// CompactLength must represent a valid variant for LengthPercentageAuto
    #[allow(unsafe_code)]
    pub unsafe fn from_raw(val: CompactLength) -> Self {
        Self(val)
    }

    /// Get the underlying `CompactLength` representation of the value
    pub fn into_raw(self) -> CompactLength {
        self.0
    }

    /// Returns true if the min track sizing function is `MinContent`, `MaxContent` or `Auto`, else false.
    #[inline(always)]
    pub fn is_intrinsic(&self) -> bool {
        self.0.is_intrinsic()
    }

    /// Returns true if the min track sizing function is `MinContent` or `MaxContent`, else false.
    #[inline(always)]
    pub fn is_min_or_max_content(&self) -> bool {
        self.0.is_min_or_max_content()
    }

    /// Returns true if the value is an fr value
    #[inline(always)]
    pub fn is_fr(&self) -> bool {
        self.0.is_fr()
    }

    /// Returns true if the is `Auto`, else false.
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        self.0.is_auto()
    }

    /// Returns true if value is MinContent
    #[inline(always)]
    pub fn is_min_content(&self) -> bool {
        self.0.is_min_content()
    }

    /// Returns true if value is MaxContent
    #[inline(always)]
    pub fn is_max_content(&self) -> bool {
        self.0.is_max_content()
    }

    /// Returns fixed point values directly. Attempts to resolve percentage values against
    /// the passed available_space and returns if this results in a concrete value (which it
    /// will if the available_space is `Some`). Otherwise returns `None`.
    #[inline(always)]
    pub fn definite_value(
        self,
        parent_size: Option<f32>,
        calc_resolver: impl Fn(*const (), f32) -> f32,
    ) -> Option<f32> {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => Some(self.0.value()),
            CompactLength::PERCENT_TAG => parent_size.map(|size| self.0.value() * size),
            #[cfg(feature = "calc")]
            _ if self.0.is_calc() => parent_size.map(|size| calc_resolver(self.0.calc_value(), size)),
            _ => None,
        }
    }

    /// Resolve percentage values against the passed parent_size, returning Some(value)
    /// Non-percentage values always return None.
    #[inline(always)]
    pub fn resolved_percentage_size(
        self,
        parent_size: f32,
        calc_resolver: impl Fn(*const (), f32) -> f32,
    ) -> Option<f32> {
        self.0.resolved_percentage_size(parent_size, calc_resolver)
    }

    /// Whether the track sizing functions depends on the size of the parent node
    #[inline(always)]
    pub fn uses_percentage(self) -> bool {
        #[cfg(feature = "calc")]
        {
            matches!(self.0.tag(), CompactLength::PERCENT_TAG) || self.0.is_calc()
        }
        #[cfg(not(feature = "calc"))]
        {
            matches!(self.0.tag(), CompactLength::PERCENT_TAG)
        }
    }
}

/// The sizing function for a grid track (row/column)
///
/// May either be a MinMax variant which specifies separate values for the min-/max- track sizing functions
/// or a scalar value which applies to both track sizing functions.
pub type TrackSizingFunction = MinMax<MinTrackSizingFunction, MaxTrackSizingFunction>;
impl TrackSizingFunction {
    /// Extract the min track sizing function
    pub fn min_sizing_function(&self) -> MinTrackSizingFunction {
        self.min
    }
    /// Extract the max track sizing function
    pub fn max_sizing_function(&self) -> MaxTrackSizingFunction {
        self.max
    }
    /// Determine whether at least one of the components ("min" and "max") are fixed sizing function
    pub fn has_fixed_component(&self) -> bool {
        self.min.0.is_length_or_percentage() || self.max.0.is_length_or_percentage()
    }
}
impl TaffyAuto for TrackSizingFunction {
    const AUTO: Self = Self { min: MinTrackSizingFunction::AUTO, max: MaxTrackSizingFunction::AUTO };
}
impl TaffyMinContent for TrackSizingFunction {
    const MIN_CONTENT: Self =
        Self { min: MinTrackSizingFunction::MIN_CONTENT, max: MaxTrackSizingFunction::MIN_CONTENT };
}
impl TaffyMaxContent for TrackSizingFunction {
    const MAX_CONTENT: Self =
        Self { min: MinTrackSizingFunction::MAX_CONTENT, max: MaxTrackSizingFunction::MAX_CONTENT };
}
impl TaffyFitContent for TrackSizingFunction {
    fn fit_content(argument: LengthPercentage) -> Self {
        Self { min: MinTrackSizingFunction::AUTO, max: MaxTrackSizingFunction::fit_content(argument) }
    }
}
impl TaffyZero for TrackSizingFunction {
    const ZERO: Self = Self { min: MinTrackSizingFunction::ZERO, max: MaxTrackSizingFunction::ZERO };
}
impl FromLength for TrackSizingFunction {
    fn from_length<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self { min: MinTrackSizingFunction::from_length(value), max: MaxTrackSizingFunction::from_length(value) }
    }
}
impl FromPercent for TrackSizingFunction {
    fn from_percent<Input: Into<f64> + Copy>(percent: Input) -> Self {
        Self { min: MinTrackSizingFunction::from_percent(percent), max: MaxTrackSizingFunction::from_percent(percent) }
    }
}
impl FromFr for TrackSizingFunction {
    fn from_fr<Input: Into<f64> + Copy>(flex: Input) -> Self {
        Self { min: MinTrackSizingFunction::AUTO, max: MaxTrackSizingFunction::from_fr(flex) }
    }
}
impl From<LengthPercentage> for TrackSizingFunction {
    fn from(input: LengthPercentage) -> Self {
        Self { min: input.into(), max: input.into() }
    }
}
impl From<LengthPercentageAuto> for TrackSizingFunction {
    fn from(input: LengthPercentageAuto) -> Self {
        Self { min: input.into(), max: input.into() }
    }
}
impl From<Dimension> for TrackSizingFunction {
    fn from(input: Dimension) -> Self {
        Self { min: input.into(), max: input.into() }
    }
}

#[cfg(feature = "parse")]
impl FromCss for TrackSizingFunction {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        // Try to parse a minmax() function
        if let Ok(value) = parser.try_parse(|parser| {
            parser.expect_function_matching("minmax")?;
            parser.parse_nested_block(|parser| {
                let min = MinTrackSizingFunction::from_css(parser)?;
                parser.expect_comma()?;
                let max = MaxTrackSizingFunction::from_css(parser)?;

                Ok(Self { min, max })
            })
        }) {
            return Ok(value);
        }

        // Else parse a max track sizing function
        let max = MaxTrackSizingFunction::from_css(parser)?;
        let min = max.into();
        Ok(Self { min, max })
    }
}

#[cfg(feature = "parse")]
from_str_from_css!(TrackSizingFunction);

/// The first argument to a repeated track definition. This type represents the type of automatic repetition to perform.
///
/// See <https://www.w3.org/TR/css-grid-1/#auto-repeat> for an explanation of how auto-repeated track definitions work
/// and the difference between AutoFit and AutoFill.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RepetitionCount {
    /// Auto-repeating tracks should be generated to fit the container
    /// See: <https://developer.mozilla.org/en-US/docs/Web/CSS/repeat#auto-fill>
    AutoFill,
    /// Auto-repeating tracks should be generated to fit the container
    /// See: <https://developer.mozilla.org/en-US/docs/Web/CSS/repeat#auto-fit>
    AutoFit,
    /// The specified tracks should be repeated exacts N times
    Count(u16),
}
impl From<u16> for RepetitionCount {
    fn from(value: u16) -> Self {
        Self::Count(value)
    }
}

/// Error returned when trying to convert a string to a GridTrackRepetition and that string is not
/// either "auto-fit" or "auto-fill"
#[derive(Debug)]
pub struct InvalidStringRepetitionValue;
#[cfg(feature = "std")]
impl std::error::Error for InvalidStringRepetitionValue {}
impl core::fmt::Display for InvalidStringRepetitionValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("&str can only be converted to GridTrackRepetition if it's value is 'auto-fit' or 'auto-fill'")
    }
}
impl TryFrom<&str> for RepetitionCount {
    type Error = InvalidStringRepetitionValue;
    fn try_from(value: &str) -> Result<Self, InvalidStringRepetitionValue> {
        match value {
            "auto-fit" => Ok(Self::AutoFit),
            "auto-fill" => Ok(Self::AutoFill),
            _ => Err(InvalidStringRepetitionValue),
        }
    }
}

#[cfg(feature = "parse")]
impl FromCss for RepetitionCount {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        match parser.next()?.clone() {
            Token::Number { int_value: Some(value), .. } if value.is_positive() => {
                Ok(Self::Count(saturating_u16(value)))
            }
            Token::Ident(ident) if ident == "auto-fit" => Ok(Self::AutoFit),
            Token::Ident(ident) if ident == "auto-fill" => Ok(Self::AutoFill),
            token => Err(parser.new_unexpected_token_error(token))?,
        }
    }
}
#[cfg(feature = "parse")]
from_str_from_css!(RepetitionCount);

/// A typed representation of a `repeat(..)` in `grid-template-*` value
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridTemplateRepetition<S: CheapCloneStr> {
    /// The number of the times the repeat is repeated
    pub count: RepetitionCount,
    /// The tracks to repeat
    pub tracks: Vec<TrackSizingFunction>,
    /// The line names for the repeated tracks
    pub line_names: Vec<Vec<S>>,
}

#[rustfmt::skip]
impl<S: CheapCloneStr> GenericRepetition for &'_ GridTemplateRepetition<S> {
    type CustomIdent = S;
    type RepetitionTrackList<'a> = core::iter::Copied<core::slice::Iter<'a, TrackSizingFunction>> where Self: 'a;
    type TemplateLineNames<'a> = core::iter::Map<core::slice::Iter<'a, Vec<S>>, fn(&Vec<S>) -> core::slice::Iter<'_, S>> where Self: 'a;
    #[inline(always)]
    fn count(&self) -> RepetitionCount {
        self.count
    }
    #[inline(always)]
    fn track_count(&self) -> u16 {
        self.tracks.len().min(u16::MAX as usize) as u16
    }
    #[inline(always)]
    fn tracks(&self) -> Self::RepetitionTrackList<'_> {
        self.tracks.iter().copied()
    }
    #[inline(always)]
    fn lines_names(&self) -> Self::TemplateLineNames<'_> {
        self.line_names.iter().map(|names| names.iter())
    }
}

/// An element in a `grid-template-columns` or `grid-template-rows` definition.
/// Either a track sizing function or a repeat().
///
/// See <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GridTemplateComponent<S: CheapCloneStr> {
    /// A single non-repeated track
    Single(TrackSizingFunction),
    /// Automatically generate grid tracks to fit the available space using the specified definite track lengths
    /// Only valid if every track in template (not just the repetition) has a fixed size.
    Repeat(GridTemplateRepetition<S>),
}

impl<S: CheapCloneStr> GridTemplateComponent<S> {
    /// Convert a `GridTemplateComponent` into a `GridTemplateComponentRef`
    pub fn as_component_ref(&self) -> GenericGridTemplateComponent<S, &GridTemplateRepetition<S>> {
        match self {
            GridTemplateComponent::Single(size) => GenericGridTemplateComponent::Single(*size),
            GridTemplateComponent::Repeat(repetition) => GenericGridTemplateComponent::Repeat(repetition),
        }
    }
}

impl<S: CheapCloneStr> GridTemplateComponent<S> {
    /// Whether the track definition is a auto-repeated fragment
    pub fn is_auto_repetition(&self) -> bool {
        matches!(
            self,
            Self::Repeat(GridTemplateRepetition { count: RepetitionCount::AutoFit | RepetitionCount::AutoFill, .. })
        )
    }
}
impl<S: CheapCloneStr> TaffyAuto for GridTemplateComponent<S> {
    const AUTO: Self = Self::Single(TrackSizingFunction::AUTO);
}
impl<S: CheapCloneStr> TaffyMinContent for GridTemplateComponent<S> {
    const MIN_CONTENT: Self = Self::Single(TrackSizingFunction::MIN_CONTENT);
}
impl<S: CheapCloneStr> TaffyMaxContent for GridTemplateComponent<S> {
    const MAX_CONTENT: Self = Self::Single(TrackSizingFunction::MAX_CONTENT);
}
impl<S: CheapCloneStr> TaffyFitContent for GridTemplateComponent<S> {
    fn fit_content(argument: LengthPercentage) -> Self {
        Self::Single(TrackSizingFunction::fit_content(argument))
    }
}
impl<S: CheapCloneStr> TaffyZero for GridTemplateComponent<S> {
    const ZERO: Self = Self::Single(TrackSizingFunction::ZERO);
}
impl<S: CheapCloneStr> FromLength for GridTemplateComponent<S> {
    fn from_length<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::Single(TrackSizingFunction::from_length(value))
    }
}
impl<S: CheapCloneStr> FromPercent for GridTemplateComponent<S> {
    fn from_percent<Input: Into<f64> + Copy>(percent: Input) -> Self {
        Self::Single(TrackSizingFunction::from_percent(percent))
    }
}
impl<S: CheapCloneStr> FromFr for GridTemplateComponent<S> {
    fn from_fr<Input: Into<f64> + Copy>(flex: Input) -> Self {
        Self::Single(TrackSizingFunction::from_fr(flex))
    }
}
impl<S: CheapCloneStr> From<MinMax<MinTrackSizingFunction, MaxTrackSizingFunction>> for GridTemplateComponent<S> {
    fn from(input: MinMax<MinTrackSizingFunction, MaxTrackSizingFunction>) -> Self {
        Self::Single(input)
    }
}

#[cfg(feature = "parse")]
impl<S: CheapCloneStr> FromCss for GridTemplateComponent<S> {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        // Try to parse a minmax() function
        if let Ok(value) = parser.try_parse(|parser| {
            parser.expect_function_matching("repeat")?;
            parser.parse_nested_block(|parser| {
                let count = RepetitionCount::from_css(parser)?;
                parser.expect_comma()?;
                let tracks = GridTemplateTracks::<S, TrackSizingFunction>::from_css(parser)?;

                Ok(Self::Repeat(GridTemplateRepetition { count, tracks: tracks.tracks, line_names: tracks.line_names }))
            })
        }) {
            return Ok(value);
        }

        // Else parse a track sizing function
        let track_sizing_function = TrackSizingFunction::from_css(parser)?;
        Ok(Self::Single(track_sizing_function))
    }
}
#[cfg(feature = "parse")]
impl<S: CheapCloneStr> core::str::FromStr for GridTemplateComponent<S> {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_css_str_entirely(input)
    }
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[doc(hidden)]
pub struct GridTemplateTracks<S: CheapCloneStr, Track> {
    /// The tracks to repeat
    pub tracks: Vec<Track>,
    /// The line names for the repeated tracks
    pub line_names: Vec<Vec<S>>,
}

impl<S: CheapCloneStr, Track> Default for GridTemplateTracks<S, Track> {
    fn default() -> Self {
        Self { tracks: Vec::new(), line_names: Vec::new() }
    }
}

#[cfg(feature = "parse")]
impl<S: CheapCloneStr, Track: FromCss + Debug> FromCss for GridTemplateTracks<S, Track> {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        fn try_parse_line_names<'i, S: CheapCloneStr>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Vec<S>> {
            parser.try_parse(|parser| {
                parser.expect_square_bracket_block()?;
                parser.parse_nested_block(|parser| {
                    let mut line_names = Vec::new();
                    while !parser.is_exhausted() {
                        line_names.push(S::from(parser.expect_ident_cloned()?.as_ref()));
                    }
                    Ok(line_names)
                })
            })
        }

        let mut tracks = Self::default();
        if let Ok(line_names) = try_parse_line_names(parser) {
            tracks.line_names.push(line_names);
        }

        while !parser.is_exhausted() {
            tracks.tracks.push(Track::from_css(parser)?);
            if let Ok(line_names) = try_parse_line_names(parser) {
                tracks.line_names.push(line_names);
            }
        }

        if tracks.tracks.is_empty() {
            return Err(parser.new_error(cssparser::BasicParseErrorKind::EndOfInput));
        }

        Ok(tracks)
    }
}
#[cfg(feature = "parse")]
impl<S: CheapCloneStr, Track: FromCss + Debug> core::str::FromStr for GridTemplateTracks<S, Track> {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_css_str_entirely(input)
    }
}

#[derive(Default)]
#[doc(hidden)]
pub struct GridAutoTracks(pub Vec<TrackSizingFunction>);

#[cfg(feature = "parse")]
impl FromCss for GridAutoTracks {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        let mut tracks = Self::default();
        while !parser.is_exhausted() {
            tracks.0.push(TrackSizingFunction::from_css(parser)?);
        }
        if tracks.0.is_empty() {
            return Err(parser.new_error(cssparser::BasicParseErrorKind::EndOfInput));
        }
        Ok(tracks)
    }
}
#[cfg(feature = "parse")]
from_str_from_css!(GridAutoTracks);

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::sys::DefaultCheapStr;

    #[test]
    fn grid_placement_parser_saturates_numeric_values() {
        assert_eq!(
            "32768".parse::<GridPlacement<DefaultCheapStr>>().unwrap(),
            GridPlacement::Line(GridLine::from(i16::MAX))
        );
        assert_eq!(
            "-32769".parse::<GridPlacement<DefaultCheapStr>>().unwrap(),
            GridPlacement::Line(GridLine::from(i16::MIN))
        );
        assert_eq!("span 65536".parse::<GridPlacement<DefaultCheapStr>>().unwrap(), GridPlacement::Span(u16::MAX));

        let named_line = "32768 line".parse::<GridPlacement<DefaultCheapStr>>().unwrap();
        assert!(matches!(named_line, GridPlacement::NamedLine(_, i16::MAX)));

        let named_span = "span 65536 line".parse::<GridPlacement<DefaultCheapStr>>().unwrap();
        assert!(matches!(named_span, GridPlacement::NamedSpan(_, u16::MAX)));
    }

    #[test]
    fn repetition_parser_saturates_numeric_values() {
        assert_eq!("65536".parse::<RepetitionCount>().unwrap(), RepetitionCount::Count(u16::MAX));

        let component = "repeat(65536, 1px)".parse::<GridTemplateComponent<DefaultCheapStr>>().unwrap();
        assert!(matches!(
            component,
            GridTemplateComponent::Repeat(GridTemplateRepetition { count: RepetitionCount::Count(u16::MAX), .. })
        ));
    }

    #[test]
    fn repetition_track_count_saturates() {
        let repetition = GridTemplateRepetition::<DefaultCheapStr> {
            count: RepetitionCount::Count(1),
            tracks: vec![TrackSizingFunction::AUTO; u16::MAX as usize + 1],
            line_names: Vec::new(),
        };
        assert_eq!((&repetition).track_count(), u16::MAX);
    }
}
