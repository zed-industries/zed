#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

float4 hsla_to_rgba(Hsla hsla);
float3 srgb_to_linear(float3 color);
float3 linear_to_srgb(float3 color);
float4 srgb_to_oklab(float4 color);
float4 oklab_to_srgb(float4 color);
float4 to_device_position(float2 unit_vertex, Bounds_ScaledPixels bounds,
                          constant Size_DevicePixels *viewport_size);
float4 to_device_position_transformed(float2 unit_vertex, Bounds_ScaledPixels bounds,
                          TransformationMatrix transformation,
                          constant Size_DevicePixels *input_viewport_size);

float2 to_tile_position(float2 unit_vertex, AtlasTile tile,
                        constant Size_DevicePixels *atlas_size);
float4 distance_from_clip_rect(float2 unit_vertex, Bounds_ScaledPixels bounds,
                               Bounds_ScaledPixels clip_bounds);
float4 distance_from_clip_rect_transformed(float2 unit_vertex, Bounds_ScaledPixels bounds,
                               Bounds_ScaledPixels clip_bounds, TransformationMatrix transformation);
float corner_dash_velocity(float dv1, float dv2);
float dash_alpha(float t, float period, float length, float dash_velocity,
                 float antialias_threshold);
float quarter_ellipse_sdf(float2 point, float2 radii);
float pick_corner_radius(float2 center_to_point, Corners_ScaledPixels corner_radii);
float pick_corner_shape(float2 center_to_point, Corners_f32 corner_shapes);
float superellipse_sdf(float2 point, float2 radii, float n);
// One corner curve as css-borders-4 "Rendering corner-shape" builds it. The
// curve runs from `start` on the horizontal edge to `end` on the vertical
// edge. Positions are distances from the corner along the horizontal edge (x)
// and along the vertical edge (y). A border moves each end inward along its
// normal by the width of that end's edge. Two different widths tilt both
// normals so the border grows evenly from one end to the other.
struct CornerCurve {
  float2 start;
  float2 end;
  float2 start_normal;
  float2 end_normal;
};

CornerCurve corner_curve(float corner_radius, float shape, float2 inset);
float corner_curve_sdf(float2 from_corner, CornerCurve curve, float shape,
                       float straight);
float2 shaped_corner_sdf(float2 corner_to_point, float corner_radius,
                         float shape, float2 reduced_border,
                         float2 straight_border_inner_corner_to_point);
float quad_sdf(float2 point, Bounds_ScaledPixels bounds,
               Corners_ScaledPixels corner_radii);
float quad_sdf_impl(float2 center_to_point, float corner_radius);
float gaussian(float x, float sigma);
float2 erf(float2 x);
float blur_along_x(float x, float y, float sigma, float corner,
                   float2 half_size);
float4 over(float4 below, float4 above);
float radians(float degrees);
float4 fill_color(Background background, float2 position, Bounds_ScaledPixels bounds,
  float4 solid_color);
float4 prepare_fill_color(Background background);

struct QuadVertexOutput {
  uint quad_id [[flat]];
  float4 position [[position]];
  float4 border_color [[flat]];
  float4 background_solid [[flat]];
  float clip_distance [[clip_distance]][4];
};

struct QuadFragmentInput {
  uint quad_id [[flat]];
  float4 position [[position]];
  float4 border_color [[flat]];
  float4 background_solid [[flat]];
};

vertex QuadVertexOutput quad_vertex(uint unit_vertex_id [[vertex_id]],
                                    uint quad_id [[instance_id]],
                                    constant float2 *unit_vertices
                                    [[buffer(QuadInputIndex_Vertices)]],
                                    constant Quad *quads
                                    [[buffer(QuadInputIndex_Quads)]],
                                    constant Size_DevicePixels *viewport_size
                                    [[buffer(QuadInputIndex_ViewportSize)]]) {
  float2 unit_vertex = unit_vertices[unit_vertex_id];
  Quad quad = quads[quad_id];
  float4 device_position =
      to_device_position(unit_vertex, quad.bounds, viewport_size);
  float4 clip_distance = distance_from_clip_rect(unit_vertex, quad.bounds,
                                                 quad.content_mask.bounds);
  float4 border_color = hsla_to_rgba(quad.border_color);

  return QuadVertexOutput{
      quad_id,
      device_position,
      border_color,
      prepare_fill_color(quad.background),
      {clip_distance.x, clip_distance.y, clip_distance.z, clip_distance.w}};
}

