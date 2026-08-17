//! Scaling support for TrueType outlines.

mod deltas;
mod hint;
mod memory;
mod outline;

#[cfg(feature = "libm")]
#[allow(unused_imports)]
use core_maths::CoreFloat;

pub use hint::{HintError, HintInstance, HintOutline};
pub use outline::{Outline, ScaledOutline};
use raw::{FontRef, ReadError};

use super::{DrawError, GlyphHMetrics, Hinting};
use crate::GLYF_COMPOSITE_RECURSION_LIMIT;
use memory::{FreeTypeOutlineMemory, HarfBuzzOutlineMemory};

use read_fonts::{
    tables::{
        glyf::{
            Anchor, CompositeGlyph, CompositeGlyphFlags, Glyf, Glyph, PointMarker, SimpleGlyph,
        },
        gvar::Gvar,
        hdmx::Hdmx,
        loca::Loca,
    },
    types::{F26Dot6, F2Dot14, Fixed, GlyphId, Point, Tag},
    TableProvider,
};

/// Number of phantom points generated at the end of an outline.
pub const PHANTOM_POINT_COUNT: usize = 4;

/// Scaler state for TrueType outlines.
#[derive(Clone)]
pub struct Outlines<'a> {
    pub(crate) font: FontRef<'a>,
    pub(crate) glyph_metrics: GlyphHMetrics<'a>,
    loca: Loca<'a>,
    glyf: Glyf<'a>,
    gvar: Option<Gvar<'a>>,
    hdmx: Option<Hdmx<'a>>,
    fpgm: &'a [u8],
    prep: &'a [u8],
    cvt_len: u32,
    max_function_defs: u16,
    max_instruction_defs: u16,
    max_twilight_points: u16,
    max_stack_elements: u16,
    max_storage: u16,
    glyph_count: u16,
    units_per_em: u16,
    os2_vmetrics: [i16; 2],
    prefer_interpreter: bool,
    pub(crate) fractional_size_hinting: bool,
}

impl<'a> Outlines<'a> {
    pub fn new(font: &FontRef<'a>) -> Option<Self> {
        let head = font.head().ok()?;
        // If bit 3 of head.flags is set, then we round ppems when
        // scaling
        let fractional_size_hinting = !head
            .flags()
            .contains(read_fonts::tables::head::Flags::FORCE_INTEGER_PPEM);
        let loca = font.loca(Some(head.index_to_loc_format() == 1)).ok()?;
        let glyf = font.glyf().ok()?;
        let glyph_metrics = GlyphHMetrics::new(font)?;
        let (
            glyph_count,
            max_function_defs,
            max_instruction_defs,
            max_twilight_points,
            max_stack_elements,
            max_storage,
            max_instructions,
        ) = font
            .maxp()
            .map(|maxp| {
                (
                    maxp.num_glyphs(),
                    maxp.max_function_defs().unwrap_or_default(),
                    maxp.max_instruction_defs().unwrap_or_default(),
                    // Add 4 for phantom points
                    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttobjs.c#L1188>
                    maxp.max_twilight_points()
                        .unwrap_or_default()
                        .saturating_add(4),
                    // Add 32 to match FreeType's heuristic for buggy fonts
                    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/truetype/ttinterp.c#L356>
                    maxp.max_stack_elements()
                        .unwrap_or_default()
                        .saturating_add(32),
                    maxp.max_storage().unwrap_or_default(),
                    maxp.max_size_of_instructions().unwrap_or_default(),
                )
            })
            .unwrap_or_default();
        let os2_vmetrics = font
            .os2()
            .map(|os2| [os2.s_typo_ascender(), os2.s_typo_descender()])
            .unwrap_or_default();
        let fpgm = font
            .data_for_tag(Tag::new(b"fpgm"))
            .unwrap_or_default()
            .as_bytes();
        let prep = font
            .data_for_tag(Tag::new(b"prep"))
            .unwrap_or_default()
            .as_bytes();
        // Copy FreeType's logic on whether to use the interpreter:
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/base/ftobjs.c#L1001>
        let prefer_interpreter = !(max_instructions == 0 && fpgm.is_empty() && prep.is_empty());
        let cvt_len = font.cvt().map(|cvt| cvt.len() as u32).unwrap_or_default();
        Some(Self {
            font: font.clone(),
            glyph_metrics,
            loca,
            glyf,
            gvar: font.gvar().ok(),
            hdmx: font.hdmx().ok(),
            fpgm,
            prep,
            cvt_len,
            max_function_defs,
            max_instruction_defs,
            max_twilight_points,
            max_stack_elements,
            max_storage,
            glyph_count,
            units_per_em: font.head().ok()?.units_per_em(),
            os2_vmetrics,
            prefer_interpreter,
            fractional_size_hinting,
        })
    }

    pub fn units_per_em(&self) -> u16 {
        self.units_per_em
    }

    pub fn glyph_count(&self) -> usize {
        self.glyph_count as usize
    }

    pub fn prefer_interpreter(&self) -> bool {
        self.prefer_interpreter
    }

    pub fn outline(&self, glyph_id: GlyphId) -> Result<Outline<'a>, DrawError> {
        let mut outline = Outline {
            glyph_id,
            has_variations: self.gvar.is_some(),
            ..Default::default()
        };
        let glyph = self.loca.get_glyf(glyph_id, &self.glyf)?;
        if let Some(glyph) = glyph.as_ref() {
            self.outline_rec(glyph, &mut outline, 0, 0)?;
        }
        outline.points += PHANTOM_POINT_COUNT;
        outline.max_stack = self.max_stack_elements as usize;
        outline.cvt_count = self.cvt_len as usize;
        outline.storage_count = self.max_storage as usize;
        outline.max_twilight_points = self.max_twilight_points as usize;
        outline.glyph = glyph;
        Ok(outline)
    }

    pub fn compute_scale(&self, ppem: Option<f32>) -> (bool, F26Dot6) {
        if let Some(ppem) = ppem {
            if self.units_per_em > 0 {
                return (
                    true,
                    F26Dot6::from_bits((ppem * 64.) as i32)
                        / F26Dot6::from_bits(self.units_per_em as i32),
                );
            }
        }
        (false, F26Dot6::from_bits(0x10000))
    }

    pub fn compute_hinted_scale(&self, ppem: Option<f32>) -> (bool, F26Dot6) {
        if let Some(ppem) = ppem {
            if !self.fractional_size_hinting {
                // Apply a fixed point round to ppem if the font doesn't
                // support fractional scaling and hinting was requested.
                // FreeType does the same.
                // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttobjs.c#L1424>
                return self.compute_scale(Some(F26Dot6::from_f64(ppem as f64).round().to_f32()));
            }
        }
        self.compute_scale(ppem)
    }
}

