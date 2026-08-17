//! Support for scaling CFF outlines.

mod hint;

use super::{GlyphHMetrics, OutlinePen};
use hint::{HintParams, HintState, HintingSink};
use raw::{tables::postscript::dict::normalize_font_matrix, FontRef};
use read_fonts::{
    tables::{
        postscript::{
            charstring::{self, CommandSink},
            dict, BlendState, Error, FdSelect, Index,
        },
        variations::ItemVariationStore,
    },
    types::{F2Dot14, Fixed, GlyphId},
    FontData, FontRead, ReadError, TableProvider,
};
use std::ops::Range;

/// Type for loading, scaling and hinting outlines in CFF/CFF2 tables.
///
/// The skrifa crate provides a higher level interface for this that handles
/// caching and abstracting over the different outline formats. Consider using
/// that if detailed control over resources is not required.
///
/// # Subfonts
///
/// CFF tables can contain multiple logical "subfonts" which determine the
/// state required for processing some subset of glyphs. This state is
/// accessed using the [`FDArray and FDSelect`](https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=28)
/// operators to select an appropriate subfont for any given glyph identifier.
/// This process is exposed on this type with the
/// [`subfont_index`](Self::subfont_index) method to retrieve the subfont
/// index for the requested glyph followed by using the
/// [`subfont`](Self::subfont) method to create an appropriately configured
/// subfont for that glyph.
#[derive(Clone)]
pub(crate) struct Outlines<'a> {
    pub(crate) font: FontRef<'a>,
    pub(crate) glyph_metrics: GlyphHMetrics<'a>,
    offset_data: FontData<'a>,
    global_subrs: Index<'a>,
    top_dict: TopDict<'a>,
    version: u16,
    units_per_em: u16,
}

impl<'a> Outlines<'a> {
    /// Creates a new scaler for the given font.
    ///
    /// This will choose an underlying CFF2 or CFF table from the font, in that
    /// order.
    pub fn new(font: &FontRef<'a>) -> Option<Self> {
        let units_per_em = font.head().ok()?.units_per_em();
        Self::from_cff2(font, units_per_em).or_else(|| Self::from_cff(font, units_per_em))
    }

    pub fn from_cff(font: &FontRef<'a>, units_per_em: u16) -> Option<Self> {
        let cff1 = font.cff().ok()?;
        let glyph_metrics = GlyphHMetrics::new(font)?;
        // "The Name INDEX in the CFF data must contain only one entry;
        // that is, there must be only one font in the CFF FontSet"
        // So we always pass 0 for Top DICT index when reading from an
        // OpenType font.
        // <https://learn.microsoft.com/en-us/typography/opentype/spec/cff>
        let top_dict_data = cff1.top_dicts().get(0).ok()?;
        let top_dict = TopDict::new(cff1.offset_data().as_bytes(), top_dict_data, false).ok()?;
        Some(Self {
            font: font.clone(),
            glyph_metrics,
            offset_data: cff1.offset_data(),
            global_subrs: cff1.global_subrs().into(),
            top_dict,
            version: 1,
            units_per_em,
        })
    }

    pub fn from_cff2(font: &FontRef<'a>, units_per_em: u16) -> Option<Self> {
        let cff2 = font.cff2().ok()?;
        let glyph_metrics = GlyphHMetrics::new(font)?;
        let table_data = cff2.offset_data().as_bytes();
        let top_dict = TopDict::new(table_data, cff2.top_dict_data(), true).ok()?;
        Some(Self {
            font: font.clone(),
            glyph_metrics,
            offset_data: cff2.offset_data(),
            global_subrs: cff2.global_subrs().into(),
            top_dict,
            version: 2,
            units_per_em,
        })
    }

    pub fn is_cff2(&self) -> bool {
        self.version == 2
    }

    pub fn units_per_em(&self) -> u16 {
        self.units_per_em
    }

    /// Returns the number of available glyphs.
    pub fn glyph_count(&self) -> usize {
        self.top_dict.charstrings.count() as usize
    }

    /// Returns the number of available subfonts.
    pub fn subfont_count(&self) -> u32 {
        // All CFF fonts have at least one logical subfont.
        self.top_dict.font_dicts.count().max(1)
    }

    /// Returns the subfont (or Font DICT) index for the given glyph
    /// identifier.
    pub fn subfont_index(&self, glyph_id: GlyphId) -> u32 {
        // For CFF tables, an FDSelect index will be present for CID-keyed
        // fonts. Otherwise, the Top DICT will contain an entry for the
        // "global" Private DICT.
        // See <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=27>
        //
        // CFF2 tables always contain a Font DICT and an FDSelect is only
        // present if the size of the DICT is greater than 1.
        // See <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#10-font-dict-index-font-dicts-and-fdselect>
        //
        // In both cases, we return a subfont index of 0 when FDSelect is missing.
        self.top_dict
            .fd_select
            .as_ref()
            .and_then(|select| select.font_index(glyph_id))
            .unwrap_or(0) as u32
    }

