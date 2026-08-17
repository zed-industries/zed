/*!
Scaling, hinting and rasterization of visual glyph representations.

Scaling is the process of generating an appropriately sized visual
representation of a glyph. The scaler can produce rendered glyph
[images](Image) from outlines, layered color outlines and embedded
bitmaps. Alternatively, you can request raw, optionally hinted
[outlines](Outline) that can then be further processed by [zeno] or
fed into other crates like [lyon](https://github.com/nical/lyon) or
[pathfinder](https://github.com/servo/pathfinder) for tessellation and
GPU rendering.

# Building the scaler

All scaling in this crate takes place within the purview of a
[`ScaleContext`]. This opaque struct manages internal LRU caches and scratch
buffers that are necessary for the scaling process. Generally, you'll
want to keep an instance with your glyph cache, or if doing multithreaded
glyph rasterization, one instance per thread.

The only method available on the context is [`builder`](ScaleContext::builder)
which takes a type that can be converted into a [`FontRef`] as an argument
and produces a [`ScalerBuilder`] that provides options for configuring and
building a [`Scaler`].

Here, we'll create a context and build a scaler for a size of 14px with
hinting enabled:
```
# use swash::{FontRef, CacheKey, scale::*};
# let font: FontRef = FontRef { data: &[], offset: 0, key: CacheKey::new() };
// let font = ...;
let mut context = ScaleContext::new();
let mut scaler = context.builder(font)
    .size(14.)
    .hint(true)
    .build();
```

You can specify variation settings by calling the [`variations`](ScalerBuilder::variations)
method with an iterator that yields a sequence of values that are convertible
to [`Setting<f32>`]. Tuples of (&str, f32) will work in a pinch. For example,
you can request a variation of the weight axis like this:
```
# use swash::{FontRef, CacheKey, scale::*};
# let font: FontRef = FontRef { data: &[], offset: 0, key: CacheKey::new() };
// let font = ...;
let mut context = ScaleContext::new();
let mut scaler = context.builder(font)
    .size(14.)
    .hint(true)
    .variations(&[("wght", 520.5)])
    .build();
```

Alternatively, you can specify variations using the
[`normalized_coords`](ScalerBuilder::normalized_coords) method which takes an iterator
that yields [`NormalizedCoord`]s (a type alias for `i16` which is a fixed point value
in 2.14 format). This method is faster than specifying variations by tag and value, but
the difference is likely negligible outside of microbenchmarks. The real advantage
is that a sequence of `i16` is more compact and easier to fold into a key in a glyph
cache. You can compute these normalized coordinates by using the
[`Variation::normalize`](crate::Variation::normalize) method for each available axis in
the font. The best strategy, however, is to simply capture these during shaping with
the [`Shaper::normalized_coords`](crate::shape::Shaper::normalized_coords) method which
will have already computed them for you.

See [`ScalerBuilder`] for available options and default values.

# Outlines and bitmaps

The [`Scaler`] struct essentially provides direct access to the outlines and embedded
bitmaps that are available in the font. In the case of outlines, it can produce the
raw outline in font units or an optionally hinted, scaled outline. For example, to
extract the raw outline for the letter 'Q':
```
# use swash::{FontRef, CacheKey, scale::*};
# let font: FontRef = FontRef { data: &[], offset: 0, key: CacheKey::new() };
// let font = ...;
let mut context = ScaleContext::new();
let mut scaler = context.builder(font).build();
let glyph_id = font.charmap().map('Q');
let outline = scaler.scale_outline(glyph_id);
```

For the same, but hinted at 12px:
```
# use swash::{FontRef, CacheKey, scale::*};
# let font: FontRef = FontRef { data: &[], offset: 0, key: CacheKey::new() };
// let font = ...;
let mut context = ScaleContext::new();
let mut scaler = context.builder(font)
    .hint(true)
    .size(12.)
    .build();
let glyph_id = font.charmap().map('Q');
let outline = scaler.scale_outline(glyph_id);
```
The [`scale_outline`](Scaler::scale_outline) method returns an [`Outline`] wrapped
in an option. It will return `None` if an outline was not available or if there was
an error during the scaling process. Note that
[`scale_color_outline`](Scaler::scale_color_outline) can be used to access layered
color outlines such as those included in the Microsoft _Segoe UI Emoji_ font. Finally,
the `_into` variants of these methods ([`scale_outline_into`](Scaler::scale_outline_into)
and [`scale_color_outline_into`](Scaler::scale_color_outline_into)) will return
their results in a previously allocated outline avoiding the extra allocations.

Similar to outlines, bitmaps can be retrieved with the [`scale_bitmap`](Scaler::scale_bitmap)
and [`scale_color_bitmap`](Scaler::scale_color_bitmap) for alpha and color bitmaps,
respectively. These methods return an [`Image`] wrapped in an option. The associated
`_into` variants are also available.

Unlike outlines, bitmaps are available in [`strike`](crate::BitmapStrike)s of various sizes.
When requesting a bitmap, you specify the strategy for strike selection using the
[`StrikeWith`] enum.

For example, if we want the largest available unscaled image for the fire emoji:
```
# use swash::{FontRef, CacheKey, scale::*};
# let font: FontRef = FontRef { data: &[], offset: 0, key: CacheKey::new() };
// let font = ...;
let mut context = ScaleContext::new();
let mut scaler = context.builder(font).build();
let glyph_id = font.charmap().map('🔥');
let image = scaler.scale_color_bitmap(glyph_id, StrikeWith::LargestSize);
```

Or, to produce a scaled image for a size of 18px:
```
# use swash::{FontRef, CacheKey, scale::*};
# let font: FontRef = FontRef { data: &[], offset: 0, key: CacheKey::new() };
// let font = ...;
let mut context = ScaleContext::new();
let mut scaler = context.builder(font)
    .size(18.)
    .build();
let glyph_id = font.charmap().map('🔥');
let image = scaler.scale_color_bitmap(glyph_id, StrikeWith::BestFit);
```
This will select the best strike for the requested size and return
a bitmap that is scaled appropriately for an 18px run of text.

Alpha bitmaps should generally be avoided unless you're rendering small East
Asian text where these are sometimes still preferred over scalable outlines. In
this case, you should only use [`StrikeWith::ExactSize`] to select the strike,
falling back to an outline if a bitmap is unavailable.

# Rendering

In the general case of text rendering, you'll likely not care about the specific
details of outlines or bitmaps and will simply want an appropriately sized
image that represents your glyph. For this purpose, you'll want to use the
[`Render`] struct which is a builder that provides options for rendering an image.
This struct is constructed with a slice of [`Source`]s in priority order and
will iterate through them until it finds one that satisfies the request. Typically,
you'll want to use the following order:
```
# use swash::scale::*;
Render::new(&[
    // Color outline with the first palette
    Source::ColorOutline(0),
    // Color bitmap with best fit selection mode
    Source::ColorBitmap(StrikeWith::BestFit),
    // Standard scalable outline
    Source::Outline,
]);
```

The [`Render`] struct offers several options that control rasterization of
outlines such as [`format`](Render::format) for selecting a subpixel rendering mode,
[`offset`](Render::offset) for applying fractional positioning, and others. See the
struct documentation for detail.

After selecting your options, call the [`render`](Render::render) method, passing your
configured [`Scaler`] and the requested glyph identifier to produce an [`Image`].
Let's put it all together by writing a simple function that will render subpixel glyphs
with fractional positioning:
```
# use swash::{scale::{*, image::Image}, FontRef, GlyphId};
fn render_glyph(
    context: &mut ScaleContext,
    font: &FontRef,
    size: f32,
    hint: bool,
    glyph_id: GlyphId,
    x: f32,
    y: f32,
) -> Option<Image> {
    use zeno::{Format, Vector};
    // Build the scaler
    let mut scaler = context.builder(*font).size(size).hint(hint).build();
    // Compute the fractional offset-- you'll likely want to quantize this
    // in a real renderer
    let offset = Vector::new(x.fract(), y.fract());
    // Select our source order
    Render::new(&[
        Source::ColorOutline(0),
        Source::ColorBitmap(StrikeWith::BestFit),
        Source::Outline,
    ])
    // Select a subpixel format
    .format(Format::Subpixel)
    // Apply the fractional offset
    .offset(offset)
    // Render the image
    .render(&mut scaler, glyph_id)
}
```
Note that rendering also takes care of correctly scaling, rasterizing and
compositing layered color outlines for us.

There are other options available for emboldening, transforming with an
affine matrix, and applying path effects. See the methods on [`Render`] for
more detail.
*/

