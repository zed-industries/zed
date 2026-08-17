//! computing closure for the colr table

use font_types::{GlyphId, GlyphId16};

use crate::{collections::IntSet, tables::variations::NO_VARIATION_INDEX, ResolveOffset};

use super::{
    Clip, ClipBox, ClipBoxFormat2, ClipList, ColorLine, ColorStop, Colr, Paint, PaintColrGlyph,
    PaintColrLayers, PaintComposite, PaintGlyph, PaintLinearGradient, PaintRadialGradient,
    PaintRotate, PaintRotateAroundCenter, PaintScale, PaintScaleAroundCenter, PaintScaleUniform,
    PaintScaleUniformAroundCenter, PaintSkew, PaintSkewAroundCenter, PaintSolid,
    PaintSweepGradient, PaintTransform, PaintTranslate, PaintVarLinearGradient,
    PaintVarRadialGradient, PaintVarRotate, PaintVarRotateAroundCenter, PaintVarScale,
    PaintVarScaleAroundCenter, PaintVarScaleUniform, PaintVarScaleUniformAroundCenter,
    PaintVarSkew, PaintVarSkewAroundCenter, PaintVarSolid, PaintVarSweepGradient,
    PaintVarTransform, PaintVarTranslate, VarAffine2x3, VarColorLine, VarColorStop,
};

impl Colr<'_> {
    //Collect the transitive closure of V0 palette indices needed for all of the input glyphs set
    //It's similar to closure glyphs but in a separate fn, because v1 closure might adds more v0 glyphs, so this fn needs to be called after v1 closure
    pub fn v0_closure_palette_indices(
        &self,
        glyph_set: &IntSet<GlyphId>,
        palette_indices: &mut IntSet<u16>,
    ) {
        let Some(Ok(records)) = self.base_glyph_records() else {
            return;
        };
        for glyph_id in glyph_set.iter() {
            let Ok(glyph_id) = glyph_id.try_into() else {
                continue;
            };
            let record = match records.binary_search_by(|rec| rec.glyph_id().cmp(&glyph_id)) {
                Ok(idx) => records[idx],
                _ => continue,
            };
            let start = record.first_layer_index() as usize;
            let end = start + record.num_layers() as usize;
            for layer_index in start..end {
                if let Ok((_gid, palette_id)) = self.v0_layer(layer_index) {
                    palette_indices.insert(palette_id);
                }
            }
        }
    }

    /// Collect the transitive closure of v1 glyphs,layer/paletted indices and variation/delta set indices for COLRv1
    pub fn v1_closure(
        &self,
        glyph_set: &mut IntSet<GlyphId>,
        layer_indices: &mut IntSet<u32>,
        palette_indices: &mut IntSet<u16>,
        variation_indices: &mut IntSet<u32>,
    ) {
        if self.version() < 1 {
            return;
        }

        let mut c =
            Colrv1ClosureContext::new(layer_indices, palette_indices, variation_indices, self);
        if let Some(Ok(base_glyph_list)) = self.base_glyph_list() {
            let base_glyph_records = base_glyph_list.base_glyph_paint_records();
            let offset_data = base_glyph_list.offset_data();
            for paint_record in base_glyph_records {
                let gid = paint_record.glyph_id();
                if !glyph_set.contains(GlyphId::from(gid)) {
                    continue;
                }
                if let Ok(paint) = paint_record.paint(offset_data) {
                    c.dispatch(&paint);
                }
            }

            glyph_set.union(&c.glyph_set);
        }

        if let Some(Ok(clip_list)) = self.clip_list() {
            c.glyph_set.union(glyph_set);
            for clip_record in clip_list.clips() {
                clip_record.v1_closure(&mut c, &clip_list);
            }
        }
    }

    /// Collect the transitive closure of V0 glyphs needed for all of the input glyphs set
    pub fn v0_closure_glyphs(
        &self,
        glyph_set: &IntSet<GlyphId>,
        glyphset_colrv0: &mut IntSet<GlyphId>,
    ) {
        glyphset_colrv0.union(glyph_set);
        let Some(Ok(records)) = self.base_glyph_records() else {
            return;
        };
        for glyph_id in glyph_set.iter() {
            let Ok(glyph_id) = glyph_id.try_into() else {
                continue;
            };
            let record = match records.binary_search_by(|rec| rec.glyph_id().cmp(&glyph_id)) {
                Ok(idx) => records[idx],
                _ => continue,
            };
            let start = record.first_layer_index() as usize;
            let end = start + record.num_layers() as usize;
            for layer_index in start..end {
                if let Ok((gid, _palette_id)) = self.v0_layer(layer_index) {
                    glyphset_colrv0.insert(GlyphId::from(gid));
                }
            }
        }
    }
}

