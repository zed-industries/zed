cbuffer GlobalParams : register(b0) {
    float2 global_viewport_size;
    uint global_premultiplied_alpha;
    uint _pad;
};

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

// --- shadows --- //

struct ShadowVertexOutput {
    float4 position: SV_POSITION;
    float4 color: COLOR;
    uint shadow_id: FLAT;
    // float4 clip_distance: SV_CLIPDISTANCE;
};

// struct ShadowFragmentInput {
//   float4 position: SV_POSITION;
//   float4 color: COLOR;
//   uint shadow_id: ID;
// };

struct Shadow {
    uint order;
    float blur_radius;
    Bounds bounds;
    Corners corner_radii;
    Bounds content_mask;
    Hsla color;
};

StructuredBuffer<Shadow> shadows : register(t0);

ShadowVertexOutput shadow_vertex(float2 uint_vertex: POSITION, uint shadow_id: SV_InstanceID) {
    Shadow shadow = shadows[shadow_id];

    float margin = 3.0 * shadow.blur_radius;
    Bounds bounds = shadow.bounds;
    bounds.origin -= margin;
    bounds.size += 2.0 * margin;

    float4 device_position = to_device_position(uint_vertex, bounds);
    // float4 clip_distance = distance_from_clip_rect(uint_vertex, bounds, shadow.content_mask);
    float4 color = hsla_to_rgba(shadow.color);

    ShadowVertexOutput output;
    output.position = device_position;
    output.color = color;
    output.shadow_id = shadow_id;
    // output.clip_distance = clip_distance;
    
    return output;
}

float4 shadow_fragment(ShadowVertexOutput input): SV_TARGET {
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

    // return input.color * float4(1., 1., 1., alpha);
    return float4(1., 1., 1., 1.);
}

/*
**
**              Shadows
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
    float4 position: SV_POSITION;
    float4 background_color: COLOR0;
    float4 border_color: COLOR1;
    uint quad_id: FLAT;
    // float4 clip_distance: SV_CLIPDISTANCE;
};

StructuredBuffer<Quad> quads : register(t1);

QuadVertexOutput quad_vertex(float2 unit_vertex: POSITION, uint quad_id: SV_InstanceID) {
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
    return output;
}

float4 quad_fragment(QuadVertexOutput input): SV_TARGET {
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
