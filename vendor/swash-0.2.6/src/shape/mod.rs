/*!
Mapping complex text to a sequence of positioned glyphs.

Shaping is the process of converting a sequence of
[character clusters](CharCluster) into a sequence of
[glyph clusters](GlyphCluster) with respect to the rules of a particular
writing system and the typographic features available in a font. The shaper
operates on one _item_ at a time where an item is a run of text with
a single script, language, direction, font, font size, and set of variation/feature
settings. The process of producing these runs is called _itemization_
and is out of scope for this crate.

# Building the shaper

All shaping in this crate takes place within the purview of a
[`ShapeContext`]. This opaque struct manages internal LRU caches and scratch
buffers that are necessary for the shaping process. Generally, you'll
want to keep an instance that persists for more than one layout pass as
this amortizes the cost of allocations, reduces contention for the global
heap and increases the hit rate for the internal acceleration structures. If
you're doing multithreaded layout, you should keep a context per thread.

The only method available on the context is [`builder`](ShapeContext::builder)
which takes a type that can be converted into a [`FontRef`] as an argument
and produces a [`ShaperBuilder`] that provides options for configuring and
building a [`Shaper`].

Here, we'll create a context and build a shaper for Arabic text at 16px:
```
# use swash::{FontRef, CacheKey, shape::*, text::Script};
# let font: FontRef = FontRef { data: &[], offset: 0, key: CacheKey::new() };
// let font = ...;
let mut context = ShapeContext::new();
let mut shaper = context.builder(font)
    .script(Script::Arabic)
    .direction(Direction::RightToLeft)
    .size(16.)
    .build();
```

You can specify feature settings by calling the [`features`](ShaperBuilder::features)
method with an iterator that yields a sequence of values that are convertible
to [`Setting<u16>`]. Tuples of (&str, u16) will work in a pinch. For example,
you can enable discretionary ligatures like this:
```
# use swash::{FontRef, CacheKey, shape::*, text::Script, tag_from_bytes};
# let font: FontRef = FontRef { data: &[], offset: 0, key: CacheKey::new() };
// let font = ...;
let mut context = ShapeContext::new();
let mut shaper = context.builder(font)
    .script(Script::Latin)
    .size(14.)
    .features(&[("dlig", 1)])
    .build();
```

A value of `0` will disable a feature while a non-zero value will enable it.
Some features use non-zero values as an argument. The stylistic alternates
feature, for example, often offers a collection of choices per glyph. The argument
is used as an index to select among them. If a requested feature is not present
in a font, the setting is ignored.

Font variation settings are specified in a similar manner with the
[`variations`](ShaperBuilder::variations) method but take an `f32`
to define the value within the variation space for the requested axis:
```
# use swash::{FontRef, CacheKey, shape::*, text::Script, tag_from_bytes};
# let font: FontRef = FontRef { data: &[], offset: 0, key: CacheKey::new() };
// let font = ...;
let mut context = ShapeContext::new();
let mut shaper = context.builder(font)
    .script(Script::Latin)
    .size(14.)
    .variations(&[("wght", 520.5)])
    .build();
```

See [`ShaperBuilder`] for available options and default values.

# Feeding the shaper

Once we have a properly configured shaper, we need to feed it some
clusters. The simplest approach is to call the [`add_str`](Shaper::add_str)
method with a string:
```
# use swash::{FontRef, CacheKey, shape::*, text::Script, tag_from_bytes};
# let font: FontRef = FontRef { data: &[], offset: 0, key: CacheKey::new() };
# let mut context = ShapeContext::new();
# let mut shaper = context.builder(font).build();
shaper.add_str("a quick brown fox?");
```

You can call [`add_str`](Shaper::add_str) multiple times to add a sequence
of text fragments to the shaper.

This simple approach is certainly reasonable when dealing with text consisting
of a single run on one line with a font that is known to contain all the
necessary glyphs. A small text label in a UI is a good example.

For more complex scenarios, the shaper can be fed a single cluster at a time.
This method allows you to provide:
- accurate source ranges per character even if your runs
  and items span multiple non-contiguous fragments
- user data per character (a single `u32`) that can be used, for
  example, to associate each resulting glyph with a style span
- boundary analysis per character, carrying word boundaries and
  line break opportunities through the shaper.

This also provides a junction point for inserting a font fallback
mechanism.

All of this is served by the functionality in the
[`text::cluster`](crate::text::cluster) module.

Let's see a somewhat contrived example that demonstrates the process:
```
use swash::text::cluster::{CharCluster, CharInfo, Parser, Token};
# use swash::{FontRef, CacheKey, shape::*, text::Script, tag_from_bytes};
# let font: FontRef = FontRef { data: &[], offset: 0, key: CacheKey::new() };
# let mut context = ShapeContext::new();
let mut shaper = context.builder(font)
    .script(Script::Latin)
    .build();
// We'll need the character map for our font
let charmap = font.charmap();
// And some storage for the cluster we're working with
let mut cluster = CharCluster::new();
// Now we build a cluster parser which takes a script and
// an iterator that yields a Token per character
let mut parser = Parser::new(
    Script::Latin,
    "a quick brown fox?".char_indices().map(|(i, ch)| Token {
        // The character
        ch,
        // Offset of the character in code units
        offset: i as u32,
        // Length of the character in code units
        len: ch.len_utf8() as u8,
        // Character information
        info: ch.into(),
        // Pass through user data
        data: 0,
    })
);
// Loop over all of the clusters
while parser.next(&mut cluster) {
    // Map all of the characters in the cluster
    // to nominal glyph identifiers
    cluster.map(|ch| charmap.map(ch));
    // Add the cluster to the shaper
    shaper.add_cluster(&cluster);
}
```

Phew! That's quite a lot of work. It also happens to be exactly what
[`add_str`](Shaper::add_str) does internally.

So why bother? As mentioned earlier, this method allows you to customize
the per-character data that passes through the shaper. Is your source text in
UTF-16 instead of UTF-8? No problem. Set the [`offset`](Token::offset) and
[`len`](Token::len) fields of your [`Token`]s to appropriate values. Are you shaping
across style spans? Set the [`data`](Token::data) field to the index of your span so
it can be recovered. Have you used the
[`Analyze`](crate::text::Analyze) iterator to generate
[`CharInfo`](crate::text::cluster::CharInfo)s containing boundary analysis? This
is where you apply them to the [`info`](Token::info) fields of your [`Token`]s.

That last one deserves a quick example, showing how you might build a cluster
parser with boundary analysis:
```
use swash::text::{analyze, Script};
use swash::text::cluster::{CharInfo, Parser, Token};
let text = "a quick brown fox?";
let mut parser = Parser::new(
    Script::Latin,
    text.char_indices()
        // Call analyze passing the same text and zip
        // the results
        .zip(analyze(text.chars()))
        // Analyze yields the tuple (Properties, Boundary)
        .map(|((i, ch), (props, boundary))| Token {
            ch,
            offset: i as u32,
            len: ch.len_utf8() as u8,
            // Create character information from properties and boundary
            info: CharInfo::new(props, boundary),
            data: 0,
        }),
);
```
That leaves us with font fallback. This crate does not provide the infrastructure
for such, but a small example can demonstrate the idea. The key is in
the return value of the [`CharCluster::map`] method which describes the
[`Status`](crate::text::cluster::Status) of the mapping operation. This function
will return the index of the best matching font:
```
use swash::FontRef;
use swash::text::cluster::{CharCluster, Status};

fn select_font<'a>(fonts: &[FontRef<'a>], cluster: &mut CharCluster) -> Option<usize> {
    let mut best = None;
    for (i, font) in fonts.iter().enumerate() {
        let charmap = font.charmap();
        match cluster.map(|ch| charmap.map(ch)) {
            // This font provided a glyph for every character
            Status::Complete => return Some(i),
            // This font provided the most complete mapping so far
            Status::Keep => best = Some(i),
            // A previous mapping was more complete
            Status::Discard => {}
        }
    }
    best
}
```

Note that [`CharCluster`] maintains internal composed and decomposed sequences
of the characters in the cluster so that it can select the best form for each
candidate font.

Since this process is done during shaping, upon return we compare the selected
font with our current font and if they're different, we complete shaping for the
clusters submitted so far and continue the process by building a new shaper with
the selected font. By doing manual cluster parsing and nominal glyph mapping
_outside_ the shaper, we can implement per-cluster font fallback without the costly
technique of heuristically shaping runs.

# Collecting the prize

Finish up shaping by calling [`Shaper::shape_with`] with a closure that will be
invoked with each resulting [`GlyphCluster`]. This structure contains borrowed data
and thus cannot be stored directly. The data you extract from each cluster and the
method in which you store it will depend entirely on the design of your text layout
system.

Please note that, unlike HarfBuzz, this shaper does _not_ reverse runs that are in
right-to-left order. The reasoning is that, for correctness, line breaking must be
done in logical order and reversing runs should occur during bidi reordering.

Also pertinent to right-to-left runs: you'll need to ensure that you reverse
_clusters_ and not _glyphs_. Intra-cluster glyphs must remain in logical order
for proper mark placement.
*/

