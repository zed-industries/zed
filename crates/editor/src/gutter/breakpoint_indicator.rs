use gpui::{Bounds, Path, PathBuilder, point};
use ui::{Pixels, px};

/// Draw the path for the breakpoint indicator.
///
/// Note: The indicator needs to be a minimum of MIN_WIDTH px wide.
/// wide to draw without graphical issues, so it will ignore narrower width.
pub(crate) fn breakpoint_indicator_path(bounds: Bounds<Pixels>) -> Path<Pixels> {
    // All dimensions mentioned are in pixels, based on an a
    // total shape size of 50px wide by 15px high

    static BASE_HEIGHT: f32 = 15.;
    static BASE_WIDTH: f32 = 32.;
    static PIXEL_ROUNDING: f32 = 8.; // Round to the nearest eighth of a pixel

    static MIN_WIDTH: f32 = 34.;

    let width = if bounds.size.width.0 < MIN_WIDTH {
        px(MIN_WIDTH)
    } else {
        bounds.size.width
    };
    let height = bounds.size.height;

    // Position the indicator on the canvas
    let base_x = bounds.origin.x;
    let base_y = bounds.origin.y;

    // Calculate the scaling factor for the height
    let scale_factor = height / px(BASE_HEIGHT);

    // Calculate how much width to allocate to the stretchable middle section
    // Shape has 32px of fixed elements (corners), so the rest is for the middle
    let fixed_width = px(BASE_WIDTH) * scale_factor;
    let middle_width = width - fixed_width;

    let pixel_rounding = |value: Pixels| -> Pixels {
        let value_f32: f32 = value.into();
        px((value_f32 * PIXEL_ROUNDING).round() / PIXEL_ROUNDING)
    };

    // Create a new path
    let mut builder = PathBuilder::fill();

    // Start at center left (0, 8)
    let start_x = pixel_rounding(base_x);
    let start_y = pixel_rounding(base_y + px(7.5) * scale_factor);
    builder.move_to(point(start_x, start_y));

    // Vertical line to (0, 5)
    let vert_y = pixel_rounding(base_y + px(5.0) * scale_factor);
    builder.line_to(point(start_x, vert_y));

    // Curve to (5, 0) - using cubic Bezier
    let curve1_end_x = pixel_rounding(base_x + px(5.0) * scale_factor);
    let curve1_end_y = pixel_rounding(base_y);
    let curve1_ctrl1_x = pixel_rounding(base_x);
    let curve1_ctrl1_y = pixel_rounding(base_y + px(1.5) * scale_factor);
    let curve1_ctrl2_x = pixel_rounding(base_x + px(1.5) * scale_factor);
    let curve1_ctrl2_y = pixel_rounding(base_y);
    builder.cubic_bezier_to(
        point(curve1_end_x, curve1_end_y),
        point(curve1_ctrl1_x, curve1_ctrl1_y),
        point(curve1_ctrl2_x, curve1_ctrl2_y),
    );

    // Horizontal line through the middle section to (37, 0)
    let middle_end_x = pixel_rounding(base_x + px(5.0) * scale_factor + middle_width);
    builder.line_to(point(middle_end_x, curve1_end_y));

    // Horizontal line to (41, 0)
    let right_section_x =
        pixel_rounding(base_x + px(5.0) * scale_factor + middle_width + px(4.0) * scale_factor);
    builder.line_to(point(right_section_x, curve1_end_y));

    // Curve to (50, 7.5) - using cubic Bezier
    let curve2_end_x =
        pixel_rounding(base_x + px(5.0) * scale_factor + middle_width + px(13.0) * scale_factor);
    let curve2_end_y = pixel_rounding(base_y + px(7.5) * scale_factor);
    let curve2_ctrl1_x =
        pixel_rounding(base_x + px(5.0) * scale_factor + middle_width + px(9.0) * scale_factor);
    let curve2_ctrl1_y = pixel_rounding(base_y);
    let curve2_ctrl2_x =
        pixel_rounding(base_x + px(5.0) * scale_factor + middle_width + px(13.0) * scale_factor);
    let curve2_ctrl2_y = pixel_rounding(base_y + px(6.0) * scale_factor);
    builder.cubic_bezier_to(
        point(curve2_end_x, curve2_end_y),
        point(curve2_ctrl1_x, curve2_ctrl1_y),
        point(curve2_ctrl2_x, curve2_ctrl2_y),
    );

    // Lower half - mirrored vertically
    // Curve from (50, 7.5) to (41, 15)
    let curve3_end_y = pixel_rounding(base_y + px(15.0) * scale_factor);
    let curve3_ctrl1_x =
        pixel_rounding(base_x + px(5.0) * scale_factor + middle_width + px(13.0) * scale_factor);
    let curve3_ctrl1_y = pixel_rounding(base_y + px(9.0) * scale_factor);
    let curve3_ctrl2_x =
        pixel_rounding(base_x + px(5.0) * scale_factor + middle_width + px(9.0) * scale_factor);
    let curve3_ctrl2_y = pixel_rounding(base_y + px(15.0) * scale_factor);
    builder.cubic_bezier_to(
        point(right_section_x, curve3_end_y),
        point(curve3_ctrl1_x, curve3_ctrl1_y),
        point(curve3_ctrl2_x, curve3_ctrl2_y),
    );

    // Horizontal line to (37, 15)
    builder.line_to(point(middle_end_x, curve3_end_y));

    // Horizontal line through the middle section to (5, 15)
    builder.line_to(point(curve1_end_x, curve3_end_y));

    // Curve to (0, 10)
    let curve4_end_y = pixel_rounding(base_y + px(10.0) * scale_factor);
    let curve4_ctrl1_x = pixel_rounding(base_x + px(1.5) * scale_factor);
    let curve4_ctrl1_y = pixel_rounding(base_y + px(15.0) * scale_factor);
    let curve4_ctrl2_x = pixel_rounding(base_x);
    let curve4_ctrl2_y = pixel_rounding(base_y + px(13.5) * scale_factor);
    builder.cubic_bezier_to(
        point(start_x, curve4_end_y),
        point(curve4_ctrl1_x, curve4_ctrl1_y),
        point(curve4_ctrl2_x, curve4_ctrl2_y),
    );

    builder.line_to(point(start_x, start_y));

    builder.build().unwrap()
}
