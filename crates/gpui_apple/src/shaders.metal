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
float quad_sdf(float2 point, Bounds_ScaledPixels bounds,
               Corners_ScaledPixels corner_radii);
float quad_sdf_impl(float2 center_to_point, float corner_radius);
float4 corner_values(Corners_ScaledPixels corner_radii);
float normalized_superellipse_power(float corner_smoothing);
float4 normalized_superellipse_reaches(
    Corners_ScaledPixels corner_radii, float corner_smoothing);
bool can_use_normalized_superellipse(
    float2 size, Corners_ScaledPixels corner_radii,
    float corner_smoothing);
float normalized_superellipse_sdf_impl(
    float2 corner_to_point, float corner_radius,
    float corner_smoothing, float power);
float normalized_superellipse_sdf(
    float2 point, Bounds_ScaledPixels bounds,
    Corners_ScaledPixels corner_radii,
    float corner_smoothing, float power);

// The Figma-style corner construction below is derived from
// Tien Pham's figma-squircle implementation:
// https://github.com/phamfoo/figma-squircle
// Copyright (c) 2021 Tien Pham
// Licensed under the MIT License. See ../THIRD_PARTY_NOTICES.md.

struct FigmaCornerLayout {
  float4 horizontal_budgets;
  float4 vertical_budgets;
};

struct FigmaCornerExtents {
  float4 horizontal;
  float4 vertical;
};

struct FigmaAxisParams {
  float a;
  float b;
  float p;
};

struct FigmaCornerParams {
  float radius;
  float smoothing;
  float arc_sweep;
  float c;
  float d;
  FigmaAxisParams horizontal;
  FigmaAxisParams vertical;
};

struct CubicClosestPoint {
  float2 point;
  float2 tangent;
  float distance;
  float path_t;
};

struct SdfSample {
  float distance;
  float2 normal;
  float path_t;
  uint segment;
};

struct FigmaRectSample {
  SdfSample sdf;
  uint corner;
};

constant uint FIGMA_SEGMENT_STRAIGHT = 0u;
// Corner progress runs from the horizontal shoulder through the full arc to
// the vertical shoulder. Cubic path_t always runs from its straight endpoint
// to the arc, while arc path_t covers the full arc from 0 to 1.
constant uint FIGMA_SEGMENT_FIRST_CUBIC = 1u;
constant uint FIGMA_SEGMENT_ARC = 2u;
constant uint FIGMA_SEGMENT_SECOND_CUBIC = 3u;
constant uint FIGMA_NO_CORNER = 4u;
constant float FIGMA_EPSILON = 0.000001;

FigmaCornerLayout figma_corner_layout(
    float2 size, Corners_ScaledPixels corner_radii);
FigmaCornerExtents figma_corner_extents(
    Corners_ScaledPixels corner_radii,
    float4 horizontal_budgets, float4 vertical_budgets,
    float corner_smoothing);
float4 figma_smoothing_factors(float corner_smoothing);
FigmaCornerParams figma_corner_params(float corner_radius,
                                      float horizontal_reach,
                                      float vertical_reach,
                                      float4 smoothing_factors);
SdfSample figma_corner_sdf_impl(float2 corner_to_point,
                                FigmaCornerParams params);
float figma_cubic_length(FigmaCornerParams params,
                         FigmaAxisParams axis, float end_t);
float figma_corner_length(FigmaCornerParams params);
float figma_corner_progress(FigmaCornerParams params,
                            SdfSample sample, float total_length);
bool figma_is_corner_candidate(float2 point, float2 size,
                               float horizontal_extent,
                               float vertical_extent, uint corner);
bool figma_has_corner_candidate(float2 point, float2 size,
                                float4 horizontal_extents,
                                float4 vertical_extents);
SdfSample figma_nearest_straight_sample(float2 point, float2 size,
                                        float4 horizontal_extents,
                                        float4 vertical_extents);
FigmaRectSample figma_smooth_rect_sdf_sample(
    float2 point, Bounds_ScaledPixels bounds,
    Corners_ScaledPixels corner_radii,
    float4 horizontal_reaches, float4 vertical_reaches,
    float4 smoothing_factors);
float figma_smooth_rect_sdf(float2 point, Bounds_ScaledPixels bounds,
                            Corners_ScaledPixels corner_radii,
                            float4 horizontal_reaches,
                            float4 vertical_reaches,
                            float4 smoothing_factors);
float styled_rect_sdf(float2 point, Bounds_ScaledPixels bounds,
                      Corners_ScaledPixels corner_radii,
                      float4 horizontal_reaches,
                      float4 vertical_reaches,
                      float4 smoothing_factors);
float gaussian_sdf_coverage(float distance, float sigma);
float gaussian(float x, float sigma);
float2 erf(float2 x);
float blur_along_x(float x, float y, float sigma, float corner,
                   float2 half_size);
float4 over(float4 below, float4 above);
float radians(float degrees);
float4 fill_color(Background background, float2 position, Bounds_ScaledPixels bounds,
  float4 solid_color, float4 color0, float4 color1);

struct GradientColor {
  float4 solid;
  float4 color0;
  float4 color1;
};
GradientColor prepare_fill_color(uint tag, uint color_space, Hsla solid, Hsla color0, Hsla color1);

struct QuadVertexOutput {
  uint quad_id [[flat]];
  float4 position [[position]];
  float4 border_color [[flat]];
  float4 background_solid [[flat]];
  float4 background_color0 [[flat]];
  float4 background_color1 [[flat]];
  float4 horizontal_corner_reaches [[flat]];
  float4 vertical_corner_reaches [[flat]];
  float4 smoothing_factors [[flat]];
  float superellipse_power [[flat]];
  float4 corner_lengths [[flat]];
  float clip_distance [[clip_distance]][4];
};

struct QuadFragmentInput {
  uint quad_id [[flat]];
  float4 position [[position]];
  float4 border_color [[flat]];
  float4 background_solid [[flat]];
  float4 background_color0 [[flat]];
  float4 background_color1 [[flat]];
  float4 horizontal_corner_reaches [[flat]];
  float4 vertical_corner_reaches [[flat]];
  float4 smoothing_factors [[flat]];
  float superellipse_power [[flat]];
  float4 corner_lengths [[flat]];
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

  GradientColor gradient = prepare_fill_color(
    quad.background.tag,
    quad.background.color_space,
    quad.background.solid,
    quad.background.colors[0].color,
    quad.background.colors[1].color
  );
  float2 quad_size = float2(quad.bounds.size.width, quad.bounds.size.height);
  float4 horizontal_corner_reaches = corner_values(quad.corner_radii);
  float4 vertical_corner_reaches = corner_values(quad.corner_radii);
  float4 smoothing_factors = float4(0.0, 1.0, 0.0, 0.0);
  float superellipse_power = 0.0;
  if (quad.corner_smoothing > 0.0) {
    bool has_no_border =
      quad.border_widths.top == 0.0 &&
      quad.border_widths.right == 0.0 &&
      quad.border_widths.bottom == 0.0 &&
      quad.border_widths.left == 0.0;
    if (quad.border_style == 0 && has_no_border &&
        can_use_normalized_superellipse(
          quad_size, quad.corner_radii, quad.corner_smoothing)) {
      float4 reaches = normalized_superellipse_reaches(
        quad.corner_radii, quad.corner_smoothing);
      horizontal_corner_reaches = reaches;
      vertical_corner_reaches = reaches;
      superellipse_power =
        normalized_superellipse_power(quad.corner_smoothing);
    } else {
      smoothing_factors = figma_smoothing_factors(quad.corner_smoothing);
      FigmaCornerLayout layout =
        figma_corner_layout(quad_size, quad.corner_radii);
      FigmaCornerExtents extents = figma_corner_extents(
        quad.corner_radii, layout.horizontal_budgets,
        layout.vertical_budgets, quad.corner_smoothing);
      horizontal_corner_reaches = extents.horizontal;
      vertical_corner_reaches = extents.vertical;
    }
  }

