//! WCAG contrast utilities for theme-aware color selection.
//!
//! This module provides utilities for ensuring colors meet accessibility standards
//! based on WCAG (Web Content Accessibility Guidelines) contrast requirements.

use gpui::Hsla;

/// Calculate the relative luminance of a color.
///
/// Relative luminance is the relative brightness of any point in a colorspace,
/// normalized to 0 for darkest black and 1 for lightest white.
///
/// This follows the WCAG 2.1 specification for relative luminance calculation.
pub fn relative_luminance(color: Hsla) -> f32 {
    let rgb = color.to_rgb();

    // Convert to linear RGB (gamma correction)
    let r_linear = if rgb.r <= 0.03928 {
        rgb.r / 12.92
    } else {
        ((rgb.r + 0.055) / 1.055).powf(2.4)
    };

    let g_linear = if rgb.g <= 0.03928 {
        rgb.g / 12.92
    } else {
        ((rgb.g + 0.055) / 1.055).powf(2.4)
    };

    let b_linear = if rgb.b <= 0.03928 {
        rgb.b / 12.92
    } else {
        ((rgb.b + 0.055) / 1.055).powf(2.4)
    };

    // Calculate luminance using sRGB primaries
    0.2126 * r_linear + 0.7152 * g_linear + 0.0722 * b_linear
}

/// Calculate the contrast ratio between two colors.
///
/// The contrast ratio ranges from 1:1 (no contrast) to 21:1 (maximum contrast).
/// This follows the WCAG 2.1 specification for contrast ratio calculation.
pub fn contrast_ratio(color1: Hsla, color2: Hsla) -> f32 {
    let l1 = relative_luminance(color1);
    let l2 = relative_luminance(color2);

    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };

    (lighter + 0.05) / (darker + 0.05)
}

/// Blend a semi-transparent color with a background color.
///
/// This simulates how a semi-transparent overlay color would appear over a background.
pub fn blend_with_background(foreground: Hsla, background: Hsla) -> Hsla {
    let fg_rgb = foreground.to_rgb();
    let bg_rgb = background.to_rgb();

    // Alpha blend
    let alpha = foreground.a;
    let r = fg_rgb.r * alpha + bg_rgb.r * (1.0 - alpha);
    let g = fg_rgb.g * alpha + bg_rgb.g * (1.0 - alpha);
    let b = fg_rgb.b * alpha + bg_rgb.b * (1.0 - alpha);

    // Convert RGB back to HSL for the result
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if max == min {
        // Achromatic
        gpui::hsla(0.0, 0.0, l, foreground.a)
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

        gpui::hsla(h, s, l, foreground.a)
    }
}

/// Ensure two colors meet a minimum contrast ratio.
///
/// Returns the adjusted foreground color that meets the minimum contrast requirement.
/// If the colors already meet the requirement, returns the original foreground.
///
/// # Arguments
///
/// * `foreground` - The foreground color to potentially adjust
/// * `background` - The background color to contrast against
/// * `minimum_ratio` - The minimum contrast ratio required (e.g., 4.5 for WCAG AA normal text)
pub fn ensure_minimum_contrast(foreground: Hsla, background: Hsla, minimum_ratio: f32) -> Hsla {
    if minimum_ratio <= 0.0 {
        return foreground;
    }

    let current_ratio = contrast_ratio(foreground, background);

    if current_ratio >= minimum_ratio {
        return foreground;
    }

    // First try adjusting lightness while preserving hue and saturation
    if let Some(adjusted) = adjust_lightness_for_contrast(foreground, background, minimum_ratio) {
        return adjusted;
    }

    // If that doesn't work, try reducing saturation too
    if let Some(adjusted) =
        adjust_lightness_and_saturation_for_contrast(foreground, background, minimum_ratio)
    {
        return adjusted;
    }

    // Last resort: use pure black or white
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

    let black_contrast = contrast_ratio(black, background);
    let white_contrast = contrast_ratio(white, background);

    if white_contrast > black_contrast {
        white
    } else {
        black
    }
}