pub mod cluster;

#[doc(hidden)]
pub mod partition;

mod aat;
mod at;
mod buffer;
mod cache;
mod engine;
mod feature;

use cluster::*;

use super::{
    cache::FontCache, charmap::Charmap, internal, metrics::Metrics, setting::Setting, FontRef,
    NormalizedCoord,
};
use crate::text::{
    cluster::{CharCluster, Parser, ShapeClass, Token},
    Language, Script,
};
use alloc::vec::Vec;
use at::{FeatureMask, FeatureStore, FeatureStoreBuilder};
use buffer::*;
use cache::{FeatureCache, FontEntry};
use core::borrow::Borrow;
use engine::{Engine, EngineMode};

const DEFAULT_SIZE: usize = 16;

/// Text direction.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
}

/// Context that manages caches and transient buffers for shaping.
///
/// See the module level [documentation](index.html#building-the-shaper) for detail.
pub struct ShapeContext {
    font_cache: FontCache<FontEntry>,
    feature_cache: FeatureCache,
    coords: Vec<i16>,
    state: State,
}

impl ShapeContext {
    /// Creates a new shaping context.
    pub fn new() -> Self {
        Self::with_max_entries(DEFAULT_SIZE)
    }

    /// Creates a new shaping context with the specified maximum number of
    /// cache entries.
    pub fn with_max_entries(max_entries: usize) -> Self {
        let max_entries = max_entries.clamp(1, 64);
        Self {
            font_cache: FontCache::new(max_entries),
            feature_cache: FeatureCache::new(max_entries),
            coords: Vec::new(),
            state: State::new(),
        }
    }