fragment float4 quad_fragment(QuadFragmentInput input [[stage_in]],
                              constant Quad *quads
                              [[buffer(QuadInputIndex_Quads)]]) {
  Quad quad = quads[input.quad_id];
  float4 background_color = fill_color(quad.background, input.position.xy, quad.bounds,
    input.background_solid);

  bool unrounded = quad.corner_radii.top_left == 0.0 &&
    quad.corner_radii.bottom_left == 0.0 &&
    quad.corner_radii.top_right == 0.0 &&
    quad.corner_radii.bottom_right == 0.0;

  // Fast path when the quad is not rounded and doesn't have any border
  if (quad.border_widths.top == 0.0 &&
      quad.border_widths.left == 0.0 &&
      quad.border_widths.right == 0.0 &&
      quad.border_widths.bottom == 0.0 &&
      unrounded) {
    return background_color;
  }

  float2 size = float2(quad.bounds.size.width, quad.bounds.size.height);
  float2 half_size = size / 2.0;
  float2 point = input.position.xy - float2(quad.bounds.origin.x, quad.bounds.origin.y);
  float2 center_to_point = point - half_size;

  // Signed distance field threshold for inclusion of pixels. 0.5 is the
  // minimum distance between the center of the pixel and the edge.
  const float antialias_threshold = 0.5;

  // Radius and shape of the nearest corner
  float corner_radius = pick_corner_radius(center_to_point, quad.corner_radii);
  float corner_shape = pick_corner_shape(center_to_point, quad.corner_shapes);

  // Width of the nearest borders
  float2 border = float2(
    center_to_point.x < 0.0 ? quad.border_widths.left : quad.border_widths.right,
    center_to_point.y < 0.0 ? quad.border_widths.top : quad.border_widths.bottom
  );

  // 0-width borders are reduced so that `inner_sdf >= antialias_threshold`.
  // The purpose of this is to not draw antialiasing pixels in this case.
  float2 reduced_border = float2(
    border.x == 0.0 ? -antialias_threshold : border.x,
    border.y == 0.0 ? -antialias_threshold : border.y);

  // Vector from the corner of the quad bounds to the point, after mirroring
  // the point into the bottom right quadrant. Both components are <= 0.
  float2 corner_to_point = fabs(center_to_point) - half_size;

  // Vector from the point to the center of the rounded corner's circle, also
  // mirrored into bottom right quadrant.
  float2 corner_center_to_point = corner_to_point + corner_radius;

  // Whether the nearest point on the border is rounded. The inner edge of a
  // concave or a bevelled corner reaches past the corner box.
  float2 corner_reach = corner_center_to_point;
  if (corner_shape != 1.0) {
    CornerCurve inner_curve = corner_curve(corner_radius, corner_shape, border);
    corner_reach += float2(inner_curve.start.x, inner_curve.end.y) - corner_radius;
  }
  bool is_near_rounded_corner =
    corner_reach.x >= 0.0 &&
    corner_reach.y >= 0.0;

  // Vector from straight border inner corner to point.
  //
  // 0-width borders are turned into width -1 so that inner_sdf is > 1.0 near
  // the border. Without this, antialiasing pixels would be drawn.
  float2 straight_border_inner_corner_to_point = corner_to_point + reduced_border;

  // Whether the point is beyond the inner edge of the straight border
  bool is_beyond_inner_straight_border =
    straight_border_inner_corner_to_point.x > 0.0 ||
    straight_border_inner_corner_to_point.y > 0.0;


  // Whether the point is far enough inside the quad, such that the pixels are
  // not affected by the straight border.
  bool is_within_inner_straight_border =
    straight_border_inner_corner_to_point.x < -antialias_threshold &&
    straight_border_inner_corner_to_point.y < -antialias_threshold;

  // Fast path for points that must be part of the background
  if (is_within_inner_straight_border && !is_near_rounded_corner) {
    return background_color;
  }

  // Signed distance of the point to the outside edge of the quad's border
  float outer_sdf;

  // Approximate signed distance of the point to the inside edge of the quad's
  // border. It is negative outside this edge (within the border), and
  // positive inside.
  //
  // This is not always an accurate signed distance:
  // * The rounded portions with varying border width use an approximation of
  //   nearest-point-on-ellipse.
  // * When it is quickly known to be outside the edge, -1.0 is used.
  float inner_sdf = 0.0;
  if (corner_shape == 1.0 || corner_radius == 0.0) {
    outer_sdf = quad_sdf_impl(corner_center_to_point, corner_radius);
      if (corner_center_to_point.x <= 0.0 || corner_center_to_point.y <= 0.0) {
      // Fast paths for straight borders
      inner_sdf = -max(straight_border_inner_corner_to_point.x,
                       straight_border_inner_corner_to_point.y);
    } else if (is_beyond_inner_straight_border) {
      // Fast path for points that must be outside the inner edge
      inner_sdf = -1.0;
    } else if (reduced_border.x == reduced_border.y) {
      // Fast path for circular inner edge.
      inner_sdf = -(outer_sdf + reduced_border.x);
    } else {
      float2 ellipse_radii = max(float2(0.0), float2(corner_radius) - reduced_border);
      inner_sdf = quarter_ellipse_sdf(corner_center_to_point, ellipse_radii);
    }
  } else {
    // Any other corner shape: a superellipse, or a bite out of the corner.
    float2 sdfs = shaped_corner_sdf(corner_to_point, corner_radius, corner_shape,
                                    reduced_border,
                                    straight_border_inner_corner_to_point);
    outer_sdf = sdfs.x;
    inner_sdf = sdfs.y;
  }

  // Negative when inside the border
  float border_sdf = max(inner_sdf, outer_sdf);

  float4 color = background_color;
  if (border_sdf < antialias_threshold) {
    float4 border_color = input.border_color;

    // Dashed border logic when border_style == 1
    if (quad.border_style == 1) {
      // Position along the perimeter in "dash space", where each dash
      // period has length 1
      float t = 0.0;

      // Total number of dash periods, so that the dash spacing can be
      // adjusted to evenly divide it
      float max_t = 0.0;

      // Border width is proportional to dash size. This is the behavior
      // used by browsers, but also avoids dashes from different segments
      // overlapping when dash size is smaller than the border width.
      //
      // Dash pattern: (2 * border width) dash, (1 * border width) gap
      const float dash_length_per_width = 2.0;
      const float dash_gap_per_width = 1.0;
      const float dash_period_per_width = dash_length_per_width + dash_gap_per_width;

      // Since the dash size is determined by border width, the density of
      // dashes varies. Multiplying a pixel distance by this returns a
      // position in dash space - it has units (dash period / pixels). So
      // a dash velocity of (1 / 10) is 1 dash every 10 pixels.
      float dash_velocity = 0.0;

      // Dividing this by the border width gives the dash velocity
      const float dv_numerator = 1.0 / dash_period_per_width;

      if (unrounded) {
        // When corners aren't rounded, the dashes are separately laid
        // out on each straight line, rather than around the whole
        // perimeter. This way each line starts and ends with a dash.
        bool is_horizontal = corner_center_to_point.x < corner_center_to_point.y;

        // Choosing the right border width for dashed borders.
        // TODO: A better solution exists taking a look at the whole file.
        // this does not fix single dashed borders at the corners
        float2 dashed_border = float2(
        fmax(quad.border_widths.bottom, quad.border_widths.top),
        fmax(quad.border_widths.right, quad.border_widths.left));

        float border_width = is_horizontal ? dashed_border.x : dashed_border.y;
        dash_velocity = dv_numerator / border_width;
        t = is_horizontal ? point.x : point.y;
        t *= dash_velocity;
        max_t = is_horizontal ? size.x : size.y;
        max_t *= dash_velocity;
      } else {
        // When corners are rounded, the dashes are laid out clockwise
        // around the whole perimeter.

        float r_tr = quad.corner_radii.top_right;
        float r_br = quad.corner_radii.bottom_right;
        float r_bl = quad.corner_radii.bottom_left;
        float r_tl = quad.corner_radii.top_left;

        float w_t = quad.border_widths.top;
        float w_r = quad.border_widths.right;
        float w_b = quad.border_widths.bottom;
        float w_l = quad.border_widths.left;

        // Straight side dash velocities
        float dv_t = w_t <= 0.0 ? 0.0 : dv_numerator / w_t;
        float dv_r = w_r <= 0.0 ? 0.0 : dv_numerator / w_r;
        float dv_b = w_b <= 0.0 ? 0.0 : dv_numerator / w_b;
        float dv_l = w_l <= 0.0 ? 0.0 : dv_numerator / w_l;

        // Straight side lengths in dash space
        float s_t = (size.x - r_tl - r_tr) * dv_t;
        float s_r = (size.y - r_tr - r_br) * dv_r;
        float s_b = (size.x - r_br - r_bl) * dv_b;
        float s_l = (size.y - r_bl - r_tl) * dv_l;

        float corner_dash_velocity_tr = corner_dash_velocity(dv_t, dv_r);
        float corner_dash_velocity_br = corner_dash_velocity(dv_b, dv_r);
        float corner_dash_velocity_bl = corner_dash_velocity(dv_b, dv_l);
        float corner_dash_velocity_tl = corner_dash_velocity(dv_t, dv_l);

        // Corner lengths in dash space
        float c_tr = r_tr * (M_PI_F / 2.0) * corner_dash_velocity_tr;
        float c_br = r_br * (M_PI_F / 2.0) * corner_dash_velocity_br;
        float c_bl = r_bl * (M_PI_F / 2.0) * corner_dash_velocity_bl;
        float c_tl = r_tl * (M_PI_F / 2.0) * corner_dash_velocity_tl;

        // Cumulative dash space upto each segment
        float upto_tr = s_t;
        float upto_r = upto_tr + c_tr;
        float upto_br = upto_r + s_r;
        float upto_b = upto_br + c_br;
        float upto_bl = upto_b + s_b;
        float upto_l = upto_bl + c_bl;
        float upto_tl = upto_l + s_l;
        max_t = upto_tl + c_tl;

        if (is_near_rounded_corner) {
          float radians = atan2(corner_center_to_point.y, corner_center_to_point.x);
          float corner_t = radians * corner_radius;

          if (center_to_point.x >= 0.0) {
            if (center_to_point.y < 0.0) {
              dash_velocity = corner_dash_velocity_tr;
              // Subtracted because radians is pi/2 to 0 when
              // going clockwise around the top right corner,
              // since the y axis has been flipped
              t = upto_r - corner_t * dash_velocity;
            } else {
              dash_velocity = corner_dash_velocity_br;
              // Added because radians is 0 to pi/2 when going
              // clockwise around the bottom-right corner
              t = upto_br + corner_t * dash_velocity;
            }
          } else {
            if (center_to_point.y >= 0.0) {
              dash_velocity = corner_dash_velocity_bl;
              // Subtracted because radians is pi/1 to 0 when
              // going clockwise around the bottom-left corner,
              // since the x axis has been flipped
              t = upto_l - corner_t * dash_velocity;
            } else {
              dash_velocity = corner_dash_velocity_tl;
              // Added because radians is 0 to pi/2 when going
              // clockwise around the top-left corner, since both
              // axis were flipped
              t = upto_tl + corner_t * dash_velocity;
            }
          }
        } else {
          // Straight borders
          bool is_horizontal = corner_center_to_point.x < corner_center_to_point.y;
          if (is_horizontal) {
            if (center_to_point.y < 0.0) {
              dash_velocity = dv_t;
              t = (point.x - r_tl) * dash_velocity;
            } else {
              dash_velocity = dv_b;
              t = upto_bl - (point.x - r_bl) * dash_velocity;
            }
          } else {
            if (center_to_point.x < 0.0) {
              dash_velocity = dv_l;
              t = upto_tl - (point.y - r_tl) * dash_velocity;
            } else {
              dash_velocity = dv_r;
              t = upto_r + (point.y - r_tr) * dash_velocity;
            }
          }
        }
      }

      float dash_length = dash_length_per_width / dash_period_per_width;
      float desired_dash_gap = dash_gap_per_width / dash_period_per_width;

      // Straight borders should start and end with a dash, so max_t is
      // reduced to cause this.
      max_t -= unrounded ? dash_length : 0.0;
      if (max_t >= 1.0) {
        // Adjust dash gap to evenly divide max_t
        float dash_count = floor(max_t);
        float dash_period = max_t / dash_count;
        border_color.a *= dash_alpha(t, dash_period, dash_length, dash_velocity,
                                     antialias_threshold);
      } else if (unrounded) {
        // When there isn't enough space for the full gap between the
        // two start / end dashes of a straight border, reduce gap to
        // make them fit.
        float dash_gap = max_t - dash_length;
        if (dash_gap > 0.0) {
          float dash_period = dash_length + dash_gap;
          border_color.a *= dash_alpha(t, dash_period, dash_length, dash_velocity,
                                       antialias_threshold);
        }
      }
    }

    // Blend the border on top of the background and then linearly interpolate
    // between the two as we slide inside the background.
    float4 blended_border = over(background_color, border_color);
    color = mix(background_color, blended_border,
                saturate(antialias_threshold - inner_sdf));
  }

  return color * float4(1.0, 1.0, 1.0, saturate(antialias_threshold - outer_sdf));
}

// Returns the dash velocity of a corner given the dash velocity of the two
// sides, by returning the slower velocity (larger dashes).
//
// Since 0 is used for dash velocity when the border width is 0 (instead of
// +inf), this returns the other dash velocity in that case.
//
// An alternative to this might be to appropriately interpolate the dash
// velocity around the corner, but that seems overcomplicated.
float corner_dash_velocity(float dv1, float dv2) {
  if (dv1 == 0.0) {
    return dv2;
  } else if (dv2 == 0.0) {
    return dv1;
  } else {
    return min(dv1, dv2);
  }
}

