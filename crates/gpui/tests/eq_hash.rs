use std::hash::{BuildHasher, RandomState};

use gpui::{FontId, GlyphId, RenderGlyphParams, point, px};

#[test]
fn pixels_hash_agrees_with_equality_on_signed_zero() {
    let positive = px(0.0);
    let negative = -px(0.0);
    assert_eq!(positive, negative);
    let hasher = RandomState::new();
    assert_eq!(hasher.hash_one(positive), hasher.hash_one(negative));
}

#[test]
fn render_glyph_params_hash_agrees_with_equality_on_signed_zero() {
    let params = RenderGlyphParams {
        font_id: FontId(0),
        glyph_id: GlyphId(0),
        font_size: px(0.0),
        subpixel_variant: point(0u8, 0u8),
        scale_factor: 0.0,
        is_emoji: false,
        subpixel_rendering: false,
        dilation: 0,
    };
    let hasher = RandomState::new();
    for (font_size, scale_factor) in [(-0.0, 0.0), (0.0, -0.0), (-0.0, -0.0)] {
        let negated = RenderGlyphParams {
            font_size: px(font_size),
            scale_factor,
            ..params.clone()
        };
        assert_eq!(params, negated);
        assert_eq!(hasher.hash_one(&params), hasher.hash_one(&negated));
    }
}
