//! Metrics scaling.
//!
//! Uses the widths and blues computations to generate unscaled metrics for a
//! given style/script.
//!
//! Then applies a scaling factor to those metrics, computes a potentially
//! modified scale, and tags active blue zones.

use super::super::{
    metrics::{
        fixed_div, fixed_mul, fixed_mul_div, pix_round, BlueZones, Scale, ScaledAxisMetrics,
        ScaledBlue, ScaledStyleMetrics, ScaledWidth, UnscaledAxisMetrics, UnscaledBlue,
        UnscaledStyleMetrics, WidthMetrics,
    },
    shape::Shaper,
    style::{ScriptGroup, StyleClass},
    topo::{Axis, Dimension},
};
use crate::{prelude::Size, MetadataProvider};
use raw::types::F2Dot14;

/// Computes unscaled metrics for the Latin writing system.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L1134>
pub(crate) fn compute_unscaled_style_metrics(
    shaper: &Shaper,
    coords: &[F2Dot14],
    style: &StyleClass,
) -> UnscaledStyleMetrics {
    let charmap = shaper.charmap();
    // We don't attempt to produce any metrics if we don't have a Unicode
    // cmap
    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L1146>
    if charmap.is_symbol() {
        return UnscaledStyleMetrics {
            class_ix: style.index as u16,
            axes: [
                UnscaledAxisMetrics {
                    dim: Axis::HORIZONTAL,
                    ..Default::default()
                },
                UnscaledAxisMetrics {
                    dim: Axis::VERTICAL,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
    }
    let [hwidths, vwidths] = super::widths::compute_widths(shaper, coords, style);
    let [hblues, vblues] = super::blues::compute_unscaled_blues(shaper, coords, style);
    let glyph_metrics = shaper.font().glyph_metrics(Size::unscaled(), coords);
    let mut digit_advance = None;
    let mut digits_have_same_width = true;
    for ch in '0'..='9' {
        if let Some(advance) = charmap
            .map(ch)
            .and_then(|gid| glyph_metrics.advance_width(gid))
        {
            if digit_advance.is_some() && digit_advance != Some(advance) {
                digits_have_same_width = false;
                break;
            }
            digit_advance = Some(advance);
        }
    }
    UnscaledStyleMetrics {
        class_ix: style.index as u16,
        digits_have_same_width,
        axes: [
            UnscaledAxisMetrics {
                dim: Axis::HORIZONTAL,
                blues: hblues,
                width_metrics: hwidths.0,
                widths: hwidths.1,
            },
            UnscaledAxisMetrics {
                dim: Axis::VERTICAL,
                blues: vblues,
                width_metrics: vwidths.0,
                widths: vwidths.1,
            },
        ],
    }
}

/// Computes scaled metrics for the Latin writing system.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L1491>
pub(crate) fn scale_style_metrics(
    unscaled_metrics: &UnscaledStyleMetrics,
    mut scale: Scale,
) -> ScaledStyleMetrics {
    let scale_axis_fn = if unscaled_metrics.style_class().script.group == ScriptGroup::Default {
        scale_default_axis_metrics
    } else {
        scale_cjk_axis_metrics
    };
    let mut scale_axis = |axis: &UnscaledAxisMetrics| {
        scale_axis_fn(
            axis.dim,
            &axis.widths,
            axis.width_metrics,
            &axis.blues,
            &mut scale,
        )
    };
    let axes = [
        scale_axis(&unscaled_metrics.axes[0]),
        scale_axis(&unscaled_metrics.axes[1]),
    ];
    ScaledStyleMetrics { scale, axes }
}

/// Computes scaled metrics for a single axis.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L1168>
fn scale_default_axis_metrics(
    dim: Dimension,
    widths: &[i32],
    width_metrics: WidthMetrics,
    blues: &[UnscaledBlue],
    scale: &mut Scale,
) -> ScaledAxisMetrics {
    let mut axis = ScaledAxisMetrics {
        dim,
        ..Default::default()
    };
    if dim == Axis::HORIZONTAL {
        axis.scale = scale.x_scale;
        axis.delta = scale.x_delta;
    } else {
        axis.scale = scale.y_scale;
        axis.delta = scale.y_delta;
    };
    // Correct Y scale to optimize alignment
    if let Some(blue_ix) = blues
        .iter()
        .position(|blue| blue.zones.contains(BlueZones::ADJUSTMENT))
    {
        let unscaled_blue = &blues[blue_ix];
        let scaled = fixed_mul(axis.scale, unscaled_blue.overshoot);
        let fitted = (scaled + 40) & !63;
        if scaled != fitted && dim == Axis::VERTICAL {
            let new_scale = fixed_mul_div(axis.scale, fitted, scaled);
            // Scaling should not adjust by more than 2 pixels
            let mut max_height = scale.units_per_em;
            for blue in blues {
                max_height = max_height.max(blue.ascender).max(-blue.descender);
            }
            let mut dist = fixed_mul(max_height, new_scale - axis.scale).abs();
            dist &= !127;
            if dist == 0 {
                axis.scale = new_scale;
                scale.y_scale = new_scale;
            }
        }
    }
    // Now scale the widths
    axis.width_metrics = width_metrics;
    for unscaled_width in widths {
        let scaled = fixed_mul(axis.scale, *unscaled_width);
        axis.widths.push(ScaledWidth {
            scaled,
            fitted: scaled,
        });
    }
    // Compute extra light property: this is a standard width that is
    // less than 5/8 pixels
    axis.width_metrics.is_extra_light =
        fixed_mul(axis.width_metrics.standard_width, axis.scale) < (32 + 8);
    if dim == Axis::VERTICAL {
        // And scale the blue zones
        for unscaled_blue in blues {
            let scaled_position = fixed_mul(axis.scale, unscaled_blue.position) + axis.delta;
            let scaled_overshoot = fixed_mul(axis.scale, unscaled_blue.overshoot) + axis.delta;
            let mut blue = ScaledBlue {
                position: ScaledWidth {
                    scaled: scaled_position,
                    fitted: scaled_position,
                },
                overshoot: ScaledWidth {
                    scaled: scaled_overshoot,
                    fitted: scaled_overshoot,
                },
                zones: unscaled_blue.zones,
                is_active: false,
            };
            // Only activate blue zones less than 3/4 pixel tall
            let dist = fixed_mul(unscaled_blue.position - unscaled_blue.overshoot, axis.scale);
            if (-48..=48).contains(&dist) {
                let mut delta = dist.abs();
                if delta < 32 {
                    delta = 0;
                } else if delta < 48 {
                    delta = 32;
                } else {
                    delta = 64;
                }
                if dist < 0 {
                    delta = -delta;
                }
                blue.position.fitted = pix_round(blue.position.scaled);
                blue.overshoot.fitted = blue.position.fitted - delta;
                blue.is_active = true;
            }
            axis.blues.push(blue);
        }
        // Use sub-top blue zone if it doesn't overlap with another
        // non-sub-top blue zone
        for blue_ix in 0..axis.blues.len() {
            let blue = axis.blues[blue_ix];
            if !blue.zones.is_sub_top() || !blue.is_active {
                continue;
            }
            for blue2 in &axis.blues {
                if blue2.zones.is_sub_top() || !blue2.is_active {
                    continue;
                }
                if blue2.position.fitted <= blue.overshoot.fitted
                    && blue2.overshoot.fitted >= blue.position.fitted
                {
                    axis.blues[blue_ix].is_active = false;
                    break;
                }
            }
        }
    }
    axis
}

/// Computes scaled metrics for a single axis for the CJK script group.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afcjk.c#L661>
fn scale_cjk_axis_metrics(
    dim: Dimension,
    widths: &[i32],
    width_metrics: WidthMetrics,
    blues: &[UnscaledBlue],
    scale: &mut Scale,
) -> ScaledAxisMetrics {
    let mut axis = ScaledAxisMetrics {
        dim,
        ..Default::default()
    };
    axis.dim = dim;
    if dim == Axis::HORIZONTAL {
        axis.scale = scale.x_scale;
        axis.delta = scale.x_delta;
    } else {
        axis.scale = scale.y_scale;
        axis.delta = scale.y_delta;
    };
    let scale = axis.scale;
    // Scale the blue zones
    for unscaled_blue in blues {
        let position = fixed_mul(unscaled_blue.position, scale) + axis.delta;
        let overshoot = fixed_mul(unscaled_blue.overshoot, scale) + axis.delta;
        let mut blue = ScaledBlue {
            position: ScaledWidth {
                scaled: position,
                fitted: position,
            },
            overshoot: ScaledWidth {
                scaled: overshoot,
                fitted: overshoot,
            },
            zones: unscaled_blue.zones,
            is_active: false,
        };
        // A blue zone is only active if it is less than 3/4 pixels tall
        let dist = fixed_mul(unscaled_blue.position - unscaled_blue.overshoot, scale);
        if (-48..=48).contains(&dist) {
            blue.position.fitted = pix_round(blue.position.scaled);
            // For CJK, "overshoot" is actually undershoot
            let delta1 = fixed_div(blue.position.fitted, scale) - unscaled_blue.overshoot;
            let mut delta2 = fixed_mul(delta1.abs(), scale);
            if delta2 < 32 {
                delta2 = 0;
            } else {
                delta2 = pix_round(delta2);
            }
            if delta1 < 0 {
                delta2 = -delta2;
            }
            blue.overshoot.fitted = blue.position.fitted - delta2;
            blue.is_active = true;
        }
        axis.blues.push(blue);
    }
    // FreeType never seems to compute scaled width values. We'll just
    // match this behavior for now.
    // <https://github.com/googlefonts/fontations/issues/1129>
    for _ in 0..widths.len() {
        axis.widths.push(ScaledWidth::default());
    }
    axis.width_metrics = width_metrics;
    axis
}

#[cfg(test)]
mod tests {
    use super::{
        super::super::{shape::ShaperMode, style},
        *,
    };
    use crate::attribute::Style;
    use raw::{FontRef, TableProvider};

    #[test]
    fn scaled_metrics_default() {
        // Note: expected values scraped from a FreeType debugging
        // session
        let scaled_metrics = make_scaled_metrics(
            font_test_data::NOTOSERIFHEBREW_AUTOHINT_METRICS,
            StyleClass::HEBR,
        );
        // Check scale and deltas
        assert_eq!(scaled_metrics.scale.x_scale, 67109);
        assert_eq!(scaled_metrics.scale.y_scale, 67109);
        assert_eq!(scaled_metrics.scale.x_delta, 0);
        assert_eq!(scaled_metrics.scale.y_delta, 0);
        // Horizontal widths
        let h_axis = &scaled_metrics.axes[0];
        let expected_h_widths = [55];
        // No horizontal blues
        check_axis(h_axis, &expected_h_widths, &[]);
        // Not extra light
        assert!(!h_axis.width_metrics.is_extra_light);
        // Vertical widths
        let v_axis = &scaled_metrics.axes[1];
        let expected_v_widths = [22, 112];
        // Vertical blues
        #[rustfmt::skip]
        let expected_v_blues = [
            // ((scaled_pos, fitted_pos), (scaled_shoot, fitted_shoot), flags, is_active)
            ScaledBlue::from(((606, 576), (606, 576), BlueZones::TOP, true)),
            ScaledBlue::from(((0, 0), (-9, 0), BlueZones::default(), true)),
            ScaledBlue::from(((-246, -256), (-246, -256), BlueZones::default(), true)),
        ];
        check_axis(v_axis, &expected_v_widths, &expected_v_blues);
        // This one is extra light
        assert!(v_axis.width_metrics.is_extra_light);
    }

    #[test]
    fn cjk_scaled_metrics() {
        // Note: expected values scraped from a FreeType debugging
        // session
        let scaled_metrics = make_scaled_metrics(
            font_test_data::NOTOSERIFTC_AUTOHINT_METRICS,
            StyleClass::HANI,
        );
        // Check scale and deltas
        assert_eq!(scaled_metrics.scale.x_scale, 67109);
        assert_eq!(scaled_metrics.scale.y_scale, 67109);
        assert_eq!(scaled_metrics.scale.x_delta, 0);
        assert_eq!(scaled_metrics.scale.y_delta, 0);
        // Horizontal widths
        let h_axis = &scaled_metrics.axes[0];
        let expected_h_widths = [0];
        check_axis(h_axis, &expected_h_widths, &[]);
        // Not extra light
        assert!(!h_axis.width_metrics.is_extra_light);
        // Vertical widths
        let v_axis = &scaled_metrics.axes[1];
        let expected_v_widths = [0];
        // Vertical blues
        #[rustfmt::skip]
        let expected_v_blues = [
            // ((scaled_pos, fitted_pos), (scaled_shoot, fitted_shoot), flags, is_active)
            ScaledBlue::from(((857, 832), (844, 832), BlueZones::TOP, true)),
            ScaledBlue::from(((-80, -64), (-68, -64), BlueZones::default(), true)),
        ];
        // No horizontal blues
        check_axis(v_axis, &expected_v_widths, &expected_v_blues);
        // Also not extra light
        assert!(!v_axis.width_metrics.is_extra_light);
    }

    fn make_scaled_metrics(font_data: &[u8], style_class: usize) -> ScaledStyleMetrics {
        let font = FontRef::new(font_data).unwrap();
        let class = &style::STYLE_CLASSES[style_class];
        let shaper = Shaper::new(&font, ShaperMode::Nominal);
        let unscaled_metrics = compute_unscaled_style_metrics(&shaper, Default::default(), class);
        let scale = Scale::new(
            16.0,
            font.head().unwrap().units_per_em() as i32,
            Style::Normal,
            Default::default(),
            class.script.group,
        );
        scale_style_metrics(&unscaled_metrics, scale)
    }

    fn check_axis(
        axis: &ScaledAxisMetrics,
        expected_widths: &[i32],
        expected_blues: &[ScaledBlue],
    ) {
        let widths = axis
            .widths
            .iter()
            .map(|width| width.scaled)
            .collect::<Vec<_>>();
        assert_eq!(widths, expected_widths);
        assert_eq!(axis.blues.as_slice(), expected_blues);
    }

    impl From<(i32, i32)> for ScaledWidth {
        fn from(value: (i32, i32)) -> Self {
            Self {
                scaled: value.0,
                fitted: value.1,
            }
        }
    }

    impl From<((i32, i32), (i32, i32), BlueZones, bool)> for ScaledBlue {
        fn from(value: ((i32, i32), (i32, i32), BlueZones, bool)) -> Self {
            Self {
                position: value.0.into(),
                overshoot: value.1.into(),
                zones: value.2,
                is_active: value.3,
            }
        }
    }
}