impl Outlines<'_> {
    fn outline_rec(
        &self,
        glyph: &Glyph,
        outline: &mut Outline,
        component_depth: usize,
        recurse_depth: usize,
    ) -> Result<(), DrawError> {
        if recurse_depth > GLYF_COMPOSITE_RECURSION_LIMIT {
            return Err(DrawError::RecursionLimitExceeded(outline.glyph_id));
        }
        match glyph {
            Glyph::Simple(simple) => {
                let num_points = simple.num_points();
                let num_points_with_phantom = num_points + PHANTOM_POINT_COUNT;
                outline.max_simple_points = outline.max_simple_points.max(num_points_with_phantom);
                outline.points += num_points;
                outline.contours += simple.end_pts_of_contours().len();
                outline.has_hinting = outline.has_hinting || simple.instruction_length() != 0;
                outline.max_other_points = outline.max_other_points.max(num_points_with_phantom);
                outline.has_overlaps |= simple.has_overlapping_contours();
            }
            Glyph::Composite(composite) => {
                let (mut count, instructions) = composite.count_and_instructions();
                count += PHANTOM_POINT_COUNT;
                let point_base = outline.points;
                for (component, flags) in composite.component_glyphs_and_flags() {
                    outline.has_overlaps |= flags.contains(CompositeGlyphFlags::OVERLAP_COMPOUND);
                    let component_glyph = self.loca.get_glyf(component.into(), &self.glyf)?;
                    let Some(component_glyph) = component_glyph else {
                        continue;
                    };
                    self.outline_rec(
                        &component_glyph,
                        outline,
                        component_depth + count,
                        recurse_depth + 1,
                    )?;
                }
                let has_hinting = !instructions.unwrap_or_default().is_empty();
                if has_hinting {
                    // We only need the "other points" buffers if the
                    // composite glyph has instructions.
                    let num_points_in_composite = outline.points - point_base + PHANTOM_POINT_COUNT;
                    outline.max_other_points =
                        outline.max_other_points.max(num_points_in_composite);
                }
                outline.max_component_delta_stack = outline
                    .max_component_delta_stack
                    .max(component_depth + count);
                outline.has_hinting = outline.has_hinting || has_hinting;
            }
        }
        Ok(())
    }

    fn hdmx_width(&self, ppem: f32, glyph_id: GlyphId) -> Option<u8> {
        let hdmx = self.hdmx.as_ref()?;
        let ppem_u8 = ppem as u8;
        // Make sure our ppem is integral and fits into u8
        if ppem_u8 as f32 == ppem {
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttgload.c#L1996>
            hdmx.record_for_size(ppem_u8)?
                .widths
                .get(glyph_id.to_u32() as usize)
                .copied()
        } else {
            None
        }
    }
}

trait Scaler {
    fn outlines(&self) -> &Outlines<'_>;
    fn setup_phantom_points(
        &mut self,
        bounds: [i16; 4],
        lsb: i32,
        advance: i32,
        tsb: i32,
        vadvance: i32,
    );
    fn load_empty(&mut self, glyph_id: GlyphId) -> Result<(), DrawError>;
    fn load_simple(&mut self, glyph: &SimpleGlyph, glyph_id: GlyphId) -> Result<(), DrawError>;
    fn load_composite(
        &mut self,
        glyph: &CompositeGlyph,
        glyph_id: GlyphId,
        recurse_depth: usize,
    ) -> Result<(), DrawError>;

    fn load(
        &mut self,
        glyph: &Option<Glyph>,
        glyph_id: GlyphId,
        recurse_depth: usize,
    ) -> Result<(), DrawError> {
        if recurse_depth > GLYF_COMPOSITE_RECURSION_LIMIT {
            return Err(DrawError::RecursionLimitExceeded(glyph_id));
        }
        let bounds = match &glyph {
            Some(glyph) => [glyph.x_min(), glyph.x_max(), glyph.y_min(), glyph.y_max()],
            _ => [0; 4],
        };
        let outlines = self.outlines();
        let lsb = outlines.glyph_metrics.lsb(glyph_id, &[]);
        let advance = outlines.glyph_metrics.advance_width(glyph_id, &[]);
        let [ascent, descent] = outlines.os2_vmetrics.map(|x| x as i32);
        let tsb = ascent - bounds[3] as i32;
        let vadvance = ascent - descent;
        self.setup_phantom_points(bounds, lsb, advance, tsb, vadvance);
        match glyph {
            Some(Glyph::Simple(simple)) => self.load_simple(simple, glyph_id),
            Some(Glyph::Composite(composite)) => {
                self.load_composite(composite, glyph_id, recurse_depth)
            }
            None => self.load_empty(glyph_id),
        }
    }
}

/// f32 all the things. Hold your rounding. No hinting.
pub(crate) struct HarfBuzzScaler<'a> {
    outlines: &'a Outlines<'a>,
    memory: HarfBuzzOutlineMemory<'a>,
    coords: &'a [F2Dot14],
    point_count: usize,
    contour_count: usize,
    component_delta_count: usize,
    ppem: f32,
    scale: F26Dot6,
    is_scaled: bool,
    /// Phantom points. These are 4 extra points appended to the end of an
    /// outline that allow the bytecode interpreter to produce hinted
    /// metrics.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructing_glyphs#phantom-points>
    phantom: [Point<f32>; PHANTOM_POINT_COUNT],
}

impl<'a> HarfBuzzScaler<'a> {
    pub(crate) fn unhinted(
        outlines: &'a Outlines<'a>,
        outline: &'a Outline,
        buf: &'a mut [u8],
        ppem: Option<f32>,
        coords: &'a [F2Dot14],
    ) -> Result<Self, DrawError> {
        outline.ensure_point_count_limit()?;
        let (is_scaled, scale) = outlines.compute_scale(ppem);
        let memory =
            HarfBuzzOutlineMemory::new(outline, buf).ok_or(DrawError::InsufficientMemory)?;
        Ok(Self {
            outlines,
            memory,
            coords,
            point_count: 0,
            contour_count: 0,
            component_delta_count: 0,
            ppem: ppem.unwrap_or_default(),
            scale,
            is_scaled,
            phantom: Default::default(),
        })
    }

    pub(crate) fn scale(
        mut self,
        glyph: &Option<Glyph>,
        glyph_id: GlyphId,
    ) -> Result<ScaledOutline<'a, f32>, DrawError> {
        self.load(glyph, glyph_id, 0)?;
        Ok(ScaledOutline::new(
            &mut self.memory.points[..self.point_count],
            self.phantom,
            &mut self.memory.flags[..self.point_count],
            &mut self.memory.contours[..self.contour_count],
            self.outlines.hdmx_width(self.ppem, glyph_id),
        ))
    }
}