    /// Creates a new subfont for the given index, size, normalized
    /// variation coordinates and hinting state.
    ///
    /// The index of a subfont for a particular glyph can be retrieved with
    /// the [`subfont_index`](Self::subfont_index) method.
    pub fn subfont(
        &self,
        index: u32,
        size: Option<f32>,
        coords: &[F2Dot14],
    ) -> Result<Subfont, Error> {
        let font_dict = self.parse_font_dict(index)?;
        let blend_state = self
            .top_dict
            .var_store
            .clone()
            .map(|store| BlendState::new(store, coords, 0))
            .transpose()?;
        let private_dict =
            PrivateDict::new(self.offset_data, font_dict.private_dict_range, blend_state)?;
        let upem = self.units_per_em as i32;
        let mut scale = match size {
            Some(ppem) if upem > 0 => {
                // Note: we do an intermediate scale to 26.6 to ensure we
                // match FreeType
                Some(Fixed::from_bits((ppem * 64.) as i32) / Fixed::from_bits(upem))
            }
            _ => None,
        };
        let scale_requested = size.is_some();
        // Compute our font matrix and adjusted UPEM
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/f1cd6dbfa0c98f352b698448f40ac27e8fb3832e/src/cff/cffobjs.c#L746>
        let font_matrix = if let Some((top_matrix, top_upem)) = self.top_dict.font_matrix {
            // We have a top dict matrix. Now check for a font dict matrix.
            if let Some((sub_matrix, sub_upem)) = font_dict.font_matrix {
                let scaling = if top_upem > 1 && sub_upem > 1 {
                    top_upem.min(sub_upem)
                } else {
                    1
                };
                // Concatenate and scale
                let matrix = matrix_mul_scaled(&top_matrix, &sub_matrix, scaling);
                let upem = Fixed::from_bits(sub_upem)
                    .mul_div(Fixed::from_bits(top_upem), Fixed::from_bits(scaling));
                // Then normalize
                Some(normalize_font_matrix(matrix, upem.to_bits()))
            } else {
                // Top matrix was already normalized on load
                Some((top_matrix, top_upem))
            }
        } else if let Some((matrix, upem)) = font_dict.font_matrix {
            // Just normalize
            Some(normalize_font_matrix(matrix, upem))
        } else {
            None
        };
        // Now adjust our scale factor if necessary
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/f1cd6dbfa0c98f352b698448f40ac27e8fb3832e/src/cff/cffgload.c#L450>
        let mut font_matrix = if let Some((matrix, matrix_upem)) = font_matrix {
            // If the scaling factor from our matrix does not equal the nominal
            // UPEM of the font then adjust the scale.
            if matrix_upem != upem {
                // In this case, we need to force a scale for "unscaled"
                // requests in order to apply the adjusted UPEM from the
                // font matrix.
                let original_scale = scale.unwrap_or(Fixed::from_i32(64));
                scale = Some(
                    original_scale.mul_div(Fixed::from_bits(upem), Fixed::from_bits(matrix_upem)),
                );
            }
            Some(matrix)
        } else {
            None
        };
        if font_matrix
            == Some([
                Fixed::ONE,
                Fixed::ZERO,
                Fixed::ZERO,
                Fixed::ONE,
                Fixed::ZERO,
                Fixed::ZERO,
            ])
        {
            // Let's not waste time applying an identity matrix. This occurs
            // fairly often after normalization.
            font_matrix = None;
        }
        let hint_scale = scale_for_hinting(scale);
        let hint_state = HintState::new(&private_dict.hint_params, hint_scale);
        Ok(Subfont {
            is_cff2: self.is_cff2(),
            scale,
            scale_requested,
            subrs_offset: private_dict.subrs_offset,
            hint_state,
            store_index: private_dict.store_index,
            font_matrix,
        })
    }

    /// Loads and scales an outline for the given subfont instance, glyph
    /// identifier and normalized variation coordinates.
    ///
    /// Before calling this method, use [`subfont_index`](Self::subfont_index)
    /// to retrieve the subfont index for the desired glyph and then
    /// [`subfont`](Self::subfont) to create an instance of the subfont for a
    /// particular size and location in variation space.
    /// Creating subfont instances is not free, so this process is exposed in
    /// discrete steps to allow for caching.
    ///
    /// The result is emitted to the specified pen.
    pub fn draw(
        &self,
        subfont: &Subfont,
        glyph_id: GlyphId,
        coords: &[F2Dot14],
        hint: bool,
        pen: &mut impl OutlinePen,
    ) -> Result<(), Error> {
        let cff_data = self.offset_data.as_bytes();
        let charstrings = self.top_dict.charstrings.clone();
        let charstring_data = charstrings.get(glyph_id.to_u32() as usize)?;
        let subrs = subfont.subrs(self)?;
        let blend_state = subfont.blend_state(self, coords)?;
        let cs_eval = CharstringEvaluator {
            cff_data,
            charstrings,
            global_subrs: self.global_subrs.clone(),
            subrs,
            blend_state,
            charstring_data,
        };
        // Only apply hinting if we have a scale
        let apply_hinting = hint && subfont.scale_requested;
        let mut pen_sink = PenSink::new(pen);
        let mut simplifying_adapter = NopFilteringSink::new(&mut pen_sink);
        if let Some(matrix) = subfont.font_matrix {
            if apply_hinting {
                let mut transform_sink =
                    HintedTransformingSink::new(&mut simplifying_adapter, matrix);
                let mut hinting_adapter =
                    HintingSink::new(&subfont.hint_state, &mut transform_sink);
                cs_eval.evaluate(&mut hinting_adapter)?;
                hinting_adapter.finish();
            } else {
                let mut transform_sink =
                    ScalingTransformingSink::new(&mut simplifying_adapter, matrix, subfont.scale);
                cs_eval.evaluate(&mut transform_sink)?;
            }
        } else if apply_hinting {
            let mut hinting_adapter =
                HintingSink::new(&subfont.hint_state, &mut simplifying_adapter);
            cs_eval.evaluate(&mut hinting_adapter)?;
            hinting_adapter.finish();
        } else {
            let mut scaling_adapter =
                ScalingSink26Dot6::new(&mut simplifying_adapter, subfont.scale);
            cs_eval.evaluate(&mut scaling_adapter)?;
        }
        simplifying_adapter.finish();
        Ok(())
    }

    fn parse_font_dict(&self, subfont_index: u32) -> Result<FontDict, Error> {
        if self.top_dict.font_dicts.count() != 0 {
            // If we have a font dict array, extract the private dict range
            // from the font dict at the given index.
            let font_dict_data = self.top_dict.font_dicts.get(subfont_index as usize)?;
            FontDict::new(font_dict_data)
        } else {
            // Use the private dict range from the top dict.
            // Note: "A Private DICT is required but may be specified as having
            // a length of 0 if there are no non-default values to be stored."
            // <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=25>
            let range = self.top_dict.private_dict_range.clone();
            Ok(FontDict {
                private_dict_range: range.start as usize..range.end as usize,
                font_matrix: None,
            })
        }
    }
}

/// When hinting, use a modified scale factor.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psft.c#L279>
fn scale_for_hinting(scale: Option<Fixed>) -> Fixed {
    Fixed::from_bits((scale.unwrap_or(Fixed::ONE).to_bits().saturating_add(32)) / 64)
}

struct CharstringEvaluator<'a> {
    cff_data: &'a [u8],
    charstrings: Index<'a>,
    global_subrs: Index<'a>,
    subrs: Option<Index<'a>>,
    blend_state: Option<BlendState<'a>>,
    charstring_data: &'a [u8],
}

impl CharstringEvaluator<'_> {
    fn evaluate(self, sink: &mut impl CommandSink) -> Result<(), Error> {
        charstring::evaluate(
            self.cff_data,
            self.charstrings,
            self.global_subrs,
            self.subrs,
            self.blend_state,
            self.charstring_data,
            sink,
        )
    }
}