    /// Creates a new builder for constructing a shaper with this context
    /// and the specified font.
    pub fn builder<'a>(&'a mut self, font: impl Into<FontRef<'a>>) -> ShaperBuilder<'a> {
        ShaperBuilder::new(self, font)
    }

    /// Creates a new builder for constructing a shaper with this context
    /// and the specified font.
    pub fn builder_with_id<'a>(
        &'a mut self,
        font: impl Into<FontRef<'a>>,
        id: [u64; 2],
    ) -> ShaperBuilder<'a> {
        ShaperBuilder::new_with_id(self, font, id)
    }
}

impl Default for ShapeContext {
    fn default() -> Self {
        Self::new()
    }
}

struct State {
    buffer: Buffer,
    store_builder: FeatureStoreBuilder,
    order: Vec<usize>,
    glyphs: Vec<GlyphData>,
    disable_kern: bool,
    features: Vec<(u32, u16)>,
    selectors: Vec<(u16, u16)>,
}

impl State {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            store_builder: FeatureStoreBuilder::default(),
            order: Vec::new(),
            glyphs: Vec::new(),
            disable_kern: false,
            features: Vec::new(),
            selectors: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.features.clear();
        self.disable_kern = false;
    }
}

/// Builder for configuring a shaper.
///
/// See the module level [documentation](index.html#building-the-shaper) for more detail.
pub struct ShaperBuilder<'a> {
    state: &'a mut State,
    feature_cache: &'a mut FeatureCache,
    font: FontRef<'a>,
    font_id: [u64; 2],
    font_entry: &'a FontEntry,
    coords: &'a mut Vec<i16>,
    charmap: Charmap<'a>,
    dotted_circle: Option<u16>,
    retain_ignorables: bool,
    size: f32,
    script: Script,
    lang: Option<Language>,
    dir: Direction,
}

