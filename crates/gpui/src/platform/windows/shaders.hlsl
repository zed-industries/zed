cbuffer GlobalParams : register(b0) {
    float2 global_viewport_size;
    uint global_premultiplied_alpha;
    uint _pad;
};

Texture2D<float4> t_sprite : register(t4);
SamplerState s_sprite : register(s0);

struct Bounds {
    float2 origin;
    float2 size;
};

struct Corners {
    float top_left;
    float top_right;
    float bottom_right;
    float bottom_left;
};

struct Edges {
    float top;
    float right;
    float bottom;
    float left;
};

struct Hsla {
    float h;
    float s;
    float l;
    float a;
};

struct AtlasTextureId {
    uint index;
    uint kind;
};

struct AtlasBounds {
    int2 origin;
    int2 size;
};

struct AtlasTile {
    AtlasTextureId texture_id;
    uint tile_id;
    uint padding;
    AtlasBounds bounds;
};

struct TransformationMatrix {
    float2x2 rotation_scale;
    float2 translation;
};

static const float M_PI_F = 3.141592653f;
static const float3 GRAYSCALE_FACTORS = float3(0.2126f, 0.7152f, 0.0722f);

float4 to_device_position(float2 unit_vertex, Bounds bounds) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    float2 device_position = position / global_viewport_size * float2(2., -2.) + float2(-1., 1.);
    return float4(device_position, 0., 1.);
}

float4 distance_from_clip_rect(float2 unit_vertex, Bounds bounds, Bounds clip_bounds) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    return float4(position.x - clip_bounds.origin.x,
                    clip_bounds.origin.x + clip_bounds.size.x - position.x,
                    position.y - clip_bounds.origin.y,
                    clip_bounds.origin.y + clip_bounds.size.y - position.y);
}