struct Colrv1ClosureContext<'a> {
    glyph_set: IntSet<GlyphId>,
    layer_indices: &'a mut IntSet<u32>,
    palette_indices: &'a mut IntSet<u16>,
    variation_indices: &'a mut IntSet<u32>,
    colr: &'a Colr<'a>,
    nesting_level_left: u8,
    visited_paints: IntSet<u32>,
    colr_head: usize,
}

impl<'a> Colrv1ClosureContext<'a> {
    pub fn new(
        layer_indices: &'a mut IntSet<u32>,
        palette_indices: &'a mut IntSet<u16>,
        variation_indices: &'a mut IntSet<u32>,
        colr: &'a Colr,
    ) -> Self {
        let colr_head = colr.offset_data().as_bytes().as_ptr() as usize;
        Self {
            glyph_set: IntSet::empty(),
            layer_indices,
            palette_indices,
            variation_indices,
            colr,
            nesting_level_left: 64,
            visited_paints: IntSet::empty(),
            colr_head,
        }
    }

    fn dispatch(&mut self, paint: &Paint) {
        if self.nesting_level_left == 0 {
            return;
        }

        if self.paint_visited(paint) {
            return;
        }
        self.nesting_level_left -= 1;
        paint.v1_closure(self);
        self.nesting_level_left += 1;
    }

    fn paint_visited(&mut self, paint: &Paint) -> bool {
        let offset = (paint.offset_data().as_bytes().as_ptr() as usize - self.colr_head) as u32;
        if self.visited_paints.contains(offset) {
            return true;
        }

        self.visited_paints.insert(offset);
        false
    }

    fn add_layer_indices(&mut self, first_layer_index: u32, last_layer_index: u32) {
        self.layer_indices
            .insert_range(first_layer_index..=last_layer_index);
    }

    fn add_palette_index(&mut self, palette_index: u16) {
        self.palette_indices.insert(palette_index);
    }

    fn add_variation_indices(&mut self, var_index_base: u32, num_vars: u8) {
        if num_vars == 0 || var_index_base == NO_VARIATION_INDEX {
            return;
        }

        let last_var_index = var_index_base + num_vars as u32 - 1;
        self.variation_indices
            .insert_range(var_index_base..=last_var_index);
    }

    fn add_glyph_id(&mut self, gid: GlyphId16) {
        self.glyph_set.insert(GlyphId::from(gid));
    }
}

impl ColorStop {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        c.add_palette_index(self.palette_index());
    }
}

impl VarColorStop {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        c.add_palette_index(self.palette_index());
        c.add_variation_indices(self.var_index_base(), 2);
    }
}

impl ColorLine<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        for colorstop in self.color_stops() {
            colorstop.v1_closure(c);
        }
    }
}

impl VarColorLine<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        for var_colorstop in self.color_stops() {
            var_colorstop.v1_closure(c);
        }
    }
}

