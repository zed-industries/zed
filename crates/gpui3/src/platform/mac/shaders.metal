#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

float4 hsla_to_rgba(Hsla hsla);
float4 to_device_position(float2 unit_vertex, Bounds_ScaledPixels bounds,
                          Bounds_ScaledPixels clip_bounds,
                          constant Size_DevicePixels *viewport_size);
float2 to_tile_position(float2 unit_vertex, AtlasTile tile,
                        constant Size_DevicePixels *atlas_size);
float quad_sdf(float2 point, Bounds_ScaledPixels bounds,
               Corners_ScaledPixels corner_radii);
float gaussian(float x, float sigma);
float2 erf(float2 x);
float blur_along_x(float x, float y, float sigma, float corner,
                   float2 half_size);

struct QuadVertexOutput {
  float4 position [[position]];
  float4 background_color [[flat]];
  float4 border_color [[flat]];
  uint quad_id [[flat]];
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
  float4 device_position = to_device_position(
      unit_vertex, quad.bounds, quad.content_mask.bounds, viewport_size);
  float4 background_color = hsla_to_rgba(quad.background);
  float4 border_color = hsla_to_rgba(quad.border_color);
  return QuadVertexOutput{device_position, background_color, border_color,
                          quad_id};
}

fragment float4 quad_fragment(QuadVertexOutput input [[stage_in]],
                              constant Quad *quads
                              [[buffer(QuadInputIndex_Quads)]]) {
  Quad quad = quads[input.quad_id];
  float2 half_size =
      float2(quad.bounds.size.width, quad.bounds.size.height) / 2.;
  float2 center =
      float2(quad.bounds.origin.x, quad.bounds.origin.y) + half_size;
  float2 center_to_point = input.position.xy - center;
  float corner_radius;
  if (center_to_point.x < 0.) {
    if (center_to_point.y < 0.) {
      corner_radius = quad.corner_radii.top_left;
    } else {
      corner_radius = quad.corner_radii.bottom_left;
    }
  } else {
    if (center_to_point.y < 0.) {
      corner_radius = quad.corner_radii.top_right;
    } else {
      corner_radius = quad.corner_radii.bottom_right;
    }
  }

  float2 rounded_edge_to_point =
      fabs(center_to_point) - half_size + corner_radius;
  float distance =
      length(max(0., rounded_edge_to_point)) +
      min(0., max(rounded_edge_to_point.x, rounded_edge_to_point.y)) -
      corner_radius;

  float vertical_border = center_to_point.x <= 0. ? quad.border_widths.left
                                                  : quad.border_widths.right;
  float horizontal_border = center_to_point.y <= 0. ? quad.border_widths.top
                                                    : quad.border_widths.bottom;
  float2 inset_size =
      half_size - corner_radius - float2(vertical_border, horizontal_border);
  float2 point_to_inset_corner = fabs(center_to_point) - inset_size;
  float border_width;
  if (point_to_inset_corner.x < 0. && point_to_inset_corner.y < 0.) {
    border_width = 0.;
  } else if (point_to_inset_corner.y > point_to_inset_corner.x) {
    border_width = horizontal_border;
  } else {
    border_width = vertical_border;
  }

  float4 color;
  if (border_width == 0.) {
    color = input.background_color;
  } else {
    float inset_distance = distance + border_width;

    // Decrease border's opacity as we move inside the background.
    input.border_color.a *= 1. - saturate(0.5 - inset_distance);

    // Alpha-blend the border and the background.
    float output_alpha =
        quad.border_color.a + quad.background.a * (1. - quad.border_color.a);
    float3 premultiplied_border_rgb =
        input.border_color.rgb * quad.border_color.a;
    float3 premultiplied_background_rgb =
        input.background_color.rgb * input.background_color.a;
    float3 premultiplied_output_rgb =
        premultiplied_border_rgb +
        premultiplied_background_rgb * (1. - input.border_color.a);
    color = float4(premultiplied_output_rgb, output_alpha);
  }

  return color * float4(1., 1., 1., saturate(0.5 - distance));
}