// Returns alpha used to render antialiased dashes.
// `t` is within the dash when `fmod(t, period) < length`.
float dash_alpha(
    float t, float period, float length, float dash_velocity,
    float antialias_threshold) {
  float half_period = period / 2.0;
  float half_length = length / 2.0;
  // Value in [-half_period, half_period]
  // The dash is in [-half_length, half_length]
  float centered = fmod(t + half_period - half_length, period) - half_period;
  // Signed distance for the dash, negative values are inside the dash
  float signed_distance = abs(centered) - half_length;
  // Antialiased alpha based on the signed distance
  return saturate(antialias_threshold - signed_distance / dash_velocity);
}

// This approximates distance to the nearest point to a quarter ellipse in a way
// that is sufficient for anti-aliasing when the ellipse is not very eccentric.
// The components of `point` are expected to be positive.
//
// Negative on the outside and positive on the inside.
float quarter_ellipse_sdf(float2 point, float2 radii) {
  // Scale the space to treat the ellipse like a unit circle
  float2 circle_vec = point / radii;
  float unit_circle_sdf = length(circle_vec) - 1.0;
  // Approximate up-scaling of the length by using the average of the radii.
  //
  // TODO: A better solution would be to use the gradient of the implicit
  // function for an ellipse to approximate a scaling factor.
  return unit_circle_sdf * (radii.x + radii.y) * -0.5;
}

struct ShadowVertexOutput {
  float4 position [[position]];
  float4 color [[flat]];
  uint shadow_id [[flat]];
  float clip_distance [[clip_distance]][4];
};

struct ShadowFragmentInput {
  float4 position [[position]];
  float4 color [[flat]];
  uint shadow_id [[flat]];
};

vertex ShadowVertexOutput shadow_vertex(
    uint unit_vertex_id [[vertex_id]], uint shadow_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(ShadowInputIndex_Vertices)]],
    constant Shadow *shadows [[buffer(ShadowInputIndex_Shadows)]],
    constant Size_DevicePixels *viewport_size
    [[buffer(ShadowInputIndex_ViewportSize)]]) {
  float2 unit_vertex = unit_vertices[unit_vertex_id];
  Shadow shadow = shadows[shadow_id];

  Bounds_ScaledPixels bounds;
  if (shadow.inset != 0u) {
    bounds = shadow.element_bounds;
  } else {
    // Leave room for the gaussian tail outside the shadow rect.
    float margin = 3. * shadow.blur_radius;
    bounds = shadow.bounds;
    bounds.origin.x -= margin;
    bounds.origin.y -= margin;
    bounds.size.width += 2. * margin;
    bounds.size.height += 2. * margin;
  }

  float4 device_position =
      to_device_position(unit_vertex, bounds, viewport_size);
  float4 clip_distance =
      distance_from_clip_rect(unit_vertex, bounds, shadow.content_mask.bounds);
  float4 color = hsla_to_rgba(shadow.color);

  return ShadowVertexOutput{
      device_position,
      color,
      shadow_id,
      {clip_distance.x, clip_distance.y, clip_distance.z, clip_distance.w}};
}

fragment float4 shadow_fragment(ShadowFragmentInput input [[stage_in]],
                                constant Shadow *shadows
                                [[buffer(ShadowInputIndex_Shadows)]]) {
  Shadow shadow = shadows[input.shadow_id];

  float2 origin = float2(shadow.bounds.origin.x, shadow.bounds.origin.y);
  float2 size = float2(shadow.bounds.size.width, shadow.bounds.size.height);
  float2 half_size = size / 2.;
  float2 center = origin + half_size;
  float2 point = input.position.xy - center;
  float corner_radius;
  if (point.x < 0.) {
    if (point.y < 0.) {
      corner_radius = shadow.corner_radii.top_left;
    } else {
      corner_radius = shadow.corner_radii.bottom_left;
    }
  } else {
    if (point.y < 0.) {
      corner_radius = shadow.corner_radii.top_right;
    } else {
      corner_radius = shadow.corner_radii.bottom_right;
    }
  }

  float alpha;
  if (shadow.blur_radius == 0.) {
    float distance = quad_sdf(input.position.xy, shadow.bounds, shadow.corner_radii);
    alpha = saturate(0.5 - distance);
  } else {
    // The signal is only non-zero in a limited range, so don't waste samples
    float low = point.y - half_size.y;
    float high = point.y + half_size.y;
    float start = clamp(-3. * shadow.blur_radius, low, high);
    float end = clamp(3. * shadow.blur_radius, low, high);

    // Accumulate samples (we can get away with surprisingly few samples)
    float step = (end - start) / 4.;
    float y = start + step * 0.5;
    alpha = 0.;
    for (int i = 0; i < 4; i++) {
      alpha += blur_along_x(point.x, point.y - y, shadow.blur_radius,
                            corner_radius, half_size) *
               gaussian(y, shadow.blur_radius) * step;
      y += step;
    }
  }

  if (shadow.inset != 0u) {
    // The inset shadow is the complement of the (blurred) hole rect, clipped to the element.
    // `saturate(0.5 - d)` gives a 1-pixel antialiased edge: d <= -0.5 -> 1, d >= 0.5 -> 0.
    alpha = 1. - alpha;
    float element_distance = quad_sdf(input.position.xy, shadow.element_bounds,
                                      shadow.element_corner_radii);
    alpha *= saturate(0.5 - element_distance);
  }

  return input.color * float4(1., 1., 1., alpha);
}

struct UnderlineVertexOutput {
  float4 position [[position]];
  float4 color [[flat]];
  uint underline_id [[flat]];
  float clip_distance [[clip_distance]][4];
};

struct UnderlineFragmentInput {
  float4 position [[position]];
  float4 color [[flat]];
  uint underline_id [[flat]];
};

vertex UnderlineVertexOutput underline_vertex(
    uint unit_vertex_id [[vertex_id]], uint underline_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(UnderlineInputIndex_Vertices)]],
    constant Underline *underlines [[buffer(UnderlineInputIndex_Underlines)]],
    constant Size_DevicePixels *viewport_size
    [[buffer(ShadowInputIndex_ViewportSize)]]) {
  float2 unit_vertex = unit_vertices[unit_vertex_id];
  Underline underline = underlines[underline_id];
  float4 device_position =
      to_device_position(unit_vertex, underline.bounds, viewport_size);
  float4 clip_distance = distance_from_clip_rect(unit_vertex, underline.bounds,
                                                 underline.content_mask.bounds);
  float4 color = hsla_to_rgba(underline.color);
  return UnderlineVertexOutput{
      device_position,
      color,
      underline_id,
      {clip_distance.x, clip_distance.y, clip_distance.z, clip_distance.w}};
}

fragment float4 underline_fragment(UnderlineFragmentInput input [[stage_in]],
                                   constant Underline *underlines
                                   [[buffer(UnderlineInputIndex_Underlines)]]) {
  const float WAVE_FREQUENCY = 2.0;
  const float WAVE_HEIGHT_RATIO = 0.8;

  Underline underline = underlines[input.underline_id];
  if (underline.wavy) {
    float half_thickness = underline.thickness * 0.5;
    float2 origin =
        float2(underline.bounds.origin.x, underline.bounds.origin.y);

    float2 st = ((input.position.xy - origin) / underline.bounds.size.height) -
                float2(0., 0.5);
    float frequency = (M_PI_F * WAVE_FREQUENCY * underline.thickness) / underline.bounds.size.height;
    float amplitude = (underline.thickness * WAVE_HEIGHT_RATIO) / underline.bounds.size.height;

    float sine = sin(st.x * frequency) * amplitude;
    float dSine = cos(st.x * frequency) * amplitude * frequency;
    float distance = (st.y - sine) / sqrt(1. + dSine * dSine);
    float distance_in_pixels = distance * underline.bounds.size.height;
    float distance_from_top_border = distance_in_pixels - half_thickness;
    float distance_from_bottom_border = distance_in_pixels + half_thickness;
    float alpha = saturate(
        0.5 - max(-distance_from_bottom_border, distance_from_top_border));
    return input.color * float4(1., 1., 1., alpha);
  } else {
    return input.color;
  }
}

struct MonochromeSpriteVertexOutput {
  float4 position [[position]];
  float2 tile_position;
  float4 color [[flat]];
  float4 clip_distance;
};

struct MonochromeSpriteFragmentInput {
  float4 position [[position]];
  float2 tile_position;
  float4 color [[flat]];
  float4 clip_distance;
};

vertex MonochromeSpriteVertexOutput monochrome_sprite_vertex(
    uint unit_vertex_id [[vertex_id]], uint sprite_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(SpriteInputIndex_Vertices)]],
    constant MonochromeSprite *sprites [[buffer(SpriteInputIndex_Sprites)]],
    constant Size_DevicePixels *viewport_size
    [[buffer(SpriteInputIndex_ViewportSize)]],
    constant Size_DevicePixels *atlas_size
    [[buffer(SpriteInputIndex_AtlasTextureSize)]]) {
  float2 unit_vertex = unit_vertices[unit_vertex_id];
  MonochromeSprite sprite = sprites[sprite_id];
  float4 device_position =
      to_device_position_transformed(unit_vertex, sprite.bounds, sprite.transformation, viewport_size);
  float4 clip_distance = distance_from_clip_rect_transformed(unit_vertex, sprite.bounds,
                                                 sprite.content_mask.bounds, sprite.transformation);
  float2 tile_position = to_tile_position(unit_vertex, sprite.tile, atlas_size);
  float4 color = hsla_to_rgba(sprite.color);
  return MonochromeSpriteVertexOutput{
      device_position,
      tile_position,
      color,
      {clip_distance.x, clip_distance.y, clip_distance.z, clip_distance.w}};
}