/// Specifies local subroutines and hinting parameters for some subset of
/// glyphs in a CFF or CFF2 table.
///
/// This type is designed to be cacheable to avoid re-evaluating the private
/// dict every time a charstring is processed.
///
/// For variable fonts, this is dependent on a location in variation space.
#[derive(Clone)]
pub(crate) struct Subfont {
    is_cff2: bool,
    scale: Option<Fixed>,
    /// When we have a font matrix, we might force a scale even if the user
    /// requested unscaled output. In this case, we shouldn't apply hinting
    /// and this keeps track of that.
    scale_requested: bool,
    subrs_offset: Option<usize>,
    pub(crate) hint_state: HintState,
    store_index: u16,
    font_matrix: Option<[Fixed; 6]>,
}

impl Subfont {
    /// Returns the local subroutine index.
    pub fn subrs<'a>(&self, scaler: &Outlines<'a>) -> Result<Option<Index<'a>>, Error> {
        if let Some(subrs_offset) = self.subrs_offset {
            let offset_data = scaler.offset_data.as_bytes();
            let index_data = offset_data.get(subrs_offset..).unwrap_or_default();
            Ok(Some(Index::new(index_data, self.is_cff2)?))
        } else {
            Ok(None)
        }
    }

    /// Creates a new blend state for the given normalized variation
    /// coordinates.
    pub fn blend_state<'a>(
        &self,
        scaler: &Outlines<'a>,
        coords: &'a [F2Dot14],
    ) -> Result<Option<BlendState<'a>>, Error> {
        if let Some(var_store) = scaler.top_dict.var_store.clone() {
            Ok(Some(BlendState::new(var_store, coords, self.store_index)?))
        } else {
            Ok(None)
        }
    }
}

/// Entries that we parse from the Private DICT to support charstring
/// evaluation.
#[derive(Default)]
struct PrivateDict {
    hint_params: HintParams,
    subrs_offset: Option<usize>,
    store_index: u16,
}

impl PrivateDict {
    fn new(
        data: FontData,
        range: Range<usize>,
        blend_state: Option<BlendState<'_>>,
    ) -> Result<Self, Error> {
        let private_dict_data = data.read_array(range.clone())?;
        let mut dict = Self::default();
        for entry in dict::entries(private_dict_data, blend_state) {
            use dict::Entry::*;
            match entry? {
                BlueValues(values) => dict.hint_params.blues = values,
                FamilyBlues(values) => dict.hint_params.family_blues = values,
                OtherBlues(values) => dict.hint_params.other_blues = values,
                FamilyOtherBlues(values) => dict.hint_params.family_other_blues = values,
                BlueScale(value) => dict.hint_params.blue_scale = value,
                BlueShift(value) => dict.hint_params.blue_shift = value,
                BlueFuzz(value) => dict.hint_params.blue_fuzz = value,
                LanguageGroup(group) => dict.hint_params.language_group = group,
                // Subrs offset is relative to the private DICT
                SubrsOffset(offset) => {
                    dict.subrs_offset = Some(
                        range
                            .start
                            .checked_add(offset)
                            .ok_or(ReadError::OutOfBounds)?,
                    )
                }
                VariationStoreIndex(index) => dict.store_index = index,
                _ => {}
            }
        }
        Ok(dict)
    }
}

/// Entries that we parse from a Font DICT.
#[derive(Clone, Default)]
struct FontDict {
    private_dict_range: Range<usize>,
    font_matrix: Option<([Fixed; 6], i32)>,
}

impl FontDict {
    fn new(font_dict_data: &[u8]) -> Result<Self, Error> {
        let mut range = None;
        let mut font_matrix = None;
        for entry in dict::entries(font_dict_data, None) {
            match entry? {
                dict::Entry::PrivateDictRange(r) => {
                    range = Some(r);
                }
                // We store this matrix unnormalized since FreeType
                // concatenates this with the top dict matrix (if present)
                // before normalizing
                dict::Entry::FontMatrix(matrix, upem) => font_matrix = Some((matrix, upem)),
                _ => {}
            }
        }
        Ok(Self {
            private_dict_range: range.ok_or(Error::MissingPrivateDict)?,
            font_matrix,
        })
    }
}

/// Entries that we parse from the Top DICT that are required to support
/// charstring evaluation.
#[derive(Clone, Default)]
struct TopDict<'a> {
    charstrings: Index<'a>,
    font_dicts: Index<'a>,
    fd_select: Option<FdSelect<'a>>,
    private_dict_range: Range<u32>,
    font_matrix: Option<([Fixed; 6], i32)>,
    var_store: Option<ItemVariationStore<'a>>,
}

impl<'a> TopDict<'a> {
    fn new(table_data: &'a [u8], top_dict_data: &'a [u8], is_cff2: bool) -> Result<Self, Error> {
        let mut items = TopDict::default();
        for entry in dict::entries(top_dict_data, None) {
            match entry? {
                dict::Entry::CharstringsOffset(offset) => {
                    items.charstrings =
                        Index::new(table_data.get(offset..).unwrap_or_default(), is_cff2)?;
                }
                dict::Entry::FdArrayOffset(offset) => {
                    items.font_dicts =
                        Index::new(table_data.get(offset..).unwrap_or_default(), is_cff2)?;
                }
                dict::Entry::FdSelectOffset(offset) => {
                    items.fd_select = Some(FdSelect::read(FontData::new(
                        table_data.get(offset..).unwrap_or_default(),
                    ))?);
                }
                dict::Entry::PrivateDictRange(range) => {
                    items.private_dict_range = range.start as u32..range.end as u32;
                }
                dict::Entry::FontMatrix(matrix, upem) => {
                    // Store this matrix normalized since FT always applies normalization
                    items.font_matrix = Some(normalize_font_matrix(matrix, upem));
                }
                dict::Entry::VariationStoreOffset(offset) if is_cff2 => {
                    // IVS is preceded by a 2 byte length, but ensure that
                    // we don't overflow
                    // See <https://github.com/googlefonts/fontations/issues/1223>
                    let offset = offset.checked_add(2).ok_or(ReadError::OutOfBounds)?;
                    items.var_store = Some(ItemVariationStore::read(FontData::new(
                        table_data.get(offset..).unwrap_or_default(),
                    ))?);
                }
                _ => {}
            }
        }
        Ok(items)
    }
}

/// Command sink that sends the results of charstring evaluation to
/// an [OutlinePen].
struct PenSink<'a, P>(&'a mut P);