float4 hsla_to_rgba(Hsla hsla) {
    float h = hsla.h * 6.0; // Now, it's an angle but scaled in [0, 6) range
    float s = hsla.s;
    float l = hsla.l;
    float a = hsla.a;

    float c = (1.0 - abs(2.0 * l - 1.0)) * s;
    float x = c * (1.0 - abs(fmod(h, 2.0) - 1.0));
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

// This approximates the error function, needed for the gaussian integral
float2 erf(float2 x) {
    float2 s = sign(x);
    float2 a = abs(x);
    x = 1. + (0.278393 + (0.230389 + 0.078108 * (a * a)) * a) * a;
    x *= x;
    return s - s / (x * x);
}

float blur_along_x(float x, float y, float sigma, float corner, float2 half_size) {
    float delta = min(half_size.y - corner - abs(y), 0.);
    float curved = half_size.x - corner + sqrt(max(0., corner * corner - delta * delta));
    float2 integral = 0.5 + 0.5 * erf((x + float2(-curved, curved)) * (sqrt(0.5) / sigma));
    return integral.y - integral.x;
}

// A standard gaussian function, used for weighting samples
float gaussian(float x, float sigma) {
    return exp(-(x * x) / (2. * sigma * sigma)) / (sqrt(2. * M_PI_F) * sigma);
}

float4 over(float4 below, float4 above) {
    float4 result;
    float alpha = above.a + below.a * (1.0 - above.a);
    result.rgb = (above.rgb * above.a + below.rgb * below.a * (1.0 - above.a)) / alpha;
    result.a = alpha;
    return result;
}

float2 to_tile_position(float2 unit_vertex, AtlasTile tile) {
    float2 atlas_size;
    t_sprite.GetDimensions(atlas_size.x, atlas_size.y);
    return (float2(tile.bounds.origin) + unit_vertex * float2(tile.bounds.size)) / atlas_size;
}

// Abstract away the final color transformation based on the
// target alpha compositing mode.
float4 blend_color(float4 color, float alpha_factor) {
    float alpha = color.a * alpha_factor;
    float multiplier = (global_premultiplied_alpha != 0) ? alpha : 1.0;
    return float4(color.rgb * multiplier, alpha);
}

float4 to_device_position_transformed(float2 unit_vertex, Bounds bounds, 
                                      TransformationMatrix transformation) {
    float2 position = unit_vertex * bounds.size + bounds.origin;

    // Apply the transformation matrix to the position via matrix multiplication.
    float2 transformed_position = float2(0, 0);
    transformed_position.x = position.x * transformation.rotation_scale[0][0] + position.y * transformation.rotation_scale[0][1];
    transformed_position.y = position.x * transformation.rotation_scale[1][0] + position.y * transformation.rotation_scale[1][1];

    // Add in the translation component of the transformation matrix.
    transformed_position += transformation.translation;

    float2 viewport_size = global_viewport_size;
    float2 device_position = transformed_position / viewport_size * float2(2.0, -2.0) + float2(-1.0, 1.0);
    return float4(device_position, 0.0, 1.0);
}

float quad_sdf(float2 pt, Bounds bounds, Corners corner_radii) {
    float2 half_size = bounds.size / 2.;
    float2 center = bounds.origin + half_size;
    float2 center_to_point = pt - center;
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

    float2 rounded_edge_to_point = abs(center_to_point) - half_size + corner_radius;
    float distance =
        length(max(0., rounded_edge_to_point)) +
        min(0., max(rounded_edge_to_point.x, rounded_edge_to_point.y)) -
        corner_radius;

    return distance;
}

/*
**
**              Shadows
**
*/

struct ShadowVertexOutput {
    float4 position: SV_Position;
    float4 color: COLOR;
    uint shadow_id: FLAT;
    float4 clip_distance: SV_ClipDistance;
};

struct ShadowFragmentInput {
  float4 position: SV_Position;
  float4 color: COLOR;
  uint shadow_id: FLAT;
};

struct Shadow {
    uint order;
    float blur_radius;
    Bounds bounds;
    Corners corner_radii;
    Bounds content_mask;
    Hsla color;
};

StructuredBuffer<Shadow> shadows : register(t0);

ShadowVertexOutput shadow_vertex(uint vertex_id: SV_VertexID, uint shadow_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    Shadow shadow = shadows[shadow_id];

    float margin = 3.0 * shadow.blur_radius;
    Bounds bounds = shadow.bounds;
    bounds.origin -= margin;
    bounds.size += 2.0 * margin;

    float4 device_position = to_device_position(unit_vertex, bounds);
    float4 clip_distance = distance_from_clip_rect(unit_vertex, bounds, shadow.content_mask);
    float4 color = hsla_to_rgba(shadow.color);

    ShadowVertexOutput output;
    output.position = device_position;
    output.color = color;
    output.shadow_id = shadow_id;
    output.clip_distance = clip_distance;
    
    return output;
}

float4 shadow_fragment(ShadowFragmentInput input): SV_TARGET {
    Shadow shadow = shadows[input.shadow_id];

    float2 half_size = shadow.bounds.size / 2.;
    float2 center = shadow.bounds.origin + half_size;
    float2 point0 = input.position.xy - center;
    float corner_radius;
    if (point0.x < 0.) {
        if (point0.y < 0.) {
            corner_radius = shadow.corner_radii.top_left;
        } else {
            corner_radius = shadow.corner_radii.bottom_left;
        }
    } else {
        if (point0.y < 0.) {
            corner_radius = shadow.corner_radii.top_right;
        } else {
            corner_radius = shadow.corner_radii.bottom_right;
        }
    }

    // The signal is only non-zero in a limited range, so don't waste samples
    float low = point0.y - half_size.y;
    float high = point0.y + half_size.y;
    float start = clamp(-3. * shadow.blur_radius, low, high);
    float end = clamp(3. * shadow.blur_radius, low, high);

    // Accumulate samples (we can get away with surprisingly few samples)
    float step = (end - start) / 4.;
    float y = start + step * 0.5;
    float alpha = 0.;
    for (int i = 0; i < 4; i++) {
        alpha += blur_along_x(point0.x, point0.y - y, shadow.blur_radius,
                            corner_radius, half_size) *
                gaussian(y, shadow.blur_radius) * step;
        y += step;
    }

    return input.color * float4(1., 1., 1., alpha);
}

/*
**
**              Quads
**
*/

struct Quad {
    uint order;
    uint pad;
    Bounds bounds;
    Bounds content_mask;
    Hsla background;
    Hsla border_color;
    Corners corner_radii;
    Edges border_widths;
};

struct QuadVertexOutput {
    float4 position: SV_Position;
    float4 background_color: COLOR0;
    float4 border_color: COLOR1;
    uint quad_id: FLAT;
    float4 clip_distance: SV_ClipDistance;
};

struct QuadFragmentInput {
    float4 position: SV_Position;
    float4 background_color: COLOR0;
    float4 border_color: COLOR1;
    uint quad_id: FLAT;
};

StructuredBuffer<Quad> quads : register(t1);

QuadVertexOutput quad_vertex(uint vertex_id: SV_VertexID, uint quad_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    Quad quad = quads[quad_id];
    float4 device_position = to_device_position(unit_vertex, quad.bounds);
    float4 clip_distance = distance_from_clip_rect(unit_vertex, quad.bounds, quad.content_mask);
    float4 background_color = hsla_to_rgba(quad.background);
    float4 border_color = hsla_to_rgba(quad.border_color);
    
    QuadVertexOutput output;
    output.position = device_position;
    output.background_color = background_color;
    output.border_color = border_color;
    output.quad_id = quad_id;
    output.clip_distance = clip_distance;
    return output;
}

float4 quad_fragment(QuadFragmentInput input): SV_TARGET {
    Quad quad = quads[input.quad_id];

    // Fast path when the quad is not rounded and doesn't have any border.
    if (quad.corner_radii.top_left == 0. && quad.corner_radii.bottom_left == 0. &&
        quad.corner_radii.top_right == 0. &&
        quad.corner_radii.bottom_right == 0. && quad.border_widths.top == 0. &&
        quad.border_widths.left == 0. && quad.border_widths.right == 0. &&
        quad.border_widths.bottom == 0.) {
        return input.background_color;
    }

    float2 half_size = quad.bounds.size / 2.;
    float2 center = quad.bounds.origin + half_size;
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

    float2 rounded_edge_to_point = abs(center_to_point) - half_size + corner_radius;
    float distance =
        length(max(0., rounded_edge_to_point)) +
        min(0., max(rounded_edge_to_point.x, rounded_edge_to_point.y)) -
        corner_radius;

    float vertical_border = center_to_point.x <= 0. ? quad.border_widths.left
                                                    : quad.border_widths.right;
    float horizontal_border = center_to_point.y <= 0. ? quad.border_widths.top
                                                        : quad.border_widths.bottom;
    float2 inset_size = half_size - corner_radius - float2(vertical_border, horizontal_border);
    float2 point_to_inset_corner = abs(center_to_point) - inset_size;
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
        color = lerp(blended_border, input.background_color,
                    saturate(0.5 - inset_distance));
    }

    return color * float4(1., 1., 1., saturate(0.5 - distance));
}