impl<'a> ShaperBuilder<'a> {
    /// Creates a new builder for configuring a shaper with the specified
    /// context and font.
    fn new(context: &'a mut ShapeContext, font: impl Into<FontRef<'a>>) -> Self {
        let font = font.into();
        let id = [font.key.value(), u64::MAX];
        Self::new_with_id(context, font, id)
    }

    /// Creates a new builder for configuring a shaper with the specified
    /// context and font.
    fn new_with_id(
        context: &'a mut ShapeContext,
        font: impl Into<FontRef<'a>>,
        id: [u64; 2],
    ) -> Self {
        let font = font.into();
        let (font_id, font_entry) = context.font_cache.get(&font, Some(id), FontEntry::new);
        context.state.reset();
        context.coords.clear();
        Self {
            state: &mut context.state,
            feature_cache: &mut context.feature_cache,
            font,
            font_id,
            font_entry,
            coords: &mut context.coords,
            charmap: font_entry.charmap.materialize(&font),
            dotted_circle: None,
            retain_ignorables: false,
            size: 0.,
            script: Script::Latin,
            lang: None,
            dir: Direction::LeftToRight,
        }
    }

    /// Specifies the script. The default value is [`Script::Latin`].
    pub fn script(mut self, script: Script) -> Self {
        self.script = script;
        self
    }

    /// Specifies the language. The default value is `None`.
    pub fn language(mut self, language: Option<Language>) -> Self {
        self.lang = language;
        self
    }

    /// Specifies the text direction. The default value is [`Direction::LeftToRight`].
    pub fn direction(mut self, direction: Direction) -> Self {
        self.dir = direction;
        self
    }

    /// Specifies the font size in pixels per em. The default value is `0`
    /// which will produce glyphs with offsets and advances in font units.
    pub fn size(mut self, ppem: f32) -> Self {
        self.size = ppem.max(0.);
        self
    }