pub mod image;
pub mod outline;

mod bitmap;
mod color;
mod hinting_cache;
mod proxy;

use hinting_cache::HintingCache;
use image::*;
use outline::*;
use skrifa::{
    instance::{NormalizedCoord as SkrifaNormalizedCoord, Size as SkrifaSize},
    outline::OutlineGlyphCollection,
    GlyphId as SkrifaGlyphId, MetadataProvider,
};

use super::internal;
use super::{cache::FontCache, setting::Setting, FontRef, GlyphId, NormalizedCoord};
use alloc::vec::Vec;
use core::borrow::Borrow;
#[cfg(all(feature = "libm", feature = "render"))]
use core_maths::CoreFloat;
use proxy::*;
use zeno::Placement;
#[cfg(feature = "render")]
use zeno::{Format, Mask, Origin, Point, Scratch, Style, Transform, Vector};

pub(crate) use bitmap::decode_png;

/// Index of a color palette.
pub type PaletteIndex = u16;

/// Index of a bitmap strike.
pub type StrikeIndex = u32;

/// Bitmap strike selection mode.
#[derive(Copy, Clone, Debug)]
pub enum StrikeWith {
    /// Load a bitmap only if the exact size is available.
    ExactSize,
    /// Load a bitmap of the best available size.
    BestFit,
    /// Loads a bitmap of the largest size available.
    LargestSize,
    /// Load a bitmap from the specified strike.
    Index(StrikeIndex),
}