/*
**
**              Path raster
**
*/

struct PathVertex {
    float2 xy_position;
    float2 st_position;
    Bounds content_mask;
};

struct PathRasterizationOutput {
    float4 position: SV_Position;
    float2 st_position: TEXCOORD0;
    float4 clip_distances: SV_ClipDistance;
};

struct PathRasterizationInput {
    float4 position: SV_Position;
    float2 st_position: TEXCOORD0;
};

StructuredBuffer<PathVertex> path_vertices : register(t2);

PathRasterizationOutput path_rasterization_vertex(uint vertex_id: SV_VertexID) {
    PathVertex vertex = path_vertices[vertex_id];
    PathRasterizationOutput output;
    float2 device_position = vertex.xy_position / global_viewport_size * float2(2.0, -2.0) + float2(-1.0, 1.0);
    float2 tl = vertex.xy_position - vertex.content_mask.origin;
    float2 br = vertex.content_mask.origin + vertex.content_mask.size - vertex.xy_position;
    
    output.position = float4(device_position, 0.0, 1.0);
    output.st_position = vertex.st_position;
    output.clip_distances = float4(tl.x, br.x, tl.y, br.y);
    return output;
}

float4 path_rasterization_fragment(PathRasterizationInput input): SV_Target {
    float2 dx = ddx(input.st_position);
    float2 dy = ddy(input.st_position);
    float2 gradient = float2((2. * input.st_position.x) * dx.x - dx.y,
                            (2. * input.st_position.x) * dy.x - dy.y);
    float f = (input.st_position.x * input.st_position.x) - input.st_position.y;
    float distance = f / length(gradient);
    float alpha = saturate(0.5 - distance);
    return float4(alpha, 0., 0., 1.);
}

