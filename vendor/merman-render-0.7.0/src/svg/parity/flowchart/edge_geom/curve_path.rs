//! Flowchart edge curve selection and bounds.
//!
//! This logic is shared between flowchart SVG emission and viewBox computation, and mirrors the
//! behavior of upstream Mermaid when selecting the D3 curve implementation and when deciding
//! whether curve bounds can be skipped for `basis`.

use super::*;

pub(in crate::svg::parity::flowchart) fn curve_path_d_and_bounds(
    line_data: &[crate::model::LayoutPoint],
    interpolate: &str,
    origin_x: f64,
    abs_top_transform: f64,
    viewbox_current_bounds: Option<(f64, f64, f64, f64)>,
) -> (String, Option<path_bounds::SvgPathBounds>, bool) {
    let curve_is_basis = !matches!(
        interpolate,
        "linear"
            | "natural"
            | "bumpY"
            | "catmullRom"
            | "step"
            | "stepAfter"
            | "stepBefore"
            | "cardinal"
            | "monotoneX"
            | "monotoneY"
            | "rounded"
    );

    if curve_is_basis {
        let _ = (origin_x, abs_top_transform, viewbox_current_bounds);
        let (d, raw_pb) = crate::svg::parity::curve::curve_basis_path_d_and_bounds(line_data);
        let d = maybe_close_single_point_path(d, line_data);
        let pb = svg_path_bounds_from_d(&d).or(raw_pb);
        (d, pb, false)
    } else {
        let (d, pb) = match interpolate {
            "linear" => crate::svg::parity::curve::curve_linear_path_d_and_bounds(line_data),
            "natural" => crate::svg::parity::curve::curve_natural_path_d_and_bounds(line_data),
            "bumpY" => crate::svg::parity::curve::curve_bump_y_path_d_and_bounds(line_data),
            "catmullRom" => {
                crate::svg::parity::curve::curve_catmull_rom_path_d_and_bounds(line_data)
            }
            "step" => crate::svg::parity::curve::curve_step_path_d_and_bounds(line_data),
            "stepAfter" => crate::svg::parity::curve::curve_step_after_path_d_and_bounds(line_data),
            "stepBefore" => {
                crate::svg::parity::curve::curve_step_before_path_d_and_bounds(line_data)
            }
            "cardinal" => {
                crate::svg::parity::curve::curve_cardinal_path_d_and_bounds(line_data, 0.0)
            }
            "monotoneX" => {
                crate::svg::parity::curve::curve_monotone_path_d_and_bounds(line_data, false)
            }
            "monotoneY" => {
                crate::svg::parity::curve::curve_monotone_path_d_and_bounds(line_data, true)
            }
            "rounded" => crate::svg::parity::curve::curve_rounded_path_d_and_bounds(line_data, 5.0),
            // Unknown curve names fall back to Mermaid's historical `basis` behavior.
            _ => crate::svg::parity::curve::curve_basis_path_d_and_bounds(line_data),
        };

        let d = maybe_close_single_point_path(d, line_data);
        let pb = svg_path_bounds_from_d(&d).or(pb);
        (d, pb, false)
    }
}

fn maybe_close_single_point_path(d: String, line_data: &[crate::model::LayoutPoint]) -> String {
    if line_data.len() == 1 && !d.ends_with('Z') {
        let mut d = d;
        d.push('Z');
        d
    } else {
        d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maybe_close_single_point_path_appends_z_once() {
        let line_data = vec![crate::model::LayoutPoint { x: 1.0, y: 2.0 }];

        assert_eq!(
            maybe_close_single_point_path("M1,2".to_string(), &line_data),
            "M1,2Z"
        );
        assert_eq!(
            maybe_close_single_point_path("M1,2Z".to_string(), &line_data),
            "M1,2Z"
        );
    }

    #[test]
    fn maybe_close_single_point_path_preserves_multi_point_paths() {
        let line_data = vec![
            crate::model::LayoutPoint { x: 1.0, y: 2.0 },
            crate::model::LayoutPoint { x: 3.0, y: 4.0 },
        ];

        assert_eq!(
            maybe_close_single_point_path("M1,2L3,4".to_string(), &line_data),
            "M1,2L3,4"
        );
    }
}
