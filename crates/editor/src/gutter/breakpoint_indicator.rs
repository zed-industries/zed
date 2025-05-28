use gpui::{Bounds, Path, PathBuilder, PathStyle, StrokeOptions, point};
use ui::{Pixels, px};

/// Draw the path for the breakpoint indicator.
///
/// Note: The indicator needs to be a minimum of MIN_WIDTH px wide.
/// wide to draw without graphical issues, so it will ignore narrower width.
pub(crate) fn breakpoint_indicator_path(
    bounds: Bounds<Pixels>,
    scale: f32,
    stroke: bool,
) -> Path<Pixels> {
    // Constants for the breakpoint shape dimensions
    // The shape is designed based on a 50px wide by 15px high template
    // and uses 9-slice style scaling to allow the shape to be stretched
    // vertically and horizontally.
    const SHAPE_BASE_HEIGHT: f32 = 15.0;
    const SHAPE_FIXED_WIDTH: f32 = 32.0; // Width of non-stretchable parts (corners)
    const SHAPE_MIN_WIDTH: f32 = 34.0; // Minimum width to render properly
    const PIXEL_ROUNDING_FACTOR: f32 = 8.0; // Round to nearest 1/8 pixel

    // Key points in the shape (in base coordinates)
    const CORNER_RADIUS: f32 = 5.0;
    const CENTER_Y: f32 = 7.5;
    const TOP_Y: f32 = 0.0;
    const BOTTOM_Y: f32 = 15.0;
    const CURVE_CONTROL_OFFSET: f32 = 1.5;
    const RIGHT_CORNER_START: f32 = 4.0;
    const RIGHT_CORNER_WIDTH: f32 = 13.0;

    // Helper function to round pixels to nearest 1/8
    let round_to_pixel_grid = |value: Pixels| -> Pixels {
        let value_f32: f32 = value.into();
        px((value_f32 * PIXEL_ROUNDING_FACTOR).round() / PIXEL_ROUNDING_FACTOR)
    };

    // Calculate actual dimensions with scaling
    let min_allowed_width = px(SHAPE_MIN_WIDTH * scale);
    let actual_width = if bounds.size.width < min_allowed_width {
        min_allowed_width
    } else {
        bounds.size.width
    };
    let actual_height = bounds.size.height;

    // Debug input parameters and initial calculations
    dbg!(&bounds);
    dbg!(scale);
    dbg!(stroke);
    dbg!(min_allowed_width);
    dbg!(actual_width);
    dbg!(actual_height);

    // Origin point for positioning
    let origin_x = bounds.origin.x;
    let origin_y = bounds.origin.y;

    // Calculate the scale factor based on height and user scale
    let shape_scale = (actual_height / px(SHAPE_BASE_HEIGHT)) * scale;

    // Calculate the width of fixed and stretchable sections
    let fixed_sections_width = px(SHAPE_FIXED_WIDTH) * shape_scale;
    let stretchable_middle_width = actual_width - fixed_sections_width;

    // Debug scaling calculations
    dbg!(shape_scale);
    dbg!(fixed_sections_width);
    dbg!(stretchable_middle_width);

    // Pre-calculate all the key x-coordinates
    let left_edge_x = round_to_pixel_grid(origin_x);
    let left_corner_end_x = round_to_pixel_grid(origin_x + px(CORNER_RADIUS) * shape_scale);
    let middle_section_end_x =
        round_to_pixel_grid(origin_x + px(CORNER_RADIUS) * shape_scale + stretchable_middle_width);
    let right_corner_start_x = round_to_pixel_grid(
        origin_x
            + px(CORNER_RADIUS) * shape_scale
            + stretchable_middle_width
            + px(RIGHT_CORNER_START) * shape_scale,
    );
    let right_edge_x = round_to_pixel_grid(
        origin_x
            + px(CORNER_RADIUS) * shape_scale
            + stretchable_middle_width
            + px(RIGHT_CORNER_WIDTH) * shape_scale,
    );

    // Debug x-coordinates
    dbg!(origin_x);
    dbg!(left_edge_x);
    dbg!(left_corner_end_x);
    dbg!(middle_section_end_x);
    dbg!(right_corner_start_x);
    dbg!(right_edge_x);

    // Pre-calculate all the key y-coordinates
    let top_edge_y = round_to_pixel_grid(origin_y);
    let center_y = round_to_pixel_grid(origin_y + px(CENTER_Y) * shape_scale);
    let bottom_edge_y = round_to_pixel_grid(origin_y + px(BOTTOM_Y) * shape_scale);

    // Y-coordinates for the left side curves
    let left_upper_curve_start_y = round_to_pixel_grid(origin_y + px(CORNER_RADIUS) * shape_scale);
    let left_lower_curve_end_y = round_to_pixel_grid(origin_y + px(10.0) * shape_scale);

    // Y-coordinates for the right side curves
    let right_upper_curve_control_y = round_to_pixel_grid(origin_y + px(6.0) * shape_scale);
    let right_lower_curve_control_y = round_to_pixel_grid(origin_y + px(9.0) * shape_scale);

    // Control point offsets
    let control_offset = px(CURVE_CONTROL_OFFSET) * shape_scale;
    let right_control_offset = px(9.0) * shape_scale;

    // Debug y-coordinates
    dbg!(origin_y);
    dbg!(top_edge_y);
    dbg!(center_y);
    dbg!(bottom_edge_y);
    dbg!(left_upper_curve_start_y);
    dbg!(left_lower_curve_end_y);
    dbg!(right_upper_curve_control_y);
    dbg!(right_lower_curve_control_y);

    // Create the path builder
    let mut builder = if stroke {
        let stroke_width = px(1.0 * scale);
        let options = StrokeOptions::default().with_line_width(stroke_width.0);
        PathBuilder::stroke(stroke_width).with_style(PathStyle::Stroke(options))
    } else {
        PathBuilder::fill()
    };

    // Build the path - starting from left center
    builder.move_to(point(left_edge_x, center_y));

    // === Upper half of the shape ===

    // Move up to start of left upper curve
    builder.line_to(point(left_edge_x, left_upper_curve_start_y));

    // Top-left corner curve
    builder.cubic_bezier_to(
        point(left_corner_end_x, top_edge_y),
        point(left_edge_x, round_to_pixel_grid(origin_y + control_offset)),
        point(round_to_pixel_grid(origin_x + control_offset), top_edge_y),
    );

    // Top edge - stretchable middle section
    builder.line_to(point(middle_section_end_x, top_edge_y));

    // Top edge - right corner start
    builder.line_to(point(right_corner_start_x, top_edge_y));

    // Top-right corner curve
    builder.cubic_bezier_to(
        point(right_edge_x, center_y),
        point(
            round_to_pixel_grid(
                origin_x
                    + px(CORNER_RADIUS) * shape_scale
                    + stretchable_middle_width
                    + right_control_offset,
            ),
            top_edge_y,
        ),
        point(right_edge_x, right_upper_curve_control_y),
    );

    // === Lower half of the shape (mirrored) ===

    // Bottom-right corner curve
    builder.cubic_bezier_to(
        point(right_corner_start_x, bottom_edge_y),
        point(right_edge_x, right_lower_curve_control_y),
        point(
            round_to_pixel_grid(
                origin_x
                    + px(CORNER_RADIUS) * shape_scale
                    + stretchable_middle_width
                    + right_control_offset,
            ),
            bottom_edge_y,
        ),
    );

    // Bottom edge - right corner to middle
    builder.line_to(point(middle_section_end_x, bottom_edge_y));

    // Bottom edge - stretchable middle section
    builder.line_to(point(left_corner_end_x, bottom_edge_y));

    // Bottom-left corner curve
    builder.cubic_bezier_to(
        point(left_edge_x, left_lower_curve_end_y),
        point(
            round_to_pixel_grid(origin_x + control_offset),
            bottom_edge_y,
        ),
        point(
            left_edge_x,
            round_to_pixel_grid(origin_y + px(13.5) * shape_scale),
        ),
    );

    // Close the path by returning to start
    builder.line_to(point(left_edge_x, center_y));

    builder.build().unwrap()
}