impl Paint<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        match self {
            Self::ColrLayers(item) => item.v1_closure(c),
            Self::Solid(item) => item.v1_closure(c),
            Self::VarSolid(item) => item.v1_closure(c),
            Self::LinearGradient(item) => item.v1_closure(c),
            Self::VarLinearGradient(item) => item.v1_closure(c),
            Self::RadialGradient(item) => item.v1_closure(c),
            Self::VarRadialGradient(item) => item.v1_closure(c),
            Self::SweepGradient(item) => item.v1_closure(c),
            Self::VarSweepGradient(item) => item.v1_closure(c),
            Self::Glyph(item) => item.v1_closure(c),
            Self::ColrGlyph(item) => item.v1_closure(c),
            Self::Transform(item) => item.v1_closure(c),
            Self::VarTransform(item) => item.v1_closure(c),
            Self::Translate(item) => item.v1_closure(c),
            Self::VarTranslate(item) => item.v1_closure(c),
            Self::Scale(item) => item.v1_closure(c),
            Self::VarScale(item) => item.v1_closure(c),
            Self::ScaleAroundCenter(item) => item.v1_closure(c),
            Self::VarScaleAroundCenter(item) => item.v1_closure(c),
            Self::ScaleUniform(item) => item.v1_closure(c),
            Self::VarScaleUniform(item) => item.v1_closure(c),
            Self::ScaleUniformAroundCenter(item) => item.v1_closure(c),
            Self::VarScaleUniformAroundCenter(item) => item.v1_closure(c),
            Self::Rotate(item) => item.v1_closure(c),
            Self::VarRotate(item) => item.v1_closure(c),
            Self::RotateAroundCenter(item) => item.v1_closure(c),
            Self::VarRotateAroundCenter(item) => item.v1_closure(c),
            Self::Skew(item) => item.v1_closure(c),
            Self::VarSkew(item) => item.v1_closure(c),
            Self::SkewAroundCenter(item) => item.v1_closure(c),
            Self::VarSkewAroundCenter(item) => item.v1_closure(c),
            Self::Composite(item) => item.v1_closure(c),
        }
    }
}

impl PaintColrLayers<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        let num_layers = self.num_layers();
        if num_layers == 0 {
            return;
        }

        let Some(Ok(layer_list)) = c.colr.layer_list() else {
            return;
        };
        let first_layer_index = self.first_layer_index();
        let last_layer_index = first_layer_index + num_layers as u32 - 1;
        c.add_layer_indices(first_layer_index, last_layer_index);

        let offset_data = layer_list.offset_data();
        for layer_index in first_layer_index..=last_layer_index {
            if let Some(paint_offset) = layer_list.paint_offsets().get(layer_index as usize) {
                if let Ok(paint) = paint_offset.get().resolve::<Paint>(offset_data) {
                    c.dispatch(&paint);
                }
            }
        }
    }
}

impl PaintSolid<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        c.add_palette_index(self.palette_index());
    }
}

impl PaintVarSolid<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        c.add_palette_index(self.palette_index());
        c.add_variation_indices(self.var_index_base(), 1);
    }
}

impl PaintLinearGradient<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(colorline) = self.color_line() {
            colorline.v1_closure(c);
        }
    }
}

impl PaintVarLinearGradient<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(var_colorline) = self.color_line() {
            var_colorline.v1_closure(c);
        }
        c.add_variation_indices(self.var_index_base(), 6);
    }
}

impl PaintRadialGradient<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(colorline) = self.color_line() {
            colorline.v1_closure(c);
        }
    }
}

impl PaintVarRadialGradient<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(var_colorline) = self.color_line() {
            var_colorline.v1_closure(c);
        }
        c.add_variation_indices(self.var_index_base(), 6);
    }
}

impl PaintSweepGradient<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(colorline) = self.color_line() {
            colorline.v1_closure(c);
        }
    }
}

impl PaintVarSweepGradient<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(var_colorline) = self.color_line() {
            var_colorline.v1_closure(c);
        }
        c.add_variation_indices(self.var_index_base(), 4);
    }
}

impl PaintGlyph<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        c.add_glyph_id(self.glyph_id());
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
        }
    }
}

impl PaintColrGlyph<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        let glyph_id = self.glyph_id();
        let Some(Ok(list)) = c.colr.base_glyph_list() else {
            return;
        };
        let records = list.base_glyph_paint_records();
        let record = match records.binary_search_by(|rec| rec.glyph_id().cmp(&glyph_id)) {
            Ok(ix) => &records[ix],
            _ => return,
        };
        if let Ok(paint) = record.paint(list.offset_data()) {
            c.add_glyph_id(glyph_id);
            c.dispatch(&paint);
        }
    }
}

impl PaintTransform<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
        }
    }
}

