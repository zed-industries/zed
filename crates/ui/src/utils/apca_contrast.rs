use gpui::Hsla;

/// APCA (Accessible Perceptual Contrast Algorithm) constants
/// Based on APCA 0.0.98G-4g W3 compatible constants
/// https://github.com/Myndex/apca-w3
struct APCAConstants {
    // Main TRC exponent for monitor perception
    main_trc: f32,

    // sRGB coefficients
    s_rco: f32,
    s_gco: f32,
    s_bco: f32,

    // G-4g constants for use with 2.4 exponent
    norm_bg: f32,
    norm_txt: f32,
    rev_txt: f32,
    rev_bg: f32,

    // G-4g Clamps and Scalers
    blk_thrs: f32,
    blk_clmp: f32,
    scale_bow: f32,
    scale_wob: f32,
    lo_bow_offset: f32,
    lo_wob_offset: f32,
    delta_y_min: f32,
    lo_clip: f32,
}

impl Default for APCAConstants {
    fn default() -> Self {
        Self {
            main_trc: 2.4,
            s_rco: 0.2126729,
            s_gco: 0.7151522,
            s_bco: 0.0721750,
            norm_bg: 0.56,
            norm_txt: 0.57,
            rev_txt: 0.62,
            rev_bg: 0.65,
            blk_thrs: 0.022,
            blk_clmp: 1.414,
            scale_bow: 1.14,
            scale_wob: 1.14,
            lo_bow_offset: 0.027,
            lo_wob_offset: 0.027,
            delta_y_min: 0.0005,
            lo_clip: 0.1,
        }
    }
}

/// Calculates the perceptual lightness contrast using APCA.
/// Returns a value between approximately -108 and 106.
/// Negative values indicate light text on dark background.
/// Positive values indicate dark text on light background.
///
/// The APCA algorithm is more perceptually accurate than WCAG 2.x,
/// especially for dark mode interfaces. Key improvements include:
/// - Better accuracy for dark backgrounds
/// - Polarity-aware (direction matters)
/// - Perceptually uniform across the range
///
/// Common APCA Lc thresholds per ARC Bronze Simple Mode:
/// https://readtech.org/ARC/tests/bronze-simple-mode/
/// - Lc 45: Minimum for large fluent text (36px+)
/// - Lc 60: Minimum for other content text
/// - Lc 75: Minimum for body text
/// - Lc 90: Preferred for body text
///
/// Most terminal themes use colors with APCA values of 40-70.
///
/// https://github.com/Myndex/apca-w3
pub fn apca_contrast(text_color: Hsla, background_color: Hsla) -> f32 {
    let constants = APCAConstants::default();

    let text_y = srgb_to_y(text_color, &constants);
    let bg_y = srgb_to_y(background_color, &constants);

    // Apply soft clamp to near-black colors
    let text_y_clamped = if text_y > constants.blk_thrs {
        text_y
    } else {
        text_y + (constants.blk_thrs - text_y).powf(constants.blk_clmp)
    };

    let bg_y_clamped = if bg_y > constants.blk_thrs {
        bg_y
    } else {
        bg_y + (constants.blk_thrs - bg_y).powf(constants.blk_clmp)
    };

    // Return 0 for extremely low delta Y
    if (bg_y_clamped - text_y_clamped).abs() < constants.delta_y_min {
        return 0.0;
    }

    let sapc;
    let output_contrast;

    if bg_y_clamped > text_y_clamped {
        // Normal polarity: dark text on light background
        sapc = (bg_y_clamped.powf(constants.norm_bg) - text_y_clamped.powf(constants.norm_txt))
            * constants.scale_bow;

        // Low contrast smooth rollout to prevent polarity reversal
        output_contrast = if sapc < constants.lo_clip {
            0.0
        } else {
            sapc - constants.lo_bow_offset
        };
    } else {
        // Reverse polarity: light text on dark background
        sapc = (bg_y_clamped.powf(constants.rev_bg) - text_y_clamped.powf(constants.rev_txt))
            * constants.scale_wob;

        output_contrast = if sapc > -constants.lo_clip {
            0.0
        } else {
            sapc + constants.lo_wob_offset
        };
    }

    // Return Lc (lightness contrast) scaled to percentage
    output_contrast * 100.0
}

/// Converts sRGB color to Y (luminance) for APCA calculation
fn srgb_to_y(color: Hsla, constants: &APCAConstants) -> f32 {
    let rgba = color.to_rgb();

    // Linearize and apply coefficients
    let r_linear = (rgba.r).powf(constants.main_trc);
    let g_linear = (rgba.g).powf(constants.main_trc);
    let b_linear = (rgba.b).powf(constants.main_trc);

    constants.s_rco * r_linear + constants.s_gco * g_linear + constants.s_bco * b_linear
}