    /// Adds feature settings to the shaper.
    pub fn features<I>(self, settings: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Setting<u16>>,
    {
        for feature in settings {
            let feature = feature.into();
            if feature.tag == feature::KERN {
                self.state.disable_kern = feature.value == 0;
            }
            self.state.features.push((feature.tag, feature.value));
        }
        self
    }

    /// Adds variation settings to the shaper.
    pub fn variations<I>(self, settings: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Setting<f32>>,
    {
        if self.font_entry.coord_count != 0 {
            let vars = self.font.variations();
            self.coords.resize(vars.len(), 0);
            for setting in settings {
                let setting = setting.into();
                for var in vars {
                    if var.tag() == setting.tag {
                        let value = var.normalize(setting.value);
                        if let Some(c) = self.coords.get_mut(var.index()) {
                            *c = value;
                        }
                    }
                }
            }
        }
        self
    }

    /// Specifies the variation settings in terms of normalized coordinates.
    pub fn normalized_coords<I>(self, coords: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<NormalizedCoord>,
    {
        self.coords.clear();
        self.coords.extend(coords.into_iter().map(|c| *c.borrow()));
        self
    }

    /// Specifies whether to insert dotted circles for broken clusters. The
    /// default value is `false`.
    pub fn insert_dotted_circles(mut self, yes: bool) -> Self {
        if yes {
            let gid = self.charmap.map('\u{25cc}');
            if gid != 0 {
                self.dotted_circle = Some(gid);
            }
        } else {
            self.dotted_circle = None;
        }
        self
    }

    /// Specifies whether characters defined as default ignorable should be
    /// retained by the shaper. The default is `false`.
    pub fn retain_ignorables(mut self, yes: bool) -> Self {
        self.retain_ignorables = yes;
        self
    }

    /// Builds a shaper for the current configuration.
    pub fn build(self) -> Shaper<'a> {
        let engine = Engine::new(
            &self.font_entry.metadata,
            self.font.data,
            &self.coords[..],
            self.script,
            self.lang,
        );
        self.state.buffer.dotted_circle = self.dotted_circle;
        let rtl = self.dir == Direction::RightToLeft;
        self.state.buffer.is_rtl = rtl;
        let (store, sub_mask, pos_mask) = if engine.use_ot {
            use cache::FeatureCacheEntry;
            let store = match self.feature_cache.entry(
                self.font_id,
                &self.coords[..],
                engine.has_feature_vars(),
                engine.tags(),
            ) {
                FeatureCacheEntry::Present(store) => store,
                FeatureCacheEntry::New(store) => {
                    engine.collect_features(&mut self.state.store_builder, store);
                    store
                }
            };
            let buf = &mut self.state.buffer;
            let (sub, pos) = store.custom_masks(
                &self.state.features[..],
                &mut buf.sub_args,
                &mut buf.pos_args,
                self.dir,
            );
            (Some(store as _), sub, pos)
        } else {
            (None, FeatureMask::default(), FeatureMask::default())
        };
        Shaper {
            state: self.state,
            font: self.font,
            font_entry: self.font_entry,
            charmap: self.charmap,
            retain_ignorables: self.retain_ignorables,
            size: self.size,
            script: self.script,
            joined: engine.use_ot && self.script.is_joined(),
            dir: self.dir,
            engine,
            store,
            sub_mask,
            pos_mask,
        }
    }
}

/// Maps character clusters to positioned glyph clusters according to
/// typographic rules and features.
///
/// See the module level [documentation](index.html#feeding-the-shaper) for detail.
pub struct Shaper<'a> {
    state: &'a mut State,
    font: FontRef<'a>,
    font_entry: &'a FontEntry,
    charmap: Charmap<'a>,
    retain_ignorables: bool,
    size: f32,
    script: Script,
    joined: bool,
    dir: Direction,
    engine: Engine<'a>,
    store: Option<&'a FeatureStore>,
    sub_mask: FeatureMask,
    pos_mask: FeatureMask,
}

impl<'a> Shaper<'a> {
    /// Adds a character cluster to the shaper.
    pub fn add_cluster(&mut self, cluster: &CharCluster) {
        let buf = &mut self.state.buffer;
        match self.engine.mode {
            EngineMode::Simple => {
                buf.push(cluster);
            }
            EngineMode::Myanmar => {
                let e = &mut self.engine;
                let s = self.store.unwrap();
                let chars = cluster.mapped_chars();
                reorder_myanmar(chars, &mut self.state.order);
                let range = buf.push_order(cluster, &self.state.order);
                e.set_classes(buf, Some(range.clone()));
                let start = range.start;
                e.gsub(s, s.groups.default, buf, Some(range));
                let end = buf.len();
                e.gsub(s, s.groups.reph, buf, Some(start..end));
                let end = buf.len();
                e.gsub(s, s.groups.pref, buf, Some(start..end));
                let end = buf.len();
                e.gsub(s, s.groups.stage1, buf, Some(start..end));
                let end = buf.len();
                e.gsub(s, s.groups.stage2, buf, Some(start..end));
            }
            EngineMode::Complex => {
                let e = &mut self.engine;
                let s = self.store.unwrap();
                let range = buf.push(cluster);
                e.set_classes(buf, Some(range.clone()));
                let start = range.start;
                // Default group
                e.gsub(s, s.groups.default, buf, Some(range.clone()));
                for g in &mut buf.glyphs[range] {
                    if g.char_class == ShapeClass::Halant && g.flags & SUBSTITUTED != 0 {
                        // Don't prevent reordering across a virama that has been substituted
                        g.char_class = ShapeClass::Other;
                    }
                }
                // Reph identification
                let len = 3.min(buf.glyphs.len() - start);
                let end = start + len;
                buf.clear_flags(buffer::SUBSTITUTED, Some(start..end));
                e.gsub(s, s.groups.reph, buf, Some(start..end));
                for g in &mut buf.glyphs[start..end] {
                    if g.flags & buffer::SUBSTITUTED != 0 {
                        g.char_class = ShapeClass::Reph;
                        break;
                    }
                }
                // Pref identification
                let end = buf.len();
                buf.clear_flags(buffer::SUBSTITUTED, Some(start..end));
                e.gsub(s, s.groups.pref, buf, Some(start..end));
                for g in &mut buf.glyphs[start..end] {
                    if g.flags & buffer::SUBSTITUTED != 0 {
                        g.char_class = ShapeClass::Pref;
                        break;
                    }
                }
                // Orthographic group
                let end = buf.len();
                e.gsub(s, s.groups.stage1, buf, Some(start..end));
                // Reordering
                let len = (buf.len() - start).min(64);
                let end = start + len;
                reorder_complex(
                    &mut buf.glyphs[start..end],
                    &mut self.state.glyphs,
                    &mut self.state.order,
                );
            }
        }
    }

