use std::collections::HashMap;
use std::hash::Hash;

use gpui::{FontId, GlyphId, RenderGlyphParams, point, px};

/// `Hash` must agree with `PartialEq`: two equal values have to land in the same
/// bucket, or a map lookup with an equal key misses.
fn assert_equal_values_hash_alike<T: Eq + Hash>(left: T, right: T) {
    let mut map = HashMap::new();
    map.insert(left, ());
    assert!(map.contains_key(&right));
}

#[test]
fn pixels_hash_agrees_with_equality_on_signed_zero() {
    let positive = px(0.0);
    let negative = -px(0.0);
    assert_eq!(positive, negative);
    assert_equal_values_hash_alike(positive, negative);
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
    let mut negated = params.clone();
    negated.font_size = px(-0.0);
    negated.scale_factor = -0.0;
    assert_eq!(params, negated);
    assert_equal_values_hash_alike(params, negated);
}