/// Adjusts the foreground color to meet the minimum APCA contrast against the background.
/// The minimum_apca_contrast should be an absolute value (e.g., 75 for Lc 75).
///
/// This implementation gradually adjusts the lightness while preserving the hue and
/// saturation as much as possible, only falling back to black/white when necessary.
pub fn ensure_minimum_contrast(
    foreground: Hsla,
    background: Hsla,
    minimum_apca_contrast: f32,
) -> Hsla {
    if minimum_apca_contrast <= 0.0 {
        return foreground;
    }

    let current_contrast = apca_contrast(foreground, background).abs();

    if current_contrast >= minimum_apca_contrast {
        return foreground;
    }

    // First, try to adjust lightness while preserving hue and saturation
    let adjusted = adjust_lightness_for_contrast(foreground, background, minimum_apca_contrast);

    let adjusted_contrast = apca_contrast(adjusted, background).abs();
    if adjusted_contrast >= minimum_apca_contrast {
        return adjusted;
    }

    // If that's not enough, gradually reduce saturation while adjusting lightness
    let desaturated =
        adjust_lightness_and_saturation_for_contrast(foreground, background, minimum_apca_contrast);

    let desaturated_contrast = apca_contrast(desaturated, background).abs();
    if desaturated_contrast >= minimum_apca_contrast {
        return desaturated;
    }

    // Last resort: use black or white
    let black = Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: foreground.a,
    };

    let white = Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: foreground.a,
    };

    let black_contrast = apca_contrast(black, background).abs();
    let white_contrast = apca_contrast(white, background).abs();

    if white_contrast > black_contrast {
        white
    } else {
        black
    }
}

/// Adjusts only the lightness to meet the minimum contrast, preserving hue and saturation
fn adjust_lightness_for_contrast(
    foreground: Hsla,
    background: Hsla,
    minimum_apca_contrast: f32,
) -> Hsla {
    // Determine if we need to go lighter or darker
    let bg_luminance = srgb_to_y(background, &APCAConstants::default());
    let should_go_darker = bg_luminance > 0.5;

    // Binary search for the optimal lightness
    let mut low = if should_go_darker { 0.0 } else { foreground.l };
    let mut high = if should_go_darker { foreground.l } else { 1.0 };
    let mut best_l = foreground.l;

    for _ in 0..20 {
        let mid = (low + high) / 2.0;
        let test_color = Hsla {
            h: foreground.h,
            s: foreground.s,
            l: mid,
            a: foreground.a,
        };

        let contrast = apca_contrast(test_color, background).abs();

        if contrast >= minimum_apca_contrast {
            best_l = mid;
            // Try to get closer to the minimum
            if should_go_darker {
                low = mid;
            } else {
                high = mid;
            }
        } else if should_go_darker {
            high = mid;
        } else {
            low = mid;
        }

        // If we're close enough to the target, stop
        if (contrast - minimum_apca_contrast).abs() < 1.0 {
            best_l = mid;
            break;
        }
    }

    Hsla {
        h: foreground.h,
        s: foreground.s,
        l: best_l,
        a: foreground.a,
    }
}

