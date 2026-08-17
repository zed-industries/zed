// Copyright 2018 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::sync::Arc;

use strict_num::NonZeroPositiveF32;
pub use svgtypes::FontFamily;

#[cfg(feature = "text")]
use crate::layout::Span;
use crate::{Fill, Group, NonEmptyString, PaintOrder, Rect, Stroke, TextRendering, Transform};

/// A font stretch property.
#[allow(missing_docs)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum FontStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl Default for FontStretch {
    #[inline]
    fn default() -> Self {
        Self::Normal
    }
}

#[cfg(feature = "text")]
impl From<fontdb::Stretch> for FontStretch {
    fn from(stretch: fontdb::Stretch) -> Self {
        match stretch {
            fontdb::Stretch::UltraCondensed => FontStretch::UltraCondensed,
            fontdb::Stretch::ExtraCondensed => FontStretch::ExtraCondensed,
            fontdb::Stretch::Condensed => FontStretch::Condensed,
            fontdb::Stretch::SemiCondensed => FontStretch::SemiCondensed,
            fontdb::Stretch::Normal => FontStretch::Normal,
            fontdb::Stretch::SemiExpanded => FontStretch::SemiExpanded,
            fontdb::Stretch::Expanded => FontStretch::Expanded,
            fontdb::Stretch::ExtraExpanded => FontStretch::ExtraExpanded,
            fontdb::Stretch::UltraExpanded => FontStretch::UltraExpanded,
        }
    }
}

#[cfg(feature = "text")]
impl From<FontStretch> for fontdb::Stretch {
    fn from(stretch: FontStretch) -> Self {
        match stretch {
            FontStretch::UltraCondensed => fontdb::Stretch::UltraCondensed,
            FontStretch::ExtraCondensed => fontdb::Stretch::ExtraCondensed,
            FontStretch::Condensed => fontdb::Stretch::Condensed,
            FontStretch::SemiCondensed => fontdb::Stretch::SemiCondensed,
            FontStretch::Normal => fontdb::Stretch::Normal,
            FontStretch::SemiExpanded => fontdb::Stretch::SemiExpanded,
            FontStretch::Expanded => fontdb::Stretch::Expanded,
            FontStretch::ExtraExpanded => fontdb::Stretch::ExtraExpanded,
            FontStretch::UltraExpanded => fontdb::Stretch::UltraExpanded,
        }
    }
}

/// A font style property.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum FontStyle {
    /// A face that is neither italic not obliqued.
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

impl Default for FontStyle {
    #[inline]
    fn default() -> FontStyle {
        Self::Normal
    }
}

#[cfg(feature = "text")]
impl From<fontdb::Style> for FontStyle {
    fn from(style: fontdb::Style) -> Self {
        match style {
            fontdb::Style::Normal => FontStyle::Normal,
            fontdb::Style::Italic => FontStyle::Italic,
            fontdb::Style::Oblique => FontStyle::Oblique,
        }
    }
}

#[cfg(feature = "text")]
impl From<FontStyle> for fontdb::Style {
    fn from(style: FontStyle) -> Self {
        match style {
            FontStyle::Normal => fontdb::Style::Normal,
            FontStyle::Italic => fontdb::Style::Italic,
            FontStyle::Oblique => fontdb::Style::Oblique,
        }
    }
}

/// Text font properties.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Font {
    pub(crate) families: Vec<FontFamily>,
    pub(crate) style: FontStyle,
    pub(crate) stretch: FontStretch,
    pub(crate) weight: u16,
}

impl Font {
    /// A list of family names.
    ///
    /// Never empty. Uses `usvg::Options::font_family` as fallback.
    pub fn families(&self) -> &[FontFamily] {
        &self.families
    }

    /// A font style.
    pub fn style(&self) -> FontStyle {
        self.style
    }

    /// A font stretch.
    pub fn stretch(&self) -> FontStretch {
        self.stretch
    }

    /// A font width.
    pub fn weight(&self) -> u16 {
        self.weight
    }
}

/// A dominant baseline property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DominantBaseline {
    Auto,
    UseScript,
    NoChange,
    ResetSize,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
    Central,
    Middle,
    TextAfterEdge,
    TextBeforeEdge,
}