struct ShadowVertexOutput {
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

  float margin = 3. * shadow.blur_radius;
  // Set the bounds of the shadow and adjust its size based on the shadow's
  // spread radius to achieve the spreading effect
  Bounds_ScaledPixels bounds = shadow.bounds;
  bounds.origin.x -= margin;
  bounds.origin.y -= margin;
  bounds.size.width += 2. * margin;
  bounds.size.height += 2. * margin;

  float4 device_position = to_device_position(
      unit_vertex, bounds, shadow.content_mask.bounds, viewport_size);
  float4 color = hsla_to_rgba(shadow.color);

  return ShadowVertexOutput{
      device_position,
      color,
      shadow_id,
  };
}

fragment float4 shadow_fragment(ShadowVertexOutput input [[stage_in]],
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

  // The signal is only non-zero in a limited range, so don't waste samples
  float low = point.y - half_size.y;
  float high = point.y + half_size.y;
  float start = clamp(-3. * shadow.blur_radius, low, high);
  float end = clamp(3. * shadow.blur_radius, low, high);

  // Accumulate samples (we can get away with surprisingly few samples)
  float step = (end - start) / 4.;
  float y = start + step * 0.5;
  float alpha = 0.;
  for (int i = 0; i < 4; i++) {
    alpha += blur_along_x(point.x, point.y - y, shadow.blur_radius,
                          corner_radius, half_size) *
             gaussian(y, shadow.blur_radius) * step;
    y += step;
  }

  return input.color * float4(1., 1., 1., alpha);
}

struct MonochromeSpriteVertexOutput {
  float4 position [[position]];
  float2 tile_position;
  float4 color [[flat]];
  uint sprite_id [[flat]];
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
  // Don't apply content mask at the vertex level because we don't have time to
  // make sampling from the texture match the mask.
  float4 device_position = to_device_position(unit_vertex, sprite.bounds,
                                              sprite.bounds, viewport_size);
  float2 tile_position = to_tile_position(unit_vertex, sprite.tile, atlas_size);
  float4 color = hsla_to_rgba(sprite.color);
  return MonochromeSpriteVertexOutput{device_position, tile_position, color,
                                      sprite_id};
}

fragment float4 monochrome_sprite_fragment(
    MonochromeSpriteVertexOutput input [[stage_in]],
    constant MonochromeSprite *sprites [[buffer(SpriteInputIndex_Sprites)]],
    texture2d<float> atlas_texture [[texture(SpriteInputIndex_AtlasTexture)]]) {
  MonochromeSprite sprite = sprites[input.sprite_id];
  constexpr sampler atlas_texture_sampler(mag_filter::linear,
                                          min_filter::linear);
  float4 sample =
      atlas_texture.sample(atlas_texture_sampler, input.tile_position);
  float clip_distance = quad_sdf(input.position.xy, sprite.content_mask.bounds,
                                 Corners_ScaledPixels{0., 0., 0., 0.});
  float4 color = input.color;
  color.a *= sample.a * saturate(0.5 - clip_distance);
  return color;
}

struct PolychromeSpriteVertexOutput {
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
  // Don't apply content mask at the vertex level because we don't have time to
  // make sampling from the texture match the mask.
  float4 device_position = to_device_position(unit_vertex, sprite.bounds,
                                              sprite.bounds, viewport_size);
  float2 tile_position = to_tile_position(unit_vertex, sprite.tile, atlas_size);
  return PolychromeSpriteVertexOutput{device_position, tile_position,
                                      sprite_id};
}

