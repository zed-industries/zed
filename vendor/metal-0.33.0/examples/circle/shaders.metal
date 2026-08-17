#include <metal_stdlib>

#include <simd/simd.h>

using namespace metal;

typedef struct {
  float x;
  float y;
}position;

typedef struct {
  float r;
  float g;
  float b;
}color;

typedef struct {
  position p;
  color c;
}AAPLVertex;

struct ColorInOut {
  float4 position[[position]];
  float4 color;
};

vertex ColorInOut vs(constant AAPLVertex * vertex_array[[buffer(0)]], unsigned int vid[[vertex_id]]) {
  ColorInOut out;

  out.position = float4(float2(vertex_array[vid].p.x, vertex_array[vid].p.y), 0.0, 1.0);
  out.color = float4(float3(vertex_array[vid].c.r, vertex_array[vid].c.g, vertex_array[vid].c.b), 1.0);

  return out;
}

fragment float4 ps(ColorInOut in [[stage_in]]) {
  return in.color;
}
