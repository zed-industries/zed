use gpui::{Hsla, Rgba};

/// Calculates the contrast ratio between two colors according to WCAG 2.0 standards.
///
/// The formula used is:
/// (L1 + 0.05) / (L2 + 0.05), where L1 is the lighter of the two luminances and L2 is the darker.
///
/// Returns a float representing the contrast ratio. A higher value indicates more contrast.
/// The range of the returned value is 1 to 21 (commonly written as 1:1 to 21:1).
pub fn calculate_contrast_ratio(fg: Hsla, bg: Hsla) -> f32 {
    let l1 = relative_luminance(fg);
    let l2 = relative_luminance(bg);

    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };

    (lighter + 0.05) / (darker + 0.05)
}

/// Calculates the relative luminance of a color.
///
/// The relative luminance is the relative brightness of any point in a colorspace,
/// normalized to 0 for darkest black and 1 for lightest white.
fn relative_luminance(color: Hsla) -> f32 {
    let rgba: Rgba = color.into();
    let r = linearize(rgba.r);
    let g = linearize(rgba.g);
    let b = linearize(rgba.b);

    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Linearizes an RGB component.
fn linearize(component: f32) -> f32 {
    if component <= 0.03928 {
        component / 12.92
    } else {
        ((component + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use gpui::hsla;

    use super::*;

    // Test the contrast ratio formula with some common color combinations to
    // prevent regressions in either the color conversions or the formula itself.
    #[test]
    fn test_contrast_ratio_formula() {
        // White on Black (should be close to 21:1)
        let white = hsla(0.0, 0.0, 1.0, 1.0);
        let black = hsla(0.0, 0.0, 0.0, 1.0);
        assert!((calculate_contrast_ratio(white, black) - 21.0).abs() < 0.1);

        // Black on White (should be close to 21:1)
        assert!((calculate_contrast_ratio(black, white) - 21.0).abs() < 0.1);

        // Mid-gray on Black (should be close to 5.32:1)
        let mid_gray = hsla(0.0, 0.0, 0.5, 1.0);
        assert!((calculate_contrast_ratio(mid_gray, black) - 5.32).abs() < 0.1);

        // White on Mid-gray (should be close to 3.95:1)
        assert!((calculate_contrast_ratio(white, mid_gray) - 3.95).abs() < 0.1);

        // Same color (should be 1:1)
        let red = hsla(0.0, 1.0, 0.5, 1.0);
        assert!((calculate_contrast_ratio(red, red) - 1.0).abs() < 0.01);
    }
}