/// Adjust only the lightness to meet the minimum contrast, preserving hue and saturation.
fn adjust_lightness_for_contrast(
    foreground: Hsla,
    background: Hsla,
    minimum_ratio: f32,
) -> Option<Hsla> {
    let bg_luminance = relative_luminance(background);
    let should_go_darker = bg_luminance > 0.5;

    // Binary search for optimal lightness
    let mut low = if should_go_darker { 0.0 } else { foreground.l };
    let mut high = if should_go_darker { foreground.l } else { 1.0 };
    let mut best_lightness = foreground.l;

    for _ in 0..20 {
        let mid = (low + high) / 2.0;
        let test_color = Hsla {
            h: foreground.h,
            s: foreground.s,
            l: mid,
            a: foreground.a,
        };

        let ratio = contrast_ratio(test_color, background);

        if ratio >= minimum_ratio {
            best_lightness = mid;
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

        // If we're close enough, stop
        if (ratio - minimum_ratio).abs() < 0.1 {
            best_lightness = mid;
            break;
        }
    }

    if (contrast_ratio(
        Hsla {
            h: foreground.h,
            s: foreground.s,
            l: best_lightness,
            a: foreground.a,
        },
        background,
    ) - minimum_ratio)
        .abs()
        < 0.5
    {
        Some(Hsla {
            h: foreground.h,
            s: foreground.s,
            l: best_lightness,
            a: foreground.a,
        })
    } else {
        None
    }
}

/// Adjust both lightness and saturation to meet the minimum contrast.
fn adjust_lightness_and_saturation_for_contrast(
    foreground: Hsla,
    background: Hsla,
    minimum_ratio: f32,
) -> Option<Hsla> {
    // Try different saturation levels
    let saturation_steps = [1.0, 0.8, 0.6, 0.4, 0.2, 0.0];

    for &sat_multiplier in &saturation_steps {
        let test_color = Hsla {
            h: foreground.h,
            s: foreground.s * sat_multiplier,
            l: foreground.l,
            a: foreground.a,
        };

        if let Some(adjusted) = adjust_lightness_for_contrast(test_color, background, minimum_ratio)
        {
            return Some(adjusted);
        }
    }

    None
}

/// Get a text color that meets WCAG AA standards for small text (5:1 contrast).
///
/// This is a convenience function for the common case of ensuring text is readable
/// against a background color.
pub fn get_accessible_text_color(text_color: Hsla, background_color: Hsla) -> Hsla {
    ensure_minimum_contrast(text_color, background_color, 5.0)
}

/// Get a text color that meets WCAG AAA standards for large text (3:1 contrast).
///
/// This is useful for UI elements like badges or labels that might be larger
/// than normal text.
pub fn get_accessible_large_text_color(text_color: Hsla, background_color: Hsla) -> Hsla {
    ensure_minimum_contrast(text_color, background_color, 3.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::hsla;

    #[test]
    fn test_relative_luminance() {
        // Black should have luminance 0
        let black = hsla(0.0, 0.0, 0.0, 1.0);
        assert!(relative_luminance(black).abs() < 0.001);

        // White should have luminance 1
        let white = hsla(0.0, 0.0, 1.0, 1.0);
        assert!((relative_luminance(white) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_contrast_ratio() {
        let black = hsla(0.0, 0.0, 0.0, 1.0);
        let white = hsla(0.0, 0.0, 1.0, 1.0);

        // Black on white should have maximum contrast (~21:1)
        let ratio = contrast_ratio(black, white);
        assert!(ratio > 20.0);

        // Same colors should have contrast ratio of 1:1
        let gray = hsla(0.0, 0.0, 0.5, 1.0);
        assert!((contrast_ratio(gray, gray) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_ensure_minimum_contrast() {
        let white_bg = hsla(0.0, 0.0, 1.0, 1.0);
        let light_gray = hsla(0.0, 0.0, 0.8, 1.0); // Poor contrast on white

        let adjusted = ensure_minimum_contrast(light_gray, white_bg, 4.5);
        let ratio = contrast_ratio(adjusted, white_bg);
        assert!(ratio >= 4.5);
    }

    #[test]
    fn test_blend_with_background() {
        let red = hsla(0.0, 1.0, 0.5, 0.5); // Semi-transparent red
        let white = hsla(0.0, 0.0, 1.0, 1.0);

        let blended = blend_with_background(red, white);
        // Should be pinkish (higher red component)
        let blended_rgb = blended.to_rgb();
        assert!(blended_rgb.r > blended_rgb.g);
        assert!(blended_rgb.r > blended_rgb.b);
    }

    #[test]
    fn test_already_sufficient_contrast() {
        let black = hsla(0.0, 0.0, 0.0, 1.0);
        let white = hsla(0.0, 0.0, 1.0, 1.0);

        // Already has excellent contrast
        let adjusted = ensure_minimum_contrast(black, white, 4.5);
        assert_eq!(adjusted, black);
    }
}