fragment float4 monochrome_sprite_fragment(
    MonochromeSpriteFragmentInput input [[stage_in]],
    constant MonochromeSprite *sprites [[buffer(SpriteInputIndex_Sprites)]],
    texture2d<float> atlas_texture [[texture(SpriteInputIndex_AtlasTexture)]]) {
  if (any(input.clip_distance < float4(0.0))) {
    return float4(0.0);
  }

  constexpr sampler atlas_texture_sampler(mag_filter::linear,
                                          min_filter::linear);
  float4 sample =
      atlas_texture.sample(atlas_texture_sampler, input.tile_position);
  float4 color = input.color;
  color.a *= sample.a;
  return color;
}

struct PolychromeSpriteVertexOutput {
  float4 position [[position]];
  float2 tile_position;
  uint sprite_id [[flat]];
  float clip_distance [[clip_distance]][4];
};

struct PolychromeSpriteFragmentInput {
  float4 position [[position]];
  float2 tile_position;
  uint sprite_id [[flat]];
};

vertex PolychromeSpriteVertexOutput polychrome_sprite_vertex(
    uint unit_vertex_id [[vertex_id]], uint sprite_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(SpriteInputIndex_Vertices)]],
    constant PolychromeSprite *sprites [[buffer(SpriteInputIndex_Sprites)]],
    constant Size_DevicePixels *viewport_size
    [[buffer(SpriteInputIndex_ViewportSize)]],
    constant Size_DevicePixels *atlas_size
    [[buffer(SpriteInputIndex_AtlasTextureSize)]]) {

  float2 unit_vertex = unit_vertices[unit_vertex_id];
  PolychromeSprite sprite = sprites[sprite_id];
  float4 device_position =
      to_device_position(unit_vertex, sprite.bounds, viewport_size);
  float4 clip_distance = distance_from_clip_rect(unit_vertex, sprite.bounds,
                                                 sprite.content_mask.bounds);
  float2 tile_position = to_tile_position(unit_vertex, sprite.tile, atlas_size);
  return PolychromeSpriteVertexOutput{
      device_position,
      tile_position,
      sprite_id,
      {clip_distance.x, clip_distance.y, clip_distance.z, clip_distance.w}};
}

fragment float4 polychrome_sprite_fragment(
    PolychromeSpriteFragmentInput input [[stage_in]],
    constant PolychromeSprite *sprites [[buffer(SpriteInputIndex_Sprites)]],
    texture2d<float> atlas_texture [[texture(SpriteInputIndex_AtlasTexture)]]) {
  PolychromeSprite sprite = sprites[input.sprite_id];
  constexpr sampler atlas_texture_sampler(mag_filter::linear,
                                          min_filter::linear);
  float4 sample =
      atlas_texture.sample(atlas_texture_sampler, input.tile_position);
  float distance =
      quad_sdf(input.position.xy, sprite.bounds, sprite.corner_radii);

  float4 color = sample;
  if (sprite.grayscale) {
    float grayscale = 0.2126 * color.r + 0.7152 * color.g + 0.0722 * color.b;
    color.r = grayscale;
    color.g = grayscale;
    color.b = grayscale;
  }
  color.a *= sprite.opacity * saturate(0.5 - distance);
  return color;
}

struct PathRasterizationVertexOutput {
  float4 position [[position]];
  float2 st_position;
  uint vertex_id [[flat]];
  float clip_rect_distance [[clip_distance]][4];
};

struct PathRasterizationFragmentInput {
  float4 position [[position]];
  float2 st_position;
  uint vertex_id [[flat]];
};

vertex PathRasterizationVertexOutput path_rasterization_vertex(
  uint vertex_id [[vertex_id]],
  constant PathRasterizationVertex *vertices [[buffer(PathRasterizationInputIndex_Vertices)]],
  constant Size_DevicePixels *atlas_size [[buffer(PathRasterizationInputIndex_ViewportSize)]]
) {
  PathRasterizationVertex v = vertices[vertex_id];
  float2 vertex_position = float2(v.xy_position.x, v.xy_position.y);
  float4 position = float4(
    vertex_position * float2(2. / atlas_size->width, -2. / atlas_size->height) + float2(-1., 1.),
    0.,
    1.
  );
  return PathRasterizationVertexOutput{
      position,
      float2(v.st_position.x, v.st_position.y),
      vertex_id,
      {
        v.xy_position.x - v.bounds.origin.x,
        v.bounds.origin.x + v.bounds.size.width - v.xy_position.x,
        v.xy_position.y - v.bounds.origin.y,
        v.bounds.origin.y + v.bounds.size.height - v.xy_position.y
      }
  };
}

fragment float4 path_rasterization_fragment(
  PathRasterizationFragmentInput input [[stage_in]],
  constant PathRasterizationVertex *vertices [[buffer(PathRasterizationInputIndex_Vertices)]]
) {
  float2 dx = dfdx(input.st_position);
  float2 dy = dfdy(input.st_position);

  PathRasterizationVertex v = vertices[input.vertex_id];
  Background background = v.color;
  Bounds_ScaledPixels path_bounds = v.bounds;
  float alpha;
  if (length(float2(dx.x, dy.x)) < 0.001) {
    alpha = 1.0;
  } else {
    float2 gradient = float2(
      (2. * input.st_position.x) * dx.x - dx.y,
      (2. * input.st_position.x) * dy.x - dy.y
    );
    float f = (input.st_position.x * input.st_position.x) - input.st_position.y;
    float distance = f / length(gradient);
    alpha = saturate(0.5 - distance);
  }

  float4 color = fill_color(
    background,
    input.position.xy,
    path_bounds,
    prepare_fill_color(background)
  );
  return float4(color.rgb * color.a * alpha, alpha * color.a);
}

struct PathSpriteVertexOutput {
  float4 position [[position]];
  float2 texture_coords;
};

vertex PathSpriteVertexOutput path_sprite_vertex(
  uint unit_vertex_id [[vertex_id]],
  uint sprite_id [[instance_id]],
  constant float2 *unit_vertices [[buffer(SpriteInputIndex_Vertices)]],
  constant PathSprite *sprites [[buffer(SpriteInputIndex_Sprites)]],
  constant Size_DevicePixels *viewport_size [[buffer(SpriteInputIndex_ViewportSize)]]
) {
  float2 unit_vertex = unit_vertices[unit_vertex_id];
  PathSprite sprite = sprites[sprite_id];
  // Don't apply content mask because it was already accounted for when
  // rasterizing the path.
  float4 device_position =
      to_device_position(unit_vertex, sprite.bounds, viewport_size);

  float2 screen_position = float2(sprite.bounds.origin.x, sprite.bounds.origin.y) + unit_vertex * float2(sprite.bounds.size.width, sprite.bounds.size.height);
  float2 texture_coords = screen_position / float2(viewport_size->width, viewport_size->height);

  return PathSpriteVertexOutput{
    device_position,
    texture_coords
  };
}

fragment float4 path_sprite_fragment(
  PathSpriteVertexOutput input [[stage_in]],
  texture2d<float> intermediate_texture [[texture(SpriteInputIndex_AtlasTexture)]]
) {
  constexpr sampler intermediate_texture_sampler(mag_filter::linear, min_filter::linear);
  return intermediate_texture.sample(intermediate_texture_sampler, input.texture_coords);
}

struct SurfaceVertexOutput {
  float4 position [[position]];
  float2 texture_position;
  float clip_distance [[clip_distance]][4];
};

struct SurfaceFragmentInput {
  float4 position [[position]];
  float2 texture_position;
};

vertex SurfaceVertexOutput surface_vertex(
    uint unit_vertex_id [[vertex_id]], uint surface_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(SurfaceInputIndex_Vertices)]],
    constant SurfaceBounds *surfaces [[buffer(SurfaceInputIndex_Surfaces)]],
    constant Size_DevicePixels *viewport_size
    [[buffer(SurfaceInputIndex_ViewportSize)]],
    constant Size_DevicePixels *texture_size
    [[buffer(SurfaceInputIndex_TextureSize)]]) {
  float2 unit_vertex = unit_vertices[unit_vertex_id];
  SurfaceBounds surface = surfaces[surface_id];
  float4 device_position =
      to_device_position(unit_vertex, surface.bounds, viewport_size);
  float4 clip_distance = distance_from_clip_rect(unit_vertex, surface.bounds,
                                                 surface.content_mask.bounds);
  // We are going to copy the whole texture, so the texture position corresponds
  // to the current vertex of the unit triangle.
  float2 texture_position = unit_vertex;
  return SurfaceVertexOutput{
      device_position,
      texture_position,
      {clip_distance.x, clip_distance.y, clip_distance.z, clip_distance.w}};
}