    /// Adds a string to the shaper.
    pub fn add_str(&mut self, s: &str) {
        use crate::text::Codepoint;
        let mut cluster = CharCluster::new();
        let mut parser = Parser::new(
            self.script,
            s.char_indices().map(|(i, ch)| Token {
                ch,
                offset: i as u32,
                len: ch.len_utf8() as u8,
                info: ch.properties().into(),
                data: 0,
            }),
        );
        let charmap = self.charmap;
        while parser.next(&mut cluster) {
            cluster.map(|ch| charmap.map(ch));
            self.add_cluster(&cluster);
        }
    }

    /// Returns the current normalized variation coordinates in use by the
    /// shaper.
    pub fn normalized_coords(&self) -> &[NormalizedCoord] {
        self.engine.coords
    }

    /// Returns the current font metrics in use by the shaper.
    pub fn metrics(&self) -> Metrics {
        let scale = if self.size != 0. { self.size } else { 1. };
        self.font_entry
            .metrics
            .materialize_metrics(&self.font, self.engine.coords)
            .scale(scale)
    }

    /// Shapes the text and invokes the specified closure with each
    /// resulting glyph cluster.
    pub fn shape_with(mut self, mut f: impl FnMut(&GlyphCluster)) {
        self.finish();
        let buf = &mut self.state.buffer;
        buf.shaped_glyphs.clear();
        let mut sentinel = (
            buffer::GlyphData::default(),
            buffer::PositionData::default(),
        );
        sentinel.0.cluster = buf.ranges.len() as u32;
        let mut last_cluster = 0;
        for (g, p) in buf
            .glyphs
            .iter()
            .zip(&buf.positions)
            .chain(core::iter::once((&sentinel.0, &sentinel.1)))
        {
            if g.cluster != last_cluster {
                // Simple and common case: no ligatures and no empty clusters.
                if last_cluster > g.cluster || g.cluster - last_cluster == 1 {
                    let index = last_cluster as usize;
                    let info = &buf.infos[index];
                    let cluster = GlyphCluster {
                        source: buf.ranges[index],
                        info: info.0,
                        glyphs: &buf.shaped_glyphs,
                        components: &[],
                        data: info.2,
                    };
                    f(&cluster);
                    buf.shaped_glyphs.clear();
                } else {
                    // Collect the range for the non-empty cluster.
                    let end = g.cluster as usize;
                    let start = last_cluster as usize;
                    let mut group_end = start + 1;
                    while group_end < end && buf.infos[group_end].1 {
                        group_end += 1;
                    }
                    if !buf.shaped_glyphs.is_empty() {
                        // We have some glyphs. Emit the cluster.
                        let mut source = buf.ranges[start];
                        source.end = buf.ranges[group_end - 1].end;
                        // If the range spans more than one cluster, we have a ligature.
                        let components = if group_end > start + 1 {
                            &buf.ranges[start..group_end]
                        } else {
                            &[]
                        };
                        let info = &buf.infos[start];
                        let cluster = GlyphCluster {
                            source,
                            info: info.0,
                            glyphs: &buf.shaped_glyphs,
                            components,
                            data: info.2,
                        };
                        f(&cluster);
                        buf.shaped_glyphs.clear();
                    }
                    if end > group_end {
                        // We have a trailing sequence of empty clusters. Emit
                        // them one by one.
                        for (info, source) in buf.infos[group_end..end]
                            .iter()
                            .zip(&buf.ranges[group_end..end])
                        {
                            let cluster = GlyphCluster {
                                source: *source,
                                info: info.0,
                                glyphs: &[],
                                components: &[],
                                data: info.2,
                            };
                            f(&cluster);
                        }
                    }
                }
            }
            last_cluster = g.cluster;
            if self.retain_ignorables || g.flags & IGNORABLE == 0 {
                buf.shaped_glyphs.push(Glyph::new(g, p));
            }
        }
    }

    // FIXME: when writing docs, I realized that it's impossible
    // to use the result of this function correctly with RTL runs
    // that contain marks.

    // /// Shapes the text and invokes the specified closure with each
    // /// resulting glyph.
    // pub fn shape_glyphs_with(mut self, mut f: impl FnMut(&Glyph)) {
    //     self.finish();
    //     let buf = &self.state.buffer;
    //     for (g, p) in buf.glyphs.iter().zip(&buf.positions) {
    //         if g.flags & IGNORABLE == 0 {
    //             f(&Glyph::new(g, p))
    //         }
    //     }
    // }