/// Glyph sources for the renderer.
#[derive(Copy, Clone, Debug)]
pub enum Source {
    /// Scalable outlines.
    Outline,
    /// Layered color scalable outlines.
    ColorOutline(PaletteIndex),
    /// Embedded alpha bitmaps.
    Bitmap(StrikeWith),
    /// Embedded color bitmaps.
    ColorBitmap(StrikeWith),
}

impl Default for Source {
    fn default() -> Self {
        Self::Outline
    }
}

/// Context that manages caches and scratch buffers for scaling.
///
/// See the module level [documentation](index.html#building-the-scaler) for detail.
pub struct ScaleContext {
    fonts: FontCache<ScalerProxy>,
    state: State,
    hinting_cache: HintingCache,
    coords: Vec<SkrifaNormalizedCoord>,
}

struct State {
    scratch0: Vec<u8>,
    scratch1: Vec<u8>,
    outline: Outline,
    #[cfg(feature = "render")]
    rcx: Scratch,
}

impl ScaleContext {
    /// Creates a new scaling context.
    pub fn new() -> Self {
        Self::with_max_entries(8)
    }

    /// Creates a new scaling context with the specified maximum number of
    /// cache entries.
    pub fn with_max_entries(max_entries: usize) -> Self {
        let max_entries = max_entries.clamp(1, 64);
        Self {
            fonts: FontCache::new(max_entries),
            state: State {
                scratch0: Vec::new(),
                scratch1: Vec::new(),
                outline: Outline::new(),
                #[cfg(feature = "render")]
                rcx: Scratch::new(),
            },
            hinting_cache: HintingCache::default(),
            coords: Vec::new(),
        }
    }

    /// Creates a new builder for constructing a scaler with this context
    /// and the specified font.
    pub fn builder<'a>(&'a mut self, font: impl Into<FontRef<'a>>) -> ScalerBuilder<'a> {
        ScalerBuilder::new(self, font, None)
    }