fragment float4 surface_fragment(SurfaceFragmentInput input [[stage_in]],
                                 texture2d<float> y_texture
                                 [[texture(SurfaceInputIndex_YTexture)]],
                                 texture2d<float> cb_cr_texture
                                 [[texture(SurfaceInputIndex_CbCrTexture)]]) {
  constexpr sampler texture_sampler(mag_filter::linear, min_filter::linear);
  const float4x4 ycbcrToRGBTransform =
      float4x4(float4(+1.0000f, +1.0000f, +1.0000f, +0.0000f),
               float4(+0.0000f, -0.3441f, +1.7720f, +0.0000f),
               float4(+1.4020f, -0.7141f, +0.0000f, +0.0000f),
               float4(-0.7010f, +0.5291f, -0.8860f, +1.0000f));
  float4 ycbcr = float4(
      y_texture.sample(texture_sampler, input.texture_position).r,
      cb_cr_texture.sample(texture_sampler, input.texture_position).rg, 1.0);

  return ycbcrToRGBTransform * ycbcr;
}

float4 hsla_to_rgba(Hsla hsla) {
  float h = hsla.h * 6.0; // Now, it's an angle but scaled in [0, 6) range
  float s = hsla.s;
  float l = hsla.l;
  float a = hsla.a;

  float c = (1.0 - fabs(2.0 * l - 1.0)) * s;
  float x = c * (1.0 - fabs(fmod(h, 2.0) - 1.0));
  float m = l - c / 2.0;

  float r = 0.0;
  float g = 0.0;
  float b = 0.0;

  if (h >= 0.0 && h < 1.0) {
    r = c;
    g = x;
    b = 0.0;
  } else if (h >= 1.0 && h < 2.0) {
    r = x;
    g = c;
    b = 0.0;
  } else if (h >= 2.0 && h < 3.0) {
    r = 0.0;
    g = c;
    b = x;
  } else if (h >= 3.0 && h < 4.0) {
    r = 0.0;
    g = x;
    b = c;
  } else if (h >= 4.0 && h < 5.0) {
    r = x;
    g = 0.0;
    b = c;
  } else {
    r = c;
    g = 0.0;
    b = x;
  }

  float4 rgba;
  rgba.x = (r + m);
  rgba.y = (g + m);
  rgba.z = (b + m);
  rgba.w = a;
  return rgba;
}

float3 srgb_to_linear(float3 color) {
  return pow(color, float3(2.2));
}

float3 linear_to_srgb(float3 color) {
  return pow(color, float3(1.0 / 2.2));
}

// Converts a sRGB color to the Oklab color space.
// Reference: https://bottosson.github.io/posts/oklab/#converting-from-linear-srgb-to-oklab
float4 srgb_to_oklab(float4 color) {
  // Convert non-linear sRGB to linear sRGB
  color = float4(srgb_to_linear(color.rgb), color.a);

  float l = 0.4122214708 * color.r + 0.5363325363 * color.g + 0.0514459929 * color.b;
  float m = 0.2119034982 * color.r + 0.6806995451 * color.g + 0.1073969566 * color.b;
  float s = 0.0883024619 * color.r + 0.2817188376 * color.g + 0.6299787005 * color.b;

  float l_ = pow(l, 1.0/3.0);
  float m_ = pow(m, 1.0/3.0);
  float s_ = pow(s, 1.0/3.0);

  return float4(
   	0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
   	1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
   	0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
   	color.a
  );
}

// Converts an Oklab color to the sRGB color space.
float4 oklab_to_srgb(float4 color) {
  float l_ = color.r + 0.3963377774 * color.g + 0.2158037573 * color.b;
  float m_ = color.r - 0.1055613458 * color.g - 0.0638541728 * color.b;
  float s_ = color.r - 0.0894841775 * color.g - 1.2914855480 * color.b;

  float l = l_ * l_ * l_;
  float m = m_ * m_ * m_;
  float s = s_ * s_ * s_;

  float3 linear_rgb = float3(
   	4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
   	-1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
   	-0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s
  );

  // Convert linear sRGB to non-linear sRGB
  return float4(linear_to_srgb(linear_rgb), color.a);
}

float4 to_device_position(float2 unit_vertex, Bounds_ScaledPixels bounds,
                          constant Size_DevicePixels *input_viewport_size) {
  float2 position =
      unit_vertex * float2(bounds.size.width, bounds.size.height) +
      float2(bounds.origin.x, bounds.origin.y);
  float2 viewport_size = float2((float)input_viewport_size->width,
                                (float)input_viewport_size->height);
  float2 device_position =
      position / viewport_size * float2(2., -2.) + float2(-1., 1.);
  return float4(device_position, 0., 1.);
}

float4 to_device_position_transformed(float2 unit_vertex, Bounds_ScaledPixels bounds,
                          TransformationMatrix transformation,
                          constant Size_DevicePixels *input_viewport_size) {
  float2 position =
      unit_vertex * float2(bounds.size.width, bounds.size.height) +
      float2(bounds.origin.x, bounds.origin.y);

  // Apply the transformation matrix to the position via matrix multiplication.
  float2 transformed_position = float2(0, 0);
  transformed_position[0] = position[0] * transformation.rotation_scale[0][0] + position[1] * transformation.rotation_scale[0][1];
  transformed_position[1] = position[0] * transformation.rotation_scale[1][0] + position[1] * transformation.rotation_scale[1][1];

  // Add in the translation component of the transformation matrix.
  transformed_position[0] += transformation.translation[0];
  transformed_position[1] += transformation.translation[1];

  float2 viewport_size = float2((float)input_viewport_size->width,
                                (float)input_viewport_size->height);
  float2 device_position =
      transformed_position / viewport_size * float2(2., -2.) + float2(-1., 1.);
  return float4(device_position, 0., 1.);
}


float2 to_tile_position(float2 unit_vertex, AtlasTile tile,
                        constant Size_DevicePixels *atlas_size) {
  float2 tile_origin = float2(tile.bounds.origin.x, tile.bounds.origin.y);
  float2 tile_size = float2(tile.bounds.size.width, tile.bounds.size.height);
  return (tile_origin + unit_vertex * tile_size) /
         float2((float)atlas_size->width, (float)atlas_size->height);
}

// Selects corner radius based on quadrant.
float pick_corner_radius(float2 center_to_point, Corners_ScaledPixels corner_radii) {
  if (center_to_point.x < 0.) {
    if (center_to_point.y < 0.) {
      return corner_radii.top_left;
    } else {
      return corner_radii.bottom_left;
    }
  } else {
    if (center_to_point.y < 0.) {
      return corner_radii.top_right;
    } else {
      return corner_radii.bottom_right;
    }
  }
}

float pick_corner_shape(float2 center_to_point, Corners_f32 corner_shapes) {
  if (center_to_point.x < 0.) {
    if (center_to_point.y < 0.) {
      return corner_shapes.top_left;
    } else {
      return corner_shapes.bottom_left;
    }
  } else {
    if (center_to_point.y < 0.) {
      return corner_shapes.top_right;
    } else {
      return corner_shapes.bottom_right;
    }
  }
}

// Signed distance from a point in the positive quadrant to the superellipse
// (x/a)^n + (y/b)^n = 1. Negative inside. n is 1 for a straight line between
// the axes, 2 for an ellipse, and infinity for the a by b box.
//
// The distance is the value of the implicit function over the length of its
// gradient, which is exact for a line and a circle and close elsewhere. The
// terms are scaled by the largest coordinate first so a big n never
// underflows the whole sum to zero.
float superellipse_sdf(float2 point, float2 radii, float n) {
  if (isinf(n)) {
    float2 to_edge = point - radii;
    return max(to_edge.x, to_edge.y);
  }
  float2 unit = point / radii;
  float largest = max(max(unit.x, unit.y), 1e-6);
  float2 scaled = unit / largest;
  float rho = largest * pow(pow(scaled.x, n) + pow(scaled.y, n), 1.0 / n);
  float2 gradient = pow(scaled * (largest / rho), n - 1.0) / radii;
  return (rho - 1.0) / max(length(gradient), 1e-6);
}

// `inset.x` is the width of the vertical edge and moves `end`. `inset.y` is
// the width of the horizontal edge and moves `start`.
CornerCurve corner_curve(float corner_radius, float shape, float2 inset) {
  float half_corner = pow(0.5, 1.0 / exp2(fabs(shape)));
  if (shape < 0.0) {
    half_corner = 1.0 - half_corner;
  }
  float control =
    clamp(half_corner / (sqrt(2.0) - 1.0) - 1.0 / sqrt(2.0), 0.0, 1.0);
  float start_control = control;
  float inset_diff = clamp(inset.x - inset.y, -corner_radius, corner_radius);
  if (inset_diff != 0.0) {
    float s = sqrt(2.0 * corner_radius * corner_radius - inset_diff * inset_diff);
    float bevel_control = (s - inset_diff) / (2.0 * s);
    start_control = shape < 0.0
      ? bevel_control * 2.0 * control
      : 1.0 - (1.0 - bevel_control) * 2.0 * (1.0 - control);
  }
  float end_control = 2.0 * control - start_control;
  CornerCurve curve;
  curve.start_normal = normalize(float2(1.0 - start_control, start_control));
  curve.end_normal = normalize(float2(end_control, 1.0 - end_control));
  curve.start = float2(corner_radius, 0.0) + inset.y * curve.start_normal;
  curve.end = float2(0.0, corner_radius) + inset.x * curve.end_normal;
  return curve;
}

