use crate::{color::ColorPalettes, GlyphNames};

use super::{
    attribute::Attributes,
    charmap::Charmap,
    color::ColorGlyphCollection,
    instance::{LocationRef, Size},
    metrics::{GlyphMetrics, Metrics},
    outline::OutlineGlyphCollection,
    string::{LocalizedStrings, StringId},
    variation::{AxisCollection, NamedInstanceCollection},
    FontRef,
};
use crate::bitmap::BitmapStrikes;

/// Interface for types that can provide font metadata.
pub trait MetadataProvider<'a>: Sized {
    /// Returns the primary attributes for font classification-- stretch,
    /// style and weight.
    fn attributes(&self) -> Attributes;

    /// Returns the collection of variation axes.
    fn axes(&self) -> AxisCollection<'a>;

    /// Returns the collection of named variation instances.
    fn named_instances(&self) -> NamedInstanceCollection<'a>;

    /// Returns an iterator over the collection of localized strings for the
    /// given informational string identifier.
    fn localized_strings(&self, id: StringId) -> LocalizedStrings<'a>;

    /// Returns the mapping from glyph identifiers to names.
    fn glyph_names(&self) -> GlyphNames<'a>;

    /// Returns the global font metrics for the specified size and location in
    /// normalized variation space.
    fn metrics(&self, size: Size, location: impl Into<LocationRef<'a>>) -> Metrics;

    /// Returns the glyph specific metrics for the specified size and location
    /// in normalized variation space.
    fn glyph_metrics(&self, size: Size, location: impl Into<LocationRef<'a>>) -> GlyphMetrics<'a>;

    /// Returns the character to nominal glyph identifier mapping.
    fn charmap(&self) -> Charmap<'a>;

    /// Returns the collection of scalable glyph outlines.
    ///
    /// If the font contains multiple outline sources, this method prioritizes
    /// `glyf`, `CFF2` and `CFF` in that order. To select a specific outline
    /// source, use the [`OutlineGlyphCollection::with_format`] method.
    fn outline_glyphs(&self) -> OutlineGlyphCollection<'a>;

    /// Returns a collection of paintable color glyphs.
    fn color_glyphs(&self) -> ColorGlyphCollection<'a>;

    /// Returns a collection of color palettes for color glyphs.
    fn color_palettes(&self) -> ColorPalettes<'a>;

    /// Returns a collection of bitmap strikes.
    fn bitmap_strikes(&self) -> BitmapStrikes<'a>;
}

impl<'a> MetadataProvider<'a> for FontRef<'a> {
    /// Returns the primary attributes for font classification-- stretch,
    /// style and weight.
    fn attributes(&self) -> Attributes {
        Attributes::new(self)
    }

    /// Returns the collection of variation axes.
    fn axes(&self) -> AxisCollection<'a> {
        AxisCollection::new(self)
    }

    /// Returns the collection of named variation instances.
    fn named_instances(&self) -> NamedInstanceCollection<'a> {
        NamedInstanceCollection::new(self)
    }

    /// Returns an iterator over the collection of localized strings for the
    /// given informational string identifier.
    fn localized_strings(&self, id: StringId) -> LocalizedStrings<'a> {
        LocalizedStrings::new(self, id)
    }

    /// Returns the mapping from glyph identifiers to names.
    fn glyph_names(&self) -> GlyphNames<'a> {
        GlyphNames::new(self)
    }

    /// Returns the global font metrics for the specified size and location in
    /// normalized variation space.
    fn metrics(&self, size: Size, location: impl Into<LocationRef<'a>>) -> Metrics {
        Metrics::new(self, size, location)
    }

    /// Returns the glyph specific metrics for the specified size and location
    /// in normalized variation space.
    fn glyph_metrics(&self, size: Size, location: impl Into<LocationRef<'a>>) -> GlyphMetrics<'a> {
        GlyphMetrics::new(self, size, location)
    }

    /// Returns the character to nominal glyph identifier mapping.
    fn charmap(&self) -> Charmap<'a> {
        Charmap::new(self)
    }

    /// Returns the collection of scalable glyph outlines.
    ///
    /// If the font contains multiple outline sources, this method prioritizes
    /// `glyf`, `CFF2` and `CFF` in that order. To select a specific outline
    /// source, use the [`OutlineGlyphCollection::with_format`] method.
    fn outline_glyphs(&self) -> OutlineGlyphCollection<'a> {
        OutlineGlyphCollection::new(self)
    }

    // Returns a collection of paintable color glyphs.
    fn color_glyphs(&self) -> ColorGlyphCollection<'a> {
        ColorGlyphCollection::new(self)
    }

    /// Returns a collection of color palettes for color glyphs.
    fn color_palettes(&self) -> ColorPalettes<'a> {
        ColorPalettes::new(self)
    }

    /// Returns a collection of bitmap strikes.
    fn bitmap_strikes(&self) -> BitmapStrikes<'a> {
        BitmapStrikes::new(self)
    }
}
