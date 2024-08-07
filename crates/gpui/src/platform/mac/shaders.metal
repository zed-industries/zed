#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

float4 hsla_to_rgba(Hsla hsla);
float4 to_device_position(float2 unit_vertex, Bounds_ScaledPixels bounds,
                          constant Size_DevicePixels *viewport_size);
float4 to_device_position_transformed(float2 unit_vertex, Bounds_ScaledPixels bounds,
                          TransformationMatrix transformation,
                          constant Size_DevicePixels *input_viewport_size);

float2 to_tile_position(float2 unit_vertex, AtlasTile tile,
                        constant Size_DevicePixels *atlas_size);
float4 distance_from_clip_rect(float2 unit_vertex, Bounds_ScaledPixels bounds,
                               Bounds_ScaledPixels clip_bounds);
float quad_sdf(float2 point, Bounds_ScaledPixels bounds,
               Corners_ScaledPixels corner_radii);
float gaussian(float x, float sigma);
float2 erf(float2 x);
float blur_along_x(float x, float y, float sigma, float corner,
                   float2 half_size);
float4 over(float4 below, float4 above);

struct QuadVertexOutput {
  float4 position [[position]];
  float4 background_color [[flat]];
  float4 border_color [[flat]];
  uint quad_id [[flat]];
  float clip_distance [[clip_distance]][4];
};

struct QuadFragmentInput {
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
  float4 device_position =
      to_device_position(unit_vertex, quad.bounds, viewport_size);
  float4 clip_distance = distance_from_clip_rect(unit_vertex, quad.bounds,
                                                 quad.content_mask.bounds);
  float4 background_color = hsla_to_rgba(quad.background);
  float4 border_color = hsla_to_rgba(quad.border_color);
  return QuadVertexOutput{
      device_position,
      background_color,
      border_color,
      quad_id,
      {clip_distance.x, clip_distance.y, clip_distance.z, clip_distance.w}};
}

fragment float4 quad_fragment(QuadFragmentInput input [[stage_in]],
                              constant Quad *quads
                              [[buffer(QuadInputIndex_Quads)]]) {
  Quad quad = quads[input.quad_id];

  // Fast path when the quad is not rounded and doesn't have any border.
  if (quad.corner_radii.top_left == 0. && quad.corner_radii.bottom_left == 0. &&
      quad.corner_radii.top_right == 0. &&
      quad.corner_radii.bottom_right == 0. && quad.border_widths.top == 0. &&
      quad.border_widths.left == 0. && quad.border_widths.right == 0. &&
      quad.border_widths.bottom == 0.) {
    return input.background_color;
  }

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
    // Blend the border on top of the background and then linearly interpolate
    // between the two as we slide inside the background.
    float4 blended_border = over(input.background_color, input.border_color);
    color = mix(blended_border, input.background_color,
                saturate(0.5 - inset_distance));
  }

  return color * float4(1., 1., 1., saturate(0.5 - distance));
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

  float margin = 3. * shadow.blur_radius;
  // Set the bounds of the shadow and adjust its size based on the shadow's
  // spread radius to achieve the spreading effect
  Bounds_ScaledPixels bounds = shadow.bounds;
  bounds.origin.x -= margin;
  bounds.origin.y -= margin;
  bounds.size.width += 2. * margin;
  bounds.size.height += 2. * margin;

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
  Underline underline = underlines[input.underline_id];
  if (underline.wavy) {
    float half_thickness = underline.thickness * 0.5;
    float2 origin =
        float2(underline.bounds.origin.x, underline.bounds.origin.y);
    float2 st = ((input.position.xy - origin) / underline.bounds.size.height) -
                float2(0., 0.5);
    float frequency = (M_PI_F * (3. * underline.thickness)) / 8.;
    float amplitude = 1. / (2. * underline.thickness);
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
  float clip_distance [[clip_distance]][4];
};

struct MonochromeSpriteFragmentInput {
  float4 position [[position]];
  float2 tile_position;
  float4 color [[flat]];
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
  float4 clip_distance = distance_from_clip_rect(unit_vertex, sprite.bounds,
                                                 sprite.content_mask.bounds);
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
  color.a *= saturate(0.5 - distance);
  return color;
}