// Signed distance from a point, given as distances from the corner along the
// two edges, to the curve. Positive on the corner side, which is outside the
// box. `straight` is the distance to the two straight edges and wins where
// the curve does not reach.
float corner_curve_sdf(float2 from_corner, CornerCurve curve, float shape,
                       float straight) {
  float n = exp2(fabs(shape));
  if (shape >= 0.0) {
    float2 center = float2(curve.start.x, curve.end.y);
    float2 radii = float2(center.x - curve.end.x, center.y - curve.start.y);
    float2 center_to_point = center - from_corner;
    if (center_to_point.x < 0.0 || center_to_point.y < 0.0 ||
        radii.x <= 0.0 || radii.y <= 0.0) {
      return straight;
    }
    return max(straight, superellipse_sdf(center_to_point, radii, n));
  }
  if (shape <= -1.0) {
    float2 origin = float2(curve.end.x, curve.start.y);
    float2 radii = float2(curve.start.x - origin.x, curve.end.y - origin.y);
    float2 origin_to_point = max(from_corner - origin, 0.0);
    return max(straight, -superellipse_sdf(origin_to_point, radii, n));
  }
  // Between a bevel and a scoop the spec draws a quarter circle mapped into
  // the frame of the two ends and the point where their tangents meet.
  float2 start_tangent = float2(-curve.start_normal.y, curve.start_normal.x);
  float2 end_tangent = float2(-curve.end_normal.y, curve.end_normal.x);
  float2 start_to_end = curve.end - curve.start;
  float tangents_cross =
    start_tangent.x * end_tangent.y - start_tangent.y * end_tangent.x;
  float2 meet = curve.start;
  if (fabs(tangents_cross) > 1e-6) {
    float t = (start_to_end.x * end_tangent.y - start_to_end.y * end_tangent.x) /
              tangents_cross;
    meet = curve.start + t * start_tangent;
  }
  float2 to_end = curve.end - meet;
  float2 to_start = curve.start - meet;
  float det = to_end.x * to_start.y - to_end.y * to_start.x;
  if (fabs(det) < 1e-6) {
    return straight;
  }
  float2 d = from_corner - meet;
  float x = (d.x * to_start.y - d.y * to_start.x) / det;
  float y = (to_end.x * d.y - to_end.y * d.x) / det;
  float2 unit = 1.0 - float2(x, y);
  float f = dot(unit, unit) - 1.0;
  float2 g = -2.0 * unit;
  float2 gradient = float2(to_start.y * g.x - to_end.y * g.y,
                           to_end.x * g.y - to_start.x * g.x) / det;
  return max(straight, -f / max(length(gradient), 1e-6));
}

// The outer and inner signed distances for one corner whose shape is not a
// plain quarter circle. `shape` is the CSS superellipse curvature.
float2 shaped_corner_sdf(float2 corner_to_point, float corner_radius,
                         float shape, float2 reduced_border,
                         float2 straight_border_inner_corner_to_point) {
  float2 from_corner = -corner_to_point;
  CornerCurve outer_curve = corner_curve(corner_radius, shape, float2(0.0));
  CornerCurve inner_curve = corner_curve(corner_radius, shape, reduced_border);
  float straight_outer = max(corner_to_point.x, corner_to_point.y);
  float straight_inner = max(straight_border_inner_corner_to_point.x,
                             straight_border_inner_corner_to_point.y);
  return float2(
    corner_curve_sdf(from_corner, outer_curve, shape, straight_outer),
    -corner_curve_sdf(from_corner, inner_curve, shape, straight_inner));
}

// Signed distance of the point to the quad's border - positive outside the
// border, and negative inside.
float quad_sdf(float2 point, Bounds_ScaledPixels bounds,
               Corners_ScaledPixels corner_radii) {
    float2 half_size = float2(bounds.size.width, bounds.size.height) / 2.0;
    float2 center = float2(bounds.origin.x, bounds.origin.y) + half_size;
    float2 center_to_point = point - center;
    float corner_radius = pick_corner_radius(center_to_point, corner_radii);
    float2 corner_to_point = fabs(center_to_point) - half_size;
    float2 corner_center_to_point = corner_to_point + corner_radius;
    return quad_sdf_impl(corner_center_to_point, corner_radius);
}

// Implementation of quad signed distance field
float quad_sdf_impl(float2 corner_center_to_point, float corner_radius) {
    if (corner_radius == 0.0) {
        // Fast path for unrounded corners
        return max(corner_center_to_point.x, corner_center_to_point.y);
    } else {
        // Signed distance of the point from a quad that is inset by corner_radius
        // It is negative inside this quad, and positive outside
        float signed_distance_to_inset_quad =
            // 0 inside the inset quad, and positive outside
            length(max(float2(0.0), corner_center_to_point)) +
            // 0 outside the inset quad, and negative inside
            min(0.0, max(corner_center_to_point.x, corner_center_to_point.y));

        return signed_distance_to_inset_quad - corner_radius;
    }
}

// A standard gaussian function, used for weighting samples
float gaussian(float x, float sigma) {
  return exp(-(x * x) / (2. * sigma * sigma)) / (sqrt(2. * M_PI_F) * sigma);
}

// This approximates the error function, needed for the gaussian integral
float2 erf(float2 x) {
  float2 s = sign(x);
  float2 a = abs(x);
  float2 r1 = 1. + (0.278393 + (0.230389 + (0.000972 + 0.078108 * a) * a) * a) * a;
  float2 r2 = r1 * r1;
  return s - s / (r2 * r2);
}

float blur_along_x(float x, float y, float sigma, float corner,
                   float2 half_size) {
  float delta = min(half_size.y - corner - abs(y), 0.);
  float curved =
      half_size.x - corner + sqrt(max(0., corner * corner - delta * delta));
  float2 integral =
      0.5 + 0.5 * erf((x + float2(-curved, curved)) * (sqrt(0.5) / sigma));
  return integral.y - integral.x;
}

float4 distance_from_clip_rect(float2 unit_vertex, Bounds_ScaledPixels bounds,
                               Bounds_ScaledPixels clip_bounds) {
  float2 position =
      unit_vertex * float2(bounds.size.width, bounds.size.height) +
      float2(bounds.origin.x, bounds.origin.y);
  return float4(position.x - clip_bounds.origin.x,
                clip_bounds.origin.x + clip_bounds.size.width - position.x,
                position.y - clip_bounds.origin.y,
                clip_bounds.origin.y + clip_bounds.size.height - position.y);
}

float4 distance_from_clip_rect_transformed(float2 unit_vertex, Bounds_ScaledPixels bounds,
                               Bounds_ScaledPixels clip_bounds, TransformationMatrix transformation) {
  float2 position =
      unit_vertex * float2(bounds.size.width, bounds.size.height) +
      float2(bounds.origin.x, bounds.origin.y);
  float2 transformed_position = float2(0, 0);
  transformed_position[0] = position[0] * transformation.rotation_scale[0][0] + position[1] * transformation.rotation_scale[0][1];
  transformed_position[1] = position[0] * transformation.rotation_scale[1][0] + position[1] * transformation.rotation_scale[1][1];
  transformed_position[0] += transformation.translation[0];
  transformed_position[1] += transformation.translation[1];

  return float4(transformed_position.x - clip_bounds.origin.x,
                clip_bounds.origin.x + clip_bounds.size.width - transformed_position.x,
                transformed_position.y - clip_bounds.origin.y,
                clip_bounds.origin.y + clip_bounds.size.height - transformed_position.y);
}

float4 over(float4 below, float4 above) {
  float4 result;
  float alpha = above.a + below.a * (1.0 - above.a);
  result.rgb =
      (above.rgb * above.a + below.rgb * below.a * (1.0 - above.a)) / alpha;
  result.a = alpha;
  return result;
}

// The solid color of a fill, converted once per vertex. Gradients convert
// their stops per fragment instead, since only two of them matter there.
float4 prepare_fill_color(Background background) {
  if (background.tag == 1) {
    return float4(0.0);
  }
  return hsla_to_rgba(background.solid);
}


// One gradient stop in the space the gradient mixes in.
float4 gradient_stop_color(Background background, uint index) {
  float4 color = hsla_to_rgba(background.colors[index].color);
  if (background.color_space == 1) {
    color = srgb_to_oklab(color);
  }
  return color;
}