    /// Creates a new builder for constructing a scaler with this context,
    /// specified font and a custom unique identifier.
    pub fn builder_with_id<'a>(
        &'a mut self,
        font: impl Into<FontRef<'a>>,
        id: [u64; 2],
    ) -> ScalerBuilder<'a> {
        ScalerBuilder::new(self, font, Some(id))
    }
}

impl Default for ScaleContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for configuring a scaler.
pub struct ScalerBuilder<'a> {
    state: &'a mut State,
    hinting_cache: &'a mut HintingCache,
    font: FontRef<'a>,
    outlines: Option<OutlineGlyphCollection<'a>>,
    proxy: &'a ScalerProxy,
    id: [u64; 2],
    coords: &'a mut Vec<SkrifaNormalizedCoord>,
    size: f32,
    hint: bool,
}

impl<'a> ScalerBuilder<'a> {
    fn new(
        context: &'a mut ScaleContext,
        font: impl Into<FontRef<'a>>,
        id: Option<[u64; 2]>,
    ) -> Self {
        let font = font.into();
        let (id, proxy) = context.fonts.get(&font, id, ScalerProxy::from_font);
        let skrifa_font = if font.offset == 0 {
            skrifa::FontRef::new(font.data).ok()
        } else {
            // TODO: make this faster
            let index = crate::FontDataRef::new(font.data)
                .and_then(|font_data| font_data.fonts().position(|f| f.offset == font.offset));
            index.and_then(|index| skrifa::FontRef::from_index(font.data, index as u32).ok())
        };
        let outlines = skrifa_font.map(|font_ref| font_ref.outline_glyphs());
        Self {
            state: &mut context.state,
            hinting_cache: &mut context.hinting_cache,
            font,
            outlines,
            proxy,
            id,
            coords: &mut context.coords,
            size: 0.,
            hint: false,
        }
    }

    /// Specifies the font size in pixels per em. The default value is `0` which will produce
    /// unscaled glyphs in original font units.
    pub fn size(mut self, ppem: f32) -> Self {
        self.size = ppem.max(0.);
        self
    }

    /// Specifies whether to apply hinting to outlines. The default value is `false`.
    pub fn hint(mut self, yes: bool) -> Self {
        self.hint = yes;
        self
    }

    /// Adds variation settings to the scaler.
    pub fn variations<I>(self, settings: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Setting<f32>>,
    {
        if self.proxy.coord_count != 0 {
            let vars = self.font.variations();
            self.coords.resize(vars.len(), Default::default());
            for setting in settings {
                let setting = setting.into();
                for var in vars {
                    if var.tag() == setting.tag {
                        let value = var.normalize(setting.value);
                        if let Some(c) = self.coords.get_mut(var.index()) {
                            *c = SkrifaNormalizedCoord::from_bits(value);
                        }
                    }
                }
            }
        }
        self
    }

    /// Specifies the variation settings in terms of normalized coordinates. This will replace
    /// any previous variation settings.
    pub fn normalized_coords<I>(self, coords: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<NormalizedCoord>,
    {
        self.coords.clear();
        self.coords.extend(
            coords
                .into_iter()
                .map(|c| SkrifaNormalizedCoord::from_bits(*c.borrow())),
        );
        self
    }

    /// Builds a scaler for the current configuration.
    pub fn build(self) -> Scaler<'a> {
        let upem = self.proxy.metrics.units_per_em();
        let skrifa_size = if self.size != 0.0 && upem != 0 {
            SkrifaSize::new(self.size)
        } else {
            SkrifaSize::unscaled()
        };
        let hinting_instance = match (self.hint, &self.outlines) {
            (true, Some(outlines)) => {
                let key = hinting_cache::HintingKey {
                    id: self.id,
                    outlines,
                    size: skrifa_size,
                    coords: self.coords,
                };
                self.hinting_cache.get(&key)
            }
            _ => None,
        };
        Scaler {
            state: self.state,
            font: self.font,
            outlines: self.outlines,
            hinting_instance,
            proxy: self.proxy,
            coords: &self.coords[..],
            size: self.size,
            skrifa_size,
        }
    }
}

/// Scales outline and bitmap glyphs.
///
/// See the module level [documentation](index.html#outlines-and-bitmaps) for detail.
pub struct Scaler<'a> {
    state: &'a mut State,
    font: FontRef<'a>,
    outlines: Option<OutlineGlyphCollection<'a>>,
    hinting_instance: Option<&'a skrifa::outline::HintingInstance>,
    proxy: &'a ScalerProxy,
    coords: &'a [SkrifaNormalizedCoord],
    size: f32,
    skrifa_size: SkrifaSize,
}