impl<'a, P> PenSink<'a, P> {
    fn new(pen: &'a mut P) -> Self {
        Self(pen)
    }
}

impl<P> CommandSink for PenSink<'_, P>
where
    P: OutlinePen,
{
    fn move_to(&mut self, x: Fixed, y: Fixed) {
        self.0.move_to(x.to_f32(), y.to_f32());
    }

    fn line_to(&mut self, x: Fixed, y: Fixed) {
        self.0.line_to(x.to_f32(), y.to_f32());
    }

    fn curve_to(&mut self, cx0: Fixed, cy0: Fixed, cx1: Fixed, cy1: Fixed, x: Fixed, y: Fixed) {
        self.0.curve_to(
            cx0.to_f32(),
            cy0.to_f32(),
            cx1.to_f32(),
            cy1.to_f32(),
            x.to_f32(),
            y.to_f32(),
        );
    }

    fn close(&mut self) {
        self.0.close();
    }
}

fn transform(matrix: &[Fixed; 6], x: Fixed, y: Fixed) -> (Fixed, Fixed) {
    (
        matrix[0] * x + matrix[2] * y + matrix[4],
        matrix[1] * x + matrix[3] * y + matrix[5],
    )
}

/// Command sink adapter that applies a transform to hinted coordinates.
struct HintedTransformingSink<'a, S> {
    inner: &'a mut S,
    matrix: [Fixed; 6],
}

impl<'a, S> HintedTransformingSink<'a, S> {
    fn new(sink: &'a mut S, matrix: [Fixed; 6]) -> Self {
        Self {
            inner: sink,
            matrix,
        }
    }

    fn transform(&self, x: Fixed, y: Fixed) -> (Fixed, Fixed) {
        // FreeType applies the transform to 26.6 values but we maintain
        // values in 16.16 so convert, transform and then convert back
        let (x, y) = transform(
            &self.matrix,
            Fixed::from_bits(x.to_bits() >> 10),
            Fixed::from_bits(y.to_bits() >> 10),
        );
        (
            Fixed::from_bits(x.to_bits() << 10),
            Fixed::from_bits(y.to_bits() << 10),
        )
    }
}

impl<S: CommandSink> CommandSink for HintedTransformingSink<'_, S> {
    fn hstem(&mut self, y: Fixed, dy: Fixed) {
        self.inner.hstem(y, dy);
    }

    fn vstem(&mut self, x: Fixed, dx: Fixed) {
        self.inner.vstem(x, dx);
    }

    fn hint_mask(&mut self, mask: &[u8]) {
        self.inner.hint_mask(mask);
    }

    fn counter_mask(&mut self, mask: &[u8]) {
        self.inner.counter_mask(mask);
    }

    fn clear_hints(&mut self) {
        self.inner.clear_hints();
    }

    fn move_to(&mut self, x: Fixed, y: Fixed) {
        let (x, y) = self.transform(x, y);
        self.inner.move_to(x, y);
    }

    fn line_to(&mut self, x: Fixed, y: Fixed) {
        let (x, y) = self.transform(x, y);
        self.inner.line_to(x, y);
    }

    fn curve_to(&mut self, cx1: Fixed, cy1: Fixed, cx2: Fixed, cy2: Fixed, x: Fixed, y: Fixed) {
        let (cx1, cy1) = self.transform(cx1, cy1);
        let (cx2, cy2) = self.transform(cx2, cy2);
        let (x, y) = self.transform(x, y);
        self.inner.curve_to(cx1, cy1, cx2, cy2, x, y);
    }

    fn close(&mut self) {
        self.inner.close();
    }
}

// Used for scaling sinks below
const ONE_OVER_64: Fixed = Fixed::from_bits(0x400);

/// Command sink adapter that applies both a transform and a scaling
/// factor.
struct ScalingTransformingSink<'a, S> {
    inner: &'a mut S,
    matrix: [Fixed; 6],
    scale: Option<Fixed>,
}

impl<'a, S> ScalingTransformingSink<'a, S> {
    fn new(sink: &'a mut S, matrix: [Fixed; 6], scale: Option<Fixed>) -> Self {
        Self {
            inner: sink,
            matrix,
            scale,
        }
    }

    fn transform(&self, x: Fixed, y: Fixed) -> (Fixed, Fixed) {
        // The following dance is necessary to exactly match FreeType's
        // application of scaling factors. This seems to be the result
        // of merging the contributed Adobe code while not breaking the
        // FreeType public API.
        //
        // The first two steps apply to both scaled and unscaled outlines:
        //
        // 1. Multiply by 1/64
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psft.c#L284>
        let ax = x * ONE_OVER_64;
        let ay = y * ONE_OVER_64;
        // 2. Truncate the bottom 10 bits. Combined with the division by 64,
        // converts to font units.
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psobjs.c#L2219>
        let bx = Fixed::from_bits(ax.to_bits() >> 10);
        let by = Fixed::from_bits(ay.to_bits() >> 10);
        // 3. Apply the transform. It must be done here to match FreeType.
        let (cx, cy) = transform(&self.matrix, bx, by);
        if let Some(scale) = self.scale {
            // Scaled case:
            // 4. Multiply by the original scale factor (to 26.6)
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/cff/cffgload.c#L721>
            let dx = cx * scale;
            let dy = cy * scale;
            // 5. Convert from 26.6 to 16.16
            (
                Fixed::from_bits(dx.to_bits() << 10),
                Fixed::from_bits(dy.to_bits() << 10),
            )
        } else {
            // Unscaled case:
            // 4. Convert from integer to 16.16
            (
                Fixed::from_bits(cx.to_bits() << 16),
                Fixed::from_bits(cy.to_bits() << 16),
            )
        }
    }
}

impl<S: CommandSink> CommandSink for ScalingTransformingSink<'_, S> {
    fn hstem(&mut self, y: Fixed, dy: Fixed) {
        self.inner.hstem(y, dy);
    }

    fn vstem(&mut self, x: Fixed, dx: Fixed) {
        self.inner.vstem(x, dx);
    }

    fn hint_mask(&mut self, mask: &[u8]) {
        self.inner.hint_mask(mask);
    }

    fn counter_mask(&mut self, mask: &[u8]) {
        self.inner.counter_mask(mask);
    }

    fn clear_hints(&mut self) {
        self.inner.clear_hints();
    }

    fn move_to(&mut self, x: Fixed, y: Fixed) {
        let (x, y) = self.transform(x, y);
        self.inner.move_to(x, y);
    }

    fn line_to(&mut self, x: Fixed, y: Fixed) {
        let (x, y) = self.transform(x, y);
        self.inner.line_to(x, y);
    }

    fn curve_to(&mut self, cx1: Fixed, cy1: Fixed, cx2: Fixed, cy2: Fixed, x: Fixed, y: Fixed) {
        let (cx1, cy1) = self.transform(cx1, cy1);
        let (cx2, cy2) = self.transform(cx2, cy2);
        let (x, y) = self.transform(x, y);
        self.inner.curve_to(cx1, cy1, cx2, cy2, x, y);
    }

    fn close(&mut self) {
        self.inner.close();
    }
}