// The color of a CSS linear gradient at `position`.
//
// The gradient line goes through the center of the box. Its length is the
// one CSS Images 3 defines, so 0% and 100% sit exactly on the corners the
// line points away from and toward. A corner keyword makes the line
// perpendicular to the diagonal between the two other corners.
float4 linear_gradient_color(Background background, float2 position,
                             Bounds_ScaledPixels bounds) {
  float2 size = float2(bounds.size.width, bounds.size.height);
  float angle;
  if (background.corner == 0) {
    angle = background.gradient_angle_or_pattern_height * (M_PI_F / 180.0);
  } else {
    float toward_top_right = atan2(size.y, size.x);
    switch (background.corner) {
      case 1: angle = 2.0 * M_PI_F - toward_top_right; break;
      case 2: angle = toward_top_right; break;
      case 3: angle = M_PI_F - toward_top_right; break;
      default: angle = M_PI_F + toward_top_right; break;
    }
  }
  float2 direction = float2(sin(angle), -cos(angle));
  float line_length = abs(size.x * sin(angle)) + abs(size.y * cos(angle));
  float2 center = float2(bounds.origin.x, bounds.origin.y) + size / 2.0;
  float t = (dot(position - center, direction) + line_length / 2.0)
    / max(line_length, 1e-6);

  // A count outside 1 to 8 can only come from a hand-built struct. Clamp it
  // so the array read stays in bounds.
  uint last = clamp(background.stop_count, 1u, 8u) - 1;
  float4 color;
  if (t <= background.colors[0].percentage) {
    color = gradient_stop_color(background, 0);
  } else if (t >= background.colors[last].percentage) {
    color = gradient_stop_color(background, last);
  } else {
    uint i = 0;
    while (i + 1 < last && t > background.colors[i + 1].percentage) {
      i++;
    }
    float start = background.colors[i].percentage;
    float end = background.colors[i + 1].percentage;
    float p = end > start ? (t - start) / (end - start) : 1.0;
    // A color hint moves the half-way point of the mix between two stops.
    float hint = background.colors[i].hint;
    if (hint > 0.0 && hint < 1.0) {
      p = pow(p, log(0.5) / log(hint));
    }
    color = mix(gradient_stop_color(background, i),
                gradient_stop_color(background, i + 1), p);
  }
  if (background.color_space == 1) {
    color = oklab_to_srgb(color);
  }

  // Dither to reduce banding in gradients (especially dark/alpha).
  // Triangular-distributed noise breaks up 8-bit quantization steps.
  // ±2/255 for RGB (enough for dark-on-dark compositing),
  // ±3/255 for alpha (needs more because alpha × dark color = tiny steps).
  float2 seed = position * 0.6180339887; // golden ratio spread
  float r1 = fract(sin(dot(seed, float2(12.9898, 78.233))) * 43758.5453);
  float r2 = fract(sin(dot(seed, float2(39.3460, 11.135))) * 24634.6345);
  float tri = r1 + r2 - 1.0; // triangular PDF, range [-1, +1]
  color.rgb += tri * 2.0 / 255.0;
  color.a   += tri * 3.0 / 255.0;
  return color;
}

float2x2 rotate2d(float angle) {
    float s = sin(angle);
    float c = cos(angle);
    return float2x2(c, -s, s, c);
}

float4 fill_color(Background background,
                      float2 position,
                      Bounds_ScaledPixels bounds,
                      float4 solid_color) {
  float4 color;

  switch (background.tag) {
    case 0:
      color = solid_color;
      break;
    case 1:
      color = linear_gradient_color(background, position, bounds);
      break;
    case 2: {
        float gradient_angle_or_pattern_height = background.gradient_angle_or_pattern_height;
        float pattern_width = (gradient_angle_or_pattern_height / 65535.0f) / 255.0f;
        float pattern_interval = fmod(gradient_angle_or_pattern_height, 65535.0f) / 255.0f;
        float pattern_height = pattern_width + pattern_interval;
        float stripe_angle = M_PI_F / 4.0;
        float pattern_period = pattern_height * sin(stripe_angle);
        float2x2 rotation = rotate2d(stripe_angle);
        float2 relative_position = position - float2(bounds.origin.x, bounds.origin.y);
        float2 rotated_point = rotation * relative_position;
        float pattern = fmod(rotated_point.x, pattern_period);
        float distance = min(pattern, pattern_period - pattern) - pattern_period * (pattern_width / pattern_height) /  2.0f;
        color = solid_color;
        color.a *= saturate(0.5 - distance);
        break;
    }
    case 3: {
        // checkerboard
        float size = background.gradient_angle_or_pattern_height;
        float2 relative_position = position - float2(bounds.origin.x, bounds.origin.y);

        float x_index = floor(relative_position.x / size);
        float y_index = floor(relative_position.y / size);
        float should_be_colored = fmod(x_index + y_index, 2.0);

        color = solid_color;
        color.a *= saturate(should_be_colored);
        break;
    }
  }

  return color;
}

// Effect layers. The renderer draws a subtree into a texture, then these
// shaders blur it and paint it over the frame with a colour matrix, a mask
// and a blend mode. Every texture here holds premultiplied colour.

struct BlurVertexOutput {
  float4 position [[position]];
  float2 uv;
};

vertex BlurVertexOutput blur_vertex(uint unit_vertex_id [[vertex_id]],
                                    constant float2 *unit_vertices
                                    [[buffer(BlurInputIndex_Vertices)]]) {
  float2 unit_vertex = unit_vertices[unit_vertex_id];
  return BlurVertexOutput{
      float4(unit_vertex.x * 2.0 - 1.0, 1.0 - unit_vertex.y * 2.0, 0.0, 1.0),
      unit_vertex};
}

fragment float4 blur_fragment(BlurVertexOutput input [[stage_in]],
                              constant BlurParams *params
                              [[buffer(BlurInputIndex_Params)]],
                              texture2d<float> source
                              [[texture(BlurInputIndex_Source)]]) {
  constexpr sampler edge_sampler(mag_filter::linear, min_filter::linear,
                                 address::clamp_to_edge);
  float2 step = float2(params->step[0], params->step[1]);
  float sigma = max(params->sigma, 1e-3);
  int radius = params->radius;
  float4 sum = float4(0.0);
  float weight_sum = 0.0;
  for (int i = -radius; i <= radius; i++) {
    float2 uv = input.uv + step * float(i);
    if (any(uv < 0.0) || any(uv > 1.0)) {
      continue;
    }
    float weight = exp(-0.5 * float(i * i) / (sigma * sigma));
    sum += weight * source.sample(edge_sampler, uv);
    weight_sum += weight;
  }
  return sum / weight_sum;
}

// A row-major 4 by 5 matrix on straight (not premultiplied) rgba.
float4 apply_color_matrix(constant float *m, float4 c) {
  return float4(
      m[0] * c.r + m[1] * c.g + m[2] * c.b + m[3] * c.a + m[4],
      m[5] * c.r + m[6] * c.g + m[7] * c.b + m[8] * c.a + m[9],
      m[10] * c.r + m[11] * c.g + m[12] * c.b + m[13] * c.a + m[14],
      m[15] * c.r + m[16] * c.g + m[17] * c.b + m[18] * c.a + m[19]);
}

float4 unpremultiply(float4 c) {
  return c.a > 0.0 ? float4(c.rgb / c.a, c.a) : float4(0.0);
}

float4 premultiply(float4 c) {
  return float4(c.rgb * c.a, c.a);
}

// Runs a premultiplied colour through a matrix, clamped, premultiplied again.
float4 filter_color(constant float *m, float4 premultiplied) {
  float4 c = apply_color_matrix(m, unpremultiply(premultiplied));
  return premultiply(saturate(c));
}

// Compositing and Blending 1 blend functions on straight colours, with the
// backdrop first and the source second.
float lum(float3 c) { return dot(c, float3(0.3, 0.59, 0.11)); }

float3 clip_color(float3 c) {
  float l = lum(c);
  float n = min(c.r, min(c.g, c.b));
  float x = max(c.r, max(c.g, c.b));
  if (n < 0.0) {
    c = l + (c - l) * l / max(l - n, 1e-5);
  }
  if (x > 1.0) {
    c = l + (c - l) * (1.0 - l) / max(x - l, 1e-5);
  }
  return c;
}

float3 set_lum(float3 c, float l) { return clip_color(c + (l - lum(c))); }

float sat(float3 c) {
  return max(c.r, max(c.g, c.b)) - min(c.r, min(c.g, c.b));
}

float3 set_sat(float3 c, float s) {
  float mx = max(c.r, max(c.g, c.b));
  float mn = min(c.r, min(c.g, c.b));
  if (mx <= mn) {
    return float3(0.0);
  }
  return (c - mn) * s / (mx - mn);
}

float blend_channel(uint mode, float b, float s) {
  switch (mode) {
    case 1: return b * s;
    case 2: return b + s - b * s;
    case 3: return b <= 0.5 ? s * 2.0 * b : 1.0 - (1.0 - s) * (1.0 - (2.0 * b - 1.0));
    case 4: return min(b, s);
    case 5: return max(b, s);
    case 6:
      if (b == 0.0) return 0.0;
      if (s >= 1.0) return 1.0;
      return min(1.0, b / (1.0 - s));
    case 7:
      if (b >= 1.0) return 1.0;
      if (s <= 0.0) return 0.0;
      return 1.0 - min(1.0, (1.0 - b) / s);
    case 8: return s <= 0.5 ? b * 2.0 * s : 1.0 - (1.0 - b) * (1.0 - (2.0 * s - 1.0));
    case 9: {
      float d = b <= 0.25 ? ((16.0 * b - 12.0) * b + 4.0) * b : sqrt(b);
      return s <= 0.5 ? b - (1.0 - 2.0 * s) * b * (1.0 - b)
                      : b + (2.0 * s - 1.0) * (d - b);
    }
    case 10: return fabs(b - s);
    case 11: return b + s - 2.0 * b * s;
    default: return s;
  }
}