    fn finish(&mut self) {
        use engine::{PosMode, SubMode};
        if self.state.buffer.glyphs.is_empty() {
            return;
        }
        let e = &mut self.engine;
        let buf = &mut self.state.buffer;
        match e.mode {
            EngineMode::Simple => match e.sub_mode {
                SubMode::Gsub => {
                    let s = self.store.unwrap();
                    e.set_classes(buf, None);
                    if self.joined {
                        buf.set_join_masks();
                    }
                    e.gsub(s, self.sub_mask, buf, None);
                }
                SubMode::Morx => {
                    e.collect_selectors(&self.state.features, &mut self.state.selectors);
                    e.morx(buf, &self.state.selectors);
                }
                _ => {}
            },
            EngineMode::Myanmar => {
                let s = self.store.unwrap();
                e.gsub(s, self.sub_mask | s.groups.stage2, buf, None);
            }
            EngineMode::Complex => {
                let s = self.store.unwrap();
                if self.joined {
                    buf.set_join_masks();
                    e.gsub(s, s.groups.stage2 | self.sub_mask, buf, None);
                } else {
                    e.gsub(s, self.sub_mask, buf, None);
                }
            }
        }
        buf.setup_positions(e.sub_mode == SubMode::Morx);
        match e.pos_mode {
            PosMode::Gpos => {
                let s = self.store.unwrap();
                e.gpos(s, self.pos_mask, buf, None);
            }
            PosMode::Kerx => {
                e.kerx(buf, self.state.disable_kern);
            }
            PosMode::Kern => {
                if !self.state.disable_kern {
                    e.kern(buf);
                }
            }
            _ => {}
        }
        // let metrics = self
        //     .font_entry
        //     .metrics
        //     .materialize_metrics(self.font.data, self.engine.coords);
        let glyph_metrics = self
            .font_entry
            .metrics
            .materialize_glyph_metrics(&self.font, self.engine.coords);
        for (g, p) in buf.glyphs.iter_mut().zip(buf.positions.iter_mut()) {
            if g.flags & MARK_ATTACH == 0 {
                p.advance += glyph_metrics.advance_width(g.id);
            }
            g.flags |= p.flags;
        }
        if buf.has_cursive {
            if self.dir == Direction::RightToLeft {
                for (i, g) in buf.glyphs.iter().enumerate().rev() {
                    if g.flags & buffer::CURSIVE_ATTACH != 0 {
                        let base_offset = buf.positions[i].base as usize;
                        if base_offset != 0 {
                            let (x, y) = {
                                let base = &buf.positions[i + base_offset];
                                (base.x, base.y)
                            };
                            let pos = &mut buf.positions[i];
                            pos.x += x;
                            pos.y += y;
                        }
                    }
                }
            } else {
                for (i, g) in buf.glyphs.iter().enumerate() {
                    if g.flags & buffer::CURSIVE_ATTACH != 0 {
                        let base_offset = buf.positions[i].base as usize;
                        if base_offset != 0 {
                            let (x, y) = {
                                let base = &buf.positions[i + base_offset];
                                (base.x, base.y)
                            };
                            let pos = &mut buf.positions[i];
                            pos.x += x;
                            pos.y += y;
                        }
                    }
                }
            }
        }
        if buf.has_marks {
            fn round_f32(f: f32) -> f32 {
                f
            }
            for (i, g) in buf.glyphs.iter().enumerate() {
                if g.flags & buffer::MARK_ATTACH != 0 {
                    let base_offset = buf.positions[i].base as usize;
                    if base_offset != 0 {
                        let (x, y) = {
                            let base = &buf.positions[i - base_offset];
                            (base.x - round_f32(base.advance), base.y)
                        };
                        let pos = &mut buf.positions[i];
                        pos.x += x;
                        pos.y += y;
                    }
                }
            }
        }
        let upem = glyph_metrics.units_per_em();
        if self.size != 0. && upem != 0 {
            let s = self.size / upem as f32;
            for p in buf.positions.iter_mut() {
                p.x *= s;
                p.y *= s;
                p.advance *= s;
            }
        }
    }
}