impl Default for DominantBaseline {
    fn default() -> Self {
        Self::Auto
    }
}

/// An alignment baseline property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AlignmentBaseline {
    Auto,
    Baseline,
    BeforeEdge,
    TextBeforeEdge,
    Middle,
    Central,
    AfterEdge,
    TextAfterEdge,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
}

impl Default for AlignmentBaseline {
    fn default() -> Self {
        Self::Auto
    }
}

/// A baseline shift property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BaselineShift {
    Baseline,
    Subscript,
    Superscript,
    Number(f32),
}

impl Default for BaselineShift {
    #[inline]
    fn default() -> BaselineShift {
        BaselineShift::Baseline
    }
}

/// A length adjust property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LengthAdjust {
    Spacing,
    SpacingAndGlyphs,
}

impl Default for LengthAdjust {
    fn default() -> Self {
        Self::Spacing
    }
}

/// A text span decoration style.
///
/// In SVG, text decoration and text it's applied to can have different styles.
/// So you can have black text and green underline.
///
/// Also, in SVG you can specify text decoration stroking.
#[derive(Clone, Debug)]
pub struct TextDecorationStyle {
    pub(crate) fill: Option<Fill>,
    pub(crate) stroke: Option<Stroke>,
}

impl TextDecorationStyle {
    /// A fill style.
    pub fn fill(&self) -> Option<&Fill> {
        self.fill.as_ref()
    }

    /// A stroke style.
    pub fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }
}

/// A text span decoration.
#[derive(Clone, Debug)]
pub struct TextDecoration {
    pub(crate) underline: Option<TextDecorationStyle>,
    pub(crate) overline: Option<TextDecorationStyle>,
    pub(crate) line_through: Option<TextDecorationStyle>,
}

impl TextDecoration {
    /// An optional underline and its style.
    pub fn underline(&self) -> Option<&TextDecorationStyle> {
        self.underline.as_ref()
    }

    /// An optional overline and its style.
    pub fn overline(&self) -> Option<&TextDecorationStyle> {
        self.overline.as_ref()
    }

    /// An optional line-through and its style.
    pub fn line_through(&self) -> Option<&TextDecorationStyle> {
        self.line_through.as_ref()
    }
}

/// A text style span.
///
/// Spans do not overlap inside a text chunk.
#[derive(Clone, Debug)]
pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) fill: Option<Fill>,
    pub(crate) stroke: Option<Stroke>,
    pub(crate) paint_order: PaintOrder,
    pub(crate) font: Font,
    pub(crate) font_size: NonZeroPositiveF32,
    pub(crate) small_caps: bool,
    pub(crate) apply_kerning: bool,
    pub(crate) decoration: TextDecoration,
    pub(crate) dominant_baseline: DominantBaseline,
    pub(crate) alignment_baseline: AlignmentBaseline,
    pub(crate) baseline_shift: Vec<BaselineShift>,
    pub(crate) visible: bool,
    pub(crate) letter_spacing: f32,
    pub(crate) word_spacing: f32,
    pub(crate) text_length: Option<f32>,
    pub(crate) length_adjust: LengthAdjust,
}

impl TextSpan {
    /// A span start in bytes.
    ///
    /// Offset is relative to the parent text chunk and not the parent text element.
    pub fn start(&self) -> usize {
        self.start
    }

    /// A span end in bytes.
    ///
    /// Offset is relative to the parent text chunk and not the parent text element.
    pub fn end(&self) -> usize {
        self.end
    }

    /// A fill style.
    pub fn fill(&self) -> Option<&Fill> {
        self.fill.as_ref()
    }