fragment float4 polychrome_sprite_fragment(
    PolychromeSpriteVertexOutput input [[stage_in]],
    constant PolychromeSprite *sprites [[buffer(SpriteInputIndex_Sprites)]],
    texture2d<float> atlas_texture [[texture(SpriteInputIndex_AtlasTexture)]]) {
  PolychromeSprite sprite = sprites[input.sprite_id];
  constexpr sampler atlas_texture_sampler(mag_filter::linear,
                                          min_filter::linear);
  float4 sample =
      atlas_texture.sample(atlas_texture_sampler, input.tile_position);
  float quad_distance =
      quad_sdf(input.position.xy, sprite.bounds, sprite.corner_radii);
  float clip_distance = quad_sdf(input.position.xy, sprite.content_mask.bounds,
                                 Corners_ScaledPixels{0., 0., 0., 0.});
  float distance = max(quad_distance, clip_distance);

  float4 color = sample;
  if (sprite.grayscale) {
    float grayscale = 0.2126 * color.r + 0.7152 * color.g + 0.0722 * color.b;
    color.r = grayscale;
    color.g = grayscale;
    color.b = grayscale;
  }
  color.a *= saturate(0.5 - distance);
  return color;
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

float4 to_device_position(float2 unit_vertex, Bounds_ScaledPixels bounds,
                          Bounds_ScaledPixels clip_bounds,
                          constant Size_DevicePixels *input_viewport_size) {
  float2 position =
      unit_vertex * float2(bounds.size.width, bounds.size.height) +
      float2(bounds.origin.x, bounds.origin.y);
  position.x = max(clip_bounds.origin.x, position.x);
  position.x = min(clip_bounds.origin.x + clip_bounds.size.width, position.x);
  position.y = max(clip_bounds.origin.y, position.y);
  position.y = min(clip_bounds.origin.y + clip_bounds.size.height, position.y);

  float2 viewport_size = float2((float)input_viewport_size->width,
                                (float)input_viewport_size->height);
  float2 device_position =
      position / viewport_size * float2(2., -2.) + float2(-1., 1.);
  return float4(device_position, 0., 1.);
}

float2 to_tile_position(float2 unit_vertex, AtlasTile tile,
                        constant Size_DevicePixels *atlas_size) {
  float2 tile_origin = float2(tile.bounds.origin.x, tile.bounds.origin.y);
  float2 tile_size = float2(tile.bounds.size.width, tile.bounds.size.height);
  return (tile_origin + unit_vertex * tile_size) /
         float2((float)atlas_size->width, (float)atlas_size->height);
}

float quad_sdf(float2 point, Bounds_ScaledPixels bounds,
               Corners_ScaledPixels corner_radii) {
  float2 half_size = float2(bounds.size.width, bounds.size.height) / 2.;
  float2 center = float2(bounds.origin.x, bounds.origin.y) + half_size;
  float2 center_to_point = point - center;
  float corner_radius;
  if (center_to_point.x < 0.) {
    if (center_to_point.y < 0.) {
      corner_radius = corner_radii.top_left;
    } else {
      corner_radius = corner_radii.bottom_left;
    }
  } else {
    if (center_to_point.y < 0.) {
      corner_radius = corner_radii.top_right;
    } else {
      corner_radius = corner_radii.bottom_right;
    }
  }

  float2 rounded_edge_to_point =
      abs(center_to_point) - half_size + corner_radius;
  float distance =
      length(max(0., rounded_edge_to_point)) +
      min(0., max(rounded_edge_to_point.x, rounded_edge_to_point.y)) -
      corner_radius;

  return distance;
}

// A standard gaussian function, used for weighting samples
float gaussian(float x, float sigma) {
  return exp(-(x * x) / (2. * sigma * sigma)) / (sqrt(2. * M_PI_F) * sigma);
}

// This approximates the error function, needed for the gaussian integral
float2 erf(float2 x) {
  float2 s = sign(x);
  float2 a = abs(x);
  x = 1. + (0.278393 + (0.230389 + 0.078108 * (a * a)) * a) * a;
  x *= x;
  return s - s / (x * x);
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