impl<'a> Scaler<'a> {
    /// Returns true if scalable glyph outlines are available.
    pub fn has_outlines(&self) -> bool {
        self.outlines
            .as_ref()
            .map(|outlines| outlines.format().is_some())
            .unwrap_or_default()
    }

    /// Scales an outline for the specified glyph into the provided outline.
    pub fn scale_outline_into(&mut self, glyph_id: GlyphId, outline: &mut Outline) -> bool {
        outline.clear();
        self.scale_outline_impl(glyph_id, None, Some(outline))
    }

    /// Scales an outline for the specified glyph.
    pub fn scale_outline(&mut self, glyph_id: GlyphId) -> Option<Outline> {
        let mut outline = Outline::new();
        if self.scale_outline_into(glyph_id, &mut outline) {
            Some(outline)
        } else {
            None
        }
    }

    /// Returns true if scalable color glyph outlines are available.
    pub fn has_color_outlines(&self) -> bool {
        self.proxy.color.colr != 0 && self.proxy.color.cpal != 0
    }

    /// Scales a color outline for the specified glyph into the provided outline.
    pub fn scale_color_outline_into(&mut self, glyph_id: GlyphId, outline: &mut Outline) -> bool {
        outline.clear();
        if !self.has_color_outlines() {
            return false;
        }
        let layers = match self.proxy.color.layers(self.font.data, glyph_id) {
            Some(layers) => layers,
            _ => return false,
        };
        for i in 0..layers.len() {
            let layer = match layers.get(i) {
                Some(layer) => layer,
                _ => return false,
            };
            if !self.scale_outline_impl(layer.glyph_id, layer.color_index, Some(outline)) {
                return false;
            }
        }
        outline.set_color(true);
        true
    }

    /// Scales a color outline for the specified glyph.
    pub fn scale_color_outline(&mut self, glyph_id: GlyphId) -> Option<Outline> {
        let mut outline = Outline::new();
        if self.scale_color_outline_into(glyph_id, &mut outline) {
            Some(outline)
        } else {
            None
        }
    }

    fn scale_outline_impl(
        &mut self,
        glyph_id: GlyphId,
        color_index: Option<u16>,
        outline: Option<&mut Outline>,
    ) -> bool {
        let mut outline = match outline {
            Some(x) => x,
            _ => &mut self.state.outline,
        };
        if let Some(outlines) = &self.outlines {
            if let Some(glyph) = outlines.get(SkrifaGlyphId::from(glyph_id)) {
                outline.begin_layer(color_index);
                let settings: skrifa::outline::DrawSettings =
                    if let Some(hinting_instance) = &self.hinting_instance {
                        (*hinting_instance).into()
                    } else {
                        (
                            self.skrifa_size,
                            skrifa::instance::LocationRef::new(self.coords),
                        )
                            .into()
                    };
                if glyph
                    .draw(settings, &mut OutlineWriter(&mut outline))
                    .is_ok()
                {
                    outline.maybe_close();
                    outline.finish();
                    return true;
                }
            }
        }
        false
    }

    // Unused when render feature is disabled.
    #[allow(dead_code)]
    fn scale_color_outline_impl(&mut self, glyph_id: GlyphId) -> bool {
        if !self.has_color_outlines() {
            return false;
        }
        let layers = match self.proxy.color.layers(self.font.data, glyph_id) {
            Some(layers) => layers,
            _ => return false,
        };
        self.state.outline.clear();
        for i in 0..layers.len() {
            let layer = match layers.get(i) {
                Some(layer) => layer,
                _ => return false,
            };
            if !self.scale_outline_impl(layer.glyph_id, layer.color_index, None) {
                return false;
            }
        }
        true
    }