/// F26Dot6 coords, Fixed deltas, and a penchant for rounding
pub(crate) struct FreeTypeScaler<'a> {
    outlines: &'a Outlines<'a>,
    memory: FreeTypeOutlineMemory<'a>,
    coords: &'a [F2Dot14],
    point_count: usize,
    contour_count: usize,
    component_delta_count: usize,
    ppem: f32,
    scale: F26Dot6,
    is_scaled: bool,
    is_hinted: bool,
    pedantic_hinting: bool,
    /// Phantom points. These are 4 extra points appended to the end of an
    /// outline that allow the bytecode interpreter to produce hinted
    /// metrics.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructing_glyphs#phantom-points>
    phantom: [Point<F26Dot6>; PHANTOM_POINT_COUNT],
    hinter: Option<&'a HintInstance>,
}

impl<'a> FreeTypeScaler<'a> {
    pub(crate) fn unhinted(
        outlines: &'a Outlines<'a>,
        outline: &'a Outline,
        buf: &'a mut [u8],
        ppem: Option<f32>,
        coords: &'a [F2Dot14],
    ) -> Result<Self, DrawError> {
        outline.ensure_point_count_limit()?;
        let (is_scaled, scale) = outlines.compute_scale(ppem);
        let memory = FreeTypeOutlineMemory::new(outline, buf, Hinting::None)
            .ok_or(DrawError::InsufficientMemory)?;
        Ok(Self {
            outlines,
            memory,
            coords,
            point_count: 0,
            contour_count: 0,
            component_delta_count: 0,
            ppem: ppem.unwrap_or_default(),
            scale,
            is_scaled,
            is_hinted: false,
            pedantic_hinting: false,
            phantom: Default::default(),
            hinter: None,
        })
    }

    pub(crate) fn hinted(
        outlines: &'a Outlines<'a>,
        outline: &'a Outline,
        buf: &'a mut [u8],
        ppem: Option<f32>,
        coords: &'a [F2Dot14],
        hinter: &'a HintInstance,
        pedantic_hinting: bool,
    ) -> Result<Self, DrawError> {
        outline.ensure_point_count_limit()?;
        let (is_scaled, scale) = outlines.compute_hinted_scale(ppem);
        let memory = FreeTypeOutlineMemory::new(outline, buf, Hinting::Embedded)
            .ok_or(DrawError::InsufficientMemory)?;
        Ok(Self {
            outlines,
            memory,
            coords,
            point_count: 0,
            contour_count: 0,
            component_delta_count: 0,
            ppem: ppem.unwrap_or_default(),
            scale,
            is_scaled,
            // We don't hint unscaled outlines
            is_hinted: is_scaled,
            pedantic_hinting,
            phantom: Default::default(),
            hinter: Some(hinter),
        })
    }

    pub(crate) fn scale(
        mut self,
        glyph: &Option<Glyph>,
        glyph_id: GlyphId,
    ) -> Result<ScaledOutline<'a, F26Dot6>, DrawError> {
        self.load(glyph, glyph_id, 0)?;
        // Use hdmx if hinting is requested and backward compatibility mode
        // is not enabled.
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/truetype/ttgload.c#L2559>
        let hdmx_width = if self.is_hinted
            && self
                .hinter
                .as_ref()
                .map(|hinter| !hinter.backward_compatibility())
                .unwrap_or(true)
        {
            self.outlines.hdmx_width(self.ppem, glyph_id)
        } else {
            None
        };
        Ok(ScaledOutline::new(
            &mut self.memory.scaled[..self.point_count],
            self.phantom,
            &mut self.memory.flags[..self.point_count],
            &mut self.memory.contours[..self.contour_count],
            hdmx_width,
        ))
    }
}