  float4 corner_lengths = corner_values(quad.corner_radii) * (M_PI_F / 2.0);
  if (quad.border_style == 1 && quad.corner_smoothing > 0.0) {
    float4 radii = corner_values(quad.corner_radii);
    for (uint corner = 0u; corner < 4u; corner++) {
      FigmaCornerParams params = figma_corner_params(
        radii[corner], horizontal_corner_reaches[corner],
        vertical_corner_reaches[corner], smoothing_factors);
      corner_lengths[corner] = figma_corner_length(params);
    }
  }

  return QuadVertexOutput{
      quad_id,
      device_position,
      border_color,
      gradient.solid,
      gradient.color0,
      gradient.color1,
      horizontal_corner_reaches,
      vertical_corner_reaches,
      smoothing_factors,
      superellipse_power,
      corner_lengths,
      {clip_distance.x, clip_distance.y, clip_distance.z, clip_distance.w}};
}

fragment float4 quad_fragment(QuadFragmentInput input [[stage_in]],
                              constant Quad *quads
                              [[buffer(QuadInputIndex_Quads)]]) {
  Quad quad = quads[input.quad_id];
  float4 background_color = fill_color(quad.background, input.position.xy, quad.bounds,
    input.background_solid, input.background_color0, input.background_color1);

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

  // Circular corners keep the existing center-quadrant fast path. Smoothed
  // corners use edge ownership because a Figma shoulder can cross a half-side.
  float corner_radius = pick_corner_radius(center_to_point, quad.corner_radii);
  float4 horizontal_corner_extents = corner_values(quad.corner_radii);
  float4 vertical_corner_extents = corner_values(quad.corner_radii);
  uint smooth_corner = FIGMA_NO_CORNER;
  bool has_smooth_corner_candidate = false;
  if (input.superellipse_power > 0.0) {
    float corner_reach = corner_radius *
      (1.0 + clamp(quad.corner_smoothing, 0.0, 1.0));
    float2 corner_offset = fabs(center_to_point) - half_size + corner_reach;
    has_smooth_corner_candidate =
      corner_offset.x >= 0.0 && corner_offset.y >= 0.0;
  } else if (quad.corner_smoothing > 0.0) {
    horizontal_corner_extents = input.horizontal_corner_reaches;
    vertical_corner_extents = input.vertical_corner_reaches;
    has_smooth_corner_candidate = figma_has_corner_candidate(
      point, size, horizontal_corner_extents,
      vertical_corner_extents);
  }

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

  // Vector from the point to the circular corner center, mirrored into the
  // bottom-right quadrant.
  float2 corner_center_to_point = corner_to_point + corner_radius;

  // Whether the nearest point on the border is rounded
  bool is_near_rounded_corner = quad.corner_smoothing > 0.0
    ? has_smooth_corner_candidate
    : corner_center_to_point.x >= 0.0 &&
      corner_center_to_point.y >= 0.0;

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
  SdfSample figma_sample;
  if (input.superellipse_power > 0.0) {
    outer_sdf = normalized_superellipse_sdf_impl(
      corner_to_point, corner_radius, quad.corner_smoothing,
      input.superellipse_power);
    float corner_reach = corner_radius *
      (1.0 + clamp(quad.corner_smoothing, 0.0, 1.0));
    is_near_rounded_corner =
      corner_to_point.x + corner_reach > 0.0 &&
      corner_to_point.y + corner_reach > 0.0;
  } else if (quad.corner_smoothing > 0.0) {
    FigmaRectSample rect_sample = figma_smooth_rect_sdf_sample(
      input.position.xy, quad.bounds, quad.corner_radii,
      input.horizontal_corner_reaches,
      input.vertical_corner_reaches, input.smoothing_factors);
    figma_sample = rect_sample.sdf;
    smooth_corner = rect_sample.corner;
    outer_sdf = figma_sample.distance;
    is_near_rounded_corner =
      figma_sample.segment != FIGMA_SEGMENT_STRAIGHT;
    if (smooth_corner != FIGMA_NO_CORNER) {
      switch (smooth_corner) {
        case 0u:
          border = float2(quad.border_widths.left, quad.border_widths.top);
          break;
        case 1u:
          border = float2(quad.border_widths.right, quad.border_widths.top);
          break;
        case 2u:
          border = float2(quad.border_widths.right, quad.border_widths.bottom);
          break;
        default:
          border = float2(quad.border_widths.left, quad.border_widths.bottom);
          break;
      }
      reduced_border = float2(
        border.x == 0.0 ? -antialias_threshold : border.x,
        border.y == 0.0 ? -antialias_threshold : border.y);
      straight_border_inner_corner_to_point =
        corner_to_point + reduced_border;
    }
  } else {
    outer_sdf = quad_sdf_impl(corner_center_to_point, corner_radius);
  }

  // Approximate signed distance of the point to the inside edge of the quad's
  // border. It is negative outside this edge (within the border), and
  // positive inside.
  //
  // This is not always an accurate signed distance:
  // * The rounded portions with varying border width use an approximation of
  //   nearest-point-on-ellipse.
  // * When it is quickly known to be outside the edge, -1.0 is used.
  float inner_sdf = 0.0;
  if (quad.corner_smoothing > 0.0) {
    if (!is_near_rounded_corner) {
      // Keep exact rectangular offsets on straight segments, including sharp
      // zero-radius corners with unequal adjacent border widths.
      inner_sdf = -max(straight_border_inner_corner_to_point.x,
                       straight_border_inner_corner_to_point.y);
    } else if (input.superellipse_power > 0.0) {
      inner_sdf = -(outer_sdf + reduced_border.x);
    } else {
      float effective_border_width = reduced_border.x;
      if (border.x != border.y) {
        float2 normal = abs(figma_sample.normal);
        float2 active_sides = float2(
          border.x > 0.0 ? 1.0 : 0.0,
          border.y > 0.0 ? 1.0 : 0.0);
        effective_border_width = length(border * normal) -
          antialias_threshold * (1.0 - length(active_sides * normal));
      }
      inner_sdf = -(outer_sdf + effective_border_width);
    }
  } else if (corner_center_to_point.x <= 0.0 ||
             corner_center_to_point.y <= 0.0) {
    // Fast paths for straight borders
    inner_sdf = -max(straight_border_inner_corner_to_point.x,
                     straight_border_inner_corner_to_point.y);
  } else if (is_beyond_inner_straight_border) {
    // Fast path for points that must be outside the inner edge
    inner_sdf = -1.0;
  } else if (reduced_border.x == reduced_border.y) {
    // Fast path for a uniform-width inner edge.
    inner_sdf = -(outer_sdf + reduced_border.x);
  } else {
    float2 ellipse_radii =
      max(float2(0.0), float2(corner_radius) - reduced_border);
    inner_sdf = quarter_ellipse_sdf(corner_center_to_point, ellipse_radii);
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
        bool is_horizontal =
          straight_border_inner_corner_to_point.x <
          straight_border_inner_corner_to_point.y;
        float border_width = is_horizontal ? border.y : border.x;
        if (border_width <= 0.0) {
          is_horizontal = !is_horizontal;
          border_width = is_horizontal ? border.y : border.x;
        }

        if (border_width > 0.0) {
          dash_velocity = dv_numerator / border_width;
          t = is_horizontal ? point.x : point.y;
          t *= dash_velocity;
          max_t = is_horizontal ? size.x : size.y;
          max_t *= dash_velocity;
        }
      } else {
        // When corners are rounded, the dashes are laid out clockwise
        // around the whole perimeter.

        float h_tl = quad.corner_smoothing > 0.0
          ? horizontal_corner_extents.x : quad.corner_radii.top_left;
        float h_tr = quad.corner_smoothing > 0.0
          ? horizontal_corner_extents.y : quad.corner_radii.top_right;
        float h_br = quad.corner_smoothing > 0.0
          ? horizontal_corner_extents.z : quad.corner_radii.bottom_right;
        float h_bl = quad.corner_smoothing > 0.0
          ? horizontal_corner_extents.w : quad.corner_radii.bottom_left;
        float v_tl = quad.corner_smoothing > 0.0
          ? vertical_corner_extents.x : quad.corner_radii.top_left;
        float v_tr = quad.corner_smoothing > 0.0
          ? vertical_corner_extents.y : quad.corner_radii.top_right;
        float v_br = quad.corner_smoothing > 0.0
          ? vertical_corner_extents.z : quad.corner_radii.bottom_right;
        float v_bl = quad.corner_smoothing > 0.0
          ? vertical_corner_extents.w : quad.corner_radii.bottom_left;

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
        float s_t = (size.x - h_tl - h_tr) * dv_t;
        float s_r = (size.y - v_tr - v_br) * dv_r;
        float s_b = (size.x - h_br - h_bl) * dv_b;
        float s_l = (size.y - v_bl - v_tl) * dv_l;

        float corner_dash_velocity_tr = corner_dash_velocity(dv_t, dv_r);
        float corner_dash_velocity_br = corner_dash_velocity(dv_b, dv_r);
        float corner_dash_velocity_bl = corner_dash_velocity(dv_b, dv_l);
        float corner_dash_velocity_tl = corner_dash_velocity(dv_t, dv_l);

        float length_tl = input.corner_lengths.x;
        float length_tr = input.corner_lengths.y;
        float length_br = input.corner_lengths.z;
        float length_bl = input.corner_lengths.w;

        float c_tr = length_tr * corner_dash_velocity_tr;
        float c_br = length_br * corner_dash_velocity_br;
        float c_bl = length_bl * corner_dash_velocity_bl;
        float c_tl = length_tl * corner_dash_velocity_tl;

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
          if (quad.corner_smoothing > 0.0) {
            float selected_radius = quad.corner_radii.top_left;
            float selected_horizontal_reach = input.horizontal_corner_reaches.x;
            float selected_vertical_reach = input.vertical_corner_reaches.x;
            float selected_length = length_tl;
            switch (smooth_corner) {
              case 1u:
                selected_radius = quad.corner_radii.top_right;
                selected_horizontal_reach = input.horizontal_corner_reaches.y;
                selected_vertical_reach = input.vertical_corner_reaches.y;
                selected_length = length_tr;
                break;
              case 2u:
                selected_radius = quad.corner_radii.bottom_right;
                selected_horizontal_reach = input.horizontal_corner_reaches.z;
                selected_vertical_reach = input.vertical_corner_reaches.z;
                selected_length = length_br;
                break;
              case 3u:
                selected_radius = quad.corner_radii.bottom_left;
                selected_horizontal_reach = input.horizontal_corner_reaches.w;
                selected_vertical_reach = input.vertical_corner_reaches.w;
                selected_length = length_bl;
                break;
              default:
                break;
            }
            FigmaCornerParams selected_params = figma_corner_params(
              selected_radius, selected_horizontal_reach,
              selected_vertical_reach, input.smoothing_factors);
            float corner_progress =
              figma_corner_progress(
                selected_params, figma_sample, selected_length);
            switch (smooth_corner) {
              case 0u:
                dash_velocity = corner_dash_velocity_tl;
                t = upto_tl +
                  (selected_length - corner_progress) * dash_velocity;
                break;
              case 1u:
                dash_velocity = corner_dash_velocity_tr;
                t = upto_tr + corner_progress * dash_velocity;
                break;
              case 2u:
                dash_velocity = corner_dash_velocity_br;
                t = upto_br +
                  (selected_length - corner_progress) * dash_velocity;
                break;
              default:
                dash_velocity = corner_dash_velocity_bl;
                t = upto_bl + corner_progress * dash_velocity;
                break;
            }
          } else {
            float radians = atan2(corner_center_to_point.y,
                                  corner_center_to_point.x);
            float corner_t = radians * corner_radius;
            if (center_to_point.x >= 0.0) {
              if (center_to_point.y < 0.0) {
                dash_velocity = corner_dash_velocity_tr;
                t = upto_r - corner_t * dash_velocity;
              } else {
                dash_velocity = corner_dash_velocity_br;
                t = upto_br + corner_t * dash_velocity;
              }
            } else if (center_to_point.y >= 0.0) {
              dash_velocity = corner_dash_velocity_bl;
              t = upto_l - corner_t * dash_velocity;
            } else {
              dash_velocity = corner_dash_velocity_tl;
              t = upto_tl + corner_t * dash_velocity;
            }
          }
        } else {
          // Straight borders
          if (quad.corner_smoothing > 0.0) {
            bool is_horizontal =
              straight_border_inner_corner_to_point.x <
              straight_border_inner_corner_to_point.y;
            float straight_width = is_horizontal ? border.y : border.x;
            if (straight_width <= 0.0) {
              is_horizontal = !is_horizontal;
            }
            if (is_horizontal) {
              if (center_to_point.y < 0.0) {
                dash_velocity = dv_t;
                t = (point.x - h_tl) * dash_velocity;
              } else {
                dash_velocity = dv_b;
                t = upto_bl - (point.x - h_bl) * dash_velocity;
              }
            } else if (center_to_point.x < 0.0) {
              dash_velocity = dv_l;
              t = upto_tl - (point.y - v_tl) * dash_velocity;
            } else {
              dash_velocity = dv_r;
              t = upto_r + (point.y - v_tr) * dash_velocity;
            }
          } else {
            bool is_horizontal =
              corner_center_to_point.x < corner_center_to_point.y;
            if (is_horizontal) {
              if (center_to_point.y < 0.0) {
                dash_velocity = dv_t;
                t = (point.x - h_tl) * dash_velocity;
              } else {
                dash_velocity = dv_b;
                t = upto_bl - (point.x - h_bl) * dash_velocity;
              }
            } else if (center_to_point.x < 0.0) {
              dash_velocity = dv_l;
              t = upto_tl - (point.y - v_tl) * dash_velocity;
            } else {
              dash_velocity = dv_r;
              t = upto_r + (point.y - v_tr) * dash_velocity;
            }
          }
        }
      }

      float dash_length = dash_length_per_width / dash_period_per_width;

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
  float4 horizontal_corner_reaches [[flat]];
  float4 vertical_corner_reaches [[flat]];
  float4 element_horizontal_corner_reaches [[flat]];
  float4 element_vertical_corner_reaches [[flat]];
  float4 smoothing_factors [[flat]];
  float clip_distance [[clip_distance]][4];
};