/// Command sink adapter that applies a scaling factor.
///
/// This assumes a 26.6 scaling factor packed into a Fixed and thus,
/// this is not public and exists only to match FreeType's exact
/// scaling process.
struct ScalingSink26Dot6<'a, S> {
    inner: &'a mut S,
    scale: Option<Fixed>,
}

impl<'a, S> ScalingSink26Dot6<'a, S> {
    fn new(sink: &'a mut S, scale: Option<Fixed>) -> Self {
        Self { scale, inner: sink }
    }

    fn scale(&self, coord: Fixed) -> Fixed {
        // The following dance is necessary to exactly match FreeType's
        // application of scaling factors. This seems to be the result
        // of merging the contributed Adobe code while not breaking the
        // FreeType public API.
        //
        // The first two steps apply to both scaled and unscaled outlines:
        //
        // 1. Multiply by 1/64
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psft.c#L284>
        let a = coord * ONE_OVER_64;
        // 2. Truncate the bottom 10 bits. Combined with the division by 64,
        // converts to font units.
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psobjs.c#L2219>
        let b = Fixed::from_bits(a.to_bits() >> 10);
        if let Some(scale) = self.scale {
            // Scaled case:
            // 3. Multiply by the original scale factor (to 26.6)
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/cff/cffgload.c#L721>
            let c = b * scale;
            // 4. Convert from 26.6 to 16.16
            Fixed::from_bits(c.to_bits() << 10)
        } else {
            // Unscaled case:
            // 3. Convert from integer to 16.16
            Fixed::from_bits(b.to_bits() << 16)
        }
    }
}

impl<S: CommandSink> CommandSink for ScalingSink26Dot6<'_, S> {
    fn hstem(&mut self, y: Fixed, dy: Fixed) {
        self.inner.hstem(y, dy);
    }

    fn vstem(&mut self, x: Fixed, dx: Fixed) {
        self.inner.vstem(x, dx);
    }

    fn hint_mask(&mut self, mask: &[u8]) {
        self.inner.hint_mask(mask);
    }

    fn counter_mask(&mut self, mask: &[u8]) {
        self.inner.counter_mask(mask);
    }

    fn clear_hints(&mut self) {
        self.inner.clear_hints();
    }

    fn move_to(&mut self, x: Fixed, y: Fixed) {
        self.inner.move_to(self.scale(x), self.scale(y));
    }

    fn line_to(&mut self, x: Fixed, y: Fixed) {
        self.inner.line_to(self.scale(x), self.scale(y));
    }

    fn curve_to(&mut self, cx1: Fixed, cy1: Fixed, cx2: Fixed, cy2: Fixed, x: Fixed, y: Fixed) {
        self.inner.curve_to(
            self.scale(cx1),
            self.scale(cy1),
            self.scale(cx2),
            self.scale(cy2),
            self.scale(x),
            self.scale(y),
        );
    }

    fn close(&mut self) {
        self.inner.close();
    }
}

#[derive(Copy, Clone)]
enum PendingElement {
    Move([Fixed; 2]),
    Line([Fixed; 2]),
    Curve([Fixed; 6]),
}

impl PendingElement {
    fn target_point(&self) -> [Fixed; 2] {
        match self {
            Self::Move(xy) | Self::Line(xy) => *xy,
            Self::Curve([.., x, y]) => [*x, *y],
        }
    }
}

/// Command sink adapter that suppresses degenerate move and line commands.
///
/// FreeType avoids emitting empty contours and zero length lines to prevent
/// artifacts when stem darkening is enabled. We don't support stem darkening
/// because it's not enabled by any of our clients but we remove the degenerate
/// elements regardless to match the output.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/pshints.c#L1786>
struct NopFilteringSink<'a, S> {
    is_open: bool,
    start: Option<(Fixed, Fixed)>,
    pending_element: Option<PendingElement>,
    inner: &'a mut S,
}

impl<'a, S> NopFilteringSink<'a, S>
where
    S: CommandSink,
{
    fn new(inner: &'a mut S) -> Self {
        Self {
            is_open: false,
            start: None,
            pending_element: None,
            inner,
        }
    }

    fn flush_pending(&mut self, for_close: bool) {
        if let Some(pending) = self.pending_element.take() {
            match pending {
                PendingElement::Move([x, y]) => {
                    if !for_close {
                        self.is_open = true;
                        self.inner.move_to(x, y);
                        self.start = Some((x, y));
                    }
                }
                PendingElement::Line([x, y]) => {
                    if !for_close || self.start != Some((x, y)) {
                        self.inner.line_to(x, y);
                    }
                }
                PendingElement::Curve([cx0, cy0, cx1, cy1, x, y]) => {
                    self.inner.curve_to(cx0, cy0, cx1, cy1, x, y);
                }
            }
        }
    }

    pub fn finish(&mut self) {
        self.close();
    }
}

impl<S> CommandSink for NopFilteringSink<'_, S>
where
    S: CommandSink,
{
    fn hstem(&mut self, y: Fixed, dy: Fixed) {
        self.inner.hstem(y, dy);
    }

    fn vstem(&mut self, x: Fixed, dx: Fixed) {
        self.inner.vstem(x, dx);
    }

    fn hint_mask(&mut self, mask: &[u8]) {
        self.inner.hint_mask(mask);
    }

    fn counter_mask(&mut self, mask: &[u8]) {
        self.inner.counter_mask(mask);
    }

    fn clear_hints(&mut self) {
        self.inner.clear_hints();
    }

    fn move_to(&mut self, x: Fixed, y: Fixed) {
        self.pending_element = Some(PendingElement::Move([x, y]));
    }

    fn line_to(&mut self, x: Fixed, y: Fixed) {
        // Omit the line if we're already at the given position
        if self
            .pending_element
            .map(|element| element.target_point() == [x, y])
            .unwrap_or_default()
        {
            return;
        }
        self.flush_pending(false);
        self.pending_element = Some(PendingElement::Line([x, y]));
    }

    fn curve_to(&mut self, cx1: Fixed, cy1: Fixed, cx2: Fixed, cy2: Fixed, x: Fixed, y: Fixed) {
        self.flush_pending(false);
        self.pending_element = Some(PendingElement::Curve([cx1, cy1, cx2, cy2, x, y]));
    }

    fn close(&mut self) {
        self.flush_pending(true);
        if self.is_open {
            self.inner.close();
            self.is_open = false;
        }
    }
}