impl Scaler for FreeTypeScaler<'_> {
    fn setup_phantom_points(
        &mut self,
        bounds: [i16; 4],
        lsb: i32,
        advance: i32,
        tsb: i32,
        vadvance: i32,
    ) {
        // The four "phantom" points as computed by FreeType.
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttgload.c#L1365>
        // horizontal:
        self.phantom[0].x = F26Dot6::from_bits(bounds[0] as i32 - lsb);
        self.phantom[0].y = F26Dot6::ZERO;
        self.phantom[1].x = self.phantom[0].x + F26Dot6::from_bits(advance);
        self.phantom[1].y = F26Dot6::ZERO;
        // vertical:
        self.phantom[2].x = F26Dot6::ZERO;
        self.phantom[2].y = F26Dot6::from_bits(bounds[3] as i32 + tsb);
        self.phantom[3].x = F26Dot6::ZERO;
        self.phantom[3].y = self.phantom[2].y - F26Dot6::from_bits(vadvance);
    }

    fn outlines(&self) -> &Outlines<'_> {
        self.outlines
    }

    fn load_empty(&mut self, glyph_id: GlyphId) -> Result<(), DrawError> {
        // Roughly corresponds to the FreeType code at
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttgload.c#L1572>
        let scale = self.scale;
        let mut unscaled = self.phantom.map(|point| point.map(|x| x.to_bits()));
        if self.outlines.gvar.is_some() && !self.coords.is_empty() {
            if let Ok(Some(deltas)) = self.outlines.gvar.as_ref().unwrap().phantom_point_deltas(
                &self.outlines.glyf,
                &self.outlines.loca,
                self.coords,
                glyph_id,
            ) {
                unscaled[0] += deltas[0].map(Fixed::to_i32);
                unscaled[1] += deltas[1].map(Fixed::to_i32);
            }
        }
        if self.is_scaled {
            for (phantom, unscaled) in self.phantom.iter_mut().zip(&unscaled) {
                *phantom = unscaled.map(F26Dot6::from_bits) * scale;
            }
        } else {
            for (phantom, unscaled) in self.phantom.iter_mut().zip(&unscaled) {
                *phantom = unscaled.map(F26Dot6::from_i32);
            }
        }
        Ok(())
    }

    fn load_simple(&mut self, glyph: &SimpleGlyph, glyph_id: GlyphId) -> Result<(), DrawError> {
        use DrawError::InsufficientMemory;
        // Compute the ranges for our point/flag buffers and slice them.
        let points_start = self.point_count;
        let point_count = glyph.num_points();
        let phantom_start = point_count;
        let points_end = points_start + point_count + PHANTOM_POINT_COUNT;
        let point_range = points_start..points_end;
        let other_points_end = point_count + PHANTOM_POINT_COUNT;
        // Scaled points and flags are accumulated as we load the outline.
        let scaled = self
            .memory
            .scaled
            .get_mut(point_range.clone())
            .ok_or(InsufficientMemory)?;
        let flags = self
            .memory
            .flags
            .get_mut(point_range)
            .ok_or(InsufficientMemory)?;
        // Unscaled points are temporary and are allocated as needed. We only
        // ever need one copy in memory for any simple or composite glyph so
        // allocate from the base of the buffer.
        let unscaled = self
            .memory
            .unscaled
            .get_mut(..other_points_end)
            .ok_or(InsufficientMemory)?;
        // Read our unscaled points and flags (up to point_count which does not
        // include phantom points).
        glyph.read_points_fast(&mut unscaled[..point_count], &mut flags[..point_count])?;
        // Compute the range for our contour end point buffer and slice it.
        let contours_start = self.contour_count;
        let contour_end_pts = glyph.end_pts_of_contours();
        let contour_count = contour_end_pts.len();
        let contours_end = contours_start + contour_count;
        let contours = self
            .memory
            .contours
            .get_mut(contours_start..contours_end)
            .ok_or(InsufficientMemory)?;
        // Read the contour end points, ensuring that they are properly
        // ordered.
        let mut last_end_pt = 0;
        for (end_pt, contour) in contour_end_pts.iter().zip(contours.iter_mut()) {
            let end_pt = end_pt.get();
            if end_pt < last_end_pt {
                return Err(ReadError::MalformedData(
                    "unordered contour end points in TrueType glyph",
                )
                .into());
            }
            last_end_pt = end_pt;
            *contour = end_pt;
        }
        // Adjust the running point/contour total counts
        self.point_count += point_count;
        self.contour_count += contour_count;
        // Append phantom points to the outline.
        for (i, phantom) in self.phantom.iter().enumerate() {
            unscaled[phantom_start + i] = phantom.map(|x| x.to_bits());
            flags[phantom_start + i] = Default::default();
        }
        let mut have_deltas = false;
        if self.outlines.gvar.is_some() && !self.coords.is_empty() {
            let gvar = self.outlines.gvar.as_ref().unwrap();
            let glyph = deltas::SimpleGlyph {
                points: &mut unscaled[..],
                flags: &mut flags[..],
                contours,
            };
            let deltas = self
                .memory
                .deltas
                .get_mut(..point_count + PHANTOM_POINT_COUNT)
                .ok_or(InsufficientMemory)?;
            let iup_buffer = self
                .memory
                .iup_buffer
                .get_mut(..point_count + PHANTOM_POINT_COUNT)
                .ok_or(InsufficientMemory)?;
            if deltas::simple_glyph(gvar, glyph_id, self.coords, glyph, iup_buffer, deltas).is_ok()
            {
                have_deltas = true;
            }
        }
        let ins = glyph.instructions();
        let is_hinted = self.is_hinted;
        if self.is_scaled {
            let scale = self.scale;
            if have_deltas {
                for ((point, unscaled), delta) in scaled
                    .iter_mut()
                    .zip(unscaled.iter())
                    .zip(self.memory.deltas.iter())
                {
                    let delta = delta.map(Fixed::to_f26dot6);
                    let scaled = (unscaled.map(F26Dot6::from_i32) + delta) * scale;
                    // The computed scale factor has an i32 -> 26.26 conversion built in. This undoes the
                    // extra shift.
                    *point = scaled.map(|v| F26Dot6::from_bits(v.to_i32()));
                }
                // FreeType applies different rounding to HVAR deltas. Since
                // we're only using gvar, mimic that behavior for phantom point
                // deltas when an HVAR table is present
                if self.outlines.glyph_metrics.hvar.is_some() {
                    for ((point, unscaled), delta) in scaled[phantom_start..]
                        .iter_mut()
                        .zip(&unscaled[phantom_start..])
                        .zip(&self.memory.deltas[phantom_start..])
                    {
                        let delta = delta.map(Fixed::to_i32).map(F26Dot6::from_i32);
                        let scaled = (unscaled.map(F26Dot6::from_i32) + delta) * scale;
                        *point = scaled.map(|v| F26Dot6::from_bits(v.to_i32()));
                    }
                }
                if is_hinted {
                    // For hinting, we need to adjust the unscaled points as well.
                    // Round off deltas for unscaled outlines.
                    for (unscaled, delta) in unscaled.iter_mut().zip(self.memory.deltas.iter()) {
                        *unscaled += delta.map(Fixed::to_i32);
                    }
                }
            } else {
                for (point, unscaled) in scaled.iter_mut().zip(unscaled.iter_mut()) {
                    *point = unscaled.map(|v| F26Dot6::from_bits(v) * scale);
                }
            }
        } else {
            if have_deltas {
                // Round off deltas for unscaled outlines.
                for (unscaled, delta) in unscaled.iter_mut().zip(self.memory.deltas.iter()) {
                    *unscaled += delta.map(Fixed::to_i32);
                }
            }
            // Unlike FreeType, we also store unscaled outlines in 26.6.
            for (point, unscaled) in scaled.iter_mut().zip(unscaled.iter()) {
                *point = unscaled.map(F26Dot6::from_i32);
            }
        }
        // Commit our potentially modified phantom points.
        self.phantom.copy_from_slice(&scaled[phantom_start..]);
        if let (Some(hinter), true) = (self.hinter.as_ref(), is_hinted) {
            if !ins.is_empty() {
                // Create a copy of our scaled points in original_scaled.
                let original_scaled = self
                    .memory
                    .original_scaled
                    .get_mut(..other_points_end)
                    .ok_or(InsufficientMemory)?;
                original_scaled.copy_from_slice(scaled);
                // When hinting, round the phantom points.
                for point in &mut scaled[phantom_start..] {
                    point.x = point.x.round();
                    point.y = point.y.round();
                }
                let mut input = HintOutline {
                    glyph_id,
                    unscaled,
                    scaled,
                    original_scaled,
                    flags,
                    contours,
                    bytecode: ins,
                    phantom: &mut self.phantom,
                    stack: self.memory.stack,
                    cvt: self.memory.cvt,
                    storage: self.memory.storage,
                    twilight_scaled: self.memory.twilight_scaled,
                    twilight_original_scaled: self.memory.twilight_original_scaled,
                    twilight_flags: self.memory.twilight_flags,
                    is_composite: false,
                    coords: self.coords,
                };
                let hint_res = hinter.hint(self.outlines, &mut input, self.pedantic_hinting);
                if let (Err(e), true) = (hint_res, self.pedantic_hinting) {
                    return Err(e)?;
                }
            } else if !hinter.backward_compatibility() {
                // Even when missing instructions, FreeType uses rounded
                // phantom points when hinting is requested and backward
                // compatibility mode is disabled.
                // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttgload.c#L823>
                // Notably, FreeType never calls TT_Hint_Glyph for composite
                // glyphs when instructions are missing so this only applies
                // to simple glyphs.
                for (scaled, phantom) in scaled[phantom_start..].iter().zip(&mut self.phantom) {
                    *phantom = scaled.map(|x| x.round());
                }
            }
        }
        if points_start != 0 {
            // If we're not the first component, shift our contour end points.
            for contour_end in contours.iter_mut() {
                *contour_end += points_start as u16;
            }
        }
        Ok(())
    }

    fn load_composite(
        &mut self,
        glyph: &CompositeGlyph,
        glyph_id: GlyphId,
        recurse_depth: usize,
    ) -> Result<(), DrawError> {
        use DrawError::InsufficientMemory;
        let scale = self.scale;
        // The base indices of the points and contours for the current glyph.
        let point_base = self.point_count;
        let contour_base = self.contour_count;
        // Compute the per component deltas. Since composites can be nested, we
        // use a stack and keep track of the base.
        let mut have_deltas = false;
        let delta_base = self.component_delta_count;
        if self.outlines.gvar.is_some() && !self.coords.is_empty() {
            let gvar = self.outlines.gvar.as_ref().unwrap();
            let count = glyph.components().count() + PHANTOM_POINT_COUNT;
            let deltas = self
                .memory
                .composite_deltas
                .get_mut(delta_base..delta_base + count)
                .ok_or(InsufficientMemory)?;
            if deltas::composite_glyph(gvar, glyph_id, self.coords, &mut deltas[..]).is_ok() {
                // Apply deltas to phantom points.
                for (phantom, delta) in self
                    .phantom
                    .iter_mut()
                    .zip(&deltas[deltas.len() - PHANTOM_POINT_COUNT..])
                {
                    *phantom += delta.map(Fixed::to_i32).map(F26Dot6::from_bits);
                }
                have_deltas = true;
            }
            self.component_delta_count += count;
        }
        if self.is_scaled {
            for point in self.phantom.iter_mut() {
                *point *= scale;
            }
        } else {
            for point in self.phantom.iter_mut() {
                *point = point.map(|x| F26Dot6::from_i32(x.to_bits()));
            }
        }
        for (i, component) in glyph.components().enumerate() {
            // Loading a component glyph will override phantom points so save a copy. We'll
            // restore them unless the USE_MY_METRICS flag is set.
            let phantom = self.phantom;
            // Load the component glyph and keep track of the points range.
            let start_point = self.point_count;
            let component_glyph = self
                .outlines
                .loca
                .get_glyf(component.glyph.into(), &self.outlines.glyf)?;
            self.load(&component_glyph, component.glyph.into(), recurse_depth + 1)?;
            let end_point = self.point_count;
            if !component
                .flags
                .contains(CompositeGlyphFlags::USE_MY_METRICS)
            {
                // If the USE_MY_METRICS flag is missing, we restore the phantom points we
                // saved at the start of the loop.
                self.phantom = phantom;
            }
            // Prepares the transform components for our conversion math below.
            fn scale_component(x: F2Dot14) -> F26Dot6 {
                F26Dot6::from_bits(x.to_bits() as i32 * 4)
            }
            let xform = &component.transform;
            let xx = scale_component(xform.xx);
            let yx = scale_component(xform.yx);
            let xy = scale_component(xform.xy);
            let yy = scale_component(xform.yy);
            let have_xform = component.flags.intersects(
                CompositeGlyphFlags::WE_HAVE_A_SCALE
                    | CompositeGlyphFlags::WE_HAVE_AN_X_AND_Y_SCALE
                    | CompositeGlyphFlags::WE_HAVE_A_TWO_BY_TWO,
            );
            if have_xform {
                let scaled = &mut self.memory.scaled[start_point..end_point];
                if self.is_scaled {
                    for point in scaled {
                        let x = point.x * xx + point.y * xy;
                        let y = point.x * yx + point.y * yy;
                        point.x = x;
                        point.y = y;
                    }
                } else {
                    for point in scaled {
                        // This juggling is necessary because, unlike FreeType, we also
                        // return unscaled outlines in 26.6 format for a consistent interface.
                        let unscaled = point.map(|c| F26Dot6::from_bits(c.to_i32()));
                        let x = unscaled.x * xx + unscaled.y * xy;
                        let y = unscaled.x * yx + unscaled.y * yy;
                        *point = Point::new(x, y).map(|c| F26Dot6::from_i32(c.to_bits()));
                    }
                }
            }
            let anchor_offset = match component.anchor {
                Anchor::Offset { x, y } => {
                    let (mut x, mut y) = (x as i32, y as i32);
                    if have_xform
                        && component.flags
                            & (CompositeGlyphFlags::SCALED_COMPONENT_OFFSET
                                | CompositeGlyphFlags::UNSCALED_COMPONENT_OFFSET)
                            == CompositeGlyphFlags::SCALED_COMPONENT_OFFSET
                    {
                        // According to FreeType, this algorithm is a "guess"
                        // and works better than the one documented by Apple.
                        // https://github.com/freetype/freetype/blob/b1c90733ee6a04882b133101d61b12e352eeb290/src/truetype/ttgload.c#L1259
                        fn hypot(a: F26Dot6, b: F26Dot6) -> Fixed {
                            let a = a.to_bits().abs();
                            let b = b.to_bits().abs();
                            Fixed::from_bits(if a > b {
                                a + ((3 * b) >> 3)
                            } else {
                                b + ((3 * a) >> 3)
                            })
                        }
                        // FreeType uses a fixed point multiplication here.
                        x = (Fixed::from_bits(x) * hypot(xx, xy)).to_bits();
                        y = (Fixed::from_bits(y) * hypot(yy, yx)).to_bits();
                    }
                    if have_deltas {
                        let delta = self
                            .memory
                            .composite_deltas
                            .get(delta_base + i)
                            .copied()
                            .unwrap_or_default();
                        // For composite glyphs, we copy FreeType and round off
                        // the fractional parts of deltas.
                        x += delta.x.to_i32();
                        y += delta.y.to_i32();
                    }
                    if self.is_scaled {
                        let mut offset = Point::new(x, y).map(F26Dot6::from_bits) * scale;
                        if self.is_hinted
                            && component
                                .flags
                                .contains(CompositeGlyphFlags::ROUND_XY_TO_GRID)
                        {
                            // Only round the y-coordinate, per FreeType.
                            offset.y = offset.y.round();
                        }
                        offset
                    } else {
                        Point::new(x, y).map(F26Dot6::from_i32)
                    }
                }
                Anchor::Point { base, component } => {
                    let (base_offset, component_offset) = (base as usize, component as usize);
                    let base_point = self
                        .memory
                        .scaled
                        .get(point_base + base_offset)
                        .ok_or(DrawError::InvalidAnchorPoint(glyph_id, base))?;
                    let component_point = self
                        .memory
                        .scaled
                        .get(start_point + component_offset)
                        .ok_or(DrawError::InvalidAnchorPoint(glyph_id, component))?;
                    *base_point - *component_point
                }
            };
            if anchor_offset.x != F26Dot6::ZERO || anchor_offset.y != F26Dot6::ZERO {
                for point in &mut self.memory.scaled[start_point..end_point] {
                    *point += anchor_offset;
                }
            }
        }
        if have_deltas {
            self.component_delta_count = delta_base;
        }
        if let (Some(hinter), true) = (self.hinter.as_ref(), self.is_hinted) {
            let ins = glyph.instructions().unwrap_or_default();
            if !ins.is_empty() {
                // For composite glyphs, the unscaled and original points are
                // simply copies of the current point set.
                let start_point = point_base;
                let end_point = self.point_count + PHANTOM_POINT_COUNT;
                let point_range = start_point..end_point;
                let phantom_start = point_range.len() - PHANTOM_POINT_COUNT;
                let scaled = &mut self.memory.scaled[point_range.clone()];
                let flags = self
                    .memory
                    .flags
                    .get_mut(point_range.clone())
                    .ok_or(InsufficientMemory)?;
                // Append the current phantom points to the outline.
                for (i, phantom) in self.phantom.iter().enumerate() {
                    scaled[phantom_start + i] = *phantom;
                    flags[phantom_start + i] = Default::default();
                }
                let other_points_end = point_range.len();
                let unscaled = self
                    .memory
                    .unscaled
                    .get_mut(..other_points_end)
                    .ok_or(InsufficientMemory)?;
                for (scaled, unscaled) in scaled.iter().zip(unscaled.iter_mut()) {
                    *unscaled = scaled.map(|x| x.to_bits());
                }
                let original_scaled = self
                    .memory
                    .original_scaled
                    .get_mut(..other_points_end)
                    .ok_or(InsufficientMemory)?;
                original_scaled.copy_from_slice(scaled);
                let contours = self
                    .memory
                    .contours
                    .get_mut(contour_base..self.contour_count)
                    .ok_or(InsufficientMemory)?;
                // Round the phantom points.
                for p in &mut scaled[phantom_start..] {
                    p.x = p.x.round();
                    p.y = p.y.round();
                }
                // Clear the "touched" flags that are used during IUP processing.
                for flag in flags.iter_mut() {
                    flag.clear_marker(PointMarker::TOUCHED);
                }
                // Make sure our contour end points accurately reflect the
                // outline slices.
                if point_base != 0 {
                    let delta = point_base as u16;
                    for contour in contours.iter_mut() {
                        *contour -= delta;
                    }
                }
                let mut input = HintOutline {
                    glyph_id,
                    unscaled,
                    scaled,
                    original_scaled,
                    flags,
                    contours,
                    bytecode: ins,
                    phantom: &mut self.phantom,
                    stack: self.memory.stack,
                    cvt: self.memory.cvt,
                    storage: self.memory.storage,
                    twilight_scaled: self.memory.twilight_scaled,
                    twilight_original_scaled: self.memory.twilight_original_scaled,
                    twilight_flags: self.memory.twilight_flags,
                    is_composite: true,
                    coords: self.coords,
                };
                let hint_res = hinter.hint(self.outlines, &mut input, self.pedantic_hinting);
                if let (Err(e), true) = (hint_res, self.pedantic_hinting) {
                    return Err(e)?;
                }
                // Undo the contour shifts if we applied them above.
                if point_base != 0 {
                    let delta = point_base as u16;
                    for contour in contours.iter_mut() {
                        *contour += delta;
                    }
                }
            }
        }
        Ok(())
    }
}

