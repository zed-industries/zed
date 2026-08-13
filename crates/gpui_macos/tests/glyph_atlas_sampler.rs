use gpui::{SUBPIXEL_VARIANTS_X, SUBPIXEL_VARIANTS_Y};

const SHADER_SOURCE: &str = include_str!("../src/shaders.metal");

#[derive(Debug, PartialEq)]
struct QuantizedGlyphOrigin {
    integer_origin_x: f32,
    integer_origin_y: f32,
    subpixel_variant_x: f32,
    subpixel_variant_y: f32,
}

fn shader_section(start_marker: &str, end_marker: &str) -> &'static str {
    let (_, after_start) = SHADER_SOURCE
        .split_once(start_marker)
        .expect("shader source should contain the requested fragment start marker");
    let (section, _) = after_start
        .split_once(end_marker)
        .expect("shader source should contain the requested fragment end marker");
    section
}

fn round_half_toward_zero(value: f32) -> f32 {
    (value.abs() - 0.5).ceil().copysign(value)
}

fn quantize_glyph_origin(device_x: f32, device_y: f32) -> QuantizedGlyphOrigin {
    let variants_x = f32::from(SUBPIXEL_VARIANTS_X);
    let variants_y = f32::from(SUBPIXEL_VARIANTS_Y);
    let quantized_x = round_half_toward_zero(device_x * variants_x) / variants_x;
    let quantized_y = round_half_toward_zero(device_y * variants_y) / variants_y;

    QuantizedGlyphOrigin {
        integer_origin_x: quantized_x.trunc(),
        integer_origin_y: quantized_y.trunc(),
        subpixel_variant_x: quantized_x.fract() * variants_x,
        subpixel_variant_y: quantized_y.fract() * variants_y,
    }
}

#[test]
fn monochrome_sprite_fragment_uses_nearest_filtering() {
    let monochrome_fragment = shader_section(
        "fragment float4 monochrome_sprite_fragment",
        "struct PolychromeSpriteVertexOutput",
    );

    assert!(
        monochrome_fragment.contains("mag_filter::nearest"),
        "monochrome glyph atlas sampler should use nearest mag_filter because subpixel variants already bake fractional glyph origins",
    );
    assert!(
        monochrome_fragment.contains("min_filter::nearest"),
        "monochrome glyph atlas sampler should use nearest min_filter because glyph atlas texels are already rasterized at the selected subpixel variant",
    );
    assert!(
        !monochrome_fragment.contains("mag_filter::linear"),
        "monochrome glyph atlas sampler should not use linear mag_filter; bilinear filtering adds redundant blur",
    );
}

#[test]
fn fractional_device_pixel_origins_select_expected_subpixel_variants() {
    for (device_x, expected_variant_x, expected_integer_x) in [
        (10.0, 0.0, 10.0),
        (10.25, 1.0, 10.0),
        (10.5, 2.0, 10.0),
        (10.75, 3.0, 10.0),
        (11.0, 0.0, 11.0),
    ] {
        let quantized = quantize_glyph_origin(device_x, 20.0);

        assert_eq!(
            quantized.subpixel_variant_x, expected_variant_x,
            "fractional device-pixel origin {device_x} should map to the pre-rasterized x subpixel variant {expected_variant_x}",
        );
        assert_eq!(
            quantized.integer_origin_x, expected_integer_x,
            "fractional device-pixel origin {device_x} should still place the sprite at an integer texel-aligned x origin",
        );
        assert_eq!(
            quantized.subpixel_variant_y, 0.0,
            "monochrome glyph y subpixel variant should remain zero because SUBPIXEL_VARIANTS_Y is one",
        );
        assert_eq!(
            quantized.integer_origin_y, 20.0,
            "integer y origin should remain texel-aligned when no vertical subpixel variants are enabled",
        );
    }
}

#[test]
fn polychrome_sprite_fragment_keeps_linear_filtering() {
    let polychrome_fragment = shader_section(
        "fragment float4 polychrome_sprite_fragment",
        "struct PathRasterizationVertexOutput",
    );

    assert!(
        polychrome_fragment.contains("mag_filter::linear"),
        "polychrome sprite sampler should keep linear mag_filter for color emoji/image atlas sampling",
    );
    assert!(
        polychrome_fragment.contains("min_filter::linear"),
        "polychrome sprite sampler should keep linear min_filter for color emoji/image atlas sampling",
    );
    assert!(
        !polychrome_fragment.contains("mag_filter::nearest"),
        "polychrome sprite sampler should not be changed when fixing monochrome glyph atlas filtering",
    );
}