/// Simple fixed point matrix multiplication with a scaling factor.
///
/// Note: this transforms the translation component of `b` by the upper 2x2 of
/// `a`. This matches the offset transform FreeType uses when concatenating
/// the matrices from the top and font dicts.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/base/ftcalc.c#L719>
fn matrix_mul_scaled(a: &[Fixed; 6], b: &[Fixed; 6], scaling: i32) -> [Fixed; 6] {
    let val = Fixed::from_i32(scaling);
    let xx = a[0].mul_div(b[0], val) + a[2].mul_div(b[1], val);
    let yx = a[1].mul_div(b[0], val) + a[3].mul_div(b[1], val);
    let xy = a[0].mul_div(b[2], val) + a[2].mul_div(b[3], val);
    let yy = a[1].mul_div(b[2], val) + a[3].mul_div(b[3], val);
    let x = b[4];
    let y = b[5];
    let dx = x.mul_div(a[0], val) + y.mul_div(a[2], val);
    let dy = x.mul_div(a[1], val) + y.mul_div(a[3], val);
    [xx, yx, xy, yy, dx, dy]
}

#[cfg(test)]
mod tests {
    use super::{super::pen::SvgPen, *};
    use crate::{
        outline::{HintingInstance, HintingOptions},
        prelude::{LocationRef, Size},
        MetadataProvider,
    };
    use dict::Blues;
    use font_test_data::bebuffer::BeBuffer;
    use raw::tables::cff2::Cff2;
    use read_fonts::FontRef;

    #[test]
    fn unscaled_scaling_sink_produces_integers() {
        let nothing = &mut ();
        let sink = ScalingSink26Dot6::new(nothing, None);
        for coord in [50.0, 50.1, 50.125, 50.5, 50.9] {
            assert_eq!(sink.scale(Fixed::from_f64(coord)).to_f32(), 50.0);
        }
    }

    #[test]
    fn scaled_scaling_sink() {
        let ppem = 20.0;
        let upem = 1000.0;
        // match FreeType scaling with intermediate conversion to 26.6
        let scale = Fixed::from_bits((ppem * 64.) as i32) / Fixed::from_bits(upem as i32);
        let nothing = &mut ();
        let sink = ScalingSink26Dot6::new(nothing, Some(scale));
        let inputs = [
            // input coord, expected scaled output
            (0.0, 0.0),
            (8.0, 0.15625),
            (16.0, 0.3125),
            (32.0, 0.640625),
            (72.0, 1.4375),
            (128.0, 2.5625),
        ];
        for (coord, expected) in inputs {
            assert_eq!(
                sink.scale(Fixed::from_f64(coord)).to_f32(),
                expected,
                "scaling coord {coord}"
            );
        }
    }

    #[test]
    fn read_cff_static() {
        let font = FontRef::new(font_test_data::NOTO_SERIF_DISPLAY_TRIMMED).unwrap();
        let cff = Outlines::new(&font).unwrap();
        assert!(!cff.is_cff2());
        assert!(cff.top_dict.var_store.is_none());
        assert!(cff.top_dict.font_dicts.count() == 0);
        assert!(!cff.top_dict.private_dict_range.is_empty());
        assert!(cff.top_dict.fd_select.is_none());
        assert_eq!(cff.subfont_count(), 1);
        assert_eq!(cff.subfont_index(GlyphId::new(1)), 0);
        assert_eq!(cff.global_subrs.count(), 17);
    }

    #[test]
    fn read_cff2_static() {
        let font = FontRef::new(font_test_data::CANTARELL_VF_TRIMMED).unwrap();
        let cff = Outlines::new(&font).unwrap();
        assert!(cff.is_cff2());
        assert!(cff.top_dict.var_store.is_some());
        assert!(cff.top_dict.font_dicts.count() != 0);
        assert!(cff.top_dict.private_dict_range.is_empty());
        assert!(cff.top_dict.fd_select.is_none());
        assert_eq!(cff.subfont_count(), 1);
        assert_eq!(cff.subfont_index(GlyphId::new(1)), 0);
        assert_eq!(cff.global_subrs.count(), 0);
    }

    #[test]
    fn read_example_cff2_table() {
        let cff2 = Cff2::read(FontData::new(font_test_data::cff2::EXAMPLE)).unwrap();
        let top_dict =
            TopDict::new(cff2.offset_data().as_bytes(), cff2.top_dict_data(), true).unwrap();
        assert!(top_dict.var_store.is_some());
        assert!(top_dict.font_dicts.count() != 0);
        assert!(top_dict.private_dict_range.is_empty());
        assert!(top_dict.fd_select.is_none());
        assert_eq!(cff2.global_subrs().count(), 0);
    }

    #[test]
    fn cff2_variable_outlines_match_freetype() {
        compare_glyphs(
            font_test_data::CANTARELL_VF_TRIMMED,
            font_test_data::CANTARELL_VF_TRIMMED_GLYPHS,
        );
    }

    #[test]
    fn cff_static_outlines_match_freetype() {
        compare_glyphs(
            font_test_data::NOTO_SERIF_DISPLAY_TRIMMED,
            font_test_data::NOTO_SERIF_DISPLAY_TRIMMED_GLYPHS,
        );
    }

    #[test]
    fn unhinted_ends_with_close() {
        let font = FontRef::new(font_test_data::CANTARELL_VF_TRIMMED).unwrap();
        let glyph = font.outline_glyphs().get(GlyphId::new(1)).unwrap();
        let mut svg = SvgPen::default();
        glyph.draw(Size::unscaled(), &mut svg).unwrap();
        assert!(svg.to_string().ends_with('Z'));
    }

