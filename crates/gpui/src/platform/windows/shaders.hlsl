cbuffer GlobalParams: register(b0) {
    float2 global_viewport_size;
    uint2 _global_pad;
};

Texture2D<float4> t_sprite: register(t0);
SamplerState s_sprite: register(s0);

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

struct LinearColorStop {
    Hsla color;
    float percentage;
};

struct Background {
    // 0u is Solid
    // 1u is LinearGradient
    uint tag;
    // 0u is sRGB linear color
    // 1u is Oklab color
    uint color_space;
    Hsla solid;
    float angle;
    LinearColorStop colors[2];
    uint pad;
};

struct GradientColor {
  float4 solid;
  float4 color0;
  float4 color1;
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

float4 to_device_position_impl(float2 position) {
    float2 device_position = position / global_viewport_size * float2(2.0, -2.0) + float2(-1.0, 1.0);
    return float4(device_position, 0., 1.);
}

float4 to_device_position(float2 unit_vertex, Bounds bounds) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    return to_device_position_impl(position);
}

float4 distance_from_clip_rect_impl(float2 position, Bounds clip_bounds) {
    return float4(position.x - clip_bounds.origin.x,
                    clip_bounds.origin.x + clip_bounds.size.x - position.x,
                    position.y - clip_bounds.origin.y,
                    clip_bounds.origin.y + clip_bounds.size.y - position.y);
}

float4 distance_from_clip_rect(float2 unit_vertex, Bounds bounds, Bounds clip_bounds) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    return distance_from_clip_rect_impl(position, clip_bounds);
}

// Convert linear RGB to sRGB
float3 linear_to_srgb(float3 color) {
    return pow(color, float3(2.2, 2.2, 2.2));
}

// Convert sRGB to linear RGB
float3 srgb_to_linear(float3 color) {
    return pow(color, float3(1.0 / 2.2, 1.0 / 2.2, 1.0 / 2.2));
}

/// Hsla to linear RGBA conversion.
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

float4 to_device_position_transformed(float2 unit_vertex, Bounds bounds, 
                                      TransformationMatrix transformation) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    float2 transformed = mul(position, transformation.rotation_scale) + transformation.translation;
    float2 device_position = transformed / global_viewport_size * float2(2.0, -2.0) + float2(-1.0, 1.0);
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

GradientColor prepare_gradient_color(uint tag, uint color_space, Hsla solid, Hsla color0, Hsla color1) {
    GradientColor output;
    if (tag == 0) {
        output.solid = hsla_to_rgba(solid);
    } else if (tag == 1) {
        output.color0 = hsla_to_rgba(color0);
        output.color1 = hsla_to_rgba(color1);

        // Prepare color space in vertex for avoid conversion
        // in fragment shader for performance reasons
        if (color_space == 1) {
        // Oklab
        output.color0 = srgb_to_oklab(output.color0);
        output.color1 = srgb_to_oklab(output.color1);
        }
    }

    return output;
}

float4 gradient_color(Background background,
                      float2 position,
                      Bounds bounds,
                      float4 solid_color, float4 color0, float4 color1) {
    float4 color;

    switch (background.tag) {
        case 0:
            color = solid_color;
            break;
        case 1: {
            // -90 degrees to match the CSS gradient angle.
            float radians = (fmod(background.angle, 360.0) - 90.0) * (M_PI_F / 180.0);
            float2 direction = float2(cos(radians), sin(radians));

            // Expand the short side to be the same as the long side
            if (bounds.size.x > bounds.size.y) {
                direction.y *= bounds.size.y / bounds.size.x;
            } else {
                direction.x *=  bounds.size.x / bounds.size.y;
            }

            // Get the t value for the linear gradient with the color stop percentages.
            float2 half_size = float2(bounds.size.x, bounds.size.y) / 2.;
            float2 center = float2(bounds.origin.x, bounds.origin.y) + half_size;
            float2 center_to_point = position - center;
            float t = dot(center_to_point, direction) / length(direction);
            // Check the direct to determine the use x or y
            if (abs(direction.x) > abs(direction.y)) {
                t = (t + half_size.x) / bounds.size.x;
            } else {
                t = (t + half_size.y) / bounds.size.y;
            }

            // Adjust t based on the stop percentages
            t = (t - background.colors[0].percentage)
                / (background.colors[1].percentage
                - background.colors[0].percentage);
            t = clamp(t, 0.0, 1.0);

            switch (background.color_space) {
                case 0:
                    color = lerp(color0, color1, t);
                    break;
                case 1: {
                    float4 oklab_color = lerp(color0, color1, t);
                    color = oklab_to_srgb(oklab_color);
                    break;
                }
            }
            break;
        }
    }

    return color;
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

StructuredBuffer<Shadow> shadows: register(t1);

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
    Background background;
    Hsla border_color;
    Corners corner_radii;
    Edges border_widths;
};