    /// A stroke style.
    pub fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }

    /// A paint order style.
    pub fn paint_order(&self) -> PaintOrder {
        self.paint_order
    }

    /// A font.
    pub fn font(&self) -> &Font {
        &self.font
    }

    /// A font size.
    pub fn font_size(&self) -> NonZeroPositiveF32 {
        self.font_size
    }

    /// Indicates that small caps should be used.
    ///
    /// Set by `font-variant="small-caps"`
    pub fn small_caps(&self) -> bool {
        self.small_caps
    }

    /// Indicates that a kerning should be applied.
    ///
    /// Supports both `kerning` and `font-kerning` properties.
    pub fn apply_kerning(&self) -> bool {
        self.apply_kerning
    }

    /// A span decorations.
    pub fn decoration(&self) -> &TextDecoration {
        &self.decoration
    }

    /// A span dominant baseline.
    pub fn dominant_baseline(&self) -> DominantBaseline {
        self.dominant_baseline
    }

    /// A span alignment baseline.
    pub fn alignment_baseline(&self) -> AlignmentBaseline {
        self.alignment_baseline
    }

    /// A list of all baseline shift that should be applied to this span.
    ///
    /// Ordered from `text` element down to the actual `span` element.
    pub fn baseline_shift(&self) -> &[BaselineShift] {
        &self.baseline_shift
    }

    /// A visibility property.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// A letter spacing property.
    pub fn letter_spacing(&self) -> f32 {
        self.letter_spacing
    }

    /// A word spacing property.
    pub fn word_spacing(&self) -> f32 {
        self.word_spacing
    }

    /// A text length property.
    pub fn text_length(&self) -> Option<f32> {
        self.text_length
    }

    /// A length adjust property.
    pub fn length_adjust(&self) -> LengthAdjust {
        self.length_adjust
    }
}

/// A text chunk anchor property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextAnchor {
    Start,
    Middle,
    End,
}

impl Default for TextAnchor {
    fn default() -> Self {
        Self::Start
    }
}

/// A path used by text-on-path.
#[derive(Debug)]
pub struct TextPath {
    pub(crate) id: NonEmptyString,
    pub(crate) start_offset: f32,
    pub(crate) path: Arc<tiny_skia_path::Path>,
}

impl TextPath {
    /// Element's ID.
    ///
    /// Taken from the SVG itself.
    pub fn id(&self) -> &str {
        self.id.get()
    }

    /// A text offset in SVG coordinates.
    ///
    /// Percentage values already resolved.
    pub fn start_offset(&self) -> f32 {
        self.start_offset
    }

    /// A path.
    pub fn path(&self) -> &tiny_skia_path::Path {
        &self.path
    }
}

/// A text chunk flow property.
#[derive(Clone, Debug)]
pub enum TextFlow {
    /// A linear layout.
    ///
    /// Includes left-to-right, right-to-left and top-to-bottom.
    Linear,
    /// A text-on-path layout.
    Path(Arc<TextPath>),
}

/// A text chunk.
///
/// Text alignment and BIDI reordering can only be done inside a text chunk.
#[derive(Clone, Debug)]
pub struct TextChunk {
    pub(crate) x: Option<f32>,
    pub(crate) y: Option<f32>,
    pub(crate) anchor: TextAnchor,
    pub(crate) spans: Vec<TextSpan>,
    pub(crate) text_flow: TextFlow,
    pub(crate) text: String,
}

impl TextChunk {
    /// An absolute X axis offset.
    pub fn x(&self) -> Option<f32> {
        self.x
    }

    /// An absolute Y axis offset.
    pub fn y(&self) -> Option<f32> {
        self.y
    }

    /// A text anchor.
    pub fn anchor(&self) -> TextAnchor {
        self.anchor
    }

    /// A list of text chunk style spans.
    pub fn spans(&self) -> &[TextSpan] {
        &self.spans
    }

    /// A text chunk flow.
    pub fn text_flow(&self) -> TextFlow {
        self.text_flow.clone()
    }

    /// A text chunk actual text.
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// A writing mode.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WritingMode {
    LeftToRight,
    TopToBottom,
}

/// A text element.
///
/// `text` element in SVG.
#[derive(Clone, Debug)]
pub struct Text {
    pub(crate) id: String,
    pub(crate) rendering_mode: TextRendering,
    pub(crate) dx: Vec<f32>,
    pub(crate) dy: Vec<f32>,
    pub(crate) rotate: Vec<f32>,
    pub(crate) writing_mode: WritingMode,
    pub(crate) chunks: Vec<TextChunk>,
    pub(crate) abs_transform: Transform,
    pub(crate) bounding_box: Rect,
    pub(crate) abs_bounding_box: Rect,
    pub(crate) stroke_bounding_box: Rect,
    pub(crate) abs_stroke_bounding_box: Rect,
    pub(crate) flattened: Box<Group>,
    #[cfg(feature = "text")]
    pub(crate) layouted: Vec<Span>,
}