    #[test]
    fn hinted_ends_with_close() {
        let font = FontRef::new(font_test_data::CANTARELL_VF_TRIMMED).unwrap();
        let glyphs = font.outline_glyphs();
        let hinter = HintingInstance::new(
            &glyphs,
            Size::unscaled(),
            LocationRef::default(),
            HintingOptions::default(),
        )
        .unwrap();
        let glyph = glyphs.get(GlyphId::new(1)).unwrap();
        let mut svg = SvgPen::default();
        glyph.draw(&hinter, &mut svg).unwrap();
        assert!(svg.to_string().ends_with('Z'));
    }

    /// Ensure we don't reject an empty Private DICT
    #[test]
    fn empty_private_dict() {
        let font = FontRef::new(font_test_data::MATERIAL_ICONS_SUBSET).unwrap();
        let outlines = super::Outlines::new(&font).unwrap();
        assert!(outlines.top_dict.private_dict_range.is_empty());
        assert!(outlines
            .parse_font_dict(0)
            .unwrap()
            .private_dict_range
            .is_empty());
    }

    /// Fuzzer caught add with overflow when computing subrs offset.
    /// See <https://issues.oss-fuzz.com/issues/377965575>
    #[test]
    fn subrs_offset_overflow() {
        // A private DICT with an overflowing subrs offset
        let private_dict = BeBuffer::new()
            .push(0u32) // pad so that range doesn't start with 0 and we overflow
            .push(29u8) // integer operator
            .push(-1i32) // integer value
            .push(19u8) // subrs offset operator
            .to_vec();
        // Just don't panic with overflow
        assert!(
            PrivateDict::new(FontData::new(&private_dict), 4..private_dict.len(), None).is_err()
        );
    }

    // Fuzzer caught add with overflow when computing offset to
    // var store.
    // See <https://issues.oss-fuzz.com/issues/377574377>
    #[test]
    fn top_dict_ivs_offset_overflow() {
        // A top DICT with a var store offset of -1 which will cause an
        // overflow
        let top_dict = BeBuffer::new()
            .push(29u8) // integer operator
            .push(-1i32) // integer value
            .push(24u8) // var store offset operator
            .to_vec();
        // Just don't panic with overflow
        assert!(TopDict::new(&[], &top_dict, true).is_err());
    }

    /// Actually apply a scale when the computed scale factor is
    /// equal to Fixed::ONE.
    ///
    /// Specifically, when upem = 512 and ppem = 8, this results in
    /// a scale factor of 65536 which was being interpreted as an
    /// unscaled draw request.
    #[test]
    fn proper_scaling_when_factor_equals_fixed_one() {
        let font = FontRef::new(font_test_data::MATERIAL_ICONS_SUBSET).unwrap();
        assert_eq!(font.head().unwrap().units_per_em(), 512);
        let glyphs = font.outline_glyphs();
        let glyph = glyphs.get(GlyphId::new(1)).unwrap();
        let mut svg = SvgPen::with_precision(6);
        glyph
            .draw((Size::new(8.0), LocationRef::default()), &mut svg)
            .unwrap();
        // This was initially producing unscaled values like M405.000...
        assert!(svg.starts_with("M6.328125,7.000000 L1.671875,7.000000"));
    }

    /// For the given font data and extracted outlines, parse the extracted
    /// outline data into a set of expected values and compare these with the
    /// results generated by the scaler.
    ///
    /// This will compare all outlines at various sizes and (for variable
    /// fonts), locations in variation space.
    fn compare_glyphs(font_data: &[u8], expected_outlines: &str) {
        use super::super::testing;
        let font = FontRef::new(font_data).unwrap();
        let expected_outlines = testing::parse_glyph_outlines(expected_outlines);
        let outlines = super::Outlines::new(&font).unwrap();
        let mut path = testing::Path::default();
        for expected_outline in &expected_outlines {
            if expected_outline.size == 0.0 && !expected_outline.coords.is_empty() {
                continue;
            }
            let size = (expected_outline.size != 0.0).then_some(expected_outline.size);
            path.elements.clear();
            let subfont = outlines
                .subfont(
                    outlines.subfont_index(expected_outline.glyph_id),
                    size,
                    &expected_outline.coords,
                )
                .unwrap();
            outlines
                .draw(
                    &subfont,
                    expected_outline.glyph_id,
                    &expected_outline.coords,
                    false,
                    &mut path,
                )
                .unwrap();
            if path.elements != expected_outline.path {
                panic!(
                    "mismatch in glyph path for id {} (size: {}, coords: {:?}): path: {:?} expected_path: {:?}",
                    expected_outline.glyph_id,
                    expected_outline.size,
                    expected_outline.coords,
                    &path.elements,
                    &expected_outline.path
                );
            }
        }
    }

    // We were overwriting family_other_blues with family_blues.
    #[test]
    fn capture_family_other_blues() {
        let private_dict_data = &font_test_data::cff2::EXAMPLE[0x4f..=0xc0];
        let store =
            ItemVariationStore::read(FontData::new(&font_test_data::cff2::EXAMPLE[18..])).unwrap();
        let coords = &[F2Dot14::from_f32(0.0)];
        let blend_state = BlendState::new(store, coords, 0).unwrap();
        let private_dict = PrivateDict::new(
            FontData::new(private_dict_data),
            0..private_dict_data.len(),
            Some(blend_state),
        )
        .unwrap();
        assert_eq!(
            private_dict.hint_params.family_other_blues,
            Blues::new([-249.0, -239.0].map(Fixed::from_f64).into_iter())
        )
    }

    #[test]
    fn implied_seac() {
        let font = FontRef::new(font_test_data::CHARSTRING_PATH_OPS).unwrap();
        let glyphs = font.outline_glyphs();
        let gid = GlyphId::new(3);
        assert_eq!(font.glyph_names().get(gid).unwrap(), "Scaron");
        let glyph = glyphs.get(gid).unwrap();
        let mut pen = SvgPen::new();
        glyph
            .draw((Size::unscaled(), LocationRef::default()), &mut pen)
            .unwrap();
        // This triggers the seac behavior in the endchar operator which
        // loads an accent character followed by a base character. Ensure
        // that we have a path to represent each by checking for two closepath
        // commands.
        assert_eq!(pen.to_string().chars().filter(|ch| *ch == 'Z').count(), 2);
    }