struct ShadowFragmentInput {
  float4 position [[position]];
  float4 color [[flat]];
  uint shadow_id [[flat]];
  float4 horizontal_corner_reaches [[flat]];
  float4 vertical_corner_reaches [[flat]];
  float4 element_horizontal_corner_reaches [[flat]];
  float4 element_vertical_corner_reaches [[flat]];
  float4 smoothing_factors [[flat]];
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
  float4 horizontal_corner_reaches = corner_values(shadow.corner_radii);
  float4 vertical_corner_reaches = corner_values(shadow.corner_radii);
  float4 element_horizontal_corner_reaches =
      corner_values(shadow.element_corner_radii);
  float4 element_vertical_corner_reaches =
      corner_values(shadow.element_corner_radii);
  float4 smoothing_factors = float4(0.0, 1.0, 0.0, 0.0);
  if (shadow.corner_smoothing > 0.0) {
    smoothing_factors = figma_smoothing_factors(shadow.corner_smoothing);
    FigmaCornerLayout layout = figma_corner_layout(
        float2(shadow.bounds.size.width, shadow.bounds.size.height),
        shadow.corner_radii);
    FigmaCornerExtents extents = figma_corner_extents(
        shadow.corner_radii, layout.horizontal_budgets,
        layout.vertical_budgets, shadow.corner_smoothing);
    horizontal_corner_reaches = extents.horizontal;
    vertical_corner_reaches = extents.vertical;

    FigmaCornerLayout element_layout = figma_corner_layout(
        float2(shadow.element_bounds.size.width,
               shadow.element_bounds.size.height),
        shadow.element_corner_radii);
    FigmaCornerExtents element_extents = figma_corner_extents(
        shadow.element_corner_radii,
        element_layout.horizontal_budgets,
        element_layout.vertical_budgets, shadow.corner_smoothing);
    element_horizontal_corner_reaches = element_extents.horizontal;
    element_vertical_corner_reaches = element_extents.vertical;
  }