impl Scaler for HarfBuzzScaler<'_> {
    fn setup_phantom_points(
        &mut self,
        bounds: [i16; 4],
        lsb: i32,
        advance: i32,
        tsb: i32,
        vadvance: i32,
    ) {
        // Same pattern as FreeType, just f32
        // horizontal:
        self.phantom[0].x = bounds[0] as f32 - lsb as f32;
        self.phantom[0].y = 0.0;
        self.phantom[1].x = self.phantom[0].x + advance as f32;
        self.phantom[1].y = 0.0;
        // vertical:
        self.phantom[2].x = 0.0;
        self.phantom[2].y = bounds[3] as f32 + tsb as f32;
        self.phantom[3].x = 0.0;
        self.phantom[3].y = self.phantom[2].y - vadvance as f32;
    }

    fn outlines(&self) -> &Outlines<'_> {
        self.outlines
    }

    fn load_empty(&mut self, glyph_id: GlyphId) -> Result<(), DrawError> {
        // HB doesn't have an equivalent so this version just copies the
        // FreeType version above but changed to use floating point
        let scale = self.scale.to_f32();
        let mut unscaled = self.phantom;
        if self.outlines.glyph_metrics.hvar.is_none()
            && self.outlines.gvar.is_some()
            && !self.coords.is_empty()
        {
            if let Ok(Some(deltas)) = self.outlines.gvar.as_ref().unwrap().phantom_point_deltas(
                &self.outlines.glyf,
                &self.outlines.loca,
                self.coords,
                glyph_id,
            ) {
                unscaled[0] += deltas[0].map(Fixed::to_f32);
                unscaled[1] += deltas[1].map(Fixed::to_f32);
            }
        }
        if self.is_scaled {
            for (phantom, unscaled) in self.phantom.iter_mut().zip(&unscaled) {
                *phantom = *unscaled * scale;
            }
        } else {
            for (phantom, unscaled) in self.phantom.iter_mut().zip(&unscaled) {
                *phantom = *unscaled;
            }
        }
        Ok(())
    }

    fn load_simple(&mut self, glyph: &SimpleGlyph, glyph_id: GlyphId) -> Result<(), DrawError> {
        use DrawError::InsufficientMemory;
        // Compute the ranges for our point/flag buffers and slice them.
        let points_start = self.point_count;
        let point_count = glyph.num_points();
        let phantom_start = point_count;
        let points_end = points_start + point_count + PHANTOM_POINT_COUNT;
        let point_range = points_start..points_end;
        // Points and flags are accumulated as we load the outline.
        let points = self
            .memory
            .points
            .get_mut(point_range.clone())
            .ok_or(InsufficientMemory)?;
        let flags = self
            .memory
            .flags
            .get_mut(point_range)
            .ok_or(InsufficientMemory)?;
        glyph.read_points_fast(&mut points[..point_count], &mut flags[..point_count])?;
        // Compute the range for our contour end point buffer and slice it.
        let contours_start = self.contour_count;
        let contour_end_pts = glyph.end_pts_of_contours();
        let contour_count = contour_end_pts.len();
        let contours_end = contours_start + contour_count;
        let contours = self
            .memory
            .contours
            .get_mut(contours_start..contours_end)
            .ok_or(InsufficientMemory)?;
        // Read the contour end points.
        for (end_pt, contour) in contour_end_pts.iter().zip(contours.iter_mut()) {
            *contour = end_pt.get();
        }
        // Adjust the running point/contour total counts
        self.point_count += point_count;
        self.contour_count += contour_count;
        // Append phantom points to the outline.
        for (i, phantom) in self.phantom.iter().enumerate() {
            points[phantom_start + i] = *phantom;
            flags[phantom_start + i] = Default::default();
        }
        // Acquire deltas
        if self.outlines.gvar.is_some() && !self.coords.is_empty() {
            let gvar = self.outlines.gvar.as_ref().unwrap();
            let glyph = deltas::SimpleGlyph {
                points: &mut points[..],
                flags: &mut flags[..],
                contours,
            };
            let deltas = self
                .memory
                .deltas
                .get_mut(..point_count + PHANTOM_POINT_COUNT)
                .ok_or(InsufficientMemory)?;
            let iup_buffer = self
                .memory
                .iup_buffer
                .get_mut(..point_count + PHANTOM_POINT_COUNT)
                .ok_or(InsufficientMemory)?;
            if deltas::simple_glyph(gvar, glyph_id, self.coords, glyph, iup_buffer, deltas).is_ok()
            {
                for (point, delta) in points.iter_mut().zip(deltas) {
                    *point += *delta;
                }
            }
        }
        // Apply scaling
        if self.is_scaled {
            let scale = self.scale.to_f32();
            for point in points.iter_mut() {
                *point = point.map(|c| c * scale);
            }
        }

        if points_start != 0 {
            // If we're not the first component, shift our contour end points.
            for contour_end in contours.iter_mut() {
                *contour_end += points_start as u16;
            }
        }
        Ok(())
    }

    fn load_composite(
        &mut self,
        glyph: &CompositeGlyph,
        glyph_id: GlyphId,
        recurse_depth: usize,
    ) -> Result<(), DrawError> {
        use DrawError::InsufficientMemory;
        let scale = self.scale.to_f32();
        // The base indices of the points for the current glyph.
        let point_base = self.point_count;
        // Compute the per component deltas. Since composites can be nested, we
        // use a stack and keep track of the base.
        let mut have_deltas = false;
        let delta_base = self.component_delta_count;
        if self.outlines.gvar.is_some() && !self.coords.is_empty() {
            let gvar = self.outlines.gvar.as_ref().unwrap();
            let count = glyph.components().count() + PHANTOM_POINT_COUNT;
            let deltas = self
                .memory
                .composite_deltas
                .get_mut(delta_base..delta_base + count)
                .ok_or(InsufficientMemory)?;
            if deltas::composite_glyph(gvar, glyph_id, self.coords, &mut deltas[..]).is_ok() {
                // Apply deltas to phantom points.
                for (phantom, delta) in self
                    .phantom
                    .iter_mut()
                    .zip(&deltas[deltas.len() - PHANTOM_POINT_COUNT..])
                {
                    *phantom += *delta;
                }
                have_deltas = true;
            }
            self.component_delta_count += count;
        }
        if self.is_scaled {
            for point in self.phantom.iter_mut() {
                *point *= scale;
            }
        }
        for (i, component) in glyph.components().enumerate() {
            // Loading a component glyph will override phantom points so save a copy. We'll
            // restore them unless the USE_MY_METRICS flag is set.
            let phantom = self.phantom;
            // Load the component glyph and keep track of the points range.
            let start_point = self.point_count;
            let component_glyph = self
                .outlines
                .loca
                .get_glyf(component.glyph.into(), &self.outlines.glyf)?;
            self.load(&component_glyph, component.glyph.into(), recurse_depth + 1)?;
            let end_point = self.point_count;
            if !component
                .flags
                .contains(CompositeGlyphFlags::USE_MY_METRICS)
            {
                // If the USE_MY_METRICS flag is missing, we restore the phantom points we
                // saved at the start of the loop.
                self.phantom = phantom;
            }
            let have_xform = component.flags.intersects(
                CompositeGlyphFlags::WE_HAVE_A_SCALE
                    | CompositeGlyphFlags::WE_HAVE_AN_X_AND_Y_SCALE
                    | CompositeGlyphFlags::WE_HAVE_A_TWO_BY_TWO,
            );
            let mut transform = if have_xform {
                let xform = &component.transform;
                [
                    xform.xx,
                    xform.yx,
                    xform.xy,
                    xform.yy,
                    F2Dot14::ZERO,
                    F2Dot14::ZERO,
                ]
                .map(|x| x.to_f32())
            } else {
                [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] // identity
            };

            let anchor_offset = match component.anchor {
                Anchor::Offset { x, y } => {
                    let (mut x, mut y) = (x as f32, y as f32);
                    if have_xform
                        && component.flags
                            & (CompositeGlyphFlags::SCALED_COMPONENT_OFFSET
                                | CompositeGlyphFlags::UNSCALED_COMPONENT_OFFSET)
                            == CompositeGlyphFlags::SCALED_COMPONENT_OFFSET
                    {
                        // Scale x by the magnitude of the x-basis, y by the y-basis
                        // FreeType implements hypot, we can just use the provided implementation
                        x *= hypot(transform[0], transform[2]);
                        y *= hypot(transform[1], transform[3]);
                    }
                    Point::new(x, y)
                        + self
                            .memory
                            .composite_deltas
                            .get(delta_base + i)
                            .copied()
                            .unwrap_or_default()
                }
                Anchor::Point { base, component } => {
                    let (base_offset, component_offset) = (base as usize, component as usize);
                    let base_point = self
                        .memory
                        .points
                        .get(point_base + base_offset)
                        .ok_or(DrawError::InvalidAnchorPoint(glyph_id, base))?;
                    let component_point = self
                        .memory
                        .points
                        .get(start_point + component_offset)
                        .ok_or(DrawError::InvalidAnchorPoint(glyph_id, component))?;
                    *base_point - *component_point
                }
            };
            transform[4] = anchor_offset.x;
            transform[5] = anchor_offset.y;

            let points = &mut self.memory.points[start_point..end_point];
            for point in points.iter_mut() {
                *point = map_point(transform, *point);
            }
        }
        if have_deltas {
            self.component_delta_count = delta_base;
        }
        Ok(())
    }
}