struct QuadVertexOutput {
    float4 position: SV_Position;
    nointerpolation float4 border_color: COLOR0;
    nointerpolation uint quad_id: TEXCOORD0;
    nointerpolation float4 background_solid: COLOR1;
    nointerpolation float4 background_color0: COLOR2;
    nointerpolation float4 background_color1: COLOR3;
    float4 clip_distance: SV_ClipDistance;
};

struct QuadFragmentInput {
    nointerpolation uint quad_id: TEXCOORD0;
    float4 position: SV_Position;
    nointerpolation float4 border_color: COLOR0;
    nointerpolation float4 background_solid: COLOR1;
    nointerpolation float4 background_color0: COLOR2;
    nointerpolation float4 background_color1: COLOR3;
};

StructuredBuffer<Quad> quads: register(t1);

QuadVertexOutput quad_vertex(uint vertex_id: SV_VertexID, uint quad_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    Quad quad = quads[quad_id];
    float4 device_position = to_device_position(unit_vertex, quad.bounds);
    float4 clip_distance = distance_from_clip_rect(unit_vertex, quad.bounds, quad.content_mask);
    float4 border_color = hsla_to_rgba(quad.border_color);

    GradientColor gradient = prepare_gradient_color(
        quad.background.tag,
        quad.background.color_space,
        quad.background.solid,
        quad.background.colors[0].color,
        quad.background.colors[1].color
    );

    QuadVertexOutput output;
    output.position = device_position;
    output.border_color = border_color;
    output.quad_id = quad_id;
    output.background_solid = gradient.solid;
    output.background_color0 = gradient.color0;
    output.background_color1 = gradient.color1;
    output.clip_distance = clip_distance;
    return output;
}

float4 quad_fragment(QuadFragmentInput input): SV_Target {
    Quad quad = quads[input.quad_id];
    float2 half_size = quad.bounds.size / 2.;
    float2 center = quad.bounds.origin + half_size;
    float2 center_to_point = input.position.xy - center;
    float4 color = gradient_color(quad.background, input.position.xy, quad.bounds,
    input.background_solid, input.background_color0, input.background_color1);

    // Fast path when the quad is not rounded and doesn't have any border.
    if (quad.corner_radii.top_left == 0. && quad.corner_radii.bottom_left == 0. &&
        quad.corner_radii.top_right == 0. &&
        quad.corner_radii.bottom_right == 0. && quad.border_widths.top == 0. &&
        quad.border_widths.left == 0. && quad.border_widths.right == 0. &&
        quad.border_widths.bottom == 0.) {
        return color;
    }

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

    if (border_width != 0.) {
        float inset_distance = distance + border_width;
        // Blend the border on top of the background and then linearly interpolate
        // between the two as we slide inside the background.
        float4 blended_border = over(color, input.border_color);
        color = lerp(blended_border, color, saturate(0.5 - inset_distance));
    }

    return color * float4(1., 1., 1., saturate(0.5 - distance));
}

struct PathVertex {
    float2 xy_position;
    Bounds content_mask;
};

/*
**
**              Paths
**
*/

struct PathSprite {
    Bounds bounds;
    Background color;
};

struct PathVertexOutput {
    float4 position: SV_Position;
    float4 clip_distance: SV_ClipDistance;
    nointerpolation uint sprite_id: TEXCOORD0;
    nointerpolation float4 solid_color: COLOR0;
    nointerpolation float4 color0: COLOR1;
    nointerpolation float4 color1: COLOR2;
};

StructuredBuffer<PathVertex> path_vertices: register(t1);
StructuredBuffer<PathSprite> path_sprites: register(t2);

PathVertexOutput paths_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
    PathVertex v = path_vertices[vertex_id];
    PathSprite sprite = path_sprites[instance_id];

    PathVertexOutput output;
    output.position = to_device_position_impl(v.xy_position);
    output.clip_distance = distance_from_clip_rect_impl(v.xy_position, v.content_mask);
    output.sprite_id = instance_id;

    GradientColor gradient = prepare_gradient_color(
        sprite.color.tag,
        sprite.color.color_space,
        sprite.color.solid,
        sprite.color.colors[0].color,
        sprite.color.colors[1].color
    );

    output.solid_color = gradient.solid;
    output.color0 = gradient.color0;
    output.color1 = gradient.color1;
    return output;
}

float4 paths_fragment(PathVertexOutput input): SV_Target {
    float4 zero = 0.0;
    if (any(input.clip_distance < zero)) {
        return zero;
    }
    
    PathSprite sprite = path_sprites[input.sprite_id];
    Background background = sprite.color;
    float4 color = gradient_color(background, input.position.xy, sprite.bounds,
        input.solid_color, input.color0, input.color1);
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

StructuredBuffer<Underline> underlines: register(t1);

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

StructuredBuffer<MonochromeSprite> mono_sprites: register(t1);

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
**              Polychrome sprites
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

StructuredBuffer<PolychromeSprite> poly_sprites: register(t1);

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
    if ((sprite.grayscale & 0xFFu) != 0u) {
        float3 grayscale = dot(color.rgb, GRAYSCALE_FACTORS);
        color = float4(grayscale, sample.a);
    }
    color.a *= saturate(0.5 - distance);
    return color;
}