  return ShadowVertexOutput{
      device_position,
      color,
      shadow_id,
      horizontal_corner_reaches,
      vertical_corner_reaches,
      element_horizontal_corner_reaches,
      element_vertical_corner_reaches,
      smoothing_factors,
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
  bool has_rounded_corners =
      any(corner_values(shadow.corner_radii) > float4(0.0));

  float alpha;
  if (shadow.blur_radius == 0.) {
    float distance = styled_rect_sdf(
        input.position.xy, shadow.bounds, shadow.corner_radii,
        input.horizontal_corner_reaches,
        input.vertical_corner_reaches, input.smoothing_factors);
    alpha = saturate(0.5 - distance);
  } else if (has_rounded_corners) {
    float blur_limit = 3.0 * shadow.blur_radius;
    float2 box_delta = abs(point) - half_size;
    float2 outside_delta = max(box_delta, float2(0.0));
    if (dot(outside_delta, outside_delta) > blur_limit * blur_limit) {
      alpha = 0.0;
    } else {
      float2 local_point = input.position.xy - origin;
      float edge_depth = min(
        min(local_point.x, size.x - local_point.x),
        min(local_point.y, size.y - local_point.y));
      bool is_corner_candidate = figma_has_corner_candidate(
        local_point, size, input.horizontal_corner_reaches,
        input.vertical_corner_reaches);
      if (!is_corner_candidate && edge_depth >= blur_limit) {
        alpha = 1.0;
      } else {
        float distance = styled_rect_sdf(
            input.position.xy, shadow.bounds, shadow.corner_radii,
            input.horizontal_corner_reaches,
            input.vertical_corner_reaches, input.smoothing_factors);
        alpha = gaussian_sdf_coverage(distance, shadow.blur_radius);
      }
    }
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
    float element_distance = styled_rect_sdf(
        input.position.xy, shadow.element_bounds,
        shadow.element_corner_radii,
        input.element_horizontal_corner_reaches,
        input.element_vertical_corner_reaches,
        input.smoothing_factors);
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
  float4 horizontal_corner_reaches [[flat]];
  float4 vertical_corner_reaches [[flat]];
  float4 smoothing_factors [[flat]];
  float superellipse_power [[flat]];
  float clip_distance [[clip_distance]][4];
};

struct PolychromeSpriteFragmentInput {
  float4 position [[position]];
  float2 tile_position;
  uint sprite_id [[flat]];
  float4 horizontal_corner_reaches [[flat]];
  float4 vertical_corner_reaches [[flat]];
  float4 smoothing_factors [[flat]];
  float superellipse_power [[flat]];
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
  float4 horizontal_corner_reaches = corner_values(sprite.corner_radii);
  float4 vertical_corner_reaches = corner_values(sprite.corner_radii);
  float4 smoothing_factors = float4(0.0, 1.0, 0.0, 0.0);
  float superellipse_power = 0.0;
  if (sprite.corner_smoothing > 0.0) {
    float2 sprite_size =
      float2(sprite.bounds.size.width, sprite.bounds.size.height);
    if (can_use_normalized_superellipse(
        sprite_size, sprite.corner_radii, sprite.corner_smoothing)) {
      float4 reaches = normalized_superellipse_reaches(
        sprite.corner_radii, sprite.corner_smoothing);
      horizontal_corner_reaches = reaches;
      vertical_corner_reaches = reaches;
      superellipse_power =
        normalized_superellipse_power(sprite.corner_smoothing);
    } else {
      smoothing_factors = figma_smoothing_factors(sprite.corner_smoothing);
      FigmaCornerLayout layout =
        figma_corner_layout(sprite_size, sprite.corner_radii);
      FigmaCornerExtents extents = figma_corner_extents(
        sprite.corner_radii, layout.horizontal_budgets,
        layout.vertical_budgets, sprite.corner_smoothing);
      horizontal_corner_reaches = extents.horizontal;
      vertical_corner_reaches = extents.vertical;
    }
  }
  return PolychromeSpriteVertexOutput{
      device_position,
      tile_position,
      sprite_id,
      horizontal_corner_reaches,
      vertical_corner_reaches,
      smoothing_factors,
      superellipse_power,
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
  float distance;
  if (input.superellipse_power > 0.0) {
    distance = normalized_superellipse_sdf(
      input.position.xy, sprite.bounds, sprite.corner_radii,
      sprite.corner_smoothing, input.superellipse_power);
  } else if (sprite.corner_smoothing > 0.0) {
    distance = figma_smooth_rect_sdf(
      input.position.xy, sprite.bounds, sprite.corner_radii,
      input.horizontal_corner_reaches,
      input.vertical_corner_reaches, input.smoothing_factors);
  } else {
    distance = quad_sdf(input.position.xy, sprite.bounds, sprite.corner_radii);
  }

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

  GradientColor gradient_color = prepare_fill_color(
    background.tag,
    background.color_space,
    background.solid,
    background.colors[0].color,
    background.colors[1].color
  );

  float4 color = fill_color(
    background,
    input.position.xy,
    path_bounds,
    gradient_color.solid,
    gradient_color.color0,
    gradient_color.color1
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

float4 corner_values(Corners_ScaledPixels corner_radii) {
  return float4(
    corner_radii.top_left,
    corner_radii.top_right,
    corner_radii.bottom_right,
    corner_radii.bottom_left);
}

// Match the superellipse's diagonal inset to that of the public circular
// radius while its edge reach grows by (1 + smoothing).
constant float SUPERELLIPSE_DIAGONAL_INSET = 0.2928932188134524;

float normalized_superellipse_power(float corner_smoothing) {
  float smoothing = clamp(corner_smoothing, 0.0, 1.0);
  float normalized_diagonal =
    1.0 - SUPERELLIPSE_DIAGONAL_INSET / (1.0 + smoothing);
  return -log(2.0) / log(normalized_diagonal);
}

float4 normalized_superellipse_reaches(
    Corners_ScaledPixels corner_radii, float corner_smoothing) {
  return max(corner_values(corner_radii), float4(0.0)) *
    (1.0 + clamp(corner_smoothing, 0.0, 1.0));
}

bool can_use_normalized_superellipse(
    float2 size, Corners_ScaledPixels corner_radii,
    float corner_smoothing) {
  float4 radii = max(corner_values(corner_radii), float4(0.0));
  float4 reaches = normalized_superellipse_reaches(
    corner_radii, corner_smoothing);
  float half_short_side = 0.5 * min(size.x, size.y);
  return corner_smoothing > 0.0 &&
    size.x > 0.0 && size.y > 0.0 &&
    any(radii > float4(0.0)) &&
    all(reaches <= float4(half_short_side));
}

float normalized_superellipse_sdf_impl(
    float2 corner_to_point, float corner_radius,
    float corner_smoothing, float power) {
  float extent = max(corner_radius, 0.0) *
    (1.0 + clamp(corner_smoothing, 0.0, 1.0));
  float2 corner_center_to_point = corner_to_point + extent;
  if (extent <= 0.0 ||
      corner_center_to_point.x <= 0.0 ||
      corner_center_to_point.y <= 0.0) {
    return max(corner_to_point.x, corner_to_point.y);
  }

  float2 normalized = corner_center_to_point / extent;
  float2 powered = pow(normalized, float2(power));
  float gradient = power * length(pow(normalized, float2(power - 1.0)));
  return extent * (powered.x + powered.y - 1.0) /
    max(gradient, 0.000001);
}

float normalized_superellipse_sdf(
    float2 point, Bounds_ScaledPixels bounds,
    Corners_ScaledPixels corner_radii,
    float corner_smoothing, float power) {
  float2 half_size = float2(bounds.size.width, bounds.size.height) / 2.0;
  float2 center = float2(bounds.origin.x, bounds.origin.y) + half_size;
  float2 center_to_point = point - center;
  float corner_radius = pick_corner_radius(center_to_point, corner_radii);
  float2 corner_to_point = abs(center_to_point) - half_size;
  return normalized_superellipse_sdf_impl(
    corner_to_point, corner_radius, corner_smoothing, power);
}

float2 figma_split_side(float length, float first_radius,
                        float second_radius) {
  float total_radius = first_radius + second_radius;
  if (total_radius == 0.0) {
    return float2(0.0);
  }
  float first_budget = length * first_radius / total_radius;
  return float2(first_budget, length - first_budget);
}

// Normalizes the radii, then assigns each corner its share of each adjacent
// edge. The stable radius order is TL, TR, BL, BR. Vectors use TL, TR, BR, BL.
FigmaCornerLayout figma_corner_layout(
    float2 size, Corners_ScaledPixels corner_radii) {
  float radii[4] = {
    max(corner_radii.top_left, 0.0),
    max(corner_radii.top_right, 0.0),
    max(corner_radii.bottom_right, 0.0),
    max(corner_radii.bottom_left, 0.0),
  };
  float budgets[4] = {-1.0, -1.0, -1.0, -1.0};
  uint order[4] = {0u, 1u, 3u, 2u};

  // Adjacent bubble passes and strict comparison preserve the tie order.
  for (uint sort_pass = 0u; sort_pass < 3u; sort_pass++) {
    for (uint i = 0u; i < 3u - sort_pass; i++) {
      if (radii[order[i + 1u]] > radii[order[i]]) {
        uint swap = order[i];
        order[i] = order[i + 1u];
        order[i + 1u] = swap;
      }
    }
  }

  for (uint rank = 0u; rank < 4u; rank++) {
    uint corner = order[rank];
    float radius = radii[corner];
    uint horizontal_neighbor;
    uint vertical_neighbor;

    switch (corner) {
      case 0u:
        horizontal_neighbor = 1u;
        vertical_neighbor = 3u;
        break;
      case 1u:
        horizontal_neighbor = 0u;
        vertical_neighbor = 2u;
        break;
      case 2u:
        horizontal_neighbor = 3u;
        vertical_neighbor = 1u;
        break;
      default:
        horizontal_neighbor = 2u;
        vertical_neighbor = 0u;
        break;
    }

    float horizontal_radius = radii[horizontal_neighbor];
    float horizontal_budget = 0.0;
    if (radius != 0.0 || horizontal_radius != 0.0) {
      if (budgets[horizontal_neighbor] >= 0.0) {
        horizontal_budget = size.x - budgets[horizontal_neighbor];
      } else {
        horizontal_budget =
          size.x * radius / (radius + horizontal_radius);
      }
    }

    float vertical_radius = radii[vertical_neighbor];
    float vertical_budget = 0.0;
    if (radius != 0.0 || vertical_radius != 0.0) {
      if (budgets[vertical_neighbor] >= 0.0) {
        vertical_budget = size.y - budgets[vertical_neighbor];
      } else {
        vertical_budget = size.y * radius / (radius + vertical_radius);
      }
    }

    float budget = max(0.0, min(horizontal_budget, vertical_budget));
    budgets[corner] = budget;
    radii[corner] = min(radius, budget);
  }

  float2 top = figma_split_side(size.x, radii[0], radii[1]);
  float2 bottom = figma_split_side(size.x, radii[3], radii[2]);
  float2 left = figma_split_side(size.y, radii[0], radii[3]);
  float2 right = figma_split_side(size.y, radii[1], radii[2]);

  FigmaCornerLayout layout;
  layout.horizontal_budgets =
    float4(top.x, top.y, bottom.y, bottom.x);
  layout.vertical_budgets =
    float4(left.x, right.x, right.y, left.y);
  return layout;
}

float4 figma_smoothing_factors(float corner_smoothing) {
  float smoothing = clamp(corner_smoothing, 0.0, 1.0);
  float arc_sweep = 0.5 * M_PI_F * (1.0 - smoothing);
  float beta = (M_PI_F / 4.0) * smoothing;
  float join_handle_factor = tan(0.5 * beta);
  return float4(
    smoothing,
    sin(0.5 * arc_sweep) * sqrt(2.0),
    join_handle_factor * cos(beta),
    join_handle_factor * sin(beta));
}

FigmaCornerParams figma_corner_params(float corner_radius,
                                      float horizontal_reach,
                                      float vertical_reach,
                                      float4 smoothing_factors) {
  horizontal_reach = max(horizontal_reach, 0.0);
  vertical_reach = max(vertical_reach, 0.0);
  float radius = min(max(corner_radius, 0.0),
                     min(horizontal_reach, vertical_reach));
  float smoothing = radius > FIGMA_EPSILON
    ? smoothing_factors.x : 0.0;
  float desired_reach = radius * (1.0 + smoothing);
  float arc_sweep = 0.5 * M_PI_F * (1.0 - smoothing);
  float arc_delta = smoothing_factors.y * radius;
  float c = smoothing_factors.z * radius;
  float d = smoothing_factors.w * radius;
  float core_length = arc_delta + c + d;
  float ideal_b = max((desired_reach - core_length) / 3.0, 0.0);
  float horizontal_available = max(horizontal_reach - core_length, 0.0);
  float vertical_available = max(vertical_reach - core_length, 0.0);
  float shared_b = min(
    ideal_b,
    min(horizontal_available * (5.0 / 6.0),
        vertical_available * (5.0 / 6.0)));

  FigmaCornerParams params;
  params.radius = radius;
  params.smoothing = smoothing;
  params.arc_sweep = arc_sweep;
  params.c = c;
  params.d = d;
  params.horizontal.a = horizontal_available - shared_b;
  params.horizontal.b = shared_b;
  params.horizontal.p = horizontal_reach;
  params.vertical.a = vertical_available - shared_b;
  params.vertical.b = shared_b;
  params.vertical.p = vertical_reach;
  return params;
}

FigmaCornerExtents figma_corner_extents(
    Corners_ScaledPixels corner_radii,
    float4 horizontal_budgets, float4 vertical_budgets,
    float corner_smoothing) {
  float4 radii = corner_values(corner_radii);
  FigmaCornerExtents extents;
  for (uint corner = 0u; corner < 4u; corner++) {
    float horizontal_budget = max(horizontal_budgets[corner], 0.0);
    float vertical_budget = max(vertical_budgets[corner], 0.0);
    float radius = min(max(radii[corner], 0.0),
                       min(horizontal_budget, vertical_budget));
    float desired_reach = radius *
      (1.0 + clamp(corner_smoothing, 0.0, 1.0));
    extents.horizontal[corner] = min(desired_reach, horizontal_budget);
    extents.vertical[corner] = min(desired_reach, vertical_budget);
  }
  return extents;
}

float2 figma_cubic_point(FigmaAxisParams axis, float c, float d,
                         float t) {
  float x1 = 3.0 * axis.a;
  float x2 = 3.0 * (axis.b - axis.a);
  float x3 = axis.a - 2.0 * axis.b + c;
  float t2 = t * t;
  return float2(
    t * (x1 + t * (x2 + t * x3)),
    d * t2 * t);
}

float2 figma_cubic_derivative(FigmaAxisParams axis, float c, float d,
                              float t) {
  float x1 = 3.0 * axis.a;
  float x2 = 3.0 * (axis.b - axis.a);
  float x3 = axis.a - 2.0 * axis.b + c;
  float t2 = t * t;
  return float2(
    x1 + 2.0 * x2 * t + 3.0 * x3 * t2,
    3.0 * d * t2);
}

float2 figma_cubic_second_derivative(FigmaAxisParams axis, float c,
                                     float d, float t) {
  float x2 = 3.0 * (axis.b - axis.a);
  float x3 = axis.a - 2.0 * axis.b + c;
  return float2(
    2.0 * x2 + 6.0 * x3 * t,
    6.0 * d * t);
}

CubicClosestPoint closest_figma_cubic(float2 point, FigmaAxisParams axis,
                                      float c, float d) {
  float y_seed = pow(
    clamp(point.y / max(d, 0.00001), 0.0, 1.0),
    1.0 / 3.0);
  float2 chord = figma_cubic_point(axis, c, d, 1.0);
  float chord_seed = clamp(
    dot(point, chord) / max(dot(chord, chord), FIGMA_EPSILON),
    0.0, 1.0);
  float2 y_delta = figma_cubic_point(axis, c, d, y_seed) - point;
  float2 chord_delta = figma_cubic_point(axis, c, d, chord_seed) - point;
  float t = dot(chord_delta, chord_delta) < dot(y_delta, y_delta)
    ? chord_seed : y_seed;

  for (uint iteration = 0u; iteration < 4u; iteration++) {
    float2 curve_point = figma_cubic_point(axis, c, d, t);
    float2 tangent = figma_cubic_derivative(axis, c, d, t);
    float2 second_derivative =
      figma_cubic_second_derivative(axis, c, d, t);
    float2 delta = curve_point - point;
    float denominator =
      dot(tangent, tangent) + dot(delta, second_derivative);
    if (abs(denominator) > FIGMA_EPSILON) {
      t = clamp(t - dot(delta, tangent) / denominator, 0.0, 1.0);
    }
  }

  float closest_t = t;
  float2 closest_point = figma_cubic_point(axis, c, d, t);
  float closest_distance = length(point - closest_point);

  float2 start = float2(0.0);
  float start_distance = length(point - start);
  if (start_distance < closest_distance) {
    closest_t = 0.0;
    closest_point = start;
    closest_distance = start_distance;
  }

  float2 end = chord;
  float end_distance = length(point - end);
  if (end_distance < closest_distance) {
    closest_t = 1.0;
    closest_point = end;
    closest_distance = end_distance;
  }

  CubicClosestPoint result;
  result.point = closest_point;
  result.tangent = figma_cubic_derivative(axis, c, d, closest_t);
  result.distance = closest_distance;
  result.path_t = closest_t;
  return result;
}

float figma_signed_distance(float2 delta, float2 normal, float distance) {
  return dot(delta, normal) >= 0.0 ? distance : -distance;
}

float figma_cross_2d(float2 a, float2 b) {
  return a.x * b.y - a.y * b.x;
}

float2 figma_unfold_normal(float2 normal, bool mirrored) {
  return mirrored
    ? float2(-normal.y, normal.x)
    : float2(normal.x, -normal.y);
}

SdfSample figma_corner_sdf_impl(float2 corner_to_point,
                                FigmaCornerParams params) {
  float2 z = corner_to_point +
    float2(params.horizontal.p, params.vertical.p);
  SdfSample sample;
  sample.path_t = 0.0;
  sample.segment = FIGMA_SEGMENT_STRAIGHT;

  if (params.radius <= FIGMA_EPSILON || z.x <= 0.0 || z.y <= 0.0) {
    sample.distance = max(corner_to_point.x, corner_to_point.y);
    sample.normal = corner_to_point.x > corner_to_point.y
      ? float2(1.0, 0.0) : float2(0.0, 1.0);
    return sample;
  }

  float2 horizontal_point = float2(z.x, params.vertical.p - z.y);
  float2 vertical_point = float2(z.y, params.horizontal.p - z.x);

  float2 circle_center =
    float2(params.horizontal.p - params.radius, params.radius);
  float2 join = float2(
    params.horizontal.a + params.horizontal.b + params.c,
    params.d);
  float2 start_direction = (join - circle_center) / params.radius;
  float2 to_point = horizontal_point - circle_center;
  float to_point_length = length(to_point);
  float2 point_direction = to_point_length > FIGMA_EPSILON
    ? to_point / max(to_point_length, FIGMA_EPSILON)
    : start_direction;
  float arc_angle = clamp(
    atan2(figma_cross_2d(start_direction, point_direction),
          dot(start_direction, point_direction)),
    0.0, params.arc_sweep);
  float arc_sine = sin(arc_angle);
  float arc_cosine = cos(arc_angle);
  float2 arc_normal = float2(
    arc_cosine * start_direction.x - arc_sine * start_direction.y,
    arc_sine * start_direction.x + arc_cosine * start_direction.y);
  float2 arc_point = circle_center + params.radius * arc_normal;
  float2 arc_point_delta = horizontal_point - arc_point;
  float arc_distance = figma_signed_distance(
    arc_point_delta, arc_normal, length(arc_point_delta));
  float arc_t = params.arc_sweep > FIGMA_EPSILON
    ? arc_angle / max(params.arc_sweep, FIGMA_EPSILON)
    : 0.0;

  sample.distance = arc_distance;
  sample.normal = figma_unfold_normal(arc_normal, false);
  sample.path_t = arc_t;
  sample.segment = FIGMA_SEGMENT_ARC;

  if (params.smoothing > FIGMA_EPSILON) {
    // The cubic stays inside its control-point bounds. Skip Newton when that
    // box cannot beat the current arc distance.
    float2 horizontal_bounds_delta = max(
      float2(0.0), max(-horizontal_point, horizontal_point - join));
    if (dot(horizontal_bounds_delta, horizontal_bounds_delta) <=
        sample.distance * sample.distance * 1.000001 + FIGMA_EPSILON) {
      CubicClosestPoint horizontal_cubic = closest_figma_cubic(
        horizontal_point, params.horizontal, params.c, params.d);
      float2 horizontal_normal = normalize(float2(
        horizontal_cubic.tangent.y, -horizontal_cubic.tangent.x));
      float horizontal_distance = figma_signed_distance(
        horizontal_point - horizontal_cubic.point,
        horizontal_normal, horizontal_cubic.distance);
      if (abs(horizontal_distance) <= abs(sample.distance)) {
        sample.distance = horizontal_distance;
        sample.normal = figma_unfold_normal(horizontal_normal, false);
        sample.path_t = horizontal_cubic.path_t;
        sample.segment = FIGMA_SEGMENT_FIRST_CUBIC;
      }
    }

    float2 vertical_join = float2(
      params.vertical.a + params.vertical.b + params.c, params.d);
    float2 vertical_bounds_delta = max(
      float2(0.0), max(-vertical_point, vertical_point - vertical_join));
    if (dot(vertical_bounds_delta, vertical_bounds_delta) <=
        sample.distance * sample.distance * 1.000001 + FIGMA_EPSILON) {
      CubicClosestPoint vertical_cubic = closest_figma_cubic(
        vertical_point, params.vertical, params.c, params.d);
      float2 vertical_normal = normalize(float2(
        vertical_cubic.tangent.y, -vertical_cubic.tangent.x));
      float vertical_distance = figma_signed_distance(
        vertical_point - vertical_cubic.point,
        vertical_normal, vertical_cubic.distance);
      if (abs(vertical_distance) <= abs(sample.distance)) {
        sample.distance = vertical_distance;
        sample.normal = figma_unfold_normal(vertical_normal, true);
        sample.path_t = vertical_cubic.path_t;
        sample.segment = FIGMA_SEGMENT_SECOND_CUBIC;
      }
    }
  }
  return sample;
}

float figma_cubic_length(FigmaCornerParams params,
                         FigmaAxisParams axis, float end_t) {
  float half_t = clamp(end_t, 0.0, 1.0) / 2.0;
  if (half_t <= 0.0 || params.smoothing <= 0.0) {
    return 0.0;
  }

  // Five-point Gauss-Legendre quadrature keeps dash phase stable without a
  // data-dependent loop.
  float center = half_t;
  float offset1 = half_t * 0.5384693101;
  float offset2 = half_t * 0.9061798459;
  float speed0 = length(figma_cubic_derivative(
    axis, params.c, params.d, center));
  float speed1 =
    length(figma_cubic_derivative(
      axis, params.c, params.d, center - offset1)) +
    length(figma_cubic_derivative(
      axis, params.c, params.d, center + offset1));
  float speed2 =
    length(figma_cubic_derivative(
      axis, params.c, params.d, center - offset2)) +
    length(figma_cubic_derivative(
      axis, params.c, params.d, center + offset2));
  return half_t * (
    0.5688888889 * speed0 +
    0.4786286705 * speed1 +
    0.2369268851 * speed2);
}

float figma_corner_length(FigmaCornerParams params) {
  return figma_cubic_length(params, params.horizontal, 1.0) +
    params.radius * params.arc_sweep +
    figma_cubic_length(params, params.vertical, 1.0);
}

float figma_corner_progress(FigmaCornerParams params,
                            SdfSample sample, float total_length) {
  if (sample.segment == FIGMA_SEGMENT_FIRST_CUBIC) {
    return figma_cubic_length(
      params, params.horizontal, sample.path_t);
  }
  if (sample.segment == FIGMA_SEGMENT_ARC) {
    return figma_cubic_length(params, params.horizontal, 1.0) +
      params.radius * params.arc_sweep * sample.path_t;
  }
  if (sample.segment == FIGMA_SEGMENT_SECOND_CUBIC) {
    return total_length - figma_cubic_length(
      params, params.vertical, sample.path_t);
  }
  return 0.0;
}

bool figma_is_corner_candidate(float2 point, float2 size,
                               float horizontal_extent,
                               float vertical_extent, uint corner) {
  if (horizontal_extent <= FIGMA_EPSILON ||
      vertical_extent <= FIGMA_EPSILON) {
    return false;
  }
  switch (corner) {
    case 0u:
      return point.x <= horizontal_extent &&
        point.y <= vertical_extent;
    case 1u:
      return size.x - point.x <= horizontal_extent &&
        point.y <= vertical_extent;
    case 2u:
      return size.x - point.x <= horizontal_extent &&
        size.y - point.y <= vertical_extent;
    default:
      return point.x <= horizontal_extent &&
        size.y - point.y <= vertical_extent;
  }
}

bool figma_has_corner_candidate(float2 point, float2 size,
                                float4 horizontal_extents,
                                float4 vertical_extents) {
  return figma_is_corner_candidate(
      point, size, horizontal_extents.x, vertical_extents.x, 0u) ||
    figma_is_corner_candidate(
      point, size, horizontal_extents.y, vertical_extents.y, 1u) ||
    figma_is_corner_candidate(
      point, size, horizontal_extents.z, vertical_extents.z, 2u) ||
    figma_is_corner_candidate(
      point, size, horizontal_extents.w, vertical_extents.w, 3u);
}

float2 figma_corner_to_point(float2 point, float2 size, uint corner) {
  switch (corner) {
    case 0u:
      return -point;
    case 1u:
      return float2(point.x - size.x, -point.y);
    case 2u:
      return point - size;
    default:
      return float2(-point.x, point.y - size.y);
  }
}

float2 figma_orient_corner_normal(float2 normal, uint corner) {
  switch (corner) {
    case 0u:
      return -normal;
    case 1u:
      return float2(normal.x, -normal.y);
    case 2u:
      return normal;
    default:
      return float2(-normal.x, normal.y);
  }
}

SdfSample figma_nearest_straight_sample(float2 point, float2 size,
                                        float4 horizontal_extents,
                                        float4 vertical_extents) {
  float2 nearest_delta = point - float2(
    clamp(point.x, horizontal_extents.x,
          size.x - horizontal_extents.y), 0.0);
  float2 nearest_normal = float2(0.0, -1.0);
  float nearest_distance_squared = dot(nearest_delta, nearest_delta);

  float2 candidate_delta = point - float2(
    size.x, clamp(point.y, vertical_extents.y,
                  size.y - vertical_extents.z));
  float candidate_distance_squared = dot(candidate_delta, candidate_delta);
  if (candidate_distance_squared < nearest_distance_squared) {
    nearest_delta = candidate_delta;
    nearest_normal = float2(1.0, 0.0);
    nearest_distance_squared = candidate_distance_squared;
  }

  candidate_delta = point - float2(
    clamp(point.x, horizontal_extents.w,
          size.x - horizontal_extents.z), size.y);
  candidate_distance_squared = dot(candidate_delta, candidate_delta);
  if (candidate_distance_squared < nearest_distance_squared) {
    nearest_delta = candidate_delta;
    nearest_normal = float2(0.0, 1.0);
    nearest_distance_squared = candidate_distance_squared;
  }

  candidate_delta = point - float2(
    0.0, clamp(point.y, vertical_extents.x,
               size.y - vertical_extents.w));
  candidate_distance_squared = dot(candidate_delta, candidate_delta);
  if (candidate_distance_squared < nearest_distance_squared) {
    nearest_delta = candidate_delta;
    nearest_normal = float2(-1.0, 0.0);
    nearest_distance_squared = candidate_distance_squared;
  }

  SdfSample sample;
  sample.distance = figma_signed_distance(
    nearest_delta, nearest_normal, sqrt(nearest_distance_squared));
  sample.normal = nearest_normal;
  sample.path_t = 0.0;
  sample.segment = FIGMA_SEGMENT_STRAIGHT;
  return sample;
}

FigmaRectSample figma_smooth_rect_sdf_sample(
    float2 point, Bounds_ScaledPixels bounds,
    Corners_ScaledPixels corner_radii,
    float4 horizontal_reaches, float4 vertical_reaches,
    float4 smoothing_factors) {
  float2 origin = float2(bounds.origin.x, bounds.origin.y);
  float2 size = float2(bounds.size.width, bounds.size.height);
  float2 local_point = point - origin;
  float4 radii = corner_values(corner_radii);
  FigmaRectSample result;
  result.corner = FIGMA_NO_CORNER;

  if (!figma_has_corner_candidate(
        local_point, size, horizontal_reaches, vertical_reaches)) {
    result.sdf = figma_nearest_straight_sample(
      local_point, size, horizontal_reaches, vertical_reaches);
    return result;
  }

  result.sdf = figma_nearest_straight_sample(
    local_point, size, horizontal_reaches, vertical_reaches);

  for (uint corner = 0u; corner < 4u; corner += 1u) {
    if (figma_is_corner_candidate(
          local_point, size, horizontal_reaches[corner],
          vertical_reaches[corner], corner)) {
      FigmaCornerParams params = figma_corner_params(
        radii[corner], horizontal_reaches[corner],
        vertical_reaches[corner], smoothing_factors);
      if (params.radius > FIGMA_EPSILON) {
        SdfSample candidate = figma_corner_sdf_impl(
          figma_corner_to_point(local_point, size, corner), params);
        candidate.normal = figma_orient_corner_normal(
          candidate.normal, corner);
        if (abs(candidate.distance) <= abs(result.sdf.distance)) {
          result.sdf = candidate;
          result.corner = corner;
        }
      }
    }
  }
  return result;
}

float figma_smooth_rect_sdf(float2 point, Bounds_ScaledPixels bounds,
                            Corners_ScaledPixels corner_radii,
                            float4 horizontal_reaches,
                            float4 vertical_reaches,
                            float4 smoothing_factors) {
  FigmaRectSample sample = figma_smooth_rect_sdf_sample(
    point, bounds, corner_radii, horizontal_reaches,
    vertical_reaches, smoothing_factors);
  return sample.sdf.distance;
}

float styled_rect_sdf(float2 point, Bounds_ScaledPixels bounds,
                      Corners_ScaledPixels corner_radii,
                      float4 horizontal_reaches,
                      float4 vertical_reaches,
                      float4 smoothing_factors) {
  if (smoothing_factors.x <= 0.0 ||
      all(corner_values(corner_radii) <= float4(0.0))) {
    return quad_sdf(point, bounds, corner_radii);
  }

  return figma_smooth_rect_sdf(
      point, bounds, corner_radii, horizontal_reaches,
      vertical_reaches, smoothing_factors);
}

float gaussian_sdf_coverage(float distance, float sigma) {
  float normalized = distance / (sqrt(2.0) * sigma);
  return saturate(0.5 - 0.5 * erf(float2(normalized)).x);
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

GradientColor prepare_fill_color(uint tag, uint color_space, Hsla solid,
                                     Hsla color0, Hsla color1) {
  GradientColor out;
  if (tag == 0 || tag == 2 || tag == 3) {
    out.solid = hsla_to_rgba(solid);
  } else if (tag == 1) {
    out.color0 = hsla_to_rgba(color0);
    out.color1 = hsla_to_rgba(color1);

    // Prepare color space in vertex for avoid conversion
    // in fragment shader for performance reasons
    if (color_space == 1) {
      // Oklab
      out.color0 = srgb_to_oklab(out.color0);
      out.color1 = srgb_to_oklab(out.color1);
    }
  }

  return out;
}

float2x2 rotate2d(float angle) {
    float s = sin(angle);
    float c = cos(angle);
    return float2x2(c, -s, s, c);
}

float4 fill_color(Background background,
                      float2 position,
                      Bounds_ScaledPixels bounds,
                      float4 solid_color, float4 color0, float4 color1) {
  float4 color;

  switch (background.tag) {
    case 0:
      color = solid_color;
      break;
    case 1: {
      // -90 degrees to match the CSS gradient angle.
      float gradient_angle = background.gradient_angle_or_pattern_height;
      float radians = (fmod(gradient_angle, 360.0) - 90.0) * (M_PI_F / 180.0);
      float2 direction = float2(cos(radians), sin(radians));

      // Expand the short side to be the same as the long side
      if (bounds.size.width > bounds.size.height) {
          direction.y *= bounds.size.height / bounds.size.width;
      } else {
          direction.x *=  bounds.size.width / bounds.size.height;
      }

      // Get the t value for the linear gradient with the color stop percentages.
      float2 half_size = float2(bounds.size.width, bounds.size.height) / 2.;
      float2 center = float2(bounds.origin.x, bounds.origin.y) + half_size;
      float2 center_to_point = position - center;
      float t = dot(center_to_point, direction) / length(direction);
      // Check the direction to determine whether to use x or y
      if (abs(direction.x) > abs(direction.y)) {
          t = (t + half_size.x) / bounds.size.width;
      } else {
          t = (t + half_size.y) / bounds.size.height;
      }

      // Adjust t based on the stop percentages
      t = (t - background.colors[0].percentage)
        / (background.colors[1].percentage
        - background.colors[0].percentage);
      t = clamp(t, 0.0, 1.0);

      switch (background.color_space) {
        case 0:
          color = mix(color0, color1, t);
          break;
        case 1: {
          float4 oklab_color = mix(color0, color1, t);
          color = oklab_to_srgb(oklab_color);
          break;
        }
      }

      // Dither to reduce banding in gradients (especially dark/alpha).
      // Triangular-distributed noise breaks up 8-bit quantization steps.
      // ±2/255 for RGB (enough for dark-on-dark compositing),
      // ±3/255 for alpha (needs more because alpha × dark color = tiny steps).
      {
        float2 seed = position * 0.6180339887; // golden ratio spread
        float r1 = fract(sin(dot(seed, float2(12.9898, 78.233))) * 43758.5453);
        float r2 = fract(sin(dot(seed, float2(39.3460, 11.135))) * 24634.6345);
        float tri = r1 + r2 - 1.0; // triangular PDF, range [-1, +1]
        color.rgb += tri * 2.0 / 255.0;
        color.a   += tri * 3.0 / 255.0;
      }

      break;
    }
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