/*
**
**              Paths
**
*/

struct PathSprite {
    Bounds bounds;
    Hsla color;
    AtlasTile tile;
};

struct PathVertexOutput {
    float4 position: SV_Position;
    float2 tile_position: POSITION1;
    float4 color: COLOR;
};

StructuredBuffer<PathSprite> path_sprites : register(t3);

PathVertexOutput paths_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    PathSprite sprite = path_sprites[instance_id];
    // Don't apply content mask because it was already accounted for when rasterizing the path.

    PathVertexOutput output;
    output.position = to_device_position(unit_vertex, sprite.bounds);
    output.tile_position = to_tile_position(unit_vertex, sprite.tile);
    // output.tile_position = float2(1., 1.);
    output.color = hsla_to_rgba(sprite.color);
    return output;
}

float4 paths_fragment(PathVertexOutput input): SV_Target {
    float sample = t_sprite.Sample(s_sprite, input.tile_position).r;
    float mask = 1.0 - abs(1.0 - sample % 2.0);
    // return blend_color(input.color, mask);
    float4 color = input.color;
    color.a *= mask;
    return color;
}

/*
**
**              Underlines
**
*/

struct Underline {
    uint order;
    uint pad;
    Bounds bounds;
    Bounds content_mask;
    Hsla color;
    float thickness;
    uint wavy;
};

struct UnderlineVertexOutput {
  float4 position: SV_Position;
  float4 color: COLOR;
  uint underline_id: FLAT;
  float4 clip_distance: SV_ClipDistance;
};

struct UnderlineFragmentInput {
  float4 position: SV_Position;
  float4 color: COLOR;
  uint underline_id: FLAT;
};

StructuredBuffer<Underline> underlines : register(t5);

UnderlineVertexOutput underline_vertex(uint vertex_id: SV_VertexID, uint underline_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    Underline underline = underlines[underline_id];
    float4 device_position = to_device_position(unit_vertex, underline.bounds);
    float4 clip_distance = distance_from_clip_rect(unit_vertex, underline.bounds, 
                                                    underline.content_mask);
    float4 color = hsla_to_rgba(underline.color);

    UnderlineVertexOutput output;
    output.position = device_position;
    output.color = color;
    output.underline_id = underline_id;
    output.clip_distance = clip_distance;
    return output;
}

float4 underline_fragment(UnderlineFragmentInput input): SV_Target {
    Underline underline = underlines[input.underline_id];
    if (underline.wavy) {
        float half_thickness = underline.thickness * 0.5;
        float2 origin =
            float2(underline.bounds.origin.x, underline.bounds.origin.y);
        float2 st = ((input.position.xy - origin) / underline.bounds.size.y) -
                    float2(0., 0.5);
        float frequency = (M_PI_F * (3. * underline.thickness)) / 8.;
        float amplitude = 1. / (2. * underline.thickness);
        float sine = sin(st.x * frequency) * amplitude;
        float dSine = cos(st.x * frequency) * amplitude * frequency;
        float distance = (st.y - sine) / sqrt(1. + dSine * dSine);
        float distance_in_pixels = distance * underline.bounds.size.y;
        float distance_from_top_border = distance_in_pixels - half_thickness;
        float distance_from_bottom_border = distance_in_pixels + half_thickness;
        float alpha = saturate(
            0.5 - max(-distance_from_bottom_border, distance_from_top_border));
        return input.color * float4(1., 1., 1., alpha);
    } else {
        return input.color;
    }
}