    #[test]
    fn implied_seac_clears_hints() {
        let font = FontRef::new(font_test_data::CHARSTRING_PATH_OPS).unwrap();
        let outlines = Outlines::from_cff(&font, 1000).unwrap();
        let subfont = outlines.subfont(0, Some(16.0), &[]).unwrap();
        let cff_data = outlines.offset_data.as_bytes();
        let charstrings = outlines.top_dict.charstrings.clone();
        let charstring_data = charstrings.get(3).unwrap();
        let subrs = subfont.subrs(&outlines).unwrap();
        let blend_state = None;
        let cs_eval = CharstringEvaluator {
            cff_data,
            charstrings,
            global_subrs: outlines.global_subrs.clone(),
            subrs,
            blend_state,
            charstring_data,
        };
        struct ClearHintsCountingSink(u32);
        impl CommandSink for ClearHintsCountingSink {
            fn move_to(&mut self, _: Fixed, _: Fixed) {}
            fn line_to(&mut self, _: Fixed, _: Fixed) {}
            fn curve_to(&mut self, _: Fixed, _: Fixed, _: Fixed, _: Fixed, _: Fixed, _: Fixed) {}
            fn close(&mut self) {}
            fn clear_hints(&mut self) {
                self.0 += 1;
            }
        }
        let mut sink = ClearHintsCountingSink(0);
        cs_eval.evaluate(&mut sink).unwrap();
        // We should have cleared hints twice.. once for the base and once
        // for the accent
        assert_eq!(sink.0, 2);
    }

    const TRANSFORM: [Fixed; 6] = [
        Fixed::ONE,
        Fixed::ZERO,
        // 0.167007446289062
        Fixed::from_bits(10945),
        Fixed::ONE,
        Fixed::ZERO,
        Fixed::ZERO,
    ];

    #[test]
    fn hinted_transform_sink() {
        // A few points taken from the test font in <https://github.com/googlefonts/fontations/issues/1581>
        // Inputs and expected values extracted from FreeType
        let input = [(383i32, 117i32), (450, 20), (555, -34), (683, -34)]
            .map(|(x, y)| (Fixed::from_bits(x << 10), Fixed::from_bits(y << 10)));
        let expected = [(403, 117i32), (453, 20), (549, -34), (677, -34)]
            .map(|(x, y)| (Fixed::from_bits(x << 10), Fixed::from_bits(y << 10)));
        let mut dummy = ();
        let sink = HintedTransformingSink::new(&mut dummy, TRANSFORM);
        let transformed = input.map(|(x, y)| sink.transform(x, y));
        assert_eq!(transformed, expected);
    }

    #[test]
    fn unhinted_scaled_transform_sink() {
        // A few points taken from the test font in <https://github.com/googlefonts/fontations/issues/1581>
        // Inputs and expected values extracted from FreeType
        let input = [(150i32, 46i32), (176, 8), (217, -13), (267, -13)]
            .map(|(x, y)| (Fixed::from_bits(x << 16), Fixed::from_bits(y << 16)));
        let expected = [(404, 118i32), (453, 20), (550, -33), (678, -33)]
            .map(|(x, y)| (Fixed::from_bits(x << 10), Fixed::from_bits(y << 10)));
        let mut dummy = ();
        let sink =
            ScalingTransformingSink::new(&mut dummy, TRANSFORM, Some(Fixed::from_bits(167772)));
        let transformed = input.map(|(x, y)| sink.transform(x, y));
        assert_eq!(transformed, expected);
    }

    #[test]
    fn unhinted_unscaled_transform_sink() {
        // A few points taken from the test font in <https://github.com/googlefonts/fontations/issues/1581>
        // Inputs and expected values extracted from FreeType
        let input = [(150i32, 46i32), (176, 8), (217, -13), (267, -13)]
            .map(|(x, y)| (Fixed::from_bits(x << 16), Fixed::from_bits(y << 16)));
        let expected = [(158, 46i32), (177, 8), (215, -13), (265, -13)]
            .map(|(x, y)| (Fixed::from_bits(x << 16), Fixed::from_bits(y << 16)));
        let mut dummy = ();
        let sink = ScalingTransformingSink::new(&mut dummy, TRANSFORM, None);
        let transformed = input.map(|(x, y)| sink.transform(x, y));
        assert_eq!(transformed, expected);
    }

    #[test]
    fn fixed_matrix_mul() {
        let a = [0.5, 0.75, -1.0, 2.0, 0.0, 0.0].map(Fixed::from_f64);
        let b = [1.5, -1.0, 0.25, -1.0, 1.0, 2.0].map(Fixed::from_f64);
        let expected = [1.75, -0.875, 1.125, -1.8125, -1.5, 4.75].map(Fixed::from_f64);
        let result = matrix_mul_scaled(&a, &b, 1);
        assert_eq!(result, expected);
    }

    /// See <https://github.com/googlefonts/fontations/issues/1638>
    #[test]
    fn nested_font_matrices() {
        // Expected values extracted from FreeType debugging session
        let font = FontRef::new(font_test_data::MATERIAL_ICONS_SUBSET_MATRIX).unwrap();
        let outlines = Outlines::from_cff(&font, 512).unwrap();
        // Check the normalized top dict matrix
        let (top_matrix, top_upem) = outlines.top_dict.font_matrix.unwrap();
        let expected_top_matrix = [65536, 0, 5604, 65536, 0, 0].map(Fixed::from_bits);
        assert_eq!(top_matrix, expected_top_matrix);
        assert_eq!(top_upem, 512);
        // Check the unnormalized font dict matrix
        let (sub_matrix, sub_upem) = outlines.parse_font_dict(0).unwrap().font_matrix.unwrap();
        let expected_sub_matrix = [327680, 0, 0, 327680, 0, 0].map(Fixed::from_bits);
        assert_eq!(sub_matrix, expected_sub_matrix);
        assert_eq!(sub_upem, 10);
        // Check the normalized combined matrix
        let subfont = outlines.subfont(0, Some(24.0), &[]).unwrap();
        let expected_combined_matrix = [65536, 0, 5604, 65536, 0, 0].map(Fixed::from_bits);
        assert_eq!(subfont.font_matrix.unwrap(), expected_combined_matrix);
        // Check the final scale
        assert_eq!(subfont.scale.unwrap().to_bits(), 98304);
    }

    /// OSS fuzz caught add with overflow for hint scale computation.
    /// See <https://oss-fuzz.com/testcase-detail/6498790355042304>
    /// and <https://issues.oss-fuzz.com/issues/444024349>
    #[test]
    fn subfont_hint_scale_overflow() {
        // Just don't panic with overflow
        let _ = scale_for_hinting(Some(Fixed::from_bits(i32::MAX)));
    }
}
