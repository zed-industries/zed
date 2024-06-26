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

float4 to_device_position(float2 unit_vertex, Bounds bounds,
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

    float4 device_position = to_device_position(unit_vertex, bounds, viewport_size);
    float4 clip_distance = distance_from_clip_rect(unit_vertex, bounds, shadow.content_mask.bounds);
    float4 color = hsla_to_rgba(shadow.color);

    ShadowVertexOutput output;
    output.position = device_position;
    output.color = color;
    output.shadow_id = shadow_id;
    output.clip_distance = clip_distance;
    
    return output;
}
