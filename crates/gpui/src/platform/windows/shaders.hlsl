cbuffer GlobalParams : register(b0) {
    float2 global_viewport_size;
    uint global_premultiplied_alpha;
    uint global_pad;
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

float4 to_device_position(float2 unit_vertex, Bounds bounds) {
  float2 position = unit_vertex * bounds.size + bounds.origin;
  float2 device_position =
      position / global_viewport_size * float2(2., -2.) + float2(-1., 1.);
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

// --- shadows --- //

struct ShadowVertexOutput {
    float4 position: SV_POSITION;
    float4 color: COLOR;
    uint shadow_id: FLAT;
    float4 clip_distance: CLIPDISTANCE;
};

struct Shadow {
    uint order;
    float blur_radius;
    Bounds bounds;
    Corners corner_radii;
    Bounds content_mask;
    Hsla color;
};

StructuredBuffer<Shadow> unit_vertices : register(t0);
StructuredBuffer<Shadow> shadows : register(t0);

ShadowVertexOutput shadow_vertex(uint unit_vertex_id : SV_VertexID, uint shadow_id : SV_InstanceID) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
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
