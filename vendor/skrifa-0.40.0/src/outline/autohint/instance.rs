//! Autohinting state for a font instance.

use crate::{attribute::Style, prelude::Size, MetadataProvider};

use super::{
    super::{
        pen::PathStyle, AdjustedMetrics, DrawError, OutlineGlyph, OutlineGlyphCollection,
        OutlinePen, Target,
    },
    metrics::{fixed_mul, pix_round, Scale, UnscaledStyleMetricsSet},
    outline::Outline,
    shape::{Shaper, ShaperMode},
    style::GlyphStyleMap,
};
use alloc::sync::Arc;
use raw::{
    types::{F26Dot6, F2Dot14},
    FontRef, TableProvider,
};

/// We enable "best effort" mode by default but allow it to be disabled with
/// a feature for testing.
const SHAPER_MODE: ShaperMode = if cfg!(feature = "autohint_shaping") {
    ShaperMode::BestEffort
} else {
    ShaperMode::Nominal
};

/// Set of derived glyph styles that are used for automatic hinting.
///
/// These are invariant per font so can be precomputed and reused for multiple
/// instances when requesting automatic hinting with [`Engine::Auto`](super::super::hint::Engine::Auto).
#[derive(Clone, Debug)]
pub struct GlyphStyles(Arc<GlyphStyleMap>);

impl GlyphStyles {
    /// Precomputes the full set of glyph styles for the given outlines.
    pub fn new(outlines: &OutlineGlyphCollection) -> Self {
        if let Some(font) = outlines.font() {
            let glyph_count = font
                .maxp()
                .map(|maxp| maxp.num_glyphs() as u32)
                .unwrap_or_default();
            let shaper = Shaper::new(font, SHAPER_MODE);
            Self(Arc::new(GlyphStyleMap::new(glyph_count, &shaper)))
        } else {
            Self(Default::default())
        }
    }
}

#[derive(Clone)]
pub(crate) struct Instance {
    styles: GlyphStyles,
    metrics: UnscaledStyleMetricsSet,
    target: Target,
    is_fixed_width: bool,
    style: Style,
}

impl Instance {
    pub fn new(
        font: &FontRef,
        outlines: &OutlineGlyphCollection,
        coords: &[F2Dot14],
        target: Target,
        styles: Option<GlyphStyles>,
        lazy_metrics: bool,
    ) -> Self {
        let styles = styles.unwrap_or_else(|| GlyphStyles::new(outlines));
        #[cfg(feature = "std")]
        let metrics = if lazy_metrics {
            UnscaledStyleMetricsSet::lazy(&styles.0)
        } else {
            UnscaledStyleMetricsSet::precomputed(font, coords, SHAPER_MODE, &styles.0)
        };
        #[cfg(not(feature = "std"))]
        let metrics = UnscaledStyleMetricsSet::precomputed(font, coords, SHAPER_MODE, &styles.0);
        let is_fixed_width = font
            .post()
            .map(|post| post.is_fixed_pitch() != 0)
            .unwrap_or_default();
        let style = font.attributes().style;
        Self {
            styles,
            metrics,
            target,
            is_fixed_width,
            style,
        }
    }

    pub fn draw(
        &self,
        size: Size,
        coords: &[F2Dot14],
        glyph: &OutlineGlyph,
        path_style: PathStyle,
        pen: &mut impl OutlinePen,
    ) -> Result<AdjustedMetrics, DrawError> {
        let font = glyph.font();
        let glyph_id = glyph.glyph_id();
        let style = self
            .styles
            .0
            .style(glyph_id)
            .ok_or(DrawError::GlyphNotFound(glyph_id))?;
        let metrics = self
            .metrics
            .get(font, coords, SHAPER_MODE, &self.styles.0, glyph_id)
            .ok_or(DrawError::GlyphNotFound(glyph_id))?;
        let units_per_em = glyph.units_per_em() as i32;
        let scale = Scale::new(
            size.ppem().unwrap_or(units_per_em as f32),
            units_per_em,
            self.style,
            self.target,
            metrics.style_class().script.group,
        );
        let mut outline = Outline::default();
        outline.fill(glyph, coords)?;
        let hinted_metrics = super::hint::hint_outline(&mut outline, &metrics, &scale, Some(style));
        let h_advance = outline.advance;
        let mut pp1x = 0;
        let mut pp2x = fixed_mul(h_advance, hinted_metrics.x_scale);
        let is_light = self.target.is_light() || self.target.preserve_linear_metrics();
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afloader.c#L422>
        if !is_light {
            if let (true, Some(edge_metrics)) = (
                scale.flags & Scale::NO_ADVANCE == 0,
                hinted_metrics.edge_metrics,
            ) {
                let old_rsb = pp2x - edge_metrics.right_opos;
                let old_lsb = edge_metrics.left_opos;
                let new_lsb = edge_metrics.left_pos;
                let mut pp1x_uh = new_lsb - old_lsb;
                let mut pp2x_uh = edge_metrics.right_pos + old_rsb;
                if old_lsb < 24 {
                    pp1x_uh -= 8;
                }
                if old_rsb < 24 {
                    pp2x_uh += 8;
                }
                pp1x = pix_round(pp1x_uh);
                pp2x = pix_round(pp2x_uh);
                if pp1x >= new_lsb && old_lsb > 0 {
                    pp1x -= 64;
                }
                if pp2x <= edge_metrics.right_pos && old_rsb > 0 {
                    pp2x += 64;
                }
            } else {
                pp1x = pix_round(pp1x);
                pp2x = pix_round(pp2x);
            }
        } else {
            pp1x = pix_round(pp1x);
            pp2x = pix_round(pp2x);
        }
        if pp1x != 0 {
            for point in &mut outline.points {
                point.x -= pp1x;
            }
        }
        let advance = if !is_light
            && (self.is_fixed_width || (metrics.digits_have_same_width && style.is_digit()))
        {
            fixed_mul(h_advance, scale.x_scale)
        } else if h_advance != 0 {
            pp2x - pp1x
        } else {
            0
        };
        outline.to_path(path_style, pen)?;
        Ok(AdjustedMetrics {
            has_overlaps: glyph.has_overlaps().unwrap_or_default(),
            lsb: None,
            advance_width: Some(F26Dot6::from_bits(pix_round(advance)).to_f32()),
        })
    }
}