impl VarAffine2x3<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        c.add_variation_indices(self.var_index_base(), 6);
    }
}

impl PaintVarTransform<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
            if let Ok(affine2x3) = self.transform() {
                affine2x3.v1_closure(c);
            }
        }
    }
}

impl PaintTranslate<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
        }
    }
}

impl PaintVarTranslate<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
            c.add_variation_indices(self.var_index_base(), 2);
        }
    }
}

impl PaintScale<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
        }
    }
}

impl PaintVarScale<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
            c.add_variation_indices(self.var_index_base(), 2);
        }
    }
}

impl PaintScaleAroundCenter<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
        }
    }
}

impl PaintVarScaleAroundCenter<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
            c.add_variation_indices(self.var_index_base(), 4);
        }
    }
}

impl PaintScaleUniform<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
        }
    }
}

impl PaintVarScaleUniform<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
            c.add_variation_indices(self.var_index_base(), 1);
        }
    }
}

impl PaintScaleUniformAroundCenter<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
        }
    }
}

impl PaintVarScaleUniformAroundCenter<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
            c.add_variation_indices(self.var_index_base(), 3);
        }
    }
}

impl PaintRotate<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
        }
    }
}

impl PaintVarRotate<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
            c.add_variation_indices(self.var_index_base(), 1);
        }
    }
}

impl PaintRotateAroundCenter<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
        }
    }
}

impl PaintVarRotateAroundCenter<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
            c.add_variation_indices(self.var_index_base(), 3);
        }
    }
}

impl PaintSkew<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
        }
    }
}

impl PaintVarSkew<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
            c.add_variation_indices(self.var_index_base(), 2);
        }
    }
}

impl PaintSkewAroundCenter<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
        }
    }
}

impl PaintVarSkewAroundCenter<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(paint) = self.paint() {
            c.dispatch(&paint);
            c.add_variation_indices(self.var_index_base(), 4);
        }
    }
}

impl PaintComposite<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Ok(source_paint) = self.source_paint() {
            c.dispatch(&source_paint);
        }

        if let Ok(backdrop_paint) = self.backdrop_paint() {
            c.dispatch(&backdrop_paint);
        }
    }
}

impl Clip {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext, clip_list: &ClipList) {
        let Ok(clip_box) = self.clip_box(clip_list.offset_data()) else {
            return;
        };
        //TODO: replace below code when we have intersects(Range) available for int-set
        let mut included_gids = IntSet::empty();
        let start_id = GlyphId::from(self.start_glyph_id());
        let end_id = GlyphId::from(self.end_glyph_id());
        included_gids.insert_range(start_id..=end_id);
        included_gids.intersect(&c.glyph_set);

        if included_gids.is_empty() {
            return;
        }
        clip_box.v1_closure(c);
    }
}

impl ClipBox<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        if let Self::Format2(item) = self {
            item.v1_closure(c)
        }
    }
}