/// Magnitude of the vector (x, y)
fn hypot(x: f32, y: f32) -> f32 {
    x.hypot(y)
}

fn map_point(transform: [f32; 6], p: Point<f32>) -> Point<f32> {
    Point {
        x: transform[0] * p.x + transform[2] * p.y + transform[4],
        y: transform[1] * p.x + transform[3] * p.y + transform[5],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MetadataProvider;
    use raw::{
        tables::{
            glyf::{CompositeGlyphFlags, Glyf, SimpleGlyphFlags},
            loca::Loca,
        },
        FontRead, FontRef, TableProvider,
    };

    #[test]
    fn overlap_flags() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        let scaler = Outlines::new(&font).unwrap();
        let glyph_count = font.maxp().unwrap().num_glyphs();
        // GID 2 is a composite glyph with the overlap bit on a component
        // GID 3 is a simple glyph with the overlap bit on the first flag
        let expected_gids_with_overlap = vec![2, 3];
        assert_eq!(
            expected_gids_with_overlap,
            (0..glyph_count)
                .filter(|gid| scaler.outline(GlyphId::from(*gid)).unwrap().has_overlaps)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn interpreter_preference() {
        // no instructions in this font...
        let font = FontRef::new(font_test_data::COLRV0V1).unwrap();
        let outlines = Outlines::new(&font).unwrap();
        // thus no preference for the interpreter
        assert!(!outlines.prefer_interpreter());
        // but this one has instructions...
        let font = FontRef::new(font_test_data::TTHINT_SUBSET).unwrap();
        let outlines = Outlines::new(&font).unwrap();
        // so let's use it
        assert!(outlines.prefer_interpreter());
    }

    #[test]
    fn empty_glyph_advance() {
        let font = FontRef::new(font_test_data::HVAR_WITH_TRUNCATED_ADVANCE_INDEX_MAP).unwrap();
        let outlines = Outlines::new(&font).unwrap();
        let coords = [F2Dot14::from_f32(0.5)];
        let ppem = Some(24.0);
        let gid = font.charmap().map(' ').unwrap();
        let outline = outlines.outline(gid).unwrap();
        // Make sure this is an empty outline since that's what we're testing
        assert!(outline.glyph.is_none());
        let mut buf = [0u8; 128];
        let scaler =
            FreeTypeScaler::unhinted(&outlines, &outline, &mut buf, ppem, &coords).unwrap();
        let scaled = scaler.scale(&outline.glyph, gid).unwrap();
        let advance = scaled.adjusted_advance_width();
        assert!(advance != F26Dot6::ZERO);
    }

    #[test]
    fn empty_glyphs_have_phantom_points_too() {
        let font = FontRef::new(font_test_data::HVAR_WITH_TRUNCATED_ADVANCE_INDEX_MAP).unwrap();
        let outlines = Outlines::new(&font).unwrap();
        let gid = font.charmap().map(' ').unwrap();
        let outline = outlines.outline(gid).unwrap();
        assert!(outline.glyph.is_none());
        assert_eq!(outline.points, PHANTOM_POINT_COUNT);
    }

    // fuzzer overflow for composite glyph with too many total points
    // <https://issues.oss-fuzz.com/issues/391753684
    #[test]
    fn composite_with_too_many_points() {
        let font = FontRef::new(font_test_data::GLYF_COMPONENTS).unwrap();
        let mut outlines = Outlines::new(&font).unwrap();
        // Hack glyf and loca to build a glyph that contains more than 64k
        // total points
        let mut glyf_buf = font_test_data::bebuffer::BeBuffer::new();
        // Make a component glyph with 40k points so we overflow the
        // total limit in a composite
        let simple_glyph_point_count = 40000;
        glyf_buf = glyf_buf.push(1u16); // number of contours
        glyf_buf = glyf_buf.extend([0i16; 4]); // bbox
        glyf_buf = glyf_buf.push((simple_glyph_point_count - 1) as u16); // contour ends
        glyf_buf = glyf_buf.push(0u16); // instruction count
        for _ in 0..simple_glyph_point_count {
            glyf_buf =
                glyf_buf.push(SimpleGlyphFlags::X_SHORT_VECTOR | SimpleGlyphFlags::Y_SHORT_VECTOR);
        }
        // x/y coords
        for _ in 0..simple_glyph_point_count * 2 {
            glyf_buf = glyf_buf.push(0u8);
        }
        let glyph0_end = glyf_buf.len();
        // Now make a composite with two components
        glyf_buf = glyf_buf.push(-1i16); // negative signifies composite
        glyf_buf = glyf_buf.extend([0i16; 4]); // bbox
        for i in 0..2 {
            let flags = if i == 0 {
                CompositeGlyphFlags::MORE_COMPONENTS | CompositeGlyphFlags::ARGS_ARE_XY_VALUES
            } else {
                CompositeGlyphFlags::ARGS_ARE_XY_VALUES
            };
            glyf_buf = glyf_buf.push(flags); // component flag
            glyf_buf = glyf_buf.push(0u16); // component gid
            glyf_buf = glyf_buf.extend([0u8; 2]); // x/y offset
        }
        let glyph1_end = glyf_buf.len();
        outlines.glyf = Glyf::read(glyf_buf.data().into()).unwrap();
        // Now create a loca table
        let mut loca_buf = font_test_data::bebuffer::BeBuffer::new();
        loca_buf = loca_buf.extend([0u32, glyph0_end as u32, glyph1_end as u32]);
        outlines.loca = Loca::read(loca_buf.data().into(), true).unwrap();
        let gid = GlyphId::new(1);
        let outline = outlines.outline(gid).unwrap();
        let mut mem_buf = vec![0u8; outline.required_buffer_size(Default::default())];
        // This outline has more than 64k points...
        assert!(outline.points > u16::MAX as usize);
        let result = FreeTypeScaler::unhinted(&outlines, &outline, &mut mem_buf, None, &[]);
        // And we get an error instead of an overflow panic
        assert!(matches!(result, Err(DrawError::TooManyPoints(_))));
    }

    #[test]
    fn fractional_size_hinting() {
        let font = FontRef::from_index(font_test_data::TINOS_SUBSET, 0).unwrap();
        let outlines = Outlines::new(&font).unwrap();
        // Make sure we capture the correct bit
        assert!(!outlines.fractional_size_hinting);
        // Check proper rounding when computing scale for fractional ppem
        // values
        for size in [10.0, 10.2, 10.5, 10.8, 11.0] {
            assert_eq!(
                outlines.compute_hinted_scale(Some(size)),
                outlines.compute_hinted_scale(Some(size.round()))
            );
        }
    }
}
