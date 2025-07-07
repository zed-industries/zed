use gpui::Hsla;

/// Calculates the contrast ratio between two colors according to WCAG 2.0.
/// The contrast ratio is a value between 1 and 21 where 1 is no contrast
/// and 21 is maximum contrast.
///
/// https://www.w3.org/TR/WCAG20/#contrast-ratiodef
pub fn contrast_ratio(foreground: Hsla, background: Hsla) -> f32 {
    let fg_luminance = luminance(foreground);
    let bg_luminance = luminance(background);

    let lighter = fg_luminance.max(bg_luminance);
    let darker = fg_luminance.min(bg_luminance);

    (lighter + 0.05) / (darker + 0.05)
}

/// Calculates the relative luminance of a color according to WCAG 2.0.
/// Returns a value between 0 and 1 where 0 is black and 1 is white.
///
/// https://www.w3.org/TR/WCAG20/#relativeluminancedef
pub fn luminance(color: Hsla) -> f32 {
    // Convert HSLA to RGB using GPUI's built-in method
    let rgba = color.to_rgb();

    // Calculate luminance using the WCAG formula
    let r_linear = srgb_to_linear(rgba.r);
    let g_linear = srgb_to_linear(rgba.g);
    let b_linear = srgb_to_linear(rgba.b);

    0.2126 * r_linear + 0.7152 * g_linear + 0.0722 * b_linear
}

/// Converts a single sRGB color component to linear RGB.
/// This is part of the WCAG luminance calculation.
fn srgb_to_linear(component: f32) -> f32 {
    if component <= 0.03928 {
        component / 12.92
    } else {
        ((component + 0.055) / 1.055).powf(2.4)
    }
}

/// Adjusts the foreground color to meet the minimum contrast ratio against the background.
/// If the current contrast is below the minimum, it returns either black or white,
/// whichever provides better contrast.
pub fn ensure_minimum_contrast(foreground: Hsla, background: Hsla, minimum_contrast: f32) -> Hsla {
    if minimum_contrast <= 1.0 {
        return foreground;
    }

    let current_contrast = contrast_ratio(foreground, background);

    if current_contrast >= minimum_contrast {
        return foreground;
    }

    // Try black and white to see which provides better contrast
    let black = Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: foreground.a, // Preserve alpha
    };

    let white = Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: foreground.a, // Preserve alpha
    };

    let black_contrast = contrast_ratio(black, background);
    let white_contrast = contrast_ratio(white, background);

    if white_contrast > black_contrast {
        white
    } else {
        black
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
    fn test_contrast_ratio() {
        // Black on white should have maximum contrast (21:1)
        let black = hsla(0.0, 0.0, 0.0, 1.0);
        let white = hsla(0.0, 0.0, 1.0, 1.0);
        let ratio = contrast_ratio(black, white);
        assert!((ratio - 21.0).abs() < 0.1, "Expected ~21, got {}", ratio);

        // Same color should have minimum contrast (1:1)
        let gray = hsla(0.0, 0.0, 0.5, 1.0);
        let ratio = contrast_ratio(gray, gray);
        assert!((ratio - 1.0).abs() < 0.01, "Expected ~1, got {}", ratio);

        // Test commutative property
        assert_eq!(contrast_ratio(black, white), contrast_ratio(white, black));
    }

    #[test]
    fn test_luminance() {
        // Test known luminance values
        let black = hsla(0.0, 0.0, 0.0, 1.0);
        assert!((luminance(black) - 0.0).abs() < 0.001);

        let white = hsla(0.0, 0.0, 1.0, 1.0);
        assert!((luminance(white) - 1.0).abs() < 0.001);

        // Middle gray should have luminance around 0.2159
        // (This is the sRGB middle gray, not perceptual middle gray)
        let gray = hsla(0.0, 0.0, 0.5, 1.0);
        let gray_lum = luminance(gray);
        assert!(
            (gray_lum - 0.2159).abs() < 0.01,
            "Expected ~0.2159, got {}",
            gray_lum
        );
    }

    #[test]
    fn test_ensure_minimum_contrast() {
        let white_bg = hsla(0.0, 0.0, 1.0, 1.0);
        let light_gray = hsla(0.0, 0.0, 0.9, 1.0);

        // Light gray on white has poor contrast
        let initial_contrast = contrast_ratio(light_gray, white_bg);
        assert!(initial_contrast < 2.0);

        // Should be adjusted to black for better contrast
        let adjusted = ensure_minimum_contrast(light_gray, white_bg, 3.0);
        assert_eq!(adjusted.l, 0.0); // Should be black
        assert_eq!(adjusted.a, light_gray.a); // Alpha preserved

        // Test with dark background
        let black_bg = hsla(0.0, 0.0, 0.0, 1.0);
        let dark_gray = hsla(0.0, 0.0, 0.1, 1.0);

        // Dark gray on black has poor contrast
        let initial_contrast = contrast_ratio(dark_gray, black_bg);
        assert!(initial_contrast < 2.0);

        // Should be adjusted to white for better contrast
        let adjusted = ensure_minimum_contrast(dark_gray, black_bg, 3.0);
        assert_eq!(adjusted.l, 1.0); // Should be white

        // Test when contrast is already sufficient
        let black = hsla(0.0, 0.0, 0.0, 1.0);
        let adjusted = ensure_minimum_contrast(black, white_bg, 3.0);
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

        // Contrast ratio should be 1.0 (no contrast)
        let ratio = contrast_ratio(fg, bg);
        assert!(
            (ratio - 1.0).abs() < 0.01,
            "Same color should have contrast ratio 1.0, got {}",
            ratio
        );

        // With minimum contrast 1.1, it should adjust
        let adjusted = ensure_minimum_contrast(fg, bg, 1.1);
        assert!(
            adjusted.l < 0.1 || adjusted.l > 0.9,
            "Color should be adjusted to black or white"
        );

        // The adjusted color should have sufficient contrast
        let new_ratio = contrast_ratio(adjusted, bg);
        assert!(
            new_ratio >= 1.1,
            "Adjusted contrast {} should be >= 1.1",
            new_ratio
        );
    }
}