float3 blend_colors(uint mode, float3 b, float3 s) {
  switch (mode) {
    case 0: return s;
    case 12: return set_lum(set_sat(s, sat(b)), lum(b));
    case 13: return set_lum(set_sat(b, sat(s)), lum(b));
    case 14: return set_lum(s, lum(b));
    case 15: return set_lum(b, lum(s));
    case 16: return s;
    default:
      return float3(blend_channel(mode, b.r, s.r), blend_channel(mode, b.g, s.g),
                    blend_channel(mode, b.b, s.b));
  }
}

// Paints a premultiplied source over a premultiplied backdrop with a blend
// mode, as Compositing and Blending 1 says.
float4 blend_over(uint mode, float4 backdrop, float4 source) {
  if (mode == 16) {
    return min(backdrop + source, 1.0);
  }
  if (mode != 0 && source.a > 0.0 && backdrop.a > 0.0) {
    float3 cs = source.rgb / source.a;
    float3 cb = backdrop.rgb / backdrop.a;
    float3 mixed = (1.0 - backdrop.a) * cs + backdrop.a * blend_colors(mode, cb, cs);
    source = float4(mixed * source.a, source.a);
  }
  return source + backdrop * (1.0 - source.a);
}

// How much of the pixel at `position` lies inside the box, with its
// corners shaped like a quad without a border.
float box_coverage(float2 position, Bounds_ScaledPixels bounds,
                   Corners_ScaledPixels corner_radii, Corners_f32 corner_shapes) {
  float2 half_size = float2(bounds.size.width, bounds.size.height) / 2.0;
  float2 center = float2(bounds.origin.x, bounds.origin.y) + half_size;
  float2 center_to_point = position - center;
  float corner_radius = pick_corner_radius(center_to_point, corner_radii);
  float corner_shape = pick_corner_shape(center_to_point, corner_shapes);
  float2 corner_to_point = fabs(center_to_point) - half_size;
  float2 corner_center_to_point = corner_to_point + corner_radius;
  float sdf;
  if (corner_shape == 1.0 || corner_radius == 0.0) {
    sdf = quad_sdf_impl(corner_center_to_point, corner_radius);
  } else {
    float2 no_border = float2(-0.5);
    sdf = shaped_corner_sdf(corner_to_point, corner_radius, corner_shape, no_border,
                            corner_to_point + no_border).x;
  }
  return saturate(0.5 - sdf);
}

struct LayerCompositeVertexOutput {
  float4 position [[position]];
  float4 mask_solid [[flat]];
  float clip_distance [[clip_distance]][4];
};

struct LayerCompositeFragmentInput {
  float4 position [[position]];
  float4 mask_solid [[flat]];
};

vertex LayerCompositeVertexOutput layer_composite_vertex(
    uint unit_vertex_id [[vertex_id]],
    constant float2 *unit_vertices [[buffer(LayerInputIndex_Vertices)]],
    constant LayerComposite *composite [[buffer(LayerInputIndex_Layer)]],
    constant Size_DevicePixels *viewport_size
    [[buffer(LayerInputIndex_ViewportSize)]]) {
  float2 unit_vertex = unit_vertices[unit_vertex_id];
  float4 device_position =
      to_device_position(unit_vertex, composite->region, viewport_size);
  float4 clip_distance = distance_from_clip_rect(
      unit_vertex, composite->region, composite->layer.content_mask.bounds);
  return LayerCompositeVertexOutput{
      device_position,
      prepare_fill_color(composite->layer.mask),
      {clip_distance.x, clip_distance.y, clip_distance.z, clip_distance.w}};
}

struct VariableBlurVertexOutput {
  float4 position [[position]];
  float2 uv;
  float4 mask_solid [[flat]];
};

vertex VariableBlurVertexOutput variable_blur_vertex(
    uint unit_vertex_id [[vertex_id]],
    constant float2 *unit_vertices [[buffer(BlurInputIndex_Vertices)]],
    constant LayerComposite *composite [[buffer(BlurInputIndex_Layer)]]) {
  float2 unit_vertex = unit_vertices[unit_vertex_id];
  return VariableBlurVertexOutput{
      float4(unit_vertex.x * 2.0 - 1.0, 1.0 - unit_vertex.y * 2.0, 0.0, 1.0),
      unit_vertex,
      prepare_fill_color(composite->layer.mask)};
}

// One pass of the variable backdrop blur, the blur a gradient mask asks
// for. The sigma at each pixel is the mask value there times the full
// sigma, the contract of the variable blur filter of iOS. Two passes,
// one per axis, come close to the true variable Gaussian, because the
// mask changes slowly against the width of the kernel. Pairs of taps
// merge into one linear read at their weighted centre, which halves the
// reads. A tap past the source is dropped and the weights renormalize,
// as in the fixed blur.
fragment float4 variable_blur_fragment(
    VariableBlurVertexOutput input [[stage_in]],
    constant BlurParams *params [[buffer(BlurInputIndex_Params)]],
    constant LayerComposite *composite [[buffer(BlurInputIndex_Layer)]],
    texture2d<float> source [[texture(BlurInputIndex_Source)]]) {
  constexpr sampler edge_sampler(mag_filter::linear, min_filter::linear,
                                 address::clamp_to_edge);
  constant EffectLayer &layer = composite->layer;
  float2 position =
      float2(composite->region.origin.x, composite->region.origin.y) +
      input.uv * float2(composite->region.size.width, composite->region.size.height);
  float2 box_min = float2(layer.bounds.origin.x, layer.bounds.origin.y);
  float2 box_max = box_min + float2(layer.bounds.size.width, layer.bounds.size.height);
  bool inside = all(position >= box_min) && all(position < box_max);
  float mask = inside
      ? saturate(fill_color(layer.mask, position, layer.bounds, input.mask_solid).a)
      : 0.0;
  float sigma = mask * params->sigma;
  float4 centre = source.sample(edge_sampler, input.uv);
  if (sigma < 0.3) {
    return centre;
  }
  // `params->radius` caps the reach, so one huge sigma cannot stall the
  // pass. The cap trims the tails past it, which slightly narrows only
  // the widest blurs.
  int radius = min(int(ceil(3.0 * sigma)), params->radius);
  float2 step = float2(params->step[0], params->step[1]);
  float4 sum = centre;
  float weight_sum = 1.0;
  for (int i = 1; i <= radius; i += 2) {
    float near_weight = exp(-0.5 * float(i * i) / (sigma * sigma));
    float far_weight =
        exp(-0.5 * float((i + 1) * (i + 1)) / (sigma * sigma));
    float pair = near_weight + far_weight;
    float offset =
        (float(i) * near_weight + float(i + 1) * far_weight) / pair;
    float2 reach = step * offset;
    float2 left = input.uv - reach;
    if (all(left >= 0.0) && all(left <= 1.0)) {
      sum += pair * source.sample(edge_sampler, left);
      weight_sum += pair;
    }
    float2 right = input.uv + reach;
    if (all(right >= 0.0) && all(right <= 1.0)) {
      sum += pair * source.sample(edge_sampler, right);
      weight_sum += pair;
    }
  }
  return sum / weight_sum;
}

fragment float4 layer_composite_fragment(
    LayerCompositeFragmentInput input [[stage_in]],
    constant LayerComposite *composite [[buffer(LayerInputIndex_Layer)]],
    texture2d<float> content_texture [[texture(LayerInputIndex_ContentTexture)]],
    texture2d<float> under_texture [[texture(LayerInputIndex_UnderTexture)]],
    texture2d<float> backdrop_texture [[texture(LayerInputIndex_BackdropTexture)]]) {
  constexpr sampler smooth(mag_filter::linear, min_filter::linear,
                           address::clamp_to_edge);
  constexpr sampler exact(mag_filter::nearest, min_filter::nearest,
                          address::clamp_to_edge);
  constant EffectLayer &layer = composite->layer;
  float2 position = input.position.xy;
  float2 uv = (position - float2(composite->region.origin.x, composite->region.origin.y)) /
              float2(composite->region.size.width, composite->region.size.height);

  float4 under = under_texture.sample(exact, uv);
  float4 content = layer.blur > 0.0 ? content_texture.sample(smooth, uv)
                                    : content_texture.sample(exact, uv);
  content = filter_color(layer.color_matrix, content) * layer.opacity;

  float shape = box_coverage(position, layer.bounds, layer.corner_radii,
                             layer.corner_shapes);
  if (layer.clips_content != 0) {
    content *= shape;
  }
  float keep = 1.0;
  if (layer.has_mask != 0) {
    float2 box_min = float2(layer.bounds.origin.x, layer.bounds.origin.y);
    float2 box_max = box_min + float2(layer.bounds.size.width, layer.bounds.size.height);
    bool inside = all(position >= box_min) && all(position < box_max);
    keep = inside ? fill_color(layer.mask, position, layer.bounds, input.mask_solid).a : 0.0;
  }

  float4 base = under;
  if (layer.has_backdrop != 0) {
    float4 backdrop = under;
    if (layer.backdrop_blur > 0.0) {
      // With a mask, the texture holds the variable blur, already at
      // the width the mask asks for at every pixel.
      backdrop = backdrop_texture.sample(smooth, uv);
    }
    backdrop = mix(backdrop, filter_color(layer.backdrop_matrix, backdrop), keep);
    base = mix(under, backdrop, shape);
  }

  float4 result = blend_over(layer.blend_mode, base, content);
  return mix(base, result, keep);
}