/// Adjusts both lightness and saturation to meet the minimum contrast
fn adjust_lightness_and_saturation_for_contrast(
    foreground: Hsla,
    background: Hsla,
    minimum_apca_contrast: f32,
) -> Hsla {
    // Try different saturation levels
    let saturation_steps = [1.0, 0.8, 0.6, 0.4, 0.2, 0.0];

    for &sat_multiplier in &saturation_steps {
        let test_color = Hsla {
            h: foreground.h,
            s: foreground.s * sat_multiplier,
            l: foreground.l,
            a: foreground.a,
        };

        let adjusted = adjust_lightness_for_contrast(test_color, background, minimum_apca_contrast);
        let contrast = apca_contrast(adjusted, background).abs();

        if contrast >= minimum_apca_contrast {
            return adjusted;
        }
    }

    // If we get here, even grayscale didn't work, so return the grayscale attempt
    Hsla {
        h: foreground.h,
        s: 0.0,
        l: foreground.l,
        a: foreground.a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
        Hsla { h, s, l, a }
    }

    fn hsla_from_hex(hex: u32) -> Hsla {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if max == min {
            // Achromatic
            Hsla {
                h: 0.0,
                s: 0.0,
                l,
                a: 1.0,
            }
        } else {
            let d = max - min;
            let s = if l > 0.5 {
                d / (2.0 - max - min)
            } else {
                d / (max + min)
            };

            let h = if max == r {
                (g - b) / d + if g < b { 6.0 } else { 0.0 }
            } else if max == g {
                (b - r) / d + 2.0
            } else {
                (r - g) / d + 4.0
            } / 6.0;

            Hsla { h, s, l, a: 1.0 }
        }
    }

    #[test]
    fn test_apca_contrast() {
        // Test black text on white background (should be positive)
        let black = hsla(0.0, 0.0, 0.0, 1.0);
        let white = hsla(0.0, 0.0, 1.0, 1.0);
        let contrast = apca_contrast(black, white);
        assert!(
            contrast > 100.0,
            "Black on white should have high positive contrast, got {}",
            contrast
        );

        // Test white text on black background (should be negative)
        let contrast_reversed = apca_contrast(white, black);
        assert!(
            contrast_reversed < -100.0,
            "White on black should have high negative contrast, got {}",
            contrast_reversed
        );

        // Same color should have zero contrast
        let gray = hsla(0.0, 0.0, 0.5, 1.0);
        let contrast_same = apca_contrast(gray, gray);
        assert!(
            contrast_same.abs() < 1.0,
            "Same color should have near-zero contrast, got {}",
            contrast_same
        );

        // APCA is NOT commutative - polarity matters
        assert!(
            (contrast + contrast_reversed).abs() > 1.0,
            "APCA should not be commutative"
        );
    }

    #[test]
    fn test_srgb_to_y() {
        let constants = APCAConstants::default();

        // Test known Y values
        let black = hsla(0.0, 0.0, 0.0, 1.0);
        let y_black = srgb_to_y(black, &constants);
        assert!(
            y_black.abs() < 0.001,
            "Black should have Y near 0, got {}",
            y_black
        );

        let white = hsla(0.0, 0.0, 1.0, 1.0);
        let y_white = srgb_to_y(white, &constants);
        assert!(
            (y_white - 1.0).abs() < 0.001,
            "White should have Y near 1, got {}",
            y_white
        );
    }

    #[test]
    fn test_srgb_to_y_nan_issue() {
        let dark_red = hsla_from_hex(0x5f0000);
        let y_dark_red = srgb_to_y(dark_red, &APCAConstants::default());
        assert!(!y_dark_red.is_nan());
    }

    #[test]
    fn test_ensure_minimum_contrast() {
        let white_bg = hsla(0.0, 0.0, 1.0, 1.0);
        let light_gray = hsla(0.0, 0.0, 0.9, 1.0);

        // Light gray on white has poor contrast
        let initial_contrast = apca_contrast(light_gray, white_bg).abs();
        assert!(
            initial_contrast < 15.0,
            "Initial contrast should be low, got {}",
            initial_contrast
        );

        // Should be adjusted to black for better contrast (using APCA Lc 45 as minimum)
        let adjusted = ensure_minimum_contrast(light_gray, white_bg, 45.0);
        assert_eq!(adjusted.l, 0.0); // Should be black
        assert_eq!(adjusted.a, light_gray.a); // Alpha preserved

        // Test with dark background
        let black_bg = hsla(0.0, 0.0, 0.0, 1.0);
        let dark_gray = hsla(0.0, 0.0, 0.1, 1.0);

        // Dark gray on black has poor contrast
        let initial_contrast = apca_contrast(dark_gray, black_bg).abs();
        assert!(
            initial_contrast < 15.0,
            "Initial contrast should be low, got {}",
            initial_contrast
        );

        // Should be adjusted to white for better contrast
        let adjusted = ensure_minimum_contrast(dark_gray, black_bg, 45.0);
        assert_eq!(adjusted.l, 1.0); // Should be white

        // Test when contrast is already sufficient
        let black = hsla(0.0, 0.0, 0.0, 1.0);
        let adjusted = ensure_minimum_contrast(black, white_bg, 45.0);
        assert_eq!(adjusted, black); // Should remain unchanged
    }

    #[test]
    fn test_one_light_theme_exact_colors() {
        // Test with exact colors from One Light theme
        // terminal.background and terminal.ansi.white are both #fafafaff
        let fafafa = hsla_from_hex(0xfafafa);

        // They should be identical
        let bg = fafafa;
        let fg = fafafa;

        // Contrast should be 0 (no contrast)
        let contrast = apca_contrast(fg, bg);
        assert!(
            contrast.abs() < 1.0,
            "Same color should have near-zero APCA contrast, got {}",
            contrast
        );

        // With minimum APCA contrast of 15 (very low, but detectable), it should adjust
        let adjusted = ensure_minimum_contrast(fg, bg, 15.0);
        // The new algorithm preserves colors, so we just need to check contrast
        let new_contrast = apca_contrast(adjusted, bg).abs();
        assert!(
            new_contrast >= 15.0,
            "Adjusted contrast {} should be >= 15.0",
            new_contrast
        );

        // The adjusted color should have sufficient contrast
        let new_contrast = apca_contrast(adjusted, bg).abs();
        assert!(
            new_contrast >= 15.0,
            "Adjusted APCA contrast {} should be >= 15.0",
            new_contrast
        );
    }
}