impl Text {
    /// Element's ID.
    ///
    /// Taken from the SVG itself.
    /// Isn't automatically generated.
    /// Can be empty.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Rendering mode.
    ///
    /// `text-rendering` in SVG.
    pub fn rendering_mode(&self) -> TextRendering {
        self.rendering_mode
    }

    /// A relative X axis offsets.
    ///
    /// One offset for each Unicode codepoint. Aka `char` in Rust.
    pub fn dx(&self) -> &[f32] {
        &self.dx
    }

    /// A relative Y axis offsets.
    ///
    /// One offset for each Unicode codepoint. Aka `char` in Rust.
    pub fn dy(&self) -> &[f32] {
        &self.dy
    }

    /// A list of rotation angles.
    ///
    /// One angle for each Unicode codepoint. Aka `char` in Rust.
    pub fn rotate(&self) -> &[f32] {
        &self.rotate
    }

    /// A writing mode.
    pub fn writing_mode(&self) -> WritingMode {
        self.writing_mode
    }

    /// A list of text chunks.
    pub fn chunks(&self) -> &[TextChunk] {
        &self.chunks
    }

    /// Element's absolute transform.
    ///
    /// Contains all ancestors transforms including elements's transform.
    ///
    /// Note that this is not the relative transform present in SVG.
    /// The SVG one would be set only on groups.
    pub fn abs_transform(&self) -> Transform {
        self.abs_transform
    }

    /// Element's text bounding box.
    ///
    /// Text bounding box is special in SVG and doesn't represent
    /// tight bounds of the element's content.
    /// You can find more about it
    /// [here](https://razrfalcon.github.io/notes-on-svg-parsing/text/bbox.html).
    ///
    /// `objectBoundingBox` in SVG terms. Meaning it isn't affected by parent transforms.
    ///
    /// Returns `None` when the `text` build feature was disabled.
    /// This is because we have to perform a text layout before calculating a bounding box.
    pub fn bounding_box(&self) -> Rect {
        self.bounding_box
    }

    /// Element's text bounding box in canvas coordinates.
    ///
    /// `userSpaceOnUse` in SVG terms.
    pub fn abs_bounding_box(&self) -> Rect {
        self.abs_bounding_box
    }

    /// Element's object bounding box including stroke.
    ///
    /// Similar to `bounding_box`, but includes stroke.
    ///
    /// Will have the same value as `bounding_box` when path has no stroke.
    pub fn stroke_bounding_box(&self) -> Rect {
        self.stroke_bounding_box
    }

    /// Element's bounding box including stroke in canvas coordinates.
    pub fn abs_stroke_bounding_box(&self) -> Rect {
        self.abs_stroke_bounding_box
    }

    /// Text converted into paths, ready to render.
    ///
    /// Note that this is only a
    /// "best-effort" attempt: The text will be converted into group/paths/image
    /// primitives, so that they can be rendered with the existing infrastructure.
    /// This process is in general lossless and should lead to correct output, with
    /// two notable exceptions:
    /// 1. For glyphs based on the `SVG` table, only glyphs that are pure SVG 1.1/2.0
    ///    are supported. Glyphs that make use of features in the OpenType specification
    ///    that are not part of the original SVG specification are not supported.
    /// 2. For glyphs based on the `COLR` table, there are a certain number of features
    ///    that are not (correctly) supported, such as conical
    ///    gradients, certain gradient transforms and some blend modes. But this shouldn't
    ///    cause any issues in 95% of the cases, as most of those are edge cases.
    ///    If the two above are not acceptable, then you will need to implement your own
    ///    glyph rendering logic based on the layouted glyphs (see the `layouted` method).
    pub fn flattened(&self) -> &Group {
        &self.flattened
    }

    /// The positioned glyphs and decoration spans of the text.
    ///
    /// This should only be used if you need more low-level access
    /// to the glyphs that make up the text. If you just need the
    /// outlines of the text, you should use `flattened` instead.
    #[cfg(feature = "text")]
    pub fn layouted(&self) -> &[Span] {
        &self.layouted
    }

    pub(crate) fn subroots(&self, f: &mut dyn FnMut(&Group)) {
        f(&self.flattened);
    }
}