struct PathRasterizationVertexOutput {
  float4 position [[position]];
  float2 st_position;
  float clip_rect_distance [[clip_distance]][4];
};

struct PathRasterizationFragmentInput {
  float4 position [[position]];
  float2 st_position;
};

vertex PathRasterizationVertexOutput path_rasterization_vertex(
    uint vertex_id [[vertex_id]],
    constant PathVertex_ScaledPixels *vertices
    [[buffer(PathRasterizationInputIndex_Vertices)]],
    constant Size_DevicePixels *atlas_size
    [[buffer(PathRasterizationInputIndex_AtlasTextureSize)]]) {
  PathVertex_ScaledPixels v = vertices[vertex_id];
  float2 vertex_position = float2(v.xy_position.x, v.xy_position.y);
  float2 viewport_size = float2(atlas_size->width, atlas_size->height);
  return PathRasterizationVertexOutput{
      float4(vertex_position / viewport_size * float2(2., -2.) +
                 float2(-1., 1.),
             0., 1.),
      float2(v.st_position.x, v.st_position.y),
      {v.xy_position.x - v.content_mask.bounds.origin.x,
       v.content_mask.bounds.origin.x + v.content_mask.bounds.size.width -
           v.xy_position.x,
       v.xy_position.y - v.content_mask.bounds.origin.y,
       v.content_mask.bounds.origin.y + v.content_mask.bounds.size.height -
           v.xy_position.y}};
}

fragment float4 path_rasterization_fragment(PathRasterizationFragmentInput input
                                            [[stage_in]]) {
  float2 dx = dfdx(input.st_position);
  float2 dy = dfdy(input.st_position);
  float2 gradient = float2((2. * input.st_position.x) * dx.x - dx.y,
                           (2. * input.st_position.x) * dy.x - dy.y);
  float f = (input.st_position.x * input.st_position.x) - input.st_position.y;
  float distance = f / length(gradient);
  float alpha = saturate(0.5 - distance);
  return float4(alpha, 0., 0., 1.);
}

struct PathSpriteVertexOutput {
  float4 position [[position]];
  float2 tile_position;
  float4 color [[flat]];
};

vertex PathSpriteVertexOutput path_sprite_vertex(
    uint unit_vertex_id [[vertex_id]], uint sprite_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(SpriteInputIndex_Vertices)]],
    constant PathSprite *sprites [[buffer(SpriteInputIndex_Sprites)]],
    constant Size_DevicePixels *viewport_size
    [[buffer(SpriteInputIndex_ViewportSize)]],
    constant Size_DevicePixels *atlas_size
    [[buffer(SpriteInputIndex_AtlasTextureSize)]]) {

  float2 unit_vertex = unit_vertices[unit_vertex_id];
  PathSprite sprite = sprites[sprite_id];
  // Don't apply content mask because it was already accounted for when
  // rasterizing the path.
  float4 device_position =
      to_device_position(unit_vertex, sprite.bounds, viewport_size);
  float2 tile_position = to_tile_position(unit_vertex, sprite.tile, atlas_size);
  float4 color = hsla_to_rgba(sprite.color);
  return PathSpriteVertexOutput{device_position, tile_position, color};
}

fragment float4 path_sprite_fragment(
    PathSpriteVertexOutput input [[stage_in]],
    constant PathSprite *sprites [[buffer(SpriteInputIndex_Sprites)]],
    texture2d<float> atlas_texture [[texture(SpriteInputIndex_AtlasTexture)]]) {
  constexpr sampler atlas_texture_sampler(mag_filter::linear,
                                          min_filter::linear);
  float4 sample =
      atlas_texture.sample(atlas_texture_sampler, input.tile_position);
  float mask = 1. - abs(1. - fmod(sample.r, 2.));
  float4 color = input.color;
  color.a *= mask;
  return color;
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

float4 over(float4 below, float4 above) {
  float4 result;
  float alpha = above.a + below.a * (1.0 - above.a);
  result.rgb =
      (above.rgb * above.a + below.rgb * below.a * (1.0 - above.a)) / alpha;
  result.a = alpha;
  return result;
}