/*
**
**              Monochrome sprites
**
*/

struct MonochromeSprite {
    uint order;
    uint pad;
    Bounds bounds;
    Bounds content_mask;
    Hsla color;
    AtlasTile tile;
    TransformationMatrix transformation;
};

struct MonochromeSpriteVertexOutput {
    float4 position: SV_Position;
    float2 tile_position: POSITION;
    float4 color: COLOR;
    float4 clip_distance: SV_ClipDistance;
};

struct MonochromeSpriteFragmentInput {
    float4 position: SV_Position;
    float2 tile_position: POSITION;
    float4 color: COLOR;
};

StructuredBuffer<MonochromeSprite> mono_sprites : register(t6);

MonochromeSpriteVertexOutput monochrome_sprite_vertex(uint vertex_id: SV_VertexID, uint sprite_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    MonochromeSprite sprite = mono_sprites[sprite_id];
    float4 device_position =
        to_device_position_transformed(unit_vertex, sprite.bounds, sprite.transformation);
    float4 clip_distance = distance_from_clip_rect(unit_vertex, sprite.bounds, sprite.content_mask);
    float2 tile_position = to_tile_position(unit_vertex, sprite.tile);
    float4 color = hsla_to_rgba(sprite.color);

    MonochromeSpriteVertexOutput output;
    output.position = device_position;
    output.tile_position = tile_position;
    output.color = color;
    output.clip_distance = clip_distance;
    return output;
}

float4 monochrome_sprite_fragment(MonochromeSpriteFragmentInput input): SV_Target {
    float4 sample = t_sprite.Sample(s_sprite, input.tile_position);
    float4 color = input.color;
    color.a *= sample.a;
    return color;
}

/*
**
**              Monochrome sprites
**
*/

struct PolychromeSprite {
    uint order;
    uint grayscale;
    Bounds bounds;
    Bounds content_mask;
    Corners corner_radii;
    AtlasTile tile;
};

struct PolychromeSpriteVertexOutput {
    float4 position: SV_Position;
    float2 tile_position: POSITION;
    uint sprite_id: FLAT;
    float4 clip_distance: SV_ClipDistance;
};

struct PolychromeSpriteFragmentInput {
    float4 position: SV_Position;
    float2 tile_position: POSITION;
    uint sprite_id: FLAT;
};

StructuredBuffer<PolychromeSprite> poly_sprites : register(t7);

PolychromeSpriteVertexOutput polychrome_sprite_vertex(uint vertex_id: SV_VertexID, uint sprite_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    PolychromeSprite sprite = poly_sprites[sprite_id];
    float4 device_position = to_device_position(unit_vertex, sprite.bounds);
    float4 clip_distance = distance_from_clip_rect(unit_vertex, sprite.bounds,
                                                    sprite.content_mask);
    float2 tile_position = to_tile_position(unit_vertex, sprite.tile);

    PolychromeSpriteVertexOutput output;
    output.position = device_position;
    output.tile_position = tile_position;
    output.sprite_id = sprite_id;
    output.clip_distance = clip_distance;
    return output;
}

float4 polychrome_sprite_fragment(PolychromeSpriteFragmentInput input): SV_Target {
    PolychromeSprite sprite = poly_sprites[input.sprite_id];
    float4 sample = t_sprite.Sample(s_sprite, input.tile_position);
    float distance = quad_sdf(input.position.xy, sprite.bounds, sprite.corner_radii);

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