    /// Returns true if alpha bitmaps are available.
    pub fn has_bitmaps(&self) -> bool {
        self.proxy.bitmaps.has_alpha()
    }

    /// Scales a bitmap for the specified glyph and mode into the provided image.
    pub fn scale_bitmap_into(
        &mut self,
        glyph_id: u16,
        strike: StrikeWith,
        image: &mut Image,
    ) -> bool {
        self.scale_bitmap_impl(glyph_id, false, strike, image) == Some(true)
    }

    /// Scales a bitmap for the specified glyph and mode.
    pub fn scale_bitmap(&mut self, glyph_id: u16, strike: StrikeWith) -> Option<Image> {
        let mut image = Image::new();
        if self.scale_bitmap_into(glyph_id, strike, &mut image) {
            Some(image)
        } else {
            None
        }
    }

    /// Returns true if color bitmaps are available.
    pub fn has_color_bitmaps(&self) -> bool {
        self.proxy.bitmaps.has_color()
    }

    /// Scales a color bitmap for the specified glyph and mode into the provided image.
    pub fn scale_color_bitmap_into(
        &mut self,
        glyph_id: u16,
        strike: StrikeWith,
        image: &mut Image,
    ) -> bool {
        self.scale_bitmap_impl(glyph_id, true, strike, image) == Some(true)
    }

    /// Scales a color bitmap for the specified glyph and mode.
    pub fn scale_color_bitmap(&mut self, glyph_id: u16, strike: StrikeWith) -> Option<Image> {
        let mut image = Image::new();
        if self.scale_color_bitmap_into(glyph_id, strike, &mut image) {
            Some(image)
        } else {
            None
        }
    }

    fn scale_bitmap_impl(
        &mut self,
        glyph_id: GlyphId,
        color: bool,
        strike: StrikeWith,
        image: &mut Image,
    ) -> Option<bool> {
        image.clear();
        let size = self.size;
        let mut strikes = if color {
            self.proxy.bitmaps.materialize_color(&self.font)
        } else {
            self.proxy.bitmaps.materialize_alpha(&self.font)
        };
        let bitmap = match strike {
            StrikeWith::ExactSize => {
                if self.size == 0. {
                    None
                } else {
                    strikes
                        .find_by_exact_ppem(size as u16, glyph_id)?
                        .get(glyph_id)
                }
            }
            StrikeWith::BestFit => {
                if self.size == 0. {
                    None
                } else {
                    strikes
                        .find_by_nearest_ppem(size as u16, glyph_id)?
                        .get(glyph_id)
                }
            }
            StrikeWith::LargestSize => strikes.find_by_largest_ppem(glyph_id)?.get(glyph_id),
            StrikeWith::Index(i) => strikes
                .nth(i as usize)
                .and_then(|strike| strike.get(glyph_id)),
        }?;
        if bitmap.ppem == 0 {
            return None;
        }
        let (_, _, bufsize) = bitmap.scaled_size(size);
        image.data.resize(bufsize, 0);
        self.state.scratch0.clear();
        self.state.scratch1.clear();
        let mut w = bitmap.width;
        let mut h = bitmap.height;
        let scale = size / bitmap.ppem as f32;
        image.placement = if size != 0. && scale != 1. {
            self.state
                .scratch0
                .resize(bitmap.format.buffer_size(w, h), 0);
            w = (w as f32 * scale) as u32;
            h = (h as f32 * scale) as u32;
            image.data.resize(bitmap.format.buffer_size(w, h), 0);
            if !bitmap.decode(Some(&mut self.state.scratch1), &mut self.state.scratch0) {
                return None;
            }
            if !bitmap::resize(
                &self.state.scratch0,
                bitmap.width,
                bitmap.height,
                bitmap.format.channels(),
                &mut image.data,
                w,
                h,
                bitmap::Filter::Mitchell,
                Some(&mut self.state.scratch1),
            ) {
                return None;
            }
            let left = (bitmap.left as f32 * scale) as i32;
            let top = (bitmap.top as f32 * scale) as i32;
            Placement {
                left,
                top,
                width: w,
                height: h,
            }
        } else {
            image.data.resize(bitmap.format.buffer_size(w, h), 0);
            if !bitmap.decode(Some(&mut self.state.scratch1), &mut image.data) {
                return None;
            }
            Placement {
                left: bitmap.left,
                top: bitmap.top,
                width: w,
                height: h,
            }
        };
        image.source = match color {
            true => Source::ColorBitmap(strike),
            false => Source::Bitmap(strike),
        };
        image.content = match bitmap.format.channels() {
            1 => Content::Mask,
            _ => Content::Color,
        };
        // let mut advance = bitmap.advance() as f32;
        // if options.size != 0. && options.size as u16 != bitmap.ppem() {
        //     advance *= options.size / bitmap.ppem() as f32;
        // }
        Some(true)
    }
}