impl ClipBoxFormat2<'_> {
    fn v1_closure(&self, c: &mut Colrv1ClosureContext) {
        c.add_variation_indices(self.var_index_base(), 4);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FontRef, GlyphId, TableProvider};

    #[test]
    fn test_colr_v0_closure() {
        let font = FontRef::new(font_test_data::COLRV0V1_VARIABLE).unwrap();
        let colr = font.colr().unwrap();

        let mut input_glyph_set = IntSet::empty();
        input_glyph_set.insert(GlyphId::new(168));

        let mut glyph_set_colred = IntSet::empty();

        colr.v0_closure_glyphs(&input_glyph_set, &mut glyph_set_colred);
        assert_eq!(glyph_set_colred.len(), 9);
        assert!(glyph_set_colred.contains(GlyphId::new(5)));
        assert!(glyph_set_colred.contains(GlyphId::new(168)));
        assert!(glyph_set_colred.contains(GlyphId::new(170)));
        assert!(glyph_set_colred.contains(GlyphId::new(171)));
        assert!(glyph_set_colred.contains(GlyphId::new(172)));
        assert!(glyph_set_colred.contains(GlyphId::new(173)));
        assert!(glyph_set_colred.contains(GlyphId::new(174)));
        assert!(glyph_set_colred.contains(GlyphId::new(175)));
        assert!(glyph_set_colred.contains(GlyphId::new(176)));

        let mut palette_indices = IntSet::empty();
        colr.v0_closure_palette_indices(&glyph_set_colred, &mut palette_indices);
        assert_eq!(palette_indices.len(), 8);
        assert!(palette_indices.contains(0));
        assert!(palette_indices.contains(1));
        assert!(palette_indices.contains(2));
        assert!(palette_indices.contains(3));
        assert!(palette_indices.contains(4));
        assert!(palette_indices.contains(5));
        assert!(palette_indices.contains(6));
        assert!(palette_indices.contains(10));
    }

    #[test]
    fn test_colr_v0_closure_not_found() {
        let font = FontRef::new(font_test_data::COLRV0V1_VARIABLE).unwrap();
        let colr = font.colr().unwrap();

        let mut input_glyph_set = IntSet::empty();
        input_glyph_set.insert(GlyphId::new(8));

        let mut glyph_set_colred = IntSet::empty();

        colr.v0_closure_glyphs(&input_glyph_set, &mut glyph_set_colred);
        assert_eq!(glyph_set_colred.len(), 1);
        assert!(glyph_set_colred.contains(GlyphId::new(8)));
    }

    #[test]
    fn test_colr_v1_closure_no_var() {
        let font = FontRef::new(font_test_data::COLRV0V1_VARIABLE).unwrap();
        let colr = font.colr().unwrap();

        let mut glyph_set = IntSet::empty();
        glyph_set.insert(GlyphId::new(220));
        glyph_set.insert(GlyphId::new(120));

        let mut layer_indices = IntSet::empty();
        let mut palette_indices = IntSet::empty();
        let mut variation_indices = IntSet::empty();

        colr.v1_closure(
            &mut glyph_set,
            &mut layer_indices,
            &mut palette_indices,
            &mut variation_indices,
        );

        assert_eq!(glyph_set.len(), 6);
        assert!(glyph_set.contains(GlyphId::new(6)));
        assert!(glyph_set.contains(GlyphId::new(7)));
        assert!(glyph_set.contains(GlyphId::new(220)));
        assert!(glyph_set.contains(GlyphId::new(3)));
        assert!(glyph_set.contains(GlyphId::new(2)));
        assert!(glyph_set.contains(GlyphId::new(120)));

        assert_eq!(palette_indices.len(), 5);
        assert!(palette_indices.contains(0));
        assert!(palette_indices.contains(4));
        assert!(palette_indices.contains(10));
        assert!(palette_indices.contains(11));
        assert!(palette_indices.contains(12));

        assert_eq!(layer_indices.len(), 2);
        assert!(layer_indices.contains(0));
        assert!(layer_indices.contains(1));

        assert!(variation_indices.is_empty());
    }

    #[test]
    fn test_colr_v1_closure_w_var() {
        let font = FontRef::new(font_test_data::COLRV0V1_VARIABLE).unwrap();
        let colr = font.colr().unwrap();

        let mut glyph_set = IntSet::empty();
        glyph_set.insert(GlyphId::new(109));

        let mut layer_indices = IntSet::empty();
        let mut palette_indices = IntSet::empty();
        let mut variation_indices = IntSet::empty();

        colr.v1_closure(
            &mut glyph_set,
            &mut layer_indices,
            &mut palette_indices,
            &mut variation_indices,
        );

        assert_eq!(glyph_set.len(), 2);
        assert!(glyph_set.contains(GlyphId::new(3)));
        assert!(glyph_set.contains(GlyphId::new(109)));

        assert_eq!(palette_indices.len(), 2);
        assert!(palette_indices.contains(1));
        assert!(palette_indices.contains(4));

        assert!(layer_indices.is_empty());

        assert_eq!(variation_indices.len(), 6);
        assert!(variation_indices.contains(51));
        assert!(variation_indices.contains(52));
        assert!(variation_indices.contains(53));
        assert!(variation_indices.contains(54));
        assert!(variation_indices.contains(55));
        assert!(variation_indices.contains(56));
    }
}
