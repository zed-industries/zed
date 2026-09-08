use gpui::{Hsla, Rgba};

/// Produces a readable text color for a given background, subtly tinted by the
/// background's own hue using the OKLCH color space.
///
/// The result keeps ~15% of the background's chroma so the text feels
/// harmonious with its surroundings rather than a flat black or white.
/// Lightness is set to ensure readable contrast against the background.
pub fn text_color_for_background(background: Hsla) -> Hsla {
    let rgba = Rgba::from(background);
    let r_lin = srgb_to_linear(rgba.r);
    let g_lin = srgb_to_linear(rgba.g);
    let b_lin = srgb_to_linear(rgba.b);

    let (_, ok_a, ok_b) = linear_rgb_to_oklab(r_lin, g_lin, b_lin);
    let chroma = (ok_a * ok_a + ok_b * ok_b).sqrt();
    let hue = ok_b.atan2(ok_a);

    let bg_luminance = relative_luminance(rgba);
    let text_l = if bg_luminance > 0.18 { 0.18 } else { 0.96 };
    let text_c = chroma * 0.15;

    let build = |c: f32| -> Rgba {
        let (tr, tg, tb) = oklab_to_linear_rgb(text_l, c * hue.cos(), c * hue.sin());
        Rgba {
            r: linear_to_srgb(tr.clamp(0.0, 1.0)),
            g: linear_to_srgb(tg.clamp(0.0, 1.0)),
            b: linear_to_srgb(tb.clamp(0.0, 1.0)),
            a: 1.0,
        }
    };

    let meets_contrast =
        |fg: Rgba| contrast_ratio_between(bg_luminance, relative_luminance(fg)) >= 4.5;

    let candidate = build(text_c);
    let result = if meets_contrast(candidate) {
        candidate
    } else {
        // Binary search for the maximum chroma that still meets 4.5:1.
        let mut lo = 0.0_f32;
        let mut hi = text_c;
        for _ in 0..16 {
            let mid = (lo + hi) * 0.5;
            if meets_contrast(build(mid)) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let best = build(lo);
        // Floating-point precision can leave the binary search result just
        // below the 4.5:1 threshold. Fall back to pure black or white.
        if meets_contrast(best) {
            best
        } else if bg_luminance > 0.18 {
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }
        } else {
            Rgba {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }
        }
    };
    Hsla::from(result)
}

fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

fn linear_rgb_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
    let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
    let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
    (
        0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    )
}

fn oklab_to_linear_rgb(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;
    (
        4.0767416621 * l_ * l_ * l_ - 3.3077115913 * m_ * m_ * m_ + 0.2309699292 * s_ * s_ * s_,
        -1.2684380046 * l_ * l_ * l_ + 2.6097574011 * m_ * m_ * m_ - 0.3413193965 * s_ * s_ * s_,
        -0.0041960863 * l_ * l_ * l_ - 0.7034186147 * m_ * m_ * m_ + 1.7076147010 * s_ * s_ * s_,
    )
}

fn relative_luminance(c: Rgba) -> f32 {
    0.2126 * srgb_to_linear(c.r) + 0.7152 * srgb_to_linear(c.g) + 0.0722 * srgb_to_linear(c.b)
}

fn contrast_ratio_between(luminance_a: f32, luminance_b: f32) -> f32 {
    let (lighter, darker) = if luminance_a > luminance_b {
        (luminance_a, luminance_b)
    } else {
        (luminance_b, luminance_a)
    };
    (lighter + 0.05) / (darker + 0.05)
}

#[cfg(test)]
fn wcag_contrast_ratio(a: Rgba, b: Rgba) -> f32 {
    contrast_ratio_between(relative_luminance(a), relative_luminance(b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::proptest::prelude::*;

    #[gpui::property_test]
    fn sufficient_contrast_for_any_opaque_background(
        #[strategy = Hsla::opaque_strategy()] bg: Hsla,
    ) -> Result<(), TestCaseError> {
        let text = text_color_for_background(bg);
        let ratio = wcag_contrast_ratio(Rgba::from(bg), Rgba::from(text));
        prop_assert!(
            ratio >= 4.5,
            "WCAG AA contrast ratio {ratio:.2} < 4.5 for bg {bg:?} -> text {text:?}",
        );
        Ok(())
    }
}