/// Builder type for rendering a glyph into an image.
///
/// See the module level [documentation](index.html#rendering) for detail.
#[cfg(feature = "render")]
pub struct Render<'a> {
    sources: &'a [Source],
    format: Format,
    offset: Point,
    transform: Option<Transform>,
    embolden: f32,
    foreground: [u8; 4],
    style: Style<'a>,
}

#[cfg(feature = "render")]
impl<'a> Render<'a> {
    /// Creates a new builder for configuring rendering using the specified
    /// prioritized list of sources.
    pub fn new(sources: &'a [Source]) -> Self {
        Self {
            sources,
            format: Format::Alpha,
            offset: Point::new(0., 0.),
            transform: None,
            embolden: 0.,
            foreground: [128, 128, 128, 255],
            style: Style::default(),
        }
    }

    /// Specifies the target format for rasterizing an outline. Default is
    /// [`Format::Alpha`].
    pub fn format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }

    /// Specifies the path style to use when rasterizing an outline. Default is
    /// [`Fill::NonZero`](zeno::Fill::NonZero).
    pub fn style(&mut self, style: impl Into<Style<'a>>) -> &mut Self {
        self.style = style.into();
        self
    }

    /// Specifies an additional offset to apply when rasterizing an outline.
    /// Default is `(0, 0)`.
    pub fn offset(&mut self, offset: Vector) -> &mut Self {
        self.offset = offset;
        self
    }

    /// Specifies a transformation matrix to apply when rasterizing an
    /// outline. Default is `None`.
    pub fn transform(&mut self, transform: Option<Transform>) -> &mut Self {
        self.transform = transform;
        self
    }

    /// Specifies the strength of a faux bold transform to apply when
    /// rasterizing an outline. Default is `0`.
    pub fn embolden(&mut self, strength: f32) -> &mut Self {
        self.embolden = strength;
        self
    }

    /// Specifies an RGBA color to use when rasterizing layers of a color
    /// outline that do not directly reference a palette color. Default is
    /// `[128, 128, 128, 255]`.
    pub fn default_color(&mut self, color: [u8; 4]) -> &mut Self {
        self.foreground = color;
        self
    }

    /// Renders the specified glyph using the current configuration into the
    /// provided image.
    pub fn render_into(&self, scaler: &mut Scaler, glyph_id: GlyphId, image: &mut Image) -> bool {
        for source in self.sources {
            match source {
                Source::Outline => {
                    if !scaler.has_outlines() {
                        continue;
                    }
                    scaler.state.outline.clear();
                    if scaler.scale_outline_impl(glyph_id, None, None) {
                        let state = &mut scaler.state;
                        let rcx = &mut state.rcx;
                        let outline = &mut state.outline;
                        if self.embolden != 0. {
                            outline.embolden(self.embolden, self.embolden);
                        }
                        if let Some(transform) = &self.transform {
                            outline.transform(transform);
                        }
                        let placement = Mask::with_scratch(outline.path(), rcx)
                            .format(self.format)
                            .origin(Origin::BottomLeft)
                            .style(self.style)
                            .offset(self.offset)
                            .render_offset(self.offset)
                            .inspect(|fmt, w, h| {
                                image.data.resize(fmt.buffer_size(w, h), 0);
                            })
                            .render_into(&mut image.data[..], None);
                        image.placement = placement;
                        image.content = if self.format == Format::Alpha {
                            Content::Mask
                        } else {
                            Content::SubpixelMask
                        };
                        image.source = Source::Outline;
                        return true;
                    }
                }
                Source::ColorOutline(palette_index) => {
                    if !scaler.has_color_outlines() {
                        continue;
                    }
                    scaler.state.outline.clear();
                    if scaler.scale_color_outline_impl(glyph_id) {
                        let font = &scaler.font;
                        let proxy = &scaler.proxy;
                        let state = &mut scaler.state;
                        let scratch = &mut state.scratch0;
                        let rcx = &mut state.rcx;
                        let outline = &mut state.outline;
                        // Cool effect, but probably not generally desirable.
                        // Maybe expose a separate option?
                        // if self.embolden != 0. {
                        //     outline.embolden(self.embolden, self.embolden);
                        // }
                        if let Some(transform) = &self.transform {
                            outline.transform(transform);
                        }
                        let palette = proxy.color.palette(font, *palette_index);

                        let total_bounds = outline.bounds();

                        // need to take offset into account when placing glyph
                        let base_x = (total_bounds.min.x + self.offset.x).floor() as i32;
                        let base_y = (total_bounds.min.y + self.offset.y).ceil() as i32;
                        let base_w = total_bounds.width().ceil() as u32;
                        let base_h = total_bounds.height().ceil() as u32;

                        image.data.resize((base_w * base_h * 4) as usize, 0);
                        image.placement.left = base_x;
                        image.placement.top = base_h as i32 + base_y;
                        image.placement.width = total_bounds.width().ceil() as u32;
                        image.placement.height = total_bounds.height().ceil() as u32;

                        let mut ok = true;
                        for i in 0..outline.len() {
                            let layer = match outline.get(i) {
                                Some(layer) => layer,
                                _ => {
                                    ok = false;
                                    break;
                                }
                            };

                            scratch.clear();
                            let placement = Mask::with_scratch(layer.path(), rcx)
                                .origin(Origin::BottomLeft)
                                .style(self.style)
                                .offset(self.offset)
                                .render_offset(self.offset)
                                .inspect(|fmt, w, h| {
                                    scratch.resize(fmt.buffer_size(w, h), 0);
                                })
                                .render_into(&mut scratch[..], None);
                            let color = layer
                                .color_index()
                                .and_then(|i| palette.map(|p| p.get(i)))
                                .unwrap_or(self.foreground);
                            bitmap::blit(
                                &scratch[..],
                                placement.width,
                                placement.height,
                                placement.left.wrapping_sub(base_x),
                                (base_h as i32 + base_y).wrapping_sub(placement.top),
                                color,
                                &mut image.data,
                                base_w,
                                base_h,
                            );
                        }
                        if ok {
                            image.source = Source::ColorOutline(*palette_index);
                            image.content = Content::Color;
                            return true;
                        }
                    }
                }
                Source::Bitmap(mode) => {
                    if !scaler.has_bitmaps() {
                        continue;
                    }
                    if scaler.scale_bitmap_into(glyph_id, *mode, image) {
                        return true;
                    }
                }
                Source::ColorBitmap(mode) => {
                    if !scaler.has_color_bitmaps() {
                        continue;
                    }
                    if scaler.scale_color_bitmap_into(glyph_id, *mode, image) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Renders the specified glyph using the current configuration.
    pub fn render(&self, scaler: &mut Scaler, glyph_id: GlyphId) -> Option<Image> {
        let mut image = Image::new();
        if self.render_into(scaler, glyph_id, &mut image) {
            Some(image)
        } else {
            None
        }
    }
}
